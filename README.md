# 🐧 linux-warez-list

[![Ubuntu 22.04](https://img.shields.io/badge/Ubuntu-22.04%20LTS-E95420?logo=ubuntu&logoColor=white)](https://ubuntu.com/download/desktop)
[![x86-64](https://img.shields.io/badge/arch-x86--64-555)](https://ubuntu.com/)
[![installer v0.13.1](https://img.shields.io/badge/installer-v0.13.1-blue)](CHANGELOG.md)
[![107 packages](https://img.shields.io/badge/catalog-107%20packages-success)](#catalog)
[![Rust TUI](https://img.shields.io/badge/TUI-ratatui-000?logo=rust&logoColor=white)](https://ratatui.rs/)

A curated Ubuntu dev environment: **107** packages, extensions, and tools across system utilities, languages, the CLI, security, desktop apps, and AI tools. Pick exactly what you want in an interactive TUI, or install the whole set with one script.

The long-form inventory, with install commands and notes, lives in [`LINUX_WAREZ_LIST.md`](LINUX_WAREZ_LIST.md). Release history is in [`CHANGELOG.md`](CHANGELOG.md).

---

## Contents

- [⚡ Quick start](#quick-start)
- [🎛️ Interactive installer](#interactive-installer)
- [📦 Catalog](#catalog)
- [🔨 Build from source](#build-from-source)
- [🗂️ Repo contents](#repo-contents)
- [✅ After you install](#after-you-install)
- [🔄 Updating](#updating)

---

<a id="quick-start"></a>

## ⚡ Quick start

```bash
# Interactive TUI (recommended)
sudo ./installer

# Or install everything unattended
sudo bash install-all.sh
```

Ubuntu **22.04 LTS**, x86-64. Run with `sudo` so root-only packages unlock. `install-all.sh` installs only the lowlatency kernel for the running Ubuntu release (`linux-lowlatency-hwe-22.04` or `linux-lowlatency-hwe-24.04`). The TUI lists both — pick the one that matches the machine. Do not install the unversioned `linux-lowlatency` package; on 24.04 it tracks the older 6.8 GA kernel.

[Claude Code](https://docs.anthropic.com/en/docs/claude-code/overview) and the [ChatGPT CLI](https://github.com/openai/codex) are checked only when they are not already installed. If they are already on the machine, they stay unchecked and the installer does not run them unless you check them yourself. When they are checked, they install first.

Each run writes a result log of successful and failed attempts to `/var/log/linux-warez-list/install-YYYYMMDD-HHMMSS.log`. If that directory is not writable, the log goes to `~/.local/state/linux-warez-list/`. Lines are tab-separated: `status`, `name`, `detail`, where `status` is `ok`, `fail`, `error`, `skip`, or `warn`. The footer prints the counts and the log path.

---

<a id="interactive-installer"></a>

## 🎛️ Interactive installer

A Rust TUI built with [ratatui](https://ratatui.rs/). Browse all 107 entries by category, read the description on the right, and toggle exactly what you want. Nothing runs until you confirm.

### Package selection

Packages that need `sudo` are locked and dimmed when the installer is not root.

![Package selection screen](docs/screenshot-select.png)

### Review before installing

`Enter` opens a review grouped by install method. Confirm there, or go back and keep editing.

![Review screen](docs/screenshot-confirm.png)

### Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` or `j` / `k` | Navigate |
| `Space` | Toggle package on/off |
| `A` | Select all unlocked packages |
| `N` | Deselect all (Claude Code and the ChatGPT CLI stay on) |
| `PgUp` / `PgDn` | Jump 10 rows |
| `Enter` | Review selected packages |
| `B` / `Esc` | Back to the list |
| `Q` | Quit |

Rows are colour-coded by install method:

| Colour | Method |
|--------|--------|
| **Cyan** `●` | [`apt`](https://wiki.debian.org/Apt) |
| **Green** `●` | shell script |
| **Magenta** `●` | [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) |
| **Blue** `●` | [`pip`](https://pip.pypa.io/) |
| **Yellow** `●` | [`snap`](https://snapcraft.io/) |

---

<a id="catalog"></a>

## 📦 Catalog

107 entries, in the same order as the TUI. Names link to the upstream project.

<a id="system-tools"></a>

### 🧰 System tools · 10

| Package | What it is | Method |
|---------|------------|--------|
| [build-essential](https://packages.ubuntu.com/jammy/build-essential) | GCC, make, and the usual compile headers | apt |
| [git](https://git-scm.com/) | Distributed version control | apt |
| [gh](https://cli.github.com/) | GitHub CLI | script |
| [Ubuntu 22.04 lowlatency kernel](https://packages.ubuntu.com/jammy/linux-lowlatency-hwe-22.04) | HWE lowlatency kernel for 22.04 | apt |
| [Ubuntu 24.04 lowlatency kernel](https://packages.ubuntu.com/noble/linux-lowlatency-hwe-24.04) | Generic 7.0 HWE kernel plus `preempt=full` (no separate GRUB entry) | apt |
| [GRUB Customizer](https://launchpad.net/grub-customizer) | Graphical GRUB menu editor | script |
| [snapd](https://snapcraft.io/docs/installing-snapd) | Snap daemon, required before snap apps | script |
| [curl](https://curl.se/) | URL transfer tool | apt |
| [wget](https://www.gnu.org/software/wget/) | Non-interactive downloader | apt |
| [unzip](https://infozip.sourceforge.net/) | Extract `.zip` archives | apt |

<a id="languages"></a>

### 💻 Languages & runtimes · 8

| Package | What it is | Method |
|---------|------------|--------|
| [Python 3.10](https://www.python.org/) + pip + venv | Interpreter, pip, venv, and headers | script |
| [Node.js 20](https://nodejs.org/) + npm | NodeSource Node 20 and npm | script |
| [npm](https://www.npmjs.com/) (latest) | Upgrade the global npm | script |
| [Bun](https://bun.sh/) | JavaScript runtime and package manager | script |
| [Rust](https://rustup.rs/) (via rustup) | rustup toolchain in the invoking user's home | script |
| [rust-analyzer](https://rust-analyzer.github.io/) | Rust language server (`rustup component`) | script |
| [GCC](https://gcc.gnu.org/) + G++ + [GDB](https://www.sourceware.org/gdb/) | GNU compiler and debugger | apt |
| [Clang 14](https://clang.llvm.org/) + LLVM + clang-format-12 | Mythos-pinned Clang 14 and clang-format 12 | script |

<a id="cli-tools"></a>

### ⌨️ CLI tools · 28

| Package | What it is | Method |
|---------|------------|--------|
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Fast recursive search (`rg`) | apt |
| [fd](https://github.com/sharkdp/fd) | Friendly `find` (`fd`; apt package is `fd-find`) | script |
| [jq](https://jqlang.org/) | JSON processor | apt |
| [AWS CLI](https://aws.amazon.com/cli/) | AWS CLI v2 (`aws`); apt `awscli` is gone on 24.04 | script |
| [SQLite](https://www.sqlite.org/) | Embedded SQL shell (`sqlite3`) | apt |
| [make](https://www.gnu.org/software/make/) | Build files | apt |
| [Just](https://github.com/casey/just) | Command runner (`justfile`) | cargo |
| [CMake](https://cmake.org/) | Build-system generator; skips apt if `/usr/local/bin/cmake` exists | script |
| [Valgrind](https://valgrind.org/) | Memory and threading profiler | apt |
| [bat](https://github.com/sharkdp/bat) | `cat` with syntax highlighting | apt |
| [Watchman](https://facebook.github.io/watchman/) | File-watching service | apt |
| [FFmpeg](https://ffmpeg.org/) | Audio and video toolkit (apt 4.4.x; do not swap in a PPA build) | apt |
| [ImageMagick](https://imagemagick.org/) | Image conversion and editing | apt |
| [fzf](https://github.com/junegunn/fzf) | Fuzzy finder | apt |
| [rsync](https://rsync.samba.org/) | Incremental file copy | apt |
| [zstd](https://facebook.github.io/zstd/) | Fast compression | apt |
| [detox](https://github.com/dharple/detox) | Sanitize filenames | apt |
| [yt-dlp](https://github.com/yt-dlp/yt-dlp) | Video and audio downloader | apt |
| [htop](https://htop.dev/) | Interactive process viewer | apt |
| [tree](https://gitlab.com/OldManProgrammer/unix-tree) | Directory tree listing | apt |
| [strace](https://strace.io/) | Trace syscalls | apt |
| [ShellCheck](https://www.shellcheck.net/) | Shell script linter | apt |
| [duf](https://github.com/muesli/duf) | Disk usage overview | apt |
| [ncdu](https://dev.yorhel.nl/ncdu) | Disk usage browser | apt |
| [xclip](https://github.com/astrand/xclip) | X11 clipboard from the terminal | apt |
| [pipx](https://pipx.pypa.io/) | Install Python CLIs in isolated envs | apt |
| [lazygit](https://github.com/jesseduffield/lazygit) | Terminal UI for git | script |
| [bottom](https://github.com/ClementTsang/bottom) | System monitor (`btm`) | script |

<a id="containers"></a>

### 🐳 Containers · 1

| Package | What it is | Method |
|---------|------------|--------|
| [Docker](https://docs.docker.com/engine/install/ubuntu/) + Compose | Engine, CLI, and Compose; adds you to the `docker` group | script |

<a id="security"></a>

### 🔐 Security & networking · 11

| Package | What it is | Method |
|---------|------------|--------|
| [nmap](https://nmap.org/) | Network scanner | apt |
| [netcat](https://packages.ubuntu.com/jammy/netcat-openbsd) | TCP/UDP swiss army knife (`nc`, OpenBSD) | apt |
| [aircrack-ng](https://www.aircrack-ng.org/) | Wi-Fi security suite | apt |
| [wifite](https://github.com/kimocoder/wifite2) + [hcxtools](https://github.com/ZerBea/hcxtools) | Automated Wi-Fi audit and handshake tools | apt |
| [Tailscale](https://tailscale.com/) | WireGuard mesh VPN | script |
| [NetBird](https://netbird.io/) | WireGuard overlay network | script |
| [NordVPN](https://nordvpn.com/download/linux/) | NordVPN client | script |
| [OpenSSH](https://www.openssh.com/) | SSH server (`sshd`) | apt |
| [net-tools](https://sourceforge.net/projects/net-tools/) | `ifconfig`, `netstat`, `route` | apt |
| [WireGuard](https://www.wireguard.com/) | `wg` userspace tools | apt |
| [VeraCrypt](https://www.veracrypt.fr/) | Disk encryption | script |

<a id="terminal"></a>

### 🐚 Terminal & shell · 6

| Package | What it is | Method |
|---------|------------|--------|
| [bash-completion](https://github.com/scop/bash-completion) | Programmable tab completion | apt |
| [direnv](https://direnv.net/) | Per-directory environment variables | apt |
| [hstr](https://github.com/dvorka/hstr) | Bash history browser | apt |
| [GNOME Terminal](https://wiki.gnome.org/Apps/Terminal) | Default GNOME terminal | apt |
| [tmux](https://github.com/tmux/tmux) | Terminal multiplexer | apt |
| [Starship](https://starship.rs/) | Cross-shell prompt | cargo |

<a id="python"></a>

### 🐍 Python packages · 7

Installed with pip. [SQLAlchemy](https://www.sqlalchemy.org/) stays at **2.0.19** and [requests](https://requests.readthedocs.io/) stays at **2.31.0** so they match the Mythos pin.

| Package | What it is | Method |
|---------|------------|--------|
| [pytest](https://docs.pytest.org/) + pytest-mock + pytest-cov | Test runner, mocks, and coverage | pip |
| [SQLAlchemy](https://www.sqlalchemy.org/) `==2.0.19` | SQL toolkit and ORM | pip |
| [Pydantic](https://docs.pydantic.dev/) + pydantic-settings | Data validation and settings | pip |
| [black](https://black.readthedocs.io/) | Code formatter | pip |
| [flake8](https://flake8.pycqa.org/) | Style linter | pip |
| [mypy](https://mypy-lang.org/) | Static type checker | pip |
| [requests](https://requests.readthedocs.io/) `==2.31.0` | HTTP client | pip |

<a id="fonts"></a>

### 🔤 Fonts · 3

| Package | What it is | Method |
|---------|------------|--------|
| [Liberation](https://github.com/liberationfonts/liberation-fonts) | Metric-compatible core fonts | apt |
| [DejaVu](https://dejavu-fonts.github.io/) | Broad Unicode coverage | apt |
| [FiraCode Nerd Font](https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/FiraCode) | Patched monospace; set as the GNOME monospace font | script |

<a id="snaps"></a>

### 📸 Snap applications · 2

Needs [snapd](#system-tools). The headless script asks before installing snaps.

| Package | What it is | Method |
|---------|------------|--------|
| [Notion](https://www.notion.com/) | Notes and docs | snap |
| [NordPass](https://nordpass.com/) | Password manager | snap |

<a id="gnome"></a>

### 🧩 GNOME Shell extensions · 12

| Extension | What it is | Method |
|-----------|------------|--------|
| [Extension Manager](https://github.com/mjakeman/extension-manager) | Browse and install extensions from a GUI | apt |
| [Ubuntu Dock](https://github.com/micheleg/dash-to-dock) | Ubuntu's dock | script |
| [Ubuntu AppIndicators](https://github.com/ubuntu/gnome-shell-extension-appindicator) | Tray icons in the top bar | script |
| [Desktop Icons NG](https://gitlab.com/rastersoft/desktop-icons-ng) | Icons on the desktop | script |
| [Just Perfection](https://github.com/justperfection/gnome-shell-just-perfection) | Hide and tweak Shell UI | script |
| [OpenWeather](https://gitlab.com/jenslody/gnome-shell-extension-openweather) | Weather in the top bar | script |
| [TopHat](https://github.com/fflewddur/tophat) | CPU, memory, and disk in the top bar | script |
| [Freon](https://github.com/UshakovVasilii/gnome-shell-extension-freon) | Temperature sensors | script |
| [Net Speed Simplified](https://github.com/prateekmedia/netspeedsimplified) | Live up/down throughput | script |
| Audio Selector | Switch output and input devices | script |
| [Bluetooth Quick Connect](https://github.com/bjarosze/gnome-bluetooth-quick-connect) | Connect paired devices from the panel | script |
| Simple Message | Short on-screen messages | script |

<a id="desktop"></a>

### 🖥️ Desktop applications · 12

| Package | What it is | Method |
|---------|------------|--------|
| [Spotify](https://www.spotify.com/download/linux/) | Music client (official apt repo) | script |
| [Discord](https://discord.com/download) | Chat client (`.deb`, not the snap) | script |
| [Slack](https://slack.com/downloads/linux) | Team chat (snap; removes leftover `slack-desktop` .deb) | script |
| [SimpleScreenRecorder](https://www.maartenbaert.be/simplescreenrecorder/) | Screen recorder | apt |
| [NoMachine](https://www.nomachine.com/) | Remote desktop | script |
| [GNOME Tweaks](https://wiki.gnome.org/Apps/Tweaks) | Extra desktop settings | apt |
| [Solaar](https://pwr-solaar.github.io/Solaar/) | Logitech device manager | apt |
| [Meld](https://meldmerge.org/) | Visual diff and merge | apt |
| [Peek](https://github.com/phw/peek) | Simple GIF screen recorder | apt |
| [Google Chrome](https://www.google.com/chrome/) | Browser | script |
| [Signal](https://signal.org/download/) | Private messenger | script |
| [Obsidian](https://obsidian.md/) | Local-first notes | script |

<a id="ai-tools"></a>

### 🤖 AI tools · 7

| Tool | What it is | Method |
|------|------------|--------|
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code/overview) | Claude CLI; always installed, always selected | script |
| [ChatGPT CLI](https://github.com/openai/codex) | Codex CLI in `~/.local/bin`; always installed, always selected | script |
| [Cursor](https://cursor.com/) | Cursor editor (Linux `.deb`) | script |
| [Claude](https://claude.ai/download) | Claude desktop | script |
| [ChatGPT](https://chatgpt.com/) | ChatGPT desktop | script |
| [jcodemunch-mcp](https://pypi.org/project/jcodemunch-mcp/) | Code-search MCP server | script |
| [memory-mcp](https://github.com/dylansparks/memory-mcp) | Local memory MCP server | script |

Editor extensions and cloud MCP servers (Gmail, Calendar, Drive, Notion) are documented in [`LINUX_WAREZ_LIST.md`](LINUX_WAREZ_LIST.md). They are not installer entries.

---

<a id="build-from-source"></a>

## 🔨 Build from source

Requires [Rust](https://www.rust-lang.org/) (stable).

```bash
cd installer-tui
cargo build --release
sudo ./target/release/installer-tui
```

Refresh the committed `installer` binary after changing `installer-tui/src/main.rs`:

```bash
cd installer-tui && cargo build --release
/usr/bin/cp target/release/installer-tui ../installer
chmod +x ../installer
```

Use `/usr/bin/cp`. A shell `cp` alias to `cp -i` will stop and ask.

`./installer --dump-json` prints the live catalog as JSON and exits. [Screenshot generation](docs/gen_screenshots.py) reads that output, so package changes do not need a Python edit.

---

<a id="repo-contents"></a>

## 🗂️ Repo contents

| File | Description |
|------|-------------|
| [`installer`](installer) | Pre-built TUI binary (Linux x86-64) |
| [`install-all.sh`](install-all.sh) | Headless install of the full catalog |
| [`LINUX_WAREZ_LIST.md`](LINUX_WAREZ_LIST.md) | Full inventory: descriptions, commands, and manual extras |
| [`CHANGELOG.md`](CHANGELOG.md) | Versioned release notes |
| [`gather-software-inventory.sh`](gather-software-inventory.sh) | Dump a JSON snapshot of what is installed |
| [`software-inventory.json`](software-inventory.json) | Snapshot from this machine |
| [`installer-tui/`](installer-tui/) | Rust source for the TUI |

---

<a id="after-you-install"></a>

## ✅ After you install

1. **Docker** — log out and back in so the `docker` group applies. Then try `docker run hello-world`.
2. **GitHub CLI** — `gh auth login`.
3. **Starship** — add `eval "$(starship init bash)"` to `~/.bashrc`.
4. **direnv** — add `eval "$(direnv hook bash)"` to `~/.bashrc`.
5. **hstr** — add `eval "$(hstr --show-configuration)"` to `~/.bashrc`.
6. **FiraCode Nerd Font** — the installer sets it as the system monospace font.
7. **Tailscale** — `sudo tailscale up`, then open the URL it prints.
8. **NordVPN** — `nordvpn login`, then `nordvpn connect`.
9. **Kernel** — reboot if you installed a lowlatency kernel. Check with `uname -r`.

---

<a id="updating"></a>

## 🔄 Updating

```bash
# apt packages
sudo apt update && sudo apt upgrade

# Rust tools
cargo install --force starship just

# Python packages (leave the Mythos pins alone)
pip install --upgrade black flake8 mypy pytest

# Snap packages
sudo snap refresh
```

Leave `SQLAlchemy==2.0.19` and `requests==2.31.0` pinned. Leave FFmpeg on the Ubuntu apt build.

---

**Last updated:** 2026-09-26 · [v0.13.1](CHANGELOG.md)
