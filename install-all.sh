#!/bin/bash
# Complete Ubuntu dev environment setup script
# Run with: sudo bash install-all.sh
# Root privileges required for most operations
#
# COMPATIBILITY: Tested on Ubuntu 22.04 LTS (x86-64).
# If this machine also runs the Mythos AV stack, see the "Mythos compatibility"
# comments throughout this script for packages that must not conflict with
# Mythos pinned versions (CMake 3.18.1, OpenSSL 1.1.1f, Clang 14, etc.).

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() { echo -e "${GREEN}✓${NC} $1"; }
warn() { echo -e "${YELLOW}⚠${NC} $1"; }
error() { echo -e "${RED}✗${NC} $1"; }

echo "================================"
echo "Ubuntu Dev Environment Setup"
echo "================================"
echo ""

# Check if running with sudo
if [[ $EUID -ne 0 ]]; then
   error "This script must be run as root (use sudo)"
   exit 1
fi

REAL_USER="${SUDO_USER:-$USER}"
REAL_HOME=$(eval echo "~${REAL_USER}")

# Update package list
log "Updating package list..."
apt update

# ===== SYSTEM PACKAGES =====
echo ""
echo "📦 Installing system packages..."

log "Installing build tools..."
apt install -y build-essential

log "Installing Git..."
apt install -y git

log "Installing low-latency kernel..."
apt install -y linux-lowlatency
warn "Reboot required after installation for low-latency kernel to take effect"
warn "After reboot, verify with: uname -r (should show *-lowlatency)"

log "Installing snapd (required for snap packages)..."
apt install -y snapd
snap install core 2>/dev/null || warn "snap core install failed (may already be present)"

log "Installing curl..."
apt install -y curl

log "Installing wget..."
apt install -y wget

log "Installing unzip..."
apt install -y unzip

log "Installing GitHub CLI..."
if ! command -v gh &> /dev/null; then
  (type -p wget >/dev/null || apt install -y wget) \
    && mkdir -p /etc/apt/keyrings \
    && wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg \
      | tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
    && chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
      | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt update \
    && apt install -y gh
else
  log "GitHub CLI already installed: $(gh --version | head -1)"
fi

# ===== PROGRAMMING LANGUAGES =====
echo ""
echo "🔧 Installing programming languages..."

log "Installing Python 3.10 and dev tools..."
apt install -y python3.10 python3.10-venv python3.10-dev python3-pip

log "Installing GCC and development headers..."
apt install -y gcc g++ gdb

# Mythos compatibility: Mythos pins Clang 14 and clang-format-12 via its own
# install.py. Installing the bare 'clang' metapackage on 22.04 resolves to
# clang-14, which is correct. Do NOT install a different clang version (e.g.
# clang-15, clang-16) as it may change what /usr/bin/clang resolves to and
# break the Mythos .bazelrc (CC=clang). We also install clang-format-12
# explicitly since Mythos formatting depends on it.
log "Installing Clang 14 and LLVM..."
apt install -y clang clang-format-12 llvm llvm-dev

# ===== NODE.JS (if not already installed) =====
if ! command -v node &> /dev/null; then
  log "Installing Node.js 20.x..."
  curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
  apt install -y nodejs
else
  log "Node.js already installed: $(node --version)"
fi

# ===== RUST (if not already installed) =====
if ! command -v rustc &> /dev/null && ! sudo -u "${REAL_USER}" bash -c 'command -v rustc' &> /dev/null; then
  log "Installing Rust for ${REAL_USER}..."
  sudo -u "${REAL_USER}" bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
else
  log "Rust already installed"
fi

# ===== CLI DEVELOPMENT TOOLS =====
echo ""
echo "⚙️  Installing CLI tools..."

log "Installing ripgrep..."
apt install -y ripgrep

log "Installing fd..."
apt install -y fd-find
if ! command -v fd &> /dev/null; then
  ln -sf /usr/bin/fdfind /usr/local/bin/fd
fi

log "Installing direnv..."
apt install -y direnv

log "Installing jq..."
apt install -y jq

log "Installing SQLite3..."
apt install -y sqlite3

log "Installing Make..."
apt install -y make

# Mythos compatibility: Mythos install.py installs CMake 3.18.1 from source to
# /usr/local/bin/cmake, which takes PATH precedence over the apt version.
# Installing CMake via apt here is safe (goes to /usr/bin/cmake) and will NOT
# shadow the Mythos version because /usr/local/bin comes first in PATH. We
# skip this if the Mythos CMake is already present to avoid confusion.
if [ -x /usr/local/bin/cmake ]; then
  warn "CMake already installed at /usr/local/bin/cmake ($(cmake --version | head -1)), skipping apt cmake"
