# Linux Warez List

> Comprehensive inventory of all software, tools, and configurations for Dylan Sparks' Ubuntu 22.04 LTS development machine.

Complete breakdown of every dev tool, CLI utility, desktop application, extension, and font installed on a high-performance Ubuntu dev box (Intel i9-14900HX, 31GB RAM).

## Contents

- **LINUX_WAREZ_LIST.md** — Full software inventory with installation instructions and usage
- **install-all.sh** — Automated installation script to set up entire environment from scratch

## Quick Start

### Review the inventory
```bash
cat LINUX_WAREZ_LIST.md
```

### Install everything automatically
```bash
sudo bash install-all.sh
```

This will install:
- **Development languages:** Python 3.10, Node.js, Rust, GCC, Clang
- **CLI tools:** ripgrep, fd, direnv, jq, CMake, Valgrind, git, gh
- **Package managers:** pip, npm, cargo
- **Testing:** pytest, pytest-mock
- **Editors:** Cursor, VS Code
- **Container tools:** Docker, Docker Compose
- **Desktop apps:** Discord, Slack, Spotify, Notion, VeraCrypt, SimpleScreenRecorder
- **Fonts:** Liberation, DejaVu, + setup for Nerd Fonts
- **Shell:** Bash with Starship prompt, direnv

## System Info

- **OS:** Ubuntu 22.04.5 LTS (Jammy Jellyfish)
- **CPU:** Intel Core i9-14900HX (24 cores)
- **RAM:** 31 GB
- **Primary Editor:** Cursor
- **Shell:** Bash + Starship prompt
- **Deployment:** Local/self-hosted only

## Tech Stack

- Python 3.10 (primary)
- C/C++ (embedded, RF security)
- JavaScript/Node.js
- Rust (systems tools)
- Vanilla HTML/CSS/JS (static sites)
- Swift 6/SwiftUI (iOS)

## Workflow

- **Version Control:** Git + GitHub Flow
- **Testing:** pytest + pytest-mock
- **Data:** SQLAlchemy + SQLite
- **Config:** Pydantic
- **CI/CD:** GitHub Actions
- **Build:** Make, CMake, Just

## Installation Notes

### First Time Setup
1. Clone this repo
2. Review `LINUX_WAREZ_LIST.md` to customize for your needs
3. Run `sudo bash install-all.sh`
4. Log out and back in (for Docker group permissions)
5. Download Cursor from https://www.cursor.com/
6. Download Nerd Fonts from https://www.nerdfonts.com/

### Updates
To update individual tools:
```bash
sudo apt update && sudo apt upgrade
pip install --upgrade <package>
cargo install --force <tool>
npm install -g <package>
```

## Post-Install Configuration

### Starship Prompt
Edit `~/.config/starship.toml` to customize prompt appearance.

### direnv Setup
Create `.envrc` files in project directories:
```bash
echo 'export PYTHONPATH=/path/to/src' > .envrc
direnv allow
```

### Docker
Add your user to docker group:
```bash
sudo usermod -aG docker $USER
# Log out and back in
```

### Git Configuration
```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

## Cursor Extensions

Key extensions installed via Cursor:
- Python
- Pylance
- Rust-analyzer
- C/C++
- CMake
- GitLens
- Prettier
- Thunder Client
- REST Client
- Error Lens
- Better Comments

## Hardware Specifications

- **CPU:** Intel Core i9-14900HX (24 cores, 5.8 GHz boost)
- **RAM:** 31 GB DDR5
- **Storage:** High-speed NVMe SSD
- **Perfect for:** Heavy computation, parallel testing, large compilations

## Notes

- **No cloud:** All services are local or self-hosted
- **Correctness first:** Prioritizes bug-free code over premature optimization
- **Small diffs:** Prefers minimal, reviewable changes
- **High agency:** Scripts and tools designed to work autonomously

## License

Public reference — customize as needed for your environment.

---

**Last Updated:** 2026-03-30

Questions or improvements? Update LINUX_WAREZ_LIST.md and push!
