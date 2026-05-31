# Changelog

## [0.9.0] — 2026-05-30

### New Packages

#### Rust Tools

| Package | Install method | Notes |
|---------|---------------|-------|
| `rust-analyzer  (LSP)` | `rustup component add rust-analyzer` (runs as real user) | Official Rust language server; installed as a rustup component so it stays in sync with the active toolchain. Dependency on `Rust  (via rustup)` declared — the TUI locks it when Rust is not selected or installed. Detected via `which rust-analyzer`. |

---

## [0.8.0] — 2026-05-30

This is the first formally versioned release. It introduces 30+ new packages
across four new categories, a complete GNOME Shell extensions section, a new
Claude & AI Tools section, a security-hardened installer, and a major
architectural change to the screenshot generator that eliminates a longstanding
sync drift problem.

---

### New Packages

#### Languages & Runtimes

| Package | Install method | Notes |
|---------|---------------|-------|
| `npm  (latest)` | Script → `npm install -g npm@latest` | Upgrades the bundled npm to latest stable independently of the Node.js LTS cadence |
| `Bun` | Script → `curl bun.sh/install \| bash` | Fast all-in-one JS runtime/bundler/test runner; installs into real user's home, not root's |

#### Security & Networking

| Package | Install method | Notes |
|---------|---------------|-------|
| `NetBird` | Script → APT repo | Open-source WireGuard overlay with optional self-hosted control plane; complementary to Tailscale |

#### Desktop Applications

| Package | Install method | Notes |
|---------|---------------|-------|
| `NoMachine` | Script → vendor .deb (fetched from nomachine.com) | High-performance NX remote desktop; URL scraped dynamically at install time |
| `SimpleScreenRecorder` | `apt install simplescreenrecorder` | Screen recorder for MP4/MKV/WebM output |
| `VeraCrypt` | `apt install veracrypt` | Disk encryption (may not be in default repos; guarded with `|| warn`) |
| `GRUB Customizer` | Script → `add-apt-repository ppa:danielrichter2007/...` | GUI for GRUB2 boot entry management |
| `Solaar` | `apt install solaar` | Logitech Unifying/Bolt device manager |
| `Meld` | `apt install meld` | Visual diff and three-way merge tool; integrates with git difftool |
| `Peek` | `apt install peek` | Lightweight GIF/MP4 screen recorder for quick demos |
| `Obsidian` | Script → GitHub Releases latest .deb | Local-first Markdown knowledge base; .deb URL fetched dynamically |

#### GNOME Shell Extensions (new category)

All extensions require `gnome-shell-extension-manager` (also new; `apt` install)
as a declared dependency via `requires_pkg`. The extensions that use
`gnome-extensions enable` run without root. Extensions sourced from the GNOME
extensions catalog use `gnome-extensions-cli` (`pip3 install --user
gnome-extensions-cli`) installed as the real user.

| Package | UUID / install method |
|---------|----------------------|
| GNOME Shell Extension Manager | `apt install gnome-shell-extension-manager` |
| Ubuntu Dock | `gnome-extensions enable ubuntu-dock@ubuntu.com` |
| Ubuntu AppIndicators | `gnome-extensions enable ubuntu-appindicators@ubuntu.com` |
| Desktop Icons NG (DING) | `gnome-extensions enable ding@rastersoft.com` |
| Just Perfection | `gext install just-perfection-desktop@just-perfection` |
| OpenWeather | `gext install openweather-extension@jenslody.de` |
| TopHat | `gext install tophat@fflewddur.github.io` |
| Freon | `gext install freon@UshakovVasilii_Github.yahoo.com` |
| Net Speed Simplified | `gext install netspeedsimplified@prateekmedia.extension` |
| Audio Selector | `gext install audio-selector@harald65.simon.gmail.com` |
| Bluetooth Quick Connect | `gext install bluetooth-quick-connect@bjarosze.gmail.com` |
| Simple Message | `gext install simple-message@freddez` |

#### Claude & AI Tools (new category)

