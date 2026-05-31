# linux-warez-list -- Agent Rules

Rules and conventions for AI agents working in this repository.
Read this file in full before making changes.

## What This Repo Does

Curated Ubuntu 22.04 LTS (x86-64) developer environment installer. Two install
paths that must stay in sync:

1. **TUI installer** (`installer-tui/src/main.rs`) -- interactive Rust TUI built
   with ratatui. Users select individual packages and confirm before anything
   runs.
2. **Headless script** (`install-all.sh`) -- bash script that installs
   everything in one shot (one interactive snap prompt).

## File Map

| File | Purpose | When to update |
|------|---------|----------------|
| `installer-tui/src/main.rs` | TUI source -- package DB + UI + install logic | Any package change |
| `install-all.sh` | Headless bash installer | Any package change |
| `README.md` | User docs, package count, package tables | Any package change |
| `LINUX_WAREZ_LIST.md` | Extended human-readable inventory | Any package change |
| `docs/gen_screenshots.py` | Generates HTML mocks for README screenshots | HTML/layout changes only (package data is auto-derived from `--dump-json`) |
| `installer` | Pre-built TUI binary (committed) | After any `main.rs` change |
| `gather-software-inventory.sh` | Dumps JSON of installed software | Rarely |
| `software-inventory.json` | Snapshot output | Rarely |
| `installer-tui/Cargo.toml` | Rust dependencies and version | Dep changes or version bumps |
| `CHANGELOG.md` | Versioned release notes | Every release |

## The Sync Rule (Critical)

The TUI and the shell script are **two hand-maintained mirrors**. They are not
generated from a single source. When changing a package, you must update **all**
of these:

1. `installer-tui/src/main.rs` -- the `build_data()` function
2. `install-all.sh` -- the matching section
3. `README.md` -- the package table in the matching category
4. `LINUX_WAREZ_LIST.md` -- the matching entry

`docs/gen_screenshots.py` **no longer needs manual updates for package
changes.** Since v0.8.0 it calls `./installer --dump-json` at runtime to
derive all package data. Only touch it if the HTML rendering logic changes.

After editing `main.rs`, rebuild and copy the binary:

```bash
cd installer-tui && cargo build --release
/usr/bin/cp target/release/installer-tui ../installer
chmod +x ../installer
```

Use `/usr/bin/cp` because the shell may alias `cp` to `cp -i` which prompts.

## TUI Architecture (main.rs)

### Types

| Type | Purpose |
|------|---------|
| `InstallCmd` | Enum: `Apt(&[&str])`, `Script(&str)`, `Cargo(&str)`, `Pip(&[&str])`, `Snap(&str)` |
| `Package` | Name, description, `InstallCmd`, default selected, requires_root |
| `Entry` | `Category(&str)` or `Pkg(usize)` -- flat list for the UI |
| `DataBuilder` | Builder pattern: `cat()` adds category headers, `pkg()` adds packages |
| `App` | TUI state: packages, entries, cursor, screen, scroll, is_root |
| `Screen` | `Select` (main list) or `Confirm` (review before install) |

### Adding a Package

Call `b.pkg(...)` inside `build_data()` under the correct category:

```rust
b.pkg(
    "display-name",                    // shown in the TUI list
    "Long description for the right panel. Keep it factual. \
     Wrap long lines with trailing backslash continuation.",
    InstallCmd::Apt(&["pkg-name"]),    // or Script/Cargo/Pip/Snap
    false,                             // selected by default
    true,                              // requires root (sudo)
);
```

- Categories are created with `b.cat("  Category Name");` (leading spaces
  are intentional for visual indent).
