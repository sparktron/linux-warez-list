#!/bin/bash
# Complete Ubuntu dev environment setup script
# Run with: bash install-all.sh
# Root privileges required for most operations

set -e

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

log "Installing GitHub CLI..."
apt install -y gh || {
  warn "gh not in default repos, installing from GitHub source..."
  curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | gpg --dearmor -o /usr/share/keyrings/githubcli-archive-keyring.gpg
  echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null
  apt update
  apt install -y gh
}

# ===== PROGRAMMING LANGUAGES =====
echo ""
echo "🔧 Installing programming languages..."

log "Installing Python 3.10 and dev tools..."
apt install -y python3.10 python3.10-venv python3.10-dev python3-pip

log "Installing GCC and development headers..."
apt install -y gcc g++ gdb

log "Installing Clang/LLVM..."
apt install -y clang llvm llvm-dev

# ===== NODE.JS (if not already installed) =====
if ! command -v node &> /dev/null; then
  log "Installing Node.js..."
  curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
  apt install -y nodejs
else
  log "Node.js already installed: $(node --version)"
fi

# ===== RUST (if not already installed) =====
if ! command -v rustc &> /dev/null; then
  log "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  log "Rust already installed: $(rustc --version)"
fi

# ===== CLI DEVELOPMENT TOOLS =====
echo ""
echo "⚙️  Installing CLI tools..."

log "Installing ripgrep..."
apt install -y ripgrep

log "Installing fd..."
apt install -y fd-find
# Create symlink if needed
if ! command -v fd &> /dev/null; then
  ln -s /usr/bin/fdfind /usr/local/bin/fd 2>/dev/null || true
fi

log "Installing direnv..."
apt install -y direnv

log "Installing jq..."
apt install -y jq

log "Installing SQLite3..."
apt install -y sqlite3

log "Installing Make..."
apt install -y make

log "Installing CMake..."
apt install -y cmake

log "Installing Valgrind..."
apt install -y valgrind

log "Installing Watchman..."
apt install -y watchman 2>/dev/null || {
  warn "Watchman not in repos, skipping (optional)"
}

log "Installing bat..."
apt install -y bat

log "Installing FFmpeg..."
apt install -y ffmpeg

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

log "Installing bottom (btm) via snap..."
snap install bottom

# ===== CONTAINER & VIRTUALIZATION =====
echo ""
echo "🐳 Installing container tools..."

if ! command -v docker &> /dev/null; then
  log "Installing Docker..."
  curl -fsSL https://get.docker.com -o get-docker.sh
  sh get-docker.sh
  rm get-docker.sh
  # Add current user to docker group
  usermod -aG docker "$SUDO_USER" || usermod -aG docker $USER
  warn "Please log out and back in for Docker group permissions to take effect"
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
snap install tailscale

log "Installing NetBird via snap..."
snap install netbird

log "Installing NordVPN via snap..."
snap install nordvpn

# ===== TERMINAL & SHELL =====
echo ""
echo "🖥️  Installing terminal tools..."

log "Installing Bash completion..."
apt install -y bash-completion

log "Installing GNOME Terminal..."
apt install -y gnome-terminal

# ===== PYTHON PACKAGES =====
echo ""
echo "🐍 Installing Python packages..."

log "Installing Python packages (global)..."
python3 -m pip install --upgrade pip

# Core packages
python3 -m pip install \
  pytest \
  pytest-mock \
  SQLAlchemy \
  pydantic \
  pydantic-settings \
  pytest-cov \
  black \
  flake8 \
  mypy \
  requests

log "Python packages installed"

# ===== RUST TOOLS (via cargo) =====
echo ""
echo "🦀 Installing Rust tools..."

if command -v cargo &> /dev/null; then
  log "Installing Starship (shell prompt)..."
  cargo install starship

  log "Installing Just (task runner)..."
  cargo install just

  log "Installing ripgrep (via cargo, already have apt version)..."
  # cargo install ripgrep
else
  warn "Cargo not found, skipping Rust tools"
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

# Setup direnv in bashrc if not present
if ! grep -q "direnv hook bash" /root/.bashrc; then
  log "Adding direnv to ~/.bashrc..."
  echo 'eval "$(direnv hook bash)"' >> /root/.bashrc
fi

# Setup starship in bashrc if not present
if ! grep -q "starship init bash" /root/.bashrc; then
  log "Adding Starship to ~/.bashrc..."
  echo 'eval "$(starship init bash)"' >> /root/.bashrc
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

read -p "Install desktop applications via snap? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
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

log "Installing Google Chrome..."
curl -fsSL https://dl.google.com/linux/linux_signing_key.pub \
  | gpg --dearmor -o /usr/share/keyrings/google-chrome.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/google-chrome.gpg] \
  https://dl.google.com/linux/chrome/deb/ stable main" \
  | tee /etc/apt/sources.list.d/google-chrome.list > /dev/null
apt update
apt install -y google-chrome-stable

log "Installing Signal..."
curl -fsSL https://updates.signal.org/desktop/apt/keys.asc \
  | gpg --dearmor -o /usr/share/keyrings/signal-desktop-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/signal-desktop-keyring.gpg] \
  https://updates.signal.org/desktop/apt xenial main" \
  | tee /etc/apt/sources.list.d/signal-xenial.list > /dev/null
apt update
apt install -y signal-desktop

log "Installing Claude (desktop)..."
curl -fsSL https://aaddrick.github.io/claude-desktop-debian/public-key.gpg \
  | gpg --dearmor -o /usr/share/keyrings/claude-desktop.gpg
echo "deb [signed-by=/usr/share/keyrings/claude-desktop.gpg arch=$(dpkg --print-architecture)] \
  https://aaddrick.github.io/claude-desktop-debian stable main" \
  | tee /etc/apt/sources.list.d/claude-desktop.list > /dev/null
apt update
apt install -y claude-desktop

log "Installing NoMachine..."
ARCH=$(dpkg --print-architecture)
NM_URL=$(curl -fsSL 'https://www.nomachine.com/download/linux&id=1' \
  | grep -oP 'https://download\.nomachine\.com/download/[^"]+\.deb' \
  | grep "${ARCH}" | head -1)
if [ -n "$NM_URL" ]; then
  curl -fsSL "$NM_URL" -o /tmp/nomachine.deb
  dpkg -i /tmp/nomachine.deb
  rm -f /tmp/nomachine.deb
else
  warn "Could not determine NoMachine download URL — visit https://www.nomachine.com/download"
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
echo "  python3 --version"
echo "  rustc --version"
echo "  node --version"
echo "  git --version"
echo "  rg --version"
echo "  fd --version"
echo "  jq --version"
echo "  cmake --version"
echo "  docker --version"
echo "  starship --version"
echo "  just --version"
echo ""
echo "Post-setup steps:"
echo "  1. Log out and back in for Docker group permissions"
echo "  2. Configure ~/.config/starship.toml"
echo "  3. Create ~/.envrc files for project-specific env vars"
echo "  4. Download Cursor editor from https://www.cursor.com/"
echo "  5. Download and install Nerd Fonts"
echo ""