| Package | Install method | Notes |
|---------|---------------|-------|
| `Claude Code  (CLI)` | Script → `npm install -g @anthropic-ai/claude-code` | Anthropic's agentic CLI; requires Node.js |
| `jcodemunch-mcp` | Script → `pip3 install jcodemunch-mcp` | Token-efficient MCP server for AST-level code navigation in Claude Code |
| `memory-mcp  (local)` | Script → `pip3 install mcp` + curl `memory_mcp.py` from GitHub | Local persistent key-value memory MCP server for Claude Desktop |

---

### Bug Fixes

These were identified during a structured multi-angle code review on 2026-05-30.

#### `install-all.sh` — `bottom` .deb left broken on dependency failure

**File:** `install-all.sh` line 294  
**Before:**
```bash
dpkg -i /tmp/bottom.deb || warn "Failed to install bottom .deb"
```
**After:**
```bash
dpkg -i /tmp/bottom.deb || apt install -f -y || warn "Failed to install bottom .deb"
```
**Why:** `dpkg -i` does not resolve unmet dependencies. If `bottom`'s .deb has
deps not yet present, `dpkg -i` fails, the `|| warn` fires, and `bottom` is left
in a half-configured broken state with no recovery. The NoMachine install (line
580) already used the correct `dpkg -i … || apt install -f -y` pattern; `bottom`
was inconsistent. The fix adds the same `apt install -f -y` fallback so apt
auto-resolves any missing dependencies before falling through to the warn.

#### `install-all.sh` — `REAL_HOME` computed via `eval` (shell injection)

**File:** `install-all.sh` line 34  
**Before:**
```bash
REAL_HOME=$(eval echo "~${REAL_USER}")
```
**After:**
```bash
REAL_HOME=$(getent passwd "${REAL_USER}" | cut -d: -f6)
```
**Why:** `eval echo "~${REAL_USER}"` expands shell metacharacters in `REAL_USER`.
If `SUDO_USER` were set to a value containing `$(…)` or backticks before the
script runs, those expressions would execute as root. `getent passwd` looks up
the home directory from the system user database without any shell evaluation,
eliminating the injection surface. Both produce identical output for normal
usernames.

#### `installer-tui/src/main.rs` — `get_real_home()` used the same `eval` pattern

**File:** `main.rs` lines 2437–2448 (the `get_real_home` function)  
**Before:**
```rust
Command::new("sh")
    .args(["-c", &format!("eval echo ~{}", u)])
    .output()
```
**After:**
```rust
Command::new("getent")
    .args(["passwd", &u])
    .output()
    .ok()
    .and_then(|o| {
        String::from_utf8_lossy(&o.stdout)
            .split(':')
            .nth(5)
            .map(|s| s.trim().to_string())
    })
```
**Why:** Same injection risk as the shell script version — running the TUI as
root via `sudo ./installer` while `SUDO_USER` contains metacharacters would
execute arbitrary commands as root. Replaced with a direct `getent passwd`
invocation. The fifth colon-delimited field of a passwd entry is always the home
directory; no shell is involved.

#### `installer-tui/src/main.rs` — `cmd_short()` sliced `&str` by byte index

**File:** `main.rs` lines 1694–1698 (the `cmd_short` function)  
**Before:**
```rust
if first.len() > 62 {
    format!("{}...", &first[..59])
}
```
**After:**
```rust
if first.len() > 62 {
    let truncated: String = first.chars().take(59).collect();
    format!("{}...", truncated)
}
```
**Why:** `str::len()` returns the byte length of a UTF-8 string; `&str[..59]`
slices by byte index. If any multi-byte UTF-8 codepoint (e.g. a non-ASCII quote
or Unicode character) straddles byte position 59, Rust panics at runtime with
"byte index N is not a char boundary". All current script strings happen to be
ASCII-only so this was a latent bug. `chars().take(59)` iterates by Unicode
scalar values, which is always safe.

#### `installer-tui/src/main.rs` — `push_sep` closure mutated a dropped temporary