- Package order within a category matches the order they appear in the UI.
- `requires_root: true` means the package is locked/dimmed when run without sudo.
- Descriptions are `&'static str` -- use `\` line continuation, not string
  concatenation.

### Adding a Category

```rust
b.cat("  New Category Name");
```

Add it between existing categories in `build_data()`. Since v0.8.0,
`docs/gen_screenshots.py` derives categories automatically from
`./installer --dump-json` — no Python edits needed.

### Installation Logic

`run_install()` iterates selected packages and dispatches by `InstallCmd`:

- `Apt` -> `apt install -y <packages>`
- `Script` -> `sh -c '<script>'`
- `Cargo` -> `sudo -u $REAL_USER ~/.cargo/bin/cargo install <name>`
- `Pip` -> `pip3 install <packages>`
- `Snap` -> `snap install <name>`

Cargo installs run as `SUDO_USER` (not root) so tools land in the real user's
`~/.cargo/bin/`.

## Shell Script Architecture (install-all.sh)

### Structure

```
Shebang + compatibility header
Helper functions (log, warn, error)
Root check
REAL_USER / REAL_HOME detection
apt update

Sections (in order):
  SYSTEM PACKAGES   -- build-essential, git, GitHub CLI, linux-lowlatency
  LANGUAGES         -- Python 3.10, Node 20, Rust (rustup), GCC, Clang 14
  CLI TOOLS         -- ripgrep, fd, cmake, bat, ffmpeg, fzf, etc.
  CONTAINERS        -- Docker
  SECURITY          -- nmap, netcat, aircrack-ng, wifite, Tailscale, NetBird, NordVPN
  TERMINAL          -- bash-completion, GNOME Terminal
  PYTHON PACKAGES   -- pytest, SQLAlchemy==2.0.19, requests==2.31.0, etc.
  RUST TOOLS        -- starship, just (via cargo, as REAL_USER)
  FONTS             -- liberation, dejavu, FiraCode Nerd Font (manual note)
  SHELL CONFIG      -- direnv + starship hooks in ~/.bashrc
  SNAP APPS         -- interactive prompt, then Discord/Slack/Spotify/Notion/NordPass
  DESKTOP APPS      -- SimpleScreenRecorder, VeraCrypt, GNOME tools, GRUB Customizer,
                       Solaar, Chrome, Signal, Claude, NoMachine

CLEANUP            -- apt autoremove, autoclean
VERIFICATION       -- loop checking installed tools
POST-SETUP         -- reminders (Docker, kernel reboot, Mythos installer)
```

### Conventions

- `set -euo pipefail` -- script aborts on error, undefined vars, pipe failures.
- `REAL_USER` / `REAL_HOME` -- used for Rust/cargo installs and shell config
  so files go to the invoking user's home, not root's.
- Idempotency -- most installs are guarded with `command -v`, `dpkg -l`, or
  similar checks to avoid errors on re-runs.
- GPG keyrings use `gpg --dearmor --yes` to safely overwrite on re-runs.
- `dpkg -i ... || apt install -f -y` pattern for .deb installs (handles
  missing dependencies).
- Emojis in section headers and log/warn/error functions are intentional. Do
  not remove them.

## Mythos AV Stack Compatibility

This machine also runs the Mythos autonomous vehicle stack (`~/mythos`). Some
packages have version pins that must not conflict:

| Constraint | Why | Where documented |
|------------|-----|------------------|
| Clang 14 + clang-format-12 | Mythos pins Clang 14 and uses clang-format-12 for `./mythos format` | Both installers |
| CMake 3.18.1 from source at `/usr/local/bin` | Mythos builds cmake from source; skip apt install if present | Script checks path; TUI uses Script cmd |
| SQLAlchemy==2.0.19 | Mythos `requirements.txt` pin | Pip install in both |
| requests==2.31.0 | Mythos `requirements.txt` pin | Pip install in both |
| FFmpeg from apt only (4.4.x) | PPA/snap versions ship different libavcodec SO versions | Description notes |
| linux-lowlatency kernel | Required on production vessels | Description notes |

When adding or updating Python packages, check `~/mythos/third_party/rules_python/requirements.txt`
for version conflicts.

## Package Count

README and TUI both reference the total package count (currently 101). Update
the count in:

- `README.md` — the total count in the header and anywhere it appears in prose

The TUI and `docs/gen_screenshots.py` both derive their counts dynamically
(TUI from `build_data()`, generator from `./installer --dump-json`) so neither
needs a manual count update.

## `--dump-json`: Live Package Data Export

Since v0.8.0, the installer binary supports a `--dump-json` flag:

```bash
./installer --dump-json
```

Prints all package data as a JSON array to stdout and exits without launching
the TUI. Each element:

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

`cmd_value` is a JSON array for `apt`/`pip`, or a JSON string for
`sh`/`cargo`/`snap`.

Uses: `docs/gen_screenshots.py` calls this at runtime. Any future tooling
(CI diff checks, README table generators, package count validation) should
use this rather than parsing Rust source directly.

**Note:** JSON is serialised manually without `serde`. The `dump_json()`
function in `main.rs` escapes `\`, `"`, and `\n` via `replace()` chains.
If you add a script string containing a literal two-character sequence `\"`,
it will be double-escaped. Avoid such strings or update the escaping logic.