else
  log "Installing CMake (apt)..."
  apt install -y cmake
fi

log "Installing Valgrind..."
apt install -y valgrind

log "Installing Watchman..."
apt install -y watchman 2>/dev/null || {
  warn "Watchman not in repos, skipping (optional)"
}

log "Installing bat..."
apt install -y bat
if ! command -v bat &> /dev/null; then
  ln -sf /usr/bin/batcat /usr/local/bin/bat
fi

# Mythos compatibility: Mythos uses FFmpeg libraries (libavcodec, libavformat)
# linked through Bazel for its video pipeline. The apt ffmpeg package provides
# the CLI tool and shared libraries. The apt version on 22.04 (4.4.x) is
# compatible. Do NOT install ffmpeg from a PPA or snap that would ship a
# different libavcodec SO version, as it could break Mythos runtime linking.
log "Installing FFmpeg..."
apt install -y ffmpeg

# Mythos compatibility: Mythos WORKSPACE vendors GraphicsMagick 1.3.42 for its
# own image processing. The system ImageMagick is a separate package and does
# not conflict (different binary names and library paths).
log "Installing ImageMagick..."
apt install -y imagemagick

log "Installing fzf..."
apt install -y fzf

log "Installing hstr (bash history)..."
apt install -y hstr

log "Installing rsync..."
apt install -y rsync

log "Installing zstd..."
apt install -y zstd

log "Installing detox..."
apt install -y detox

log "Installing yt-dlp..."
apt install -y yt-dlp

log "Installing htop..."
apt install -y htop

log "Installing tree..."
apt install -y tree

log "Installing strace..."
apt install -y strace

log "Installing ShellCheck..."
apt install -y shellcheck

log "Installing duf..."
apt install -y duf

log "Installing ncdu..."
apt install -y ncdu

log "Installing xclip..."
apt install -y xclip

log "Installing pipx..."
apt install -y pipx

log "Installing lazygit..."
if ! command -v lazygit &> /dev/null; then
  LAZYGIT_VERSION=$(curl -s https://api.github.com/repos/jesseduffield/lazygit/releases/latest \
    | grep -Po '"tag_name": "v\K[^"]*') || true
  if [ -n "${LAZYGIT_VERSION:-}" ]; then
    curl -Lo /tmp/lazygit.tar.gz \
      "https://github.com/jesseduffield/lazygit/releases/latest/download/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
    tar xf /tmp/lazygit.tar.gz -C /tmp lazygit
    install /tmp/lazygit /usr/local/bin
    rm -f /tmp/lazygit /tmp/lazygit.tar.gz
  else
    warn "Could not determine lazygit version -- install manually"
  fi
else
  log "lazygit already installed: $(lazygit --version | head -1)"
fi

log "Installing bottom (btm) via snap..."
snap install bottom 2>/dev/null || warn "Failed to install bottom via snap"

# ===== CONTAINER & VIRTUALIZATION =====
echo ""
echo "🐳 Installing container tools..."

# Mythos compatibility: Mythos install.py also installs Docker from Docker's
# official apt repo. Both paths produce the same result. If Docker is already
# installed (by Mythos or otherwise), we skip re-installation.
if ! command -v docker &> /dev/null; then
  log "Installing Docker..."
  curl -fsSL https://get.docker.com -o /tmp/get-docker.sh
  sh /tmp/get-docker.sh
  rm -f /tmp/get-docker.sh
  usermod -aG docker "${REAL_USER}"
  warn "Log out and back in for Docker group permissions to take effect"
else
  log "Docker already installed: $(docker --version)"
fi

# ===== SECURITY & NETWORKING =====
echo ""
echo "🔒 Installing security and networking tools..."

log "Installing nmap..."
apt install -y nmap

log "Installing netcat (OpenBSD)..."
apt install -y netcat-openbsd

log "Installing aircrack-ng..."
apt install -y aircrack-ng

log "Installing wifite + hcxtools..."
apt install -y wifite hcxtools

log "Installing Tailscale via snap..."
snap install tailscale 2>/dev/null || warn "Failed to install Tailscale via snap"

log "Installing NetBird via snap..."
snap install netbird 2>/dev/null || warn "Failed to install NetBird via snap"

log "Installing NordVPN via snap..."
snap install nordvpn 2>/dev/null || warn "Failed to install NordVPN via snap"

log "Installing OpenSSH Server..."
apt install -y openssh-server
systemctl enable ssh 2>/dev/null || true

log "Installing net-tools (ifconfig, netstat, route)..."
apt install -y net-tools