**File:** `main.rs` lines 2326–2399 (warning block in `render_confirm`)  
**Before:**
```rust
let mut warned = false;
let push_sep = |ls: &mut Vec<Line>, first: &mut bool| {
    if !*first { return; }
    *first = false;   // writes to a dropped temporary, not `warned`
    ls.push(…sep…);
};
if app.has_selected_cargo() … {
    push_sep(&mut lines, &mut !warned);  // &mut !warned = mutable ref to temp
    warned = true;
}
```
**After:**
```rust
let mut sep_pushed = false;
let mut push_sep_once = |ls: &mut Vec<Line>| {
    if sep_pushed { return; }
    sep_pushed = true;
    ls.push(…sep…);
};
if app.has_selected_cargo() … {
    push_sep_once(&mut lines);
}
```
**Why:** `&mut !warned` creates a temporary `bool` on the stack. The closure's
`*first = false` assignment wrote back to that temporary, which was immediately
dropped after the call. The only reason duplicate separators didn't appear was
because `warned = true` was set manually after every call site — the closure's
mutation was dead code. If a fourth warning block were added without the manual
`warned = true`, two separators would appear. The rewrite makes the guard
(`sep_pushed`) owned by the closure itself so the logic is self-contained and
can't be broken by future additions.

---

### Architecture Changes

#### `installer --dump-json`: live package data export

**Files affected:** `installer-tui/src/main.rs` (new `dump_json()` function and
`--dump-json` arg check in `main()`), `installer` (rebuilt binary)

When invoked as `./installer --dump-json`, the binary prints all package data as
a JSON array to stdout and exits without launching the TUI. Each element has:

```json
{
  "category":         "CLI Tools",
  "name":             "fzf",
  "description":      "General-purpose interactive fuzzy finder…",
  "cmd_type":         "apt",
  "cmd_value":        ["fzf"],
  "requires_root":    true,
  "default_selected": false
}
```

`cmd_value` is a JSON array for `apt` and `pip` types, or a JSON string for
`sh`, `cargo`, and `snap` types.

This is used by `docs/gen_screenshots.py` (see below) and can also be used by
any future tooling (diff scripts, README generators, CI checks) that needs to
introspect the package list without parsing Rust source.

**JSON escaping in `dump_json()`:** The function manually serialises JSON
without `serde` (no new dependency). Backslashes, double-quotes, and newlines
in description and script strings are escaped with `replace()` chains before
interpolation into the format string. If a future script string contains a
literal `\n` or `\"` that should render as those two characters rather than as
escape sequences, the current escaping will misrepresent it — but no such
strings exist today.

#### `docs/gen_screenshots.py`: eliminated static package list

**Before:** `gen_screenshots.py` maintained a hand-written `PKGS` list and
hardcoded package counts (`"8/84 selected"`, `" Packages (84 total) "`). Every
time a package was added to `main.rs`, both files had to be updated in sync.
Drift between the two was the norm: the generator was consistently missing the
most recently added packages.

**After:** At module load, the script invokes `./installer --dump-json` via
`subprocess.run` and parses the JSON. All package counts, category names, and
package properties (name, cmd_type, requires_root, default_selected,
description) come from the live binary output. The `PKGS` static list is gone.

The binary discovery order (in `_find_installer()`):
1. `./installer` (pre-built binary at repo root — normal case)
2. `installer-tui/target/release/installer-tui`
3. `installer-tui/target/debug/installer-tui`

If none is found, a `FileNotFoundError` is raised with instructions.

The right-panel description for the cursor package in the select screen mock-up
is now generated by word-wrapping the real package description from Rust
(`textwrap.wrap`), not a hand-copied snippet. The confirm screen example set is
auto-derived: all `default_selected: true` packages plus one representative
package per remaining install type (`_CONFIRM_EXTRAS`).

The `left_row()` helper function (defined but never called — its logic was
duplicated inline in `build_select()`) was removed.

**Consequence for the sync rule:** `docs/gen_screenshots.py` no longer needs to
be touched when adding, removing, or renaming packages. The sync rule drops it
from the mandatory update list. The generator only needs attention if the HTML
rendering logic itself changes (CSS, layout, new panel types).

---

### Version Bump

`installer-tui/Cargo.toml`: `0.1.0` → `0.8.0`

The jump from 0.1.0 reflects that the codebase has accumulated significant
functionality across several unversioned sessions before formal versioning was
introduced.
