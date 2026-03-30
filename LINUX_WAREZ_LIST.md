# Linux Warez List — Dylan Sparks' Ubuntu Dev Environment

> Comprehensive inventory of software, tools, extensions, and configurations for Ubuntu 22.04 LTS development machine (Intel i9-14900HX, 31GB RAM).

---

## Table of Contents

1. [System & Core](#system--core)
2. [Development Languages & Runtimes](#development-languages--runtimes)
3. [CLI Development Tools](#cli-development-tools)
4. [Development Editors & IDEs](#development-editors--ides)
5. [Build & Automation Tools](#build--automation-tools)
6. [Database & Data Tools](#database--data-tools)
7. [Version Control](#version-control)
8. [Container & Virtualization](#container--virtualization)
9. [Security & Encryption](#security--encryption)
10. [Communication & Collaboration](#communication--collaboration)
11. [Media & Content Creation](#media--content-creation)
12. [Fonts](#fonts)
13. [Shell & Terminal Configuration](#shell--terminal-configuration)
14. [Cursor Editor Extensions](#cursor-editor-extensions)
15. [Desktop Environments & Window Managers](#desktop-environments--window-managers)

---

## System & Core

### Ubuntu 22.04.5 LTS
- **What:** Linux distribution based on Debian, long-term support release
- **Installed:** Base OS
- **Usage:** Primary operating system for development
- **Install:** Base system installation

### GNOME Desktop Environment
- **What:** Default Ubuntu desktop environment, modern GTK-based UI
- **Installed:** Yes (default)
- **Usage:** Desktop, window management, file browser (Nautilus)
- **Install:** `sudo apt install gnome-shell gnome-control-center`

### systemd
- **What:** System and service manager
- **Installed:** Yes (core)
- **Usage:** Service management, system initialization
- **Install:** Included in base Ubuntu

---

## Development Languages & Runtimes

### Python 3.10.12
- **What:** Python programming language, version 3.10.12
- **Installed:** Yes (`/usr/bin/python3`)
- **Usage:** Primary development language for CLI tools, scripts, backends
- **Install:** `sudo apt install python3.10 python3.10-venv python3.10-dev`
- **Version Check:** `python3 --version`
- **Package Manager:** pip (`python3 -m pip`)

### Node.js (Recently Added)
- **What:** JavaScript runtime, event-driven, asynchronous
- **Installed:** Yes
- **Usage:** Frontend development, JavaScript tooling, npm ecosystem
- **Install:** `curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - && sudo apt install -y nodejs`
- **Version Check:** `node --version && npm --version`

### Rust & Cargo (Recently Added)
- **What:** Systems programming language, memory-safe without garbage collection
- **Installed:** Yes
- **Usage:** Systems tools, performance-critical code, embedded systems
- **Install:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Version Check:** `rustc --version && cargo --version`

### GCC 11.4.0
- **What:** GNU Compiler Collection — C/C++ compiler
- **Installed:** Yes (`/usr/bin/gcc`)
- **Usage:** Compiling C code for embedded systems, RF security projects
- **Install:** `sudo apt install build-essential`
- **Version Check:** `gcc --version`

### Clang / LLVM
- **What:** C/C++ compiler frontend, alternative to GCC with better diagnostics
- **Installed:** Yes
- **Usage:** Alternative compiler, static analysis, better error messages
- **Install:** `sudo apt install clang llvm`
- **Version Check:** `clang --version`

---

## CLI Development Tools

### Git
- **What:** Distributed version control system
- **Installed:** Yes (`/usr/bin/git`)
- **Usage:** Source code management, GitHub Flow workflow
- **Install:** `sudo apt install git`
- **Version Check:** `git --version`
- **Config:** `~/.gitconfig`

### GitHub CLI (gh)
- **What:** Official GitHub command-line tool
- **Installed:** Yes (assumed)
- **Usage:** Manage repos, PRs, issues from terminal
- **Install:** `sudo apt install gh` or `curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo gpg --dearmor -o /usr/share/keyrings/githubcli-archive-keyring.gpg && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null && sudo apt update && sudo apt install gh`
- **Version Check:** `gh --version`

### ripgrep (rg)
- **What:** Fast recursive pattern searcher, better than grep
- **Installed:** Yes
- **Usage:** Code search, pattern matching across projects
- **Install:** `sudo apt install ripgrep`
- **Version Check:** `rg --version`
- **Usage Example:** `rg "pattern" src/ --type py`

### fd
- **What:** User-friendly alternative to `find` command
- **Installed:** Yes
- **Usage:** Fast file searching with intuitive syntax
- **Install:** `sudo apt install fd-find && ln -s $(which fdfind) ~/.local/bin/fd`
- **Version Check:** `fd --version`
- **Usage Example:** `fd "\.rs$" src/`

### direnv
- **What:** Environment variable management per directory
- **Installed:** Yes
- **Usage:** Auto-load project-specific environment variables from `.envrc` files
- **Install:** `sudo apt install direnv`
- **Version Check:** `direnv --version`
- **Setup:** Add to `~/.bashrc`: `eval "$(direnv hook bash)"`
- **Usage Example:** Create `.envrc` with `export PYTHONPATH=/path/to/src`

### jq
- **What:** Lightweight and flexible JSON processor
- **Installed:** Yes
- **Usage:** Parse, filter, and transform JSON in scripts
- **Install:** `sudo apt install jq`
- **Version Check:** `jq --version`
- **Usage Example:** `curl api.example.com | jq '.data[] | select(.status=="active")'`

### SQLite3
- **What:** Embedded SQL database engine
- **Installed:** Yes (`/usr/bin/sqlite3`)
- **Usage:** Local development databases for Python/Node projects
- **Install:** `sudo apt install sqlite3`
- **Version Check:** `sqlite3 --version`
- **Usage Example:** `sqlite3 dev.db ".tables"`

### Make
- **What:** Build automation tool
- **Installed:** Yes (`/usr/bin/make`)
- **Usage:** Automate build, test, and deploy tasks
- **Install:** `sudo apt install make`
- **Version Check:** `make --version`
- **Config:** `Makefile` in project root

### CMake
- **What:** Cross-platform build system generator
- **Installed:** Yes
- **Usage:** C/C++ project builds, especially for embedded systems
- **Install:** `sudo apt install cmake`
- **Version Check:** `cmake --version`
- **Usage Example:** `cmake -B build && cd build && make`

### Valgrind
- **What:** Memory debugging and profiling tool for C/C++
- **Installed:** Yes (recently added)
- **Usage:** Detect memory leaks, buffer overflows, use-after-free bugs
- **Install:** `sudo apt install valgrind`
- **Version Check:** `valgrind --version`
- **Usage Example:** `valgrind --leak-check=full ./program`

### Watchman
- **What:** File change monitoring service
- **Installed:** Yes
- **Usage:** Trigger rebuilds/tests on file changes
- **Install:** `sudo apt install watchman`
- **Version Check:** `watchman --version`

### bat
- **What:** Syntax-highlighted `cat` replacement
- **Installed:** Yes
- **Usage:** Better code viewing in terminal with syntax highlighting
- **Install:** `sudo apt install bat`
- **Version Check:** `bat --version`
- **Usage Example:** `bat src/main.py`

---

## Development Editors & IDEs

### Cursor
- **What:** AI-powered code editor built on VS Code
- **Installed:** Yes
- **Usage:** Primary code editor for all languages
- **Install:** Download from https://www.cursor.com/
- **Config:** `~/.config/Cursor/User/`
- **Key Extensions:** See [Cursor Editor Extensions](#cursor-editor-extensions)

### VS Code (if installed)
- **What:** Visual Studio Code lightweight editor
- **Installed:** Possibly (Cursor is primary)
- **Usage:** Fallback editor, shared extensions with Cursor
- **Install:** `sudo apt install code`
- **Config:** `~/.config/Code/User/`

---

## Build & Automation Tools

### Just
- **What:** User-friendly Make alternative, cleaner syntax
- **Installed:** Yes (recommended earlier)
- **Usage:** Project task runner, clearer than Makefiles
- **Install:** `cargo install just`
- **Version Check:** `just --version`
- **Config:** `justfile` in project root

### Fastlane
- **What:** Build automation tool for iOS/Android
- **Installed:** Yes (used for iOS projects)
- **Usage:** Automate iOS builds, signing, releases
- **Install:** `sudo gem install fastlane -NV` or via Homebrew on macOS
- **Version Check:** `fastlane --version`
- **Config:** `fastlane/` directory in iOS projects

---

## Database & Data Tools

### SQLAlchemy
- **What:** Python SQL toolkit and ORM
- **Installed:** Yes (via pip)
- **Usage:** Database abstraction layer for Python projects
- **Install:** `pip install SQLAlchemy`
- **Version Check:** `python3 -c "import sqlalchemy; print(sqlalchemy.__version__)"`

### Pydantic
- **What:** Data validation and settings management for Python
- **Installed:** Yes (via pip)
- **Usage:** Type-safe configuration and data validation
- **Install:** `pip install pydantic pydantic-settings`
- **Version Check:** `python3 -c "import pydantic; print(pydantic.__version__)"`

### pytest
- **What:** Python testing framework
- **Installed:** Yes (via pip)
- **Usage:** Unit testing, integration testing for Python projects
- **Install:** `pip install pytest pytest-mock`
- **Version Check:** `pytest --version`
- **Usage Example:** `pytest tests/ -v`

### pytest-mock
- **What:** pytest plugin for mocking
- **Installed:** Yes (via pip)
- **Usage:** Mock objects and functions in unit tests
- **Install:** `pip install pytest-mock`

---

## Version Control

### Git (See [CLI Development Tools](#cli-development-tools))

### GitHub CLI (See [CLI Development Tools](#cli-development-tools))

---

## Container & Virtualization

### Docker
- **What:** Containerization platform for applications
- **Installed:** Yes
- **Usage:** Container builds, local testing, deployment
- **Install:** `curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh && sudo usermod -aG docker $USER`
- **Version Check:** `docker --version`
- **Verify:** `docker run hello-world`
- **Post-Install:** Log out and back in for group permissions

### Docker Compose
- **What:** Multi-container Docker applications
- **Installed:** Yes (included with Docker)
- **Usage:** Define and run multi-container setups
- **Install:** Included in Docker installation
- **Version Check:** `docker compose version`

---

## Security & Encryption

### VeraCrypt
- **What:** Disk encryption software, successor to TrueCrypt
- **Installed:** Yes
- **Usage:** Encrypt sensitive files and volumes
- **Install:** `sudo apt install veracrypt` or download from https://www.veracrypt.fr/
- **Version Check:** `veracrypt --text --help | head -1`

### NordPass
- **What:** Password manager and credential vault
- **Installed:** Yes
- **Usage:** Secure password storage and autofill
- **Install:** Download from https://nordpass.com/ or `snap install nordpass`
- **Usage:** GUI application, password management

---

## Communication & Collaboration

### Discord
- **What:** Voice, video, and text communication platform
- **Installed:** Yes
- **Usage:** Team communication, community interaction
- **Install:** `snap install discord` or download from https://discord.com/
- **Usage:** Desktop application

### Slack
- **What:** Team messaging and collaboration platform
- **Installed:** Yes
- **Usage:** Workplace communication, notifications
- **Install:** `snap install slack` or download from https://slack.com/
- **Usage:** Desktop application

### Spotify
- **What:** Music streaming service
- **Installed:** Yes
- **Usage:** Background music while coding
- **Install:** `snap install spotify` or download from https://www.spotify.com/
- **Usage:** Desktop application

### Notion
- **What:** All-in-one workspace for notes, databases, documentation
- **Installed:** Yes
- **Usage:** Project planning, documentation, knowledge base
- **Install:** `snap install notion-snap` or use web app at https://www.notion.so
- **Usage:** Desktop app or web browser

---

## Media & Content Creation

### SimpleScreenRecorder
- **What:** Screen recording software
- **Installed:** Yes
- **Usage:** Record screen for tutorials, demos, bug reproduction
- **Install:** `sudo apt install simplescreenrecorder`
- **Version Check:** `simplescreenrecorder --version`
- **Usage:** GUI application

### FFmpeg
- **What:** Multimedia framework, encoding/decoding/streaming
- **Installed:** Yes (likely)
- **Usage:** Video/audio processing, format conversion
- **Install:** `sudo apt install ffmpeg`
- **Version Check:** `ffmpeg -version | head -1`

### ImageMagick
- **What:** Image manipulation library and tools
- **Installed:** Yes (likely)
- **Usage:** Image processing scripts, batch operations
- **Install:** `sudo apt install imagemagick`
- **Version Check:** `convert --version | head -1`

---

## Fonts

### Nerd Fonts (Recommended for Terminal)
- **What:** Font collection with glyph icons (powerline, devicons, etc.)
- **Installed:** Yes (assumed)
- **Usage:** Terminal font with proper glyph rendering for Starship, shell prompts
- **Install Options:**
  - Manual: Download from https://www.nerdfonts.com/ and place in `~/.local/share/fonts/`
  - Via package: `sudo apt install fonts-nerd-font-*` (varies by distro)
  - Script: Download to `~/.local/share/fonts/` and run `fc-cache -fv`
- **Common Choices:** FiraCode Nerd Font, JetBrains Mono Nerd Font, Meslo Nerd Font
- **Usage:** Set in terminal emulator preferences

### DejaVu Sans Mono
- **What:** TrueType monospace font
- **Installed:** Yes (default)
- **Usage:** Terminal fallback font
- **Install:** `sudo apt install fonts-dejavu`

### Liberation Mono
- **What:** Liberation font family, metric-compatible with Times New Roman, Arial, Courier New
- **Installed:** Yes (default)
- **Usage:** System monospace font
- **Install:** `sudo apt install fonts-liberation`

---

## Shell & Terminal Configuration

### Bash
- **What:** GNU Bourne-Again Shell, command interpreter
- **Installed:** Yes (default shell at `/bin/bash`)
- **Usage:** Command execution, scripting, automation
- **Config:** `~/.bashrc`, `~/.bash_profile`

### Starship
- **What:** Minimal, fast shell prompt written in Rust
- **Installed:** Yes (recommended)
- **Usage:** Modern, customizable shell prompt with git status, language version indicators
- **Install:** `cargo install starship`
- **Version Check:** `starship --version`
- **Config:** `~/.config/starship.toml`
- **Setup:** Add to `~/.bashrc`:
  ```bash
  eval "$(starship init bash)"
  ```

### direnv (See [CLI Development Tools](#cli-development-tools))

### bash-completion
- **What:** Bash command completion scripts
- **Installed:** Yes (default)
- **Usage:** Tab-completion for commands and arguments
- **Install:** `sudo apt install bash-completion`

---

## Cursor Editor Extensions

### Python
- **ID:** `ms-python.python`
- **What:** Official Python extension for Cursor/VS Code
- **Usage:** Language support, linting, debugging
- **Install:** Via Cursor Extensions marketplace

### Pylance
- **ID:** `ms-python.vscode-pylance`
- **What:** Static type checker and language server for Python
- **Usage:** Type checking, IntelliSense, code analysis
- **Install:** Via Cursor Extensions marketplace

### Prettier - Code Formatter
- **ID:** `esbenp.prettier-vscode`
- **What:** Opinionated code formatter for JavaScript, JSON, CSS, Markdown
- **Usage:** Auto-format code on save
- **Install:** Via Cursor Extensions marketplace

### GitLens
- **ID:** `eamodio.gitlens`
- **What:** Git integration, blame annotations, history
- **Usage:** View commit history, blame, compare branches
- **Install:** Via Cursor Extensions marketplace

### Thunder Client
- **ID:** `rangav.vscode-thunder-client`
- **What:** Lightweight REST API client
- **Usage:** API testing without leaving editor
- **Install:** Via Cursor Extensions marketplace

### Error Lens
- **ID:** `usernamehw.errorlens`
- **What:** Show errors inline in editor
- **Usage:** See diagnostic messages right where errors occur
- **Install:** Via Cursor Extensions marketplace

### Better Comments
- **ID:** `aaron-bond.better-comments`
- **What:** Highlight and color-code comments
- **Usage:** Distinguish TODO, FIXME, NOTE comments
- **Install:** Via Cursor Extensions marketplace

### Markdown All in One
- **ID:** `yzhang.markdown-all-in-one`
- **What:** Markdown support, preview, keyboard shortcuts
- **Usage:** Write and preview Markdown efficiently
- **Install:** Via Cursor Extensions marketplace

### DotENV
- **ID:** `mikestead.dotenv`
- **What:** Syntax highlighting for .env files
- **Usage:** Environment variable file highlighting
- **Install:** Via Cursor Extensions marketplace

### REST Client
- **ID:** `humao.rest-client`
- **What:** REST API client for testing HTTP requests
- **Usage:** Test APIs directly from .rest files in editor
- **Install:** Via Cursor Extensions marketplace

### Rust
- **ID:** `rust-lang.rust-analyzer`
- **What:** Official Rust language server
- **Usage:** Language support, linting, debugging for Rust
- **Install:** Via Cursor Extensions marketplace

### C/C++
- **ID:** `ms-vscode.cpptools`
- **What:** Official C/C++ extension
- **Usage:** Language support, debugging, IntelliSense for C/C++
- **Install:** Via Cursor Extensions marketplace

### CMake
- **ID:** `twxs.cmake`
- **What:** CMake language support
- **Usage:** Syntax highlighting and snippets for CMakeLists.txt
- **Install:** Via Cursor Extensions marketplace

### Swift
- **ID:** `scramblee.swift`
- **What:** Swift language support
- **Usage:** Swift/SwiftUI development support
- **Install:** Via Cursor Extensions marketplace (if developing iOS on this machine)

### GitHub Copilot
- **ID:** `GitHub.copilot`
- **What:** AI code completion
- **Usage:** AI-powered code suggestions and completion
- **Install:** Via Cursor Extensions marketplace
- **Note:** May be built-in to Cursor already

### Indent Rainbow
- **ID:** `oderwat.indent-rainbow`
- **What:** Color-code indentation levels
- **Usage:** Visualize code structure better
- **Install:** Via Cursor Extensions marketplace

---

## Desktop Environments & Window Managers

### GNOME (Default Desktop Environment)
- **What:** Modern desktop environment
- **Installed:** Yes (default)
- **Usage:** Window management, system tray, application launcher
- **Config:** GNOME Settings (GUI)

### GNOME Terminal
- **What:** Default terminal emulator for GNOME
- **Installed:** Yes
- **Usage:** Terminal access
- **Install:** `sudo apt install gnome-terminal`

### Nautilus (Files)
- **What:** GNOME file manager
- **Installed:** Yes (default)
- **Usage:** File browsing and management
- **Config:** GUI preferences

---

## Installation Script

See `install-all.sh` — automated installation of all tools above.

---

## System Information

- **OS:** Ubuntu 22.04.5 LTS (Jammy Jellyfish)
- **Kernel:** Linux 6.8.0-106-generic x86_64
- **CPU:** Intel Core i9-14900HX (24 cores)
- **RAM:** 31 GB
- **Editor:** Cursor (primary)
- **Shell:** Bash with Starship prompt
- **Package Managers:** apt, pip, npm, cargo, snap, flatpak

---

## Last Updated

Generated: 2026-03-30

---

## Notes

- **Local-only deployment:** All projects use local/self-hosted infrastructure
- **GitHub Flow:** Primary workflow is feature branch → PR → main
- **Testing:** Python projects use pytest + pytest-mock
- **CI/CD:** GitHub Actions for automated testing and deployment
- **No cloud providers:** AWS, Azure, GCP not used by default