## Versioning

The version is defined in one place:

```
installer-tui/Cargo.toml  →  version = "X.Y.Z"
```

The TUI title bar reads it at compile time via `env!("CARGO_PKG_VERSION")` in
`render_title()`. There is no separate version string to keep in sync — bumping
`Cargo.toml` and rebuilding the binary is the entire update.

### When to bump

| Change type | Version component | Example |
|-------------|-------------------|---------|
| New packages, new categories, major features | Minor (`Y`) | `0.8.0 → 0.9.0` |
| Bug fixes, security patches, doc-only changes | Patch (`Z`) | `0.8.0 → 0.8.1` |
| Breaking changes to CLI flags or JSON schema | Major (`X`) | `0.8.0 → 1.0.0` |

### How to bump

1. Edit `installer-tui/Cargo.toml`:
   ```toml
   version = "0.9.0"
   ```
2. Rebuild and copy the binary:
   ```bash
   cd installer-tui && cargo build --release
   /usr/bin/cp target/release/installer-tui ../installer && chmod +x ../installer
   ```
3. Add a `## [X.Y.Z] — YYYY-MM-DD` section to `CHANGELOG.md` documenting
   what changed and why (see existing entries for the expected level of detail).

The TUI title and `--dump-json` output both reflect the new version automatically
after the rebuild.

## Build

Requires Rust (stable). No other build system.

```bash
cd installer-tui
cargo build --release
```

Copy the binary to repo root after building:

```bash
/usr/bin/cp target/release/installer-tui ../installer
chmod +x ../installer
```

## Style

- Rust: standard `rustfmt` formatting.
- Bash: 2-space indent, functions use `log`/`warn`/`error` helpers.
- Descriptions in the TUI should be factual and practical -- what the tool does,
  why you'd want it, and a concrete example where relevant.
- Keep emoji icons in `install-all.sh` section headers and helper functions.

## Common Mistakes to Avoid

- **Forgetting to update one of the mirrors.** Every package change touches at
  least 4 files. Use the checklist in "The Sync Rule" above.
- **Not rebuilding the binary.** The committed `installer` binary must match
  `main.rs`. Always rebuild after source changes.
- **Using `cp` instead of `/usr/bin/cp`.** The shell aliases `cp` to `cp -i`
  which blocks on interactive confirmation.
- **Installing pip packages without version pins.** Check Mythos compatibility
  before adding or upgrading any pip package.
- **Breaking `set -euo pipefail`.** Every command in `install-all.sh` must
  handle failure gracefully or the script aborts. Use `|| true`, `|| warn`,
  or conditional checks as appropriate.
- **Removing emoji from install-all.sh.** The icons in section headers and
  log/warn/error are intentional and should be preserved.
- **Adding packages that require interactive input.** Both the TUI and the
  script run non-interactively (apt uses `-y`, add-apt-repository uses `-y`,
  etc.). Any new Script command must run without prompts.