log "Installing WireGuard tools..."
apt install -y wireguard-tools

# ===== TERMINAL & SHELL =====
echo ""
echo "🖥️  Installing terminal tools..."

log "Installing Bash completion..."
apt install -y bash-completion

log "Installing GNOME Terminal..."
apt install -y gnome-terminal

log "Installing tmux..."
apt install -y tmux

# ===== PYTHON PACKAGES =====
echo ""
echo "🐍 Installing Python packages..."

# Mythos compatibility: Mythos uses a hermetic Python 3.10.12 managed by Bazel
# (bazel/py_interpreter.bzl) with its own pinned pip packages in
# third_party/rules_python/requirements.txt. System-level pip packages do NOT
# affect the Bazel hermetic environment, so these installs are safe for general
# development use.
#
# However, if you run Mythos Python tools outside of Bazel (e.g. ETL scripts
# directly), version mismatches can occur. The following packages overlap with
# Mythos pins:
#   - SQLAlchemy: Mythos pins 2.0.19
#   - requests:   Mythos pins 2.31.0
#
# We pin these to Mythos-compatible versions to avoid surprises.

log "Upgrading pip..."
python3 -m pip install --upgrade pip

log "Installing Python packages (system-wide)..."
python3 -m pip install \
  pytest \
  pytest-mock \
  pytest-cov \
  'SQLAlchemy==2.0.19' \
  pydantic \
  pydantic-settings \
  black \
  flake8 \
  mypy \
  'requests==2.31.0'

log "Python packages installed"

# ===== RUST TOOLS (via cargo) =====
echo ""
echo "🦀 Installing Rust tools..."

CARGO_BIN="${REAL_HOME}/.cargo/bin/cargo"
if [ -x "${CARGO_BIN}" ]; then
  log "Installing Starship (shell prompt)..."
  sudo -u "${REAL_USER}" "${CARGO_BIN}" install starship

  log "Installing Just (task runner)..."
  sudo -u "${REAL_USER}" "${CARGO_BIN}" install just
else
  warn "Cargo not found at ${CARGO_BIN}, skipping Rust tools"
fi

# ===== FONTS =====
echo ""
echo "🔤 Installing fonts..."

log "Installing Liberation fonts..."
apt install -y fonts-liberation

log "Installing DejaVu fonts..."
apt install -y fonts-dejavu

warn "For Nerd Fonts, visit https://www.nerdfonts.com/ and install manually to ~/.local/share/fonts/"

# ===== SHELL CONFIGURATION =====
echo ""
echo "⚙️  Shell configuration..."

BASHRC="${REAL_HOME}/.bashrc"

if ! grep -q "direnv hook bash" "${BASHRC}" 2>/dev/null; then
  log "Adding direnv to ${BASHRC}..."
  echo 'eval "$(direnv hook bash)"' >> "${BASHRC}"
  chown "${REAL_USER}:${REAL_USER}" "${BASHRC}"
fi

if ! grep -q "starship init bash" "${BASHRC}" 2>/dev/null; then
  log "Adding Starship to ${BASHRC}..."
  echo 'eval "$(starship init bash)"' >> "${BASHRC}"
  chown "${REAL_USER}:${REAL_USER}" "${BASHRC}"
fi

# ===== OPTIONAL: DESKTOP APPLICATIONS (snap) =====
echo ""
echo "📱 Installing desktop applications (optional)..."
warn "These require snap. Install manually if preferred:"
warn "  - Discord: snap install discord"
warn "  - Slack: snap install slack"
warn "  - Spotify: snap install spotify"
warn "  - Notion: snap install notion-snap"
warn "  - NordPass: snap install nordpass"
warn "  - SimpleScreenRecorder: apt install simplescreenrecorder"

REPLY=""
read -p "Install desktop applications via snap? (y/n) " -n 1 -r REPLY || true
echo
if [[ "${REPLY}" =~ ^[Yy]$ ]]; then
  log "Installing desktop apps..."
  snap install discord 2>/dev/null || warn "Discord snap failed"
  snap install slack 2>/dev/null || warn "Slack snap failed"
  snap install spotify 2>/dev/null || warn "Spotify snap failed"
  snap install notion-snap 2>/dev/null || warn "Notion snap failed"
  snap install nordpass 2>/dev/null || warn "NordPass snap failed"
fi

log "Installing SimpleScreenRecorder..."
apt install -y simplescreenrecorder

log "Installing VeraCrypt..."
apt install -y veracrypt 2>/dev/null || warn "VeraCrypt not in default repos"

log "Installing GNOME Tweaks..."
apt install -y gnome-tweaks

log "Installing GNOME Shell Extension Manager..."
apt install -y gnome-shell-extension-manager

log "Installing GRUB Customizer..."
add-apt-repository -y ppa:danielrichter2007/grub-customizer
apt update
apt install -y grub-customizer

log "Installing Solaar (Logitech device manager)..."
apt install -y solaar

log "Installing Meld (visual diff/merge)..."
apt install -y meld

log "Installing Peek (GIF screen recorder)..."
apt install -y peek

log "Installing Google Chrome..."
if ! command -v google-chrome-stable &> /dev/null; then
  curl -fsSL https://dl.google.com/linux/linux_signing_key.pub \
    | gpg --dearmor --yes -o /usr/share/keyrings/google-chrome.gpg
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/google-chrome.gpg] https://dl.google.com/linux/chrome/deb/ stable main" \
    | tee /etc/apt/sources.list.d/google-chrome.list > /dev/null
  apt update
  apt install -y google-chrome-stable
else
  log "Google Chrome already installed"
fi

log "Installing Signal..."
if ! command -v signal-desktop &> /dev/null; then
  curl -fsSL https://updates.signal.org/desktop/apt/keys.asc \
    | gpg --dearmor --yes -o /usr/share/keyrings/signal-desktop-keyring.gpg
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/signal-desktop-keyring.gpg] https://updates.signal.org/desktop/apt xenial main" \
    | tee /etc/apt/sources.list.d/signal-xenial.list > /dev/null
  apt update
  apt install -y signal-desktop
else
  log "Signal already installed"
fi

log "Installing Claude (desktop)..."
if ! command -v claude-desktop &> /dev/null && ! dpkg -l claude-desktop &>/dev/null; then
  curl -fsSL https://aaddrick.github.io/claude-desktop-debian/public-key.gpg \
    | gpg --dearmor --yes -o /usr/share/keyrings/claude-desktop.gpg
  echo "deb [signed-by=/usr/share/keyrings/claude-desktop.gpg arch=$(dpkg --print-architecture)] https://aaddrick.github.io/claude-desktop-debian stable main" \
    | tee /etc/apt/sources.list.d/claude-desktop.list > /dev/null
  apt update
  apt install -y claude-desktop
else
  log "Claude desktop already installed"
fi

log "Installing NoMachine..."
if ! dpkg -l nomachine &>/dev/null; then
  NM_URL=$(curl -fsSL 'https://www.nomachine.com/download/linux&id=1' 2>/dev/null \
    | grep -oP 'https://download\.nomachine\.com/download/[^"]+\.deb' \
    | grep "$(dpkg --print-architecture)" | head -1) || true
  if [ -n "${NM_URL:-}" ]; then
    curl -fsSL "$NM_URL" -o /tmp/nomachine.deb
    dpkg -i /tmp/nomachine.deb || apt install -f -y
    rm -f /tmp/nomachine.deb
  else
    warn "Could not determine NoMachine download URL -- visit https://www.nomachine.com/download"
  fi
else
  log "NoMachine already installed"
fi

# ===== CLEANUP =====
echo ""
echo "🧹 Cleaning up..."
apt autoremove -y
apt autoclean -y

# ===== VERIFICATION =====
echo ""
echo "================================"
echo "✅ Installation complete!"
echo "================================"
echo ""
echo "Verify installations:"
declare -a VERIFY_CMDS=(
  "python3 --version"
  "node --version"
  "git --version"
  "rg --version"
  "jq --version"
  "cmake --version"
  "docker --version"
  "clang --version"
  "clang-format-12 --version"
)
for cmd in "${VERIFY_CMDS[@]}"; do
  if eval "${cmd}" &>/dev/null; then
    log "${cmd}: $(eval "${cmd}" 2>&1 | head -1)"
  else
    warn "${cmd}: not found"
  fi
done

# Check Rust tools under the real user
for tool in rustc cargo starship just; do
  tool_path="${REAL_HOME}/.cargo/bin/${tool}"
  if [ -x "${tool_path}" ]; then
    log "${tool}: $(sudo -u "${REAL_USER}" "${tool_path}" --version 2>&1 | head -1)"
  fi
done

echo ""
echo "Post-setup steps:"
echo "  1. Log out and back in for Docker group permissions"
echo "  2. Reboot to activate the low-latency kernel"
echo "  3. Configure ~/.config/starship.toml"
echo "  4. Create ~/.envrc files for project-specific env vars"
echo "  5. Download Cursor editor from https://www.cursor.com/"
echo "  6. Download and install Nerd Fonts"
echo ""
echo "If this machine runs the Mythos AV stack, run the Mythos installer next:"
echo "  cd ~/mythos && python3 install.py"
echo ""
