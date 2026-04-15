use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, process::Command};
extern crate libc;

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
enum InstallCmd {
    Apt(&'static [&'static str]),
    Script(&'static str),
    Cargo(&'static str),
    Pip(&'static [&'static str]),
    Snap(&'static str),
}

#[derive(Clone)]
struct Package {
    name: &'static str,
    description: &'static str,
    cmd: InstallCmd,
    selected: bool,
    requires_root: bool,
}

enum Entry {
    Category(&'static str),
    Pkg(usize),
}

#[derive(PartialEq, Clone, Copy)]
enum Screen {
    Select,
    Confirm,
}

// ─── App State ────────────────────────────────────────────────────────────────

struct App {
    packages: Vec<Package>,
    entries: Vec<Entry>,
    list_state: ListState,
    cursor: usize,
    screen: Screen,
    confirm_scroll: u16,
    is_root: bool,
}

impl App {
    fn new() -> Self {
        let (packages, entries) = build_data();
        let cursor = entries
            .iter()
            .position(|e| matches!(e, Entry::Pkg(_)))
            .unwrap_or(0);
        let mut list_state = ListState::default();
        list_state.select(Some(cursor));
        Self {
            packages,
            entries,
            list_state,
            cursor,
            screen: Screen::Select,
            confirm_scroll: 0,
            is_root: is_root(),
        }
    }

    fn current_pkg_idx(&self) -> Option<usize> {
        match self.entries.get(self.cursor) {
            Some(Entry::Pkg(i)) => Some(*i),
            _ => None,
        }
    }

    fn toggle(&mut self) {
        if let Some(i) = self.current_pkg_idx() {
            let locked = !self.is_root && self.packages[i].requires_root;
            if !locked {
                self.packages[i].selected = !self.packages[i].selected;
            }
        }
    }

    fn select_all(&mut self) {
        for p in &mut self.packages {
            if self.is_root || !p.requires_root {
                p.selected = true;
            }
        }
    }

    fn select_none(&mut self) {
        for p in &mut self.packages {
            p.selected = false;
        }
    }

    fn move_up(&mut self) {
        let mut i = self.cursor;
        while i > 0 {
            i -= 1;
            if matches!(self.entries[i], Entry::Pkg(_)) {
                self.cursor = i;
                self.list_state.select(Some(i));
                return;
            }
        }
    }

    fn move_down(&mut self) {
        let mut i = self.cursor + 1;
        while i < self.entries.len() {
            if matches!(self.entries[i], Entry::Pkg(_)) {
                self.cursor = i;
                self.list_state.select(Some(i));
                return;
            }
            i += 1;
        }
    }

    fn selected_count(&self) -> usize {
        self.packages.iter().filter(|p| p.selected).count()
    }

    fn selected_packages(&self) -> Vec<&Package> {
        self.packages.iter().filter(|p| p.selected).collect()
    }

    fn has_selected_cargo(&self) -> bool {
        self.packages
            .iter()
            .any(|p| p.selected && matches!(p.cmd, InstallCmd::Cargo(_)))
    }

    fn has_selected_pip(&self) -> bool {
        self.packages
            .iter()
            .any(|p| p.selected && matches!(p.cmd, InstallCmd::Pip(_)))
    }

    fn rust_will_be_installed(&self) -> bool {
        self.packages.iter().any(|p| {
            p.selected && matches!(&p.cmd, InstallCmd::Script(s) if s.contains("rustup"))
        })
    }

    fn python_will_be_installed(&self) -> bool {
        self.packages.iter().any(|p| {
            p.selected
                && matches!(&p.cmd, InstallCmd::Apt(pkgs) if pkgs.contains(&"python3.10"))
        })
    }
}

// ─── Package Data ─────────────────────────────────────────────────────────────

struct DataBuilder {
    packages: Vec<Package>,
    entries: Vec<Entry>,
}

impl DataBuilder {
    fn new() -> Self {
        Self {
            packages: Vec::new(),
            entries: Vec::new(),
        }
    }

    fn cat(&mut self, name: &'static str) -> &mut Self {
        self.entries.push(Entry::Category(name));
        self
    }

    fn pkg(
        &mut self,
        name: &'static str,
        description: &'static str,
        cmd: InstallCmd,
        selected: bool,
        requires_root: bool,
    ) -> &mut Self {
        let idx = self.packages.len();
        self.packages.push(Package {
            name,
            description,
            cmd,
            selected,
            requires_root,
        });
        self.entries.push(Entry::Pkg(idx));
        self
    }

    fn build(self) -> (Vec<Package>, Vec<Entry>) {
        (self.packages, self.entries)
    }
}

fn build_data() -> (Vec<Package>, Vec<Entry>) {
    let mut b = DataBuilder::new();

    // ── System Tools ──────────────────────────────────────────────────────────
    b.cat("  System Tools");

    b.pkg(
        "build-essential",
        "Essential C/C++ compilation toolchain: GCC, G++, make, and standard development \
         libraries (libc-dev, dpkg-dev). Required to compile virtually any software from source \
         on Ubuntu. This is usually the very first package installed on a fresh system and a \
         prerequisite for many others. Highly recommended.",
        InstallCmd::Apt(&["build-essential"]),
        true,
        true,
    );

    b.pkg(
        "git",
        "Distributed version control system. Track changes across your entire project history, \
         create branches for new features, merge code, roll back mistakes, and collaborate \
         via GitHub or GitLab. Absolutely essential for any development workflow. If you do \
         one thing, install this.",
        InstallCmd::Apt(&["git"]),
        true,
        true,
    );

    b.pkg(
        "gh  (GitHub CLI)",
        "Official GitHub command-line tool. Create pull requests, review code, manage issues, \
         clone repos, trigger GitHub Actions workflows, and browse repositories without leaving \
         your terminal. Run `gh auth login` after installation to authenticate with your \
         GitHub account.",
        InstallCmd::Script(
            "(type -p wget >/dev/null || apt install -y wget) \
             && mkdir -p /etc/apt/keyrings \
             && wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg \
             | tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
             && chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
             && echo \"deb [arch=$(dpkg --print-architecture) \
             signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] \
             https://cli.github.com/packages stable main\" \
             | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
             && apt update && apt install -y gh",
        ),
        false,
        true,
    );

    b.pkg(
        "linux-lowlatency  (kernel)",
        "Ubuntu's low-latency kernel flavor, tuned to minimize scheduling latency for \
         real-time workloads and interactive use. Replaces the default generic kernel with \
         one that uses a 1000 Hz timer, voluntary preemption, and reduced latency \
         optimizations throughout the I/O and CPU scheduler paths. Required on Mythos \
         production vessels for real-time control loops and sensor processing — the \
         sys_monitor_service checks the running kernel version against the configured \
         instance config. Also recommended for audio production and video editing. \
         A reboot is required after installation. Verify with: uname -r",
        InstallCmd::Apt(&["linux-lowlatency"]),
        false,
        true,
    );

    b.pkg(
        "snapd",
        "Snap package manager daemon. Required for installing snap packages (Discord, \
         Slack, Spotify, Tailscale, NordVPN, bottom, etc.). Usually pre-installed on \
         Ubuntu desktop but may be absent on minimal or server installs. If snap commands \
         fail with 'command not found', install this first. Also installs the snap core \
         runtime.",
        InstallCmd::Script(
            "apt install -y snapd && snap install core",
        ),
        false,
        true,
    );

    b.pkg(
        "curl",
        "Command-line tool for transferring data with URLs. Supports HTTP, HTTPS, FTP, \
         SCP, SFTP, and dozens more protocols. Fundamental building block used by many \
         other install scripts in this installer (Docker, Rust, Node.js, Chrome, etc.). \
         Usually pre-installed on Ubuntu desktop but may be absent on minimal installs. \
         Example: `curl -fsSL https://example.com/script.sh | sh`",
        InstallCmd::Apt(&["curl"]),
        true,
        true,
    );

    b.pkg(
        "wget",
        "Non-interactive network downloader. Retrieves files from HTTP, HTTPS, and FTP \
         servers with support for recursive downloads, retry on failure, and bandwidth \
         throttling. More robust than curl for bulk downloads and mirroring. Used by the \
         GitHub CLI installer in this script. \
         Example: `wget -qO- https://example.com/key.gpg | tee keyring.gpg`",
        InstallCmd::Apt(&["wget"]),
        true,
        true,
    );

    b.pkg(
        "unzip",
        "Extraction utility for .zip archives. Required by the FiraCode Nerd Font \
         installer in this script and commonly needed for downloading release archives \
         from GitHub. Not always present on minimal Ubuntu installs. \
         Example: `unzip -o archive.zip -d /target/dir`",
        InstallCmd::Apt(&["unzip"]),
        true,
        true,
    );

    // ── Languages & Runtimes ──────────────────────────────────────────────────
    b.cat("  Languages & Runtimes");

    b.pkg(
        "Python 3.10  +  pip  +  venv",
        "Python 3.10 interpreter with pip (package manager) and venv (virtual environments). \
         Python is the go-to language for scripting, automation, data science, machine learning, \
         and web backends (Flask, FastAPI, Django). venv lets you create isolated per-project \
         environments so package versions never conflict.",
        InstallCmd::Apt(&["python3.10", "python3.10-venv", "python3.10-dev", "python3-pip"]),
        false,
        true,
    );

    b.pkg(
        "Node.js 20  +  npm",
        "JavaScript runtime built on Chrome's V8 engine. Required for frontend development \
         (React, Vue, Svelte), TypeScript compilation, and the vast npm package ecosystem. \
         npm comes bundled and manages JavaScript dependencies. Version 20 is the current \
         Long-Term Support (LTS) release.",
        InstallCmd::Script(
            "curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
             && apt install -y nodejs",
        ),
        false,
        true,
    );

    b.pkg(
        "Rust  (via rustup)",
        "Systems programming language with compile-time memory safety and no garbage collector. \
         Installed via rustup (official toolchain manager), which also installs cargo \
         (package/build manager) and rustc (compiler). Excellent for CLI tools, WebAssembly, \
         embedded systems, and high-performance code. Required for Starship and Just below. \
         When run via sudo, installs into the invoking user's home directory (not root's).",
        InstallCmd::Script(
            "REAL_USER=\"${SUDO_USER:-$USER}\" \
             && sudo -u \"${REAL_USER}\" bash -c \
             'curl --proto \"=https\" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'",
        ),
        false,
        false,
    );

    b.pkg(
        "GCC  +  G++  +  GDB",
        "GNU Compiler Collection for C and C++, plus GDB (GNU Debugger). GCC compiles C, G++ \
         handles C++, and GDB lets you set breakpoints and step through program execution \
         line by line. Essential for low-level programming, embedded systems, and compiling \
         native libraries. Note: build-essential already pulls in GCC.",
        InstallCmd::Apt(&["gcc", "g++", "gdb"]),
        false,
        true,
    );

    b.pkg(
        "Clang 14  +  LLVM  +  clang-format-12",
        "Alternative C/C++ compiler front-end with significantly better error messages than \
         GCC. LLVM provides the underlying infrastructure and enables powerful tools: \
         clang-format (auto-formatting), clang-tidy (static analysis), AddressSanitizer \
         (memory error detection), and UBSan (undefined behavior detector). Includes \
         clang-format-12 which is required by the Mythos AV stack for code formatting. \
         On Ubuntu 22.04 the bare 'clang' metapackage resolves to Clang 14, which is the \
         version Mythos pins. Do NOT install a different clang version (e.g. clang-15/16).",
        InstallCmd::Apt(&["clang", "clang-format-12", "llvm", "llvm-dev"]),
        false,
        true,
    );

    // ── CLI Tools ─────────────────────────────────────────────────────────────
    b.cat("  CLI Tools");

    b.pkg(
        "ripgrep  (rg)",
        "Blazing-fast recursive text search, much faster than grep or ag. Automatically \
         respects .gitignore files, handles binary files intelligently, and supports PCRE2 \
         regex. An essential everyday tool for searching large codebases. \
         Example: `rg 'fn main' --type rust` or `rg -l 'TODO'`",
        InstallCmd::Apt(&["ripgrep"]),
        false,
        true,
    );

    b.pkg(
        "fd",
        "User-friendly alternative to the `find` command with simpler syntax, colorized \
         output, and automatic .gitignore awareness. Example: `fd '*.rs'` finds all Rust \
         files, `fd -t d src` finds directories named src. Installed as `fdfind` with an \
         `fd` symlink created at /usr/local/bin/fd.",
        InstallCmd::Script(
            "apt install -y fd-find \
             && ln -sf $(which fdfind) /usr/local/bin/fd 2>/dev/null || true",
        ),
        false,
        true,
    );

    b.pkg(
        "direnv",
        "Automatically loads and unloads environment variables when you cd into or out of a \
         directory. Define project-specific vars in a .envrc file (DATABASE_URL, API_KEY, \
         PYTHONPATH, etc.) and they appear in your shell automatically. After install, add \
         `eval \"$(direnv hook bash)\"` to ~/.bashrc and run `direnv allow` in each project.",
        InstallCmd::Apt(&["direnv"]),
        false,
        true,
    );

    b.pkg(
        "jq",
        "Lightweight and flexible command-line JSON processor. Parse, filter, transform, and \
         pretty-print JSON in scripts and pipelines. Indispensable when working with APIs or \
         JSON config files. \
         Example: `curl api.example.com | jq '.users[] | select(.active) | .email'`",
        InstallCmd::Apt(&["jq"]),
        false,
        true,
    );

    b.pkg(
        "SQLite3",
        "Self-contained, serverless SQL database engine with a CLI shell. Perfect for local \
         development databases, data exploration, and rapid prototyping without needing a \
         running server. The database is a single .db file you can copy anywhere. \
         Example: `sqlite3 dev.db \".tables\"` or `sqlite3 data.db < schema.sql`",
        InstallCmd::Apt(&["sqlite3"]),
        false,
        true,
    );

    b.pkg(
        "make",
        "Classic build automation tool. Define targets and their dependencies in a Makefile \
         and run them with `make <target>`. Used across C/C++ projects, Python projects, and \
         general automation (build, test, clean, deploy). Likely already installed if you \
         selected build-essential above.",
        InstallCmd::Apt(&["make"]),
        false,
        true,
    );

    b.pkg(
        "CMake",
        "Cross-platform build system generator — the standard for C and C++ projects. Write \
         one CMakeLists.txt and generate platform-appropriate build files (Makefiles, Ninja, \
         MSVC projects). Note: Mythos installs CMake 3.18.1 from source to /usr/local/bin, \
         which takes PATH precedence over this apt version. Both can coexist safely. \
         Example: `cmake -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build`",
        InstallCmd::Script(
            "if [ -x /usr/local/bin/cmake ]; then \
               echo 'CMake already at /usr/local/bin/cmake, skipping apt install'; \
             else \
               apt install -y cmake; \
             fi",
        ),
        false,
        true,
    );

    b.pkg(
        "Valgrind",
        "Memory debugging and profiling suite for C and C++ programs. Detects memory leaks, \
         buffer overflows, use-after-free bugs, and uninitialized reads — without recompiling. \
         Essential when writing low-level or embedded code. \
         Example: `valgrind --leak-check=full --show-leak-kinds=all ./my_program`",
        InstallCmd::Apt(&["valgrind"]),
        false,
        true,
    );

    b.pkg(
        "bat",
        "A `cat` clone with syntax highlighting for hundreds of languages, line numbers, git \
         change indicators, and automatic paging. Makes reading source code in the terminal \
         much more pleasant than plain cat. Can be aliased as `cat` in bashrc. \
         Example: `bat src/main.rs` or `bat --style=plain Makefile`",
        InstallCmd::Apt(&["bat"]),
        false,
        true,
    );

    b.pkg(
        "Watchman",
        "File watching service developed by Meta. Monitors directories for changes and \
         triggers configurable actions in response. Used by test runners and build tools for \
         live reloading. Particularly useful in large codebases where Linux inotify watch \
         limits may be hit.",
        InstallCmd::Apt(&["watchman"]),
        false,
        true,
    );

    b.pkg(
        "FFmpeg",
        "Comprehensive multimedia framework for encoding, decoding, transcoding, muxing, and \
         streaming audio and video. Convert between virtually any format, extract frames, \
         strip audio, add subtitles, generate thumbnails, or livestream — all from the CLI. \
         The apt version on 22.04 (4.4.x) is compatible with the Mythos video pipeline. Do \
         NOT install FFmpeg from a PPA or snap that ships a different libavcodec SO version. \
         Example: `ffmpeg -i input.mov -c:v libx264 output.mp4`",
        InstallCmd::Apt(&["ffmpeg"]),
        false,
        true,
    );

    b.pkg(
        "ImageMagick",
        "Powerful image manipulation suite (convert, mogrify, identify commands). Resize, \
         crop, rotate, composite, apply filters, and batch-process images in virtually any \
         format from the command line. \
         Example: `convert input.png -resize 800x600 output.jpg` or \
         `mogrify -format webp -quality 85 *.png`",
        InstallCmd::Apt(&["imagemagick"]),
        false,
        true,
    );

    b.pkg(
        "fzf",
        "General-purpose interactive fuzzy finder. Pipe any list into it for instant \
         filtering — most commonly used as a shell history searcher (replaces Ctrl+R), \
         interactive file picker, and git branch selector. Comes with keybindings for \
         bash (Ctrl+R history, Ctrl+T file search, Alt+C cd). \
         Example: `git log --oneline | fzf` or `cd $(fd -t d | fzf)`",
        InstallCmd::Apt(&["fzf"]),
        false,
        true,
    );

    b.pkg(
        "hstr  (bash history)",
        "Greatly improved bash command history. Sorts entries by frequency and recency, \
         highlights matches, and lets you search, favorite, and re-run past commands \
         interactively. After install, add `eval \"$(hstr --show-configuration)\"` to \
         ~/.bashrc and bind it to Ctrl+R. Far more useful than readline's default \
         reverse-search — especially on a machine with thousands of stored commands.",
        InstallCmd::Apt(&["hstr"]),
        false,
        true,
    );

    b.pkg(
        "rsync",
        "Fast incremental file transfer and sync tool. Transfers only changed bytes \
         between source and destination, making repeated large-directory syncs much \
         faster than scp or cp. Supports SSH transport, compression, permission \
         preservation, delete sync, and dry-run mode. \
         Example: `rsync -avz --progress src/ user@host:/dest/`",
        InstallCmd::Apt(&["rsync"]),
        false,
        true,
    );

    b.pkg(
        "zstd  (compression)",
        "Facebook's fast lossless compression algorithm and CLI. Significantly faster \
         than gzip/bzip2 at comparable or better ratios — ideal for build artifacts, \
         backups, and container layers. Widely used in modern package formats and kernel \
         compression. Example: `zstd -T0 -19 archive.tar` (max compression, all CPU \
         threads) or `zstd -d file.zst` to decompress.",
        InstallCmd::Apt(&["zstd"]),
        false,
        true,
    );

    b.pkg(
        "detox",
        "Cleans up filenames by replacing or removing problematic characters — spaces, \
         parentheses, Unicode, and other characters that break scripts and shell \
         globbing. Handles accented characters, CJK, and various encodings. \
         Example: `detox -r ~/Downloads` cleans recursively; \
         `detox -n -r .` previews changes without applying them.",
        InstallCmd::Apt(&["detox"]),
        false,
        true,
    );

    b.pkg(
        "yt-dlp",
        "Feature-rich video and audio downloader supporting 1,000+ sites including \
         YouTube, Twitch, and SoundCloud. Fork of youtube-dl with active maintenance, \
         faster downloads, and richer format selection. Supports playlist download, \
         subtitle extraction, thumbnail embedding, and post-processing via FFmpeg. \
         Example: `yt-dlp -f 'bestvideo+bestaudio' URL` or \
         `yt-dlp --extract-audio --audio-format mp3 URL`",
        InstallCmd::Apt(&["yt-dlp"]),
        false,
        true,
    );

    b.pkg(
        "htop",
        "Interactive process viewer and system monitor. Shows CPU cores, memory, swap, \
         and a sortable/filterable process list with tree view. Lighter and more widely \
         available than bottom (btm). Universally expected on Linux systems. \
         Example: `htop` then press F4 to filter, F5 for tree view, F9 to kill.",
        InstallCmd::Apt(&["htop"]),
        false,
        true,
    );

    b.pkg(
        "tree",
        "Recursive directory listing that produces a tree-like indented output of file \
         and directory structure. Invaluable for understanding project layouts, documenting \
         directory hierarchies, and quick orientation in unfamiliar codebases. \
         Example: `tree -L 2 src/` or `tree -I 'node_modules|__pycache__'`",
        InstallCmd::Apt(&["tree"]),
        false,
        true,
    );

    b.pkg(
        "strace",
        "System call tracer for Linux. Intercepts and records every system call made by a \
         process and every signal received. Essential for debugging programs that fail \
         silently, diagnosing permission issues, and understanding program behavior at \
         the OS level. Pairs well with GDB and Valgrind. \
         Example: `strace -f -e trace=open,read,write ./program`",
        InstallCmd::Apt(&["strace"]),
        false,
        true,
    );

    b.pkg(
        "ShellCheck",
        "Static analysis tool for bash and shell scripts. Finds common bugs, pitfalls, \
         and style issues — unquoted variables, useless uses of cat, wrong test syntax, \
         and hundreds more. Integrates with most editors (Cursor, VS Code, Vim). Run on \
         any shell script before committing. \
         Example: `shellcheck install-all.sh`",
        InstallCmd::Apt(&["shellcheck"]),
        false,
        true,
    );

    b.pkg(
        "duf",
        "Modern replacement for `df` (disk free). Displays mounted filesystems with \
         usage bars, color-coded by fullness, in a clean table. Groups local disks, \
         network mounts, and special filesystems separately. Much easier to read than \
         `df -h`. Example: `duf` or `duf /home`",
        InstallCmd::Apt(&["duf"]),
        false,
        true,
    );

    b.pkg(
        "ncdu",
        "Interactive disk usage analyzer with an ncurses TUI. Scans a directory tree and \
         presents a sorted, navigable list of what's consuming space. Navigate with arrow \
         keys, press d to delete, and drill into subdirectories. Much faster than manually \
         running `du | sort`. Example: `ncdu /` or `ncdu ~/projects`",
        InstallCmd::Apt(&["ncdu"]),
        false,
        true,
    );

    b.pkg(
        "xclip",
        "Command-line interface to the X11 clipboard. Pipe stdout to the clipboard or \
         paste clipboard contents to stdout. Essential for terminal-to-GUI workflows. \
         Example: `cat file.txt | xclip -selection clipboard` (copy), \
         `xclip -selection clipboard -o` (paste), \
         or `pwd | xclip -sel clip` to copy the current path.",
        InstallCmd::Apt(&["xclip"]),
        false,
        true,
    );

    b.pkg(
        "pipx",
        "Install and run Python CLI applications in isolated virtual environments. Unlike \
         `pip install`, pipx gives each tool its own venv so packages never conflict. \
         The right way to install Python CLI tools globally (black, flake8, mypy, etc.) \
         without polluting your system Python. \
         Example: `pipx install black` then just run `black .`",
        InstallCmd::Apt(&["pipx"]),
        false,
        true,
    );

    b.pkg(
        "lazygit",
        "Terminal UI for git. Stage individual hunks, resolve merge conflicts, browse \
         commit history, manage branches, cherry-pick, rebase interactively — all from \
         a fast keyboard-driven TUI. Far more productive than raw git commands for \
         complex operations. Installed from the official GitHub release binary. \
         Launch with `lazygit` from any git repository.",
        InstallCmd::Script(
            "LAZYGIT_VERSION=$(curl -s https://api.github.com/repos/jesseduffield/lazygit/releases/latest \
             | grep -Po '\"tag_name\": \"v\\K[^\"]*') \
             && curl -Lo /tmp/lazygit.tar.gz \
             \"https://github.com/jesseduffield/lazygit/releases/latest/download/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz\" \
             && tar xf /tmp/lazygit.tar.gz -C /tmp lazygit \
             && install /tmp/lazygit /usr/local/bin \
             && rm -f /tmp/lazygit /tmp/lazygit.tar.gz",
        ),
        false,
        true,
    );

    b.pkg(
        "bottom  (btm)",
        "Cross-platform graphical system resource monitor for the terminal. Displays \
         CPU usage per core, memory, swap, disk I/O, network traffic, and a filterable \
         process list — all in one interactive TUI with scrollable graphs. Supports \
         zooming, process killing, and multiple layout presets. A modern, more readable \
         replacement for top and htop. Launch with `btm`. Installed via snap.",
        InstallCmd::Snap("bottom"),
        false,
        true,
    );

    // ── Containers ────────────────────────────────────────────────────────────
    b.cat("  Containers");

    b.pkg(
        "Docker  +  Docker Compose",
        "Industry-standard container platform. Package your application and all dependencies \
         into a portable container that runs identically on any machine. Docker Compose \
         (included) defines multi-container setups via a YAML file (app + db + cache + proxy). \
         NOTE: You must log out and back in after install for group permissions to take effect.",
        InstallCmd::Script(
            "curl -fsSL https://get.docker.com -o /tmp/get-docker.sh \
             && sh /tmp/get-docker.sh \
             && rm -f /tmp/get-docker.sh \
             && usermod -aG docker \"${SUDO_USER:-$USER}\" \
             && echo 'Log out and back in for Docker group permissions'",
        ),
        false,
        true,
    );

    // ── Security & Networking ─────────────────────────────────────────────────
    b.cat("  Security & Networking");

    b.pkg(
        "nmap",
        "Network exploration and security scanning tool. Discovers live hosts, open \
         ports, running services, software versions, and OS fingerprints across any \
         network. The industry-standard tool for network inventory and security auditing. \
         Example: `nmap -sV -O 192.168.1.0/24` (version + OS on subnet) or \
         `nmap -p 22,80,443 host` (check specific ports).",
        InstallCmd::Apt(&["nmap"]),
        false,
        true,
    );

    b.pkg(
        "netcat  (nc)",
        "OpenBSD netcat — the Swiss army knife of TCP/UDP networking. Create arbitrary \
         connections, set up simple listeners, transfer files, and debug network \
         services. Available as `nc` after install. \
         Example: `nc -zv host 22` (test port), `nc -l 9999 > file` (receive), \
         `nc host 9999 < file` (send), `nc -l 4444 | sh` (simple shell server).",
        InstallCmd::Apt(&["netcat-openbsd"]),
        false,
        true,
    );

    b.pkg(
        "aircrack-ng",
        "Complete 802.11 WiFi security auditing suite. Capture frames (airodump-ng), \
         inject packets (aireplay-ng), deauthenticate clients, capture WPA handshakes, \
         and crack WEP/WPA-PSK keys (aircrack-ng). Requires a WiFi adapter with monitor \
         mode support. Essential for authorized penetration testing on networks you own \
         or have explicit written permission to assess.",
        InstallCmd::Apt(&["aircrack-ng"]),
        false,
        true,
    );

    b.pkg(
        "wifite  +  hcxtools",
        "Automated WiFi auditing suite. wifite orchestrates the full capture workflow — \
         scanning for targets, deauthing clients, capturing WPA handshakes and PMKIDs — \
         with minimal manual steps. hcxtools converts captures to hashcat format for \
         offline GPU cracking. Requires monitor-mode hardware. For authorized use only \
         on networks you own or have explicit permission to test.",
        InstallCmd::Apt(&["wifite", "hcxtools"]),
        false,
        true,
    );

    b.pkg(
        "Tailscale",
        "Zero-config WireGuard-based mesh VPN. Every device gets a stable 100.x.x.x \
         IP reachable from anywhere with automatic NAT traversal — no port forwarding \
         or static IPs required. Free tier covers 100 devices. Supports Magic DNS for \
         hostname routing, subnet routing, exit nodes, and granular ACLs. After install, \
         run `sudo tailscale up` and authenticate via the printed URL. Via snap.",
        InstallCmd::Snap("tailscale"),
        false,
        true,
    );

    b.pkg(
        "NetBird",
        "Open-source WireGuard overlay network with peer-to-peer connections, automatic \
         NAT traversal, and an optional self-hosted control plane. Connect machines into \
         a private mesh network with ACLs, DNS routes, and network policies. Good choice \
         when you want the flexibility of a managed VPN but with self-hosting as an \
         option. After install: `sudo netbird up`. Installed via snap.",
        InstallCmd::Snap("netbird"),
        false,
        true,
    );

    b.pkg(
        "NordVPN",
        "Commercial VPN client with 6,000+ servers in 100+ countries. Uses NordLynx \
         (WireGuard-based), OpenVPN, or IKEv2. Includes Threat Protection Lite for \
         DNS-level ad and tracker blocking, an automatic kill switch, and split \
         tunneling. After install: `nordvpn login` then `nordvpn connect`. \
         Requires a NordVPN subscription. Installed via snap.",
        InstallCmd::Snap("nordvpn"),
        false,
        true,
    );

    b.pkg(
        "OpenSSH Server  (sshd)",
        "Secure Shell daemon — allows remote login to this machine over SSH. Not installed \
         by default on Ubuntu desktop. Essential for remote access, SCP/SFTP file transfers, \
         and tunneling. Starts automatically on boot after install. \
         After install: `sudo systemctl enable --now ssh`. \
         Connect from another machine: `ssh user@hostname`",
        InstallCmd::Apt(&["openssh-server"]),
        false,
        true,
    );

    b.pkg(
        "net-tools",
        "Legacy but widely-used networking utilities: ifconfig, netstat, route, arp, and \
         others. Officially superseded by iproute2 (`ip`, `ss`) but still expected by many \
         scripts, tutorials, and muscle-memory habits. Useful alongside the modern tools. \
         Example: `ifconfig`, `netstat -tlnp`, `route -n`",
        InstallCmd::Apt(&["net-tools"]),
        false,
        true,
    );

    b.pkg(
        "WireGuard Tools  (wg)",
        "Userspace utilities for WireGuard, the modern in-kernel VPN. Create, configure, \
         and manage WireGuard tunnels with `wg` and `wg-quick`. Useful even alongside \
         Tailscale or NetBird for manual point-to-point tunnels or debugging. The kernel \
         module is built-in since Linux 5.6. \
         Example: `sudo wg show` or `sudo wg-quick up wg0`",
        InstallCmd::Apt(&["wireguard-tools"]),
        false,
        true,
    );

    // ── Terminal & Shell ──────────────────────────────────────────────────────
    b.cat("  Terminal & Shell");

    b.pkg(
        "bash-completion",
        "Tab-completion scripts for bash. Press Tab to auto-complete command names, \
         sub-commands, file paths, options, and arguments for hundreds of programs including \
         git, apt, systemctl, ssh, and more. Makes navigating the shell dramatically faster \
         and helps discover available command options.",
        InstallCmd::Apt(&["bash-completion"]),
        false,
        true,
    );

    b.pkg(
        "GNOME Terminal",
        "Default terminal emulator for GNOME (standard Ubuntu desktop). Supports multiple \
         tabs, named profiles, custom fonts, transparency, and color schemes. A solid default \
         choice; popular alternatives include Kitty (GPU-accelerated), Alacritty (fast/minimal), \
         and WezTerm (Lua-configurable).",
        InstallCmd::Apt(&["gnome-terminal"]),
        false,
        true,
    );

    b.pkg(
        "tmux",
        "Terminal multiplexer — run multiple shell sessions inside a single terminal, \
         split into panes, and detach/reattach sessions that survive SSH disconnects. \
         Essential for remote work: start a long build, detach, disconnect, come back \
         later and reattach. Supports custom keybindings and scripting. \
         Example: `tmux new -s work`, then `Ctrl+B d` to detach, `tmux attach -t work`",
        InstallCmd::Apt(&["tmux"]),
        false,
        true,
    );

    // ── Rust Tools ────────────────────────────────────────────────────────────
    b.cat("  Rust Tools  (cargo required)");

    b.pkg(
        "Starship  (shell prompt)",
        "Minimal, fast, and infinitely customizable shell prompt written in Rust. Shows git \
         branch and status, language versions (Python, Node, Rust, Go...), exit codes, AWS \
         profile, and much more — only when relevant. After install, add \
         `eval \"$(starship init bash)\"` to ~/.bashrc. Configure via ~/.config/starship.toml. \
         Requires cargo — install Rust above first if needed.",
        InstallCmd::Cargo("starship"),
        false,
        false,
    );

    b.pkg(
        "Just  (task runner)",
        "Modern Make alternative with cleaner, more readable syntax and much better error \
         messages. Define project tasks in a justfile with optional parameters, env file \
         loading, and shell completions. Cross-platform and significantly more pleasant than \
         Makefiles. Example: `just build`, `just test filter`, `just deploy staging`. \
         Requires cargo — install Rust above first if needed.",
        InstallCmd::Cargo("just"),
        false,
        false,
    );

    // ── Python Packages ───────────────────────────────────────────────────────
    b.cat("  Python Packages  (pip required)");

    b.pkg(
        "pytest  +  pytest-mock  +  pytest-cov",
        "Python's most popular testing framework with two essential plugins. pytest provides \
         simple, readable test discovery with powerful fixtures. pytest-mock adds the `mocker` \
         fixture for clean, boilerplate-free mocking. pytest-cov generates HTML/XML coverage \
         reports. Example: `pytest tests/ -v --cov=src --cov-report=html`",
        InstallCmd::Pip(&["pytest", "pytest-mock", "pytest-cov"]),
        false,
        false,
    );

    b.pkg(
        "SQLAlchemy  (pinned 2.0.19)",
        "The most widely-used Python SQL toolkit and ORM (Object-Relational Mapper). Define \
         database tables as Python classes and query with Python instead of raw SQL. Supports \
         PostgreSQL, MySQL, SQLite — switch backends with minimal code changes. Used in Flask, \
         FastAPI, and standalone scripts. Pairs well with Alembic for migrations. Pinned to \
         2.0.19 for compatibility with the Mythos AV stack requirements.txt.",
        InstallCmd::Pip(&["SQLAlchemy==2.0.19"]),
        false,
        false,
    );

    b.pkg(
        "Pydantic  +  pydantic-settings",
        "Data validation and settings management using Python type hints. Define schemas as \
         Python classes and get automatic JSON validation, serialization, and clear error \
         messages. pydantic-settings loads config from environment variables and .env files \
         automatically. The backbone of FastAPI and a great pattern for any Python app.",
        InstallCmd::Pip(&["pydantic", "pydantic-settings"]),
        false,
        false,
    );

    b.pkg(
        "black  (code formatter)",
        "Opinionated Python code formatter — the \"uncompromising\" formatter. Automatically \
         reformats your entire codebase to a consistent style with zero configuration needed. \
         Run `black .` to format all Python files in place, or use as a pre-commit hook. \
         Integrates with most editors and CI pipelines.",
        InstallCmd::Pip(&["black"]),
        false,
        false,
    );

    b.pkg(
        "flake8  (linter)",
        "Python style guide enforcement tool. Checks code against PEP 8, detects common \
         errors (undefined names, unused imports, unreachable code), and measures cyclomatic \
         complexity. Fast and easy to add to CI. Configure via .flake8 or setup.cfg. \
         Example: `flake8 src/ --max-line-length=100`",
        InstallCmd::Pip(&["flake8"]),
        false,
        false,
    );

    b.pkg(
        "mypy  (static type checker)",
        "Optional static type checker for Python. Add type hints to your functions and mypy \
         catches type mismatches at analysis-time rather than at runtime. Works great with \
         Pydantic and modern Python 3.10+ syntax (e.g. `int | None`). Gradually adoptable — \
         start with the most critical modules. Run `mypy src/` to check.",
        InstallCmd::Pip(&["mypy"]),
        false,
        false,
    );

    b.pkg(
        "requests  (pinned 2.31.0)",
        "The most popular HTTP library for Python — simple, elegant, and human-friendly. \
         Make GET, POST, PUT, DELETE requests with automatic JSON handling, session management, \
         authentication helpers, and connection pooling. Pinned to 2.31.0 for compatibility \
         with the Mythos AV stack requirements.txt. \
         Example: `r = requests.get(url, headers={...}); data = r.json()`",
        InstallCmd::Pip(&["requests==2.31.0"]),
        false,
        false,
    );

    // ── Fonts ─────────────────────────────────────────────────────────────────
    b.cat("  Fonts");

    b.pkg(
        "fonts-liberation",
        "Liberation font family — metrically compatible with Microsoft's Times New Roman, \
         Arial, and Courier New. Documents and web pages designed for Windows fonts render \
         with the correct sizing and spacing. Liberation Mono is also a clean monospace \
         option suitable for terminal use.",
        InstallCmd::Apt(&["fonts-liberation"]),
        false,
        true,
    );

    b.pkg(
        "fonts-dejavu",
        "DejaVu font family with excellent Unicode character coverage far beyond what \
         standard Latin-only fonts provide. DejaVu Sans Mono is popular for terminal use \
         due to its readability and wide language support across Latin, Greek, Cyrillic, \
         Hebrew, Arabic, and many more scripts.",
        InstallCmd::Apt(&["fonts-dejavu"]),
        false,
        true,
    );

    b.pkg(
        "FiraCode Nerd Font",
        "FiraCode with Nerd Fonts glyph patches applied — adds ~3,600 icons (file type icons, \
         git branch symbols, powerline arrows, devicons, Font Awesome, etc.) on top of \
         FiraCode's programming ligatures. Required for full Starship prompt glyph support \
         and any terminal theme that uses Powerline or devicon symbols. Downloads the latest \
         release zip from GitHub, installs to ~/.local/share/fonts/, refreshes fc-cache, \
         then sets the system monospace font and GNOME Terminal default profile font to \
         FiraCode Nerd Font Mono 11 via gsettings.",
        InstallCmd::Script(
            "apt-get install -y unzip \
             && curl -fLo /tmp/FiraCode.zip \
             https://github.com/ryanoasis/nerd-fonts/releases/latest/download/FiraCode.zip \
             && mkdir -p ~/.local/share/fonts/FiraCodeNerdFont \
             && unzip -o /tmp/FiraCode.zip -d ~/.local/share/fonts/FiraCodeNerdFont \
             && fc-cache -fv \
             && rm /tmp/FiraCode.zip \
             && gsettings set org.gnome.desktop.interface monospace-font-name 'FiraCode Nerd Font Mono 11' \
             && PROFILE=$(gsettings get org.gnome.Terminal.ProfilesList default | tr -d \"'\") \
             && gsettings set \"org.gnome.Terminal.Legacy.Profile:/org/gnome/terminal/legacy/profiles:/:${PROFILE}/\" use-system-font false \
             && gsettings set \"org.gnome.Terminal.Legacy.Profile:/org/gnome/terminal/legacy/profiles:/:${PROFILE}/\" font 'FiraCode Nerd Font Mono 11'",
        ),
        false,
        false,
    );

    // ── Snap Applications ─────────────────────────────────────────────────────
    b.cat("  Snap Applications  (snapd required)");

    b.pkg(
        "Discord  (snap)",
        "Voice, video, and text communication platform. Widely used by developer communities, \
         open source projects, and teams. Supports screen share, rich presence, bots, webhooks, \
         and role-based channel permissions. Installed via snap.",
        InstallCmd::Snap("discord"),
        false,
        true,
    );

    b.pkg(
        "Slack  (snap)",
        "Team messaging and collaboration platform. Organized into channels by topic with \
         direct messaging, file sharing, video/audio huddles, and integrations with GitHub, \
         Jira, PagerDuty, Google Calendar, and other dev tools. Installed via snap.",
        InstallCmd::Snap("slack"),
        false,
        true,
    );

    b.pkg(
        "Spotify  (snap)",
        "Music streaming service with a catalog of 100M+ tracks. Great for background music \
         during long coding sessions. The desktop app supports media key controls \
         (play/pause, next/prev) and system notifications. Installed via snap.",
        InstallCmd::Snap("spotify"),
        false,
        true,
    );

    b.pkg(
        "Notion  (snap)",
        "All-in-one workspace for notes, documentation, databases, kanban boards, and project \
         management. Great for personal knowledge bases, team wikis, and meeting notes. \
         Embed code blocks, tables, calendars, and more. Installed via snap as notion-snap.",
        InstallCmd::Snap("notion-snap"),
        false,
        true,
    );

    b.pkg(
        "NordPass  (snap)",
        "Secure password manager with a zero-knowledge encrypted vault. Store passwords, \
         passkeys, credit cards, and secure notes. Syncs across all devices. Developed by \
         Nord Security (makers of NordVPN), using XChaCha20 encryption. Installed via snap.",
        InstallCmd::Snap("nordpass"),
        false,
        true,
    );

    // ── Desktop Applications ──────────────────────────────────────────────────
    b.cat("  Desktop Applications");

    b.pkg(
        "SimpleScreenRecorder",
        "Easy-to-use screen recording application for Linux. Record your full screen, a \
         specific window, or a custom rectangular region. Supports multiple output formats \
         (MP4, MKV, WebM) and quality settings. Good for tutorials, bug reproductions, and \
         demo videos.",
        InstallCmd::Apt(&["simplescreenrecorder"]),
        false,
        true,
    );

    b.pkg(
        "VeraCrypt",
        "Disk encryption software and successor to the discontinued TrueCrypt. Create \
         encrypted file containers or encrypt entire partitions and drives. Supports AES, \
         Serpent, Twofish, and cascade combinations, plus hidden volumes for plausible \
         deniability. Note: may not be in default Ubuntu repos.",
        InstallCmd::Apt(&["veracrypt"]),
        false,
        true,
    );

    b.pkg(
        "NoMachine",
        "High-performance remote desktop solution using the NX protocol — significantly \
         faster than VNC or RDP for both LAN and WAN connections. Supports full desktop \
         access, multi-session management, file transfer, audio/video streaming, and USB \
         device forwarding. Ideal for accessing Ubuntu desktops remotely with near-native \
         responsiveness. Fetches the latest .deb directly from nomachine.com and installs \
         with dpkg. The nxserver service starts automatically on boot.",
        InstallCmd::Script(
            "ARCH=$(dpkg --print-architecture) \
             && NM_URL=$(curl -fsSL 'https://www.nomachine.com/download/linux&id=1' \
             | grep -oP 'https://download\\.nomachine\\.com/download/[^\"]+\\.deb' \
             | grep \"${ARCH}\" | head -1) \
             && [ -n \"$NM_URL\" ] \
             && curl -fsSL \"$NM_URL\" -o /tmp/nomachine.deb \
             && (dpkg -i /tmp/nomachine.deb || apt install -f -y) \
             && rm -f /tmp/nomachine.deb",
        ),
        false,
        true,
    );

    b.pkg(
        "GNOME Tweaks",
        "Advanced GNOME settings not exposed in the standard Settings app. Control \
         titlebar buttons (add minimize/maximize), adjust font rendering and hinting, \
         manage GNOME Shell extensions, configure startup applications, and set \
         cursor/icon/GTK/shell themes. Essential for anyone who wants meaningful \
         control over their GNOME desktop appearance and behavior beyond the defaults.",
        InstallCmd::Apt(&["gnome-tweaks"]),
        false,
        true,
    );

    b.pkg(
        "GNOME Shell Extension Manager",
        "Native GUI for discovering, installing, enabling, and configuring GNOME Shell \
         extensions without a browser or the extensions.gnome.org web UI. Browse the \
         full extensions catalog, toggle extensions on/off, access per-extension \
         settings, and check for updates — all from a clean desktop app. Replaces the \
         cumbersome browser-extension workflow.",
        InstallCmd::Apt(&["gnome-shell-extension-manager"]),
        false,
        true,
    );

    b.pkg(
        "GRUB Customizer",
        "GUI for configuring the GRUB2 boot loader. Reorder and rename boot entries, \
         set the default OS, change the timeout, enable/disable the splash screen, and \
         set boot resolution — without manually editing grub config files. Useful on \
         dual-boot systems where GRUB isn't ordering entries correctly or you want a \
         cleaner boot menu. Requires the danielrichter2007 PPA — the installer adds \
         it automatically before installing.",
        InstallCmd::Script(
            "add-apt-repository -y ppa:danielrichter2007/grub-customizer \
             && apt update && apt install -y grub-customizer",
        ),
        false,
        true,
    );

    b.pkg(
        "Solaar",
        "Linux manager for Logitech Unifying, Bolt, and Bluetooth devices. Pair and \
         unpair mice, keyboards, and headsets; view battery levels; configure per-device \
         DPI, scroll speed, and button assignments; and toggle Logitech-specific \
         protocol features. Essential for Logitech hardware users — Linux has no native \
         Logitech Options equivalent.",
        InstallCmd::Apt(&["solaar"]),
        false,
        true,
    );

    b.pkg(
        "Meld",
        "Visual diff and merge tool for files and directories. Side-by-side comparison \
         with inline highlighting, three-way merge support, and directory diff/sync. \
         Integrates with git as a difftool and mergetool. Far more readable than terminal \
         diffs for large changes. \
         Setup: `git config --global diff.tool meld` then `git difftool`",
        InstallCmd::Apt(&["meld"]),
        false,
        true,
    );

    b.pkg(
        "Peek",
        "Simple animated GIF screen recorder. Select a screen region and record to GIF, \
         APNG, MP4, or WebM. Ideal for quick bug reproductions, PR demos, and documentation. \
         Much lighter than SimpleScreenRecorder when you just need a short clip. \
         Limitation: Wayland support is limited — works best on X11.",
        InstallCmd::Apt(&["peek"]),
        false,
        true,
    );

    b.pkg(
        "Google Chrome",
        "Google's web browser built on Chromium. Adds Google account sync, Widevine DRM \
         (required for Netflix and other streaming services), Google Translate, and \
         enhanced Chrome Web Store integration on top of the Chromium base. Adds \
         Google's apt repository so Chrome updates automatically with `apt upgrade`. \
         Sets up the signing key and apt source, then installs via apt.",
        InstallCmd::Script(
            "curl -fsSL https://dl.google.com/linux/linux_signing_key.pub \
             | gpg --dearmor --yes -o /usr/share/keyrings/google-chrome.gpg \
             && echo \"deb [arch=amd64 \
             signed-by=/usr/share/keyrings/google-chrome.gpg] \
             https://dl.google.com/linux/chrome/deb/ stable main\" \
             | tee /etc/apt/sources.list.d/google-chrome.list > /dev/null \
             && apt update && apt install -y google-chrome-stable",
        ),
        false,
        true,
    );

    b.pkg(
        "Signal",
        "End-to-end encrypted messaging, voice, and video calls. Open-source protocol, \
         no ads, no metadata collection, independently audited — the gold standard for \
         private communication. The desktop app syncs with your phone and supports \
         note-to-self, group chats, disappearing messages, and voice/video. Adds \
         Signal's official apt repository for automatic updates.",
        InstallCmd::Script(
            "curl -fsSL https://updates.signal.org/desktop/apt/keys.asc \
             | gpg --dearmor --yes -o /usr/share/keyrings/signal-desktop-keyring.gpg \
             && echo \"deb [arch=amd64 \
             signed-by=/usr/share/keyrings/signal-desktop-keyring.gpg] \
             https://updates.signal.org/desktop/apt xenial main\" \
             | tee /etc/apt/sources.list.d/signal-xenial.list > /dev/null \
             && apt update && apt install -y signal-desktop",
        ),
        false,
        true,
    );

    b.pkg(
        "Claude  (desktop)",
        "Anthropic's Claude AI assistant as a native desktop application. Full-featured \
         interface with conversation history, file uploads, artifact rendering, and \
         Projects for long-context work. Installed from the community-maintained Debian \
         package repository at aaddrick.github.io/claude-desktop-debian, which tracks \
         official Claude Desktop releases and provides apt-managed updates so Claude \
         stays current with `apt upgrade`.",
        InstallCmd::Script(
            "curl -fsSL \
             https://aaddrick.github.io/claude-desktop-debian/public-key.gpg \
             | gpg --dearmor --yes -o /usr/share/keyrings/claude-desktop.gpg \
             && echo \"deb [signed-by=/usr/share/keyrings/claude-desktop.gpg \
             arch=$(dpkg --print-architecture)] \
             https://aaddrick.github.io/claude-desktop-debian stable main\" \
             | tee /etc/apt/sources.list.d/claude-desktop.list > /dev/null \
             && apt update && apt install -y claude-desktop",
        ),
        false,
        true,
    );

    b.build()
}

// ─── Display Helpers ──────────────────────────────────────────────────────────

fn cmd_short(cmd: &InstallCmd) -> String {
    match cmd {
        InstallCmd::Apt(pkgs) => format!("apt install -y {}", pkgs.join(" ")),
        InstallCmd::Script(s) => {
            let first = s.lines().next().unwrap_or("").trim();
            if first.len() > 62 {
                format!("{}...", &first[..59])
            } else {
                first.to_string()
            }
        }
        InstallCmd::Cargo(name) => format!("cargo install {}", name),
        InstallCmd::Pip(pkgs) => format!("pip3 install {}", pkgs.join(" ")),
        InstallCmd::Snap(name) => format!("snap install {}", name),
    }
}

/// Returns (dot glyph, badge label, accent color) per install type.
fn type_meta(cmd: &InstallCmd) -> (&'static str, &'static str, Color) {
    match cmd {
        InstallCmd::Apt(_)    => ("●", "apt",   Color::Cyan),
        InstallCmd::Script(_) => ("●", "sh",    Color::LightGreen),
        InstallCmd::Cargo(_)  => ("●", "cargo", Color::LightMagenta),
        InstallCmd::Pip(_)    => ("●", "pip",   Color::LightBlue),
        InstallCmd::Snap(_)   => ("●", "snap",  Color::Yellow),
    }
}

fn progress_bar(selected: usize, total: usize, bar_w: usize) -> String {
    if total == 0 || bar_w == 0 {
        return format!("[{}]", "░".repeat(bar_w));
    }
    let filled = (selected * bar_w / total).min(bar_w);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_w - filled))
}

// ─── UI Rendering ─────────────────────────────────────────────────────────────

// Palette shortcuts — keep styling consistent across all render fns.
const C_BORDER:  Color = Color::Cyan;
const C_CURSOR:  Color = Color::Rgb(22, 52, 95);   // dark navy row bg
const C_CAT:     Color = Color::Yellow;
const C_DIM:     Color = Color::Rgb(90, 90, 110);  // muted separator / meta
const C_ROOT:    Color = Color::LightRed;
const C_OK:      Color = Color::LightGreen;
const C_WARN:    Color = Color::Yellow;

fn render(f: &mut Frame, app: &mut App) {
    // Solid dark canvas behind everything
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        f.area(),
    );
    match app.screen {
        Screen::Select  => render_select(f, app),
        Screen::Confirm => render_confirm(f, app),
    }
}

fn render_select(f: &mut Frame, app: &mut App) {
    let area = f.area();
    // Title bar is 4 rows normally; grows to 5 when not root to fit the warning line.
    let title_h = if app.is_root { 4 } else { 5 };
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(title_h), // title bar
            Constraint::Min(0),          // list | description
            Constraint::Length(3),       // controls
        ])
        .split(area);

    render_title(f, app, outer[0]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(outer[1]);

    render_package_list(f, app, cols[0]);
    render_description(f, app, cols[1]);
    render_controls(f, app, outer[2]);
}

// ── Title bar ────────────────────────────────────────────────────────────────

fn render_title(f: &mut Frame, app: &App, area: Rect) {
    let inner_w = area.width.saturating_sub(2) as usize;
    let total    = app.packages.len();
    let selected = app.selected_count();

    // Line 1 – app name left-justified, selection count right-justified
    let left  = "  Ubuntu Dev Environment Installer";
    let right = format!("{}/{} selected  ", selected, total);
    let gap   = inner_w.saturating_sub(left.len() + right.len());

    let line1 = Line::from(vec![
        Span::styled(left,  Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" ".repeat(gap)),
        Span::styled(
            right,
            if selected > 0 {
                Style::default().fg(C_OK).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]);

    // Line 2 – inline keybind cheat-sheet
    let k = |s: &'static str| Span::styled(s, Style::default().fg(C_WARN).add_modifier(Modifier::BOLD));
    let d = |s: &'static str| Span::styled(s, Style::default().fg(Color::DarkGray));
    let dot = Span::styled("  ·  ", Style::default().fg(C_DIM));

    let line2 = Line::from(vec![
        Span::raw("  "),
        k("↑↓"), d(" navigate"), dot.clone(),
        k("Space"), d(" toggle"), dot.clone(),
        k("A"), d(" all"), dot.clone(),
        k("N"), d(" none"), dot.clone(),
        Span::styled("Enter", Style::default().fg(C_OK).add_modifier(Modifier::BOLD)),
        d(" review"), dot.clone(),
        Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        d(" quit"),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .title(" ubuntu-installer ")
        .title_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    if app.is_root {
        f.render_widget(Paragraph::new(vec![line1, line2]).block(block), area);
    } else {
        let locked_count = app.packages.iter().filter(|p| p.requires_root).count();
        let line3 = Line::from(vec![
            Span::styled(
                "  ⚠ Not running as root — ",
                Style::default().fg(C_WARN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} package(s) requiring sudo are locked. ", locked_count),
                Style::default().fg(C_WARN),
            ),
            Span::styled(
                "Re-run with: sudo ./installer",
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
        ]);
        f.render_widget(Paragraph::new(vec![line1, line2, line3]).block(block), area);
    }
}

// ── Package list ─────────────────────────────────────────────────────────────

fn render_package_list(f: &mut Frame, app: &mut App, area: Rect) {
    let inner_w = area.width.saturating_sub(2) as usize;

    //  Row layout (chars):
    //  "▶ " (2) + "● " (2) + "[x] " (4) + <name> + " [root]  " (9)
    //   prefix = 8,  suffix = 9,  name gets the rest
    const PREFIX: usize = 8;
    const SUFFIX: usize = 9;
    let name_w = inner_w.saturating_sub(PREFIX + SUFFIX);

    let cursor   = app.cursor;
    let cur_bg   = Style::default().bg(C_CURSOR);
    let cur_bold = Style::default().bg(C_CURSOR).fg(Color::White).add_modifier(Modifier::BOLD);

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| match entry {
            // ── Category header ──────────────────────────────────────────────
            Entry::Category(name) => {
                let head  = format!("  ─── {} ", name);
                let hlen  = head.chars().count();
                let fill  = if inner_w > hlen { "─".repeat(inner_w - hlen) } else { String::new() };
                ListItem::new(Line::from(Span::styled(
                    format!("{}{}", head, fill),
                    Style::default().fg(C_CAT).add_modifier(Modifier::BOLD),
                )))
            }

            // ── Package row ──────────────────────────────────────────────────
            Entry::Pkg(idx) => {
                let pkg       = &app.packages[*idx];
                let is_cursor = i == cursor;
                let locked    = !app.is_root && pkg.requires_root;
                let (dot, _badge, dot_col) = type_meta(&pkg.cmd);

                // Cursor arrow (2 chars)
                let (arrow, arrow_style) = if is_cursor && !locked {
                    ("▶ ", Style::default().fg(Color::Cyan).bg(C_CURSOR).add_modifier(Modifier::BOLD))
                } else if is_cursor {
                    ("▶ ", Style::default().fg(C_DIM).bg(C_CURSOR))
                } else {
                    ("  ", Style::default().fg(C_DIM))
                };

                // Type dot (1 char + 1 space = 2 chars)
                let dot_style = if locked {
                    Style::default().fg(C_DIM)
                } else if is_cursor {
                    Style::default().fg(Color::White).bg(C_CURSOR)
                } else {
                    Style::default().fg(dot_col)
                };

                // Checkbox "[x] " (4 chars) — locked packages show "[-]"
                let (ch, ch_col) = if locked {
                    ("-", C_DIM)
                } else if pkg.selected {
                    ("x", if is_cursor { C_OK } else { Color::Green })
                } else {
                    (" ", Color::DarkGray)
                };
                let brk_col   = if locked { C_DIM } else if is_cursor { Color::White } else { C_DIM };
                let brk_style = Style::default().fg(brk_col)
                    .bg(if is_cursor { C_CURSOR } else { Color::Reset });
                let ch_style  = Style::default().fg(ch_col)
                    .bg(if is_cursor { C_CURSOR } else { Color::Reset });

                // Name (padded / truncated to name_w chars)
                let display_name = if pkg.name.len() <= name_w {
                    format!("{:<width$}", pkg.name, width = name_w)
                } else if name_w >= 2 {
                    format!("{}..", &pkg.name[..name_w.saturating_sub(2)])
                } else {
                    " ".repeat(name_w)
                };
                let name_style = if locked {
                    Style::default().fg(C_DIM)
                } else if is_cursor {
                    cur_bold
                } else if pkg.selected {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                // Root badge " [root]  " (9 chars) or 9 spaces
                // Locked packages show " [sudo]  " in dim to signal why they're locked.
                let (root_txt, root_sty) = if locked {
                    (
                        " [sudo]  ",
                        Style::default().fg(C_DIM)
                            .bg(if is_cursor { C_CURSOR } else { Color::Reset }),
                    )
                } else if pkg.requires_root {
                    (
                        " [root]  ",
                        if is_cursor {
                            Style::default().fg(C_ROOT).bg(C_CURSOR)
                        } else {
                            Style::default().fg(C_ROOT)
                        },
                    )
                } else {
                    ("         ", Style::default().bg(if is_cursor { C_CURSOR } else { Color::Reset }))
                };

                let line = Line::from(vec![
                    Span::styled(arrow,        arrow_style),
                    Span::styled(dot,          dot_style),
                    Span::raw(" "),
                    Span::styled("[",          brk_style),
                    Span::styled(ch,           ch_style),
                    Span::styled("] ",         brk_style),
                    Span::styled(display_name, name_style),
                    Span::styled(root_txt,     root_sty),
                ]);

                if is_cursor {
                    ListItem::new(line).style(cur_bg)
                } else {
                    ListItem::new(line)
                }
            }
        })
        .collect();

    let legend = " ● apt  ● sh  ● cargo  ● pip  ● snap ";
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .title(format!(" Packages ({} total) ", app.packages.len()))
        .title_style(Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD))
        .title_bottom(
            ratatui::text::Line::from(Span::styled(legend, Style::default().fg(C_DIM)))
        );

    // Use default highlight (no-op) — we do all coloring in each ListItem.
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default());

    f.render_stateful_widget(list, area, &mut app.list_state);
}

// ── Description panel ────────────────────────────────────────────────────────

fn render_description(f: &mut Frame, app: &App, area: Rect) {
    let inner_w = area.width.saturating_sub(2) as usize;
    let sep     = "─".repeat(inner_w);

    let (title, lines) = match app.current_pkg_idx() {
        None => (
            " Details ".to_string(),
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Use ↑↓ / j k to navigate",
                    Style::default().fg(C_DIM),
                )),
            ],
        ),
        Some(i) => {
            let pkg = &app.packages[i];
            let (dot, badge, dot_col) = type_meta(&pkg.cmd);
            let mut ls: Vec<Line> = vec![];

            // Description (wraps naturally with Paragraph)
            ls.push(Line::from(""));
            ls.push(Line::from(Span::styled(
                pkg.description,
                Style::default().fg(Color::White),
            )));
            ls.push(Line::from(""));

            // Separator
            ls.push(Line::from(Span::styled(sep.clone(), Style::default().fg(C_DIM))));
            ls.push(Line::from(""));

            // Type row
            ls.push(Line::from(vec![
                Span::styled("  Type   ", Style::default().fg(C_DIM)),
                Span::styled(dot, Style::default().fg(dot_col)),
                Span::raw(" "),
                Span::styled(badge, Style::default().fg(dot_col).add_modifier(Modifier::BOLD)),
            ]));

            // Root row
            let (root_str, root_col) = if pkg.requires_root {
                ("yes  (sudo required)", C_ROOT)
            } else {
                ("no", C_OK)
            };
            ls.push(Line::from(vec![
                Span::styled("  Root   ", Style::default().fg(C_DIM)),
                Span::styled(root_str, Style::default().fg(root_col).add_modifier(Modifier::BOLD)),
            ]));

            ls.push(Line::from(""));
            ls.push(Line::from(Span::styled(sep, Style::default().fg(C_DIM))));
            ls.push(Line::from(""));

            // Command
            ls.push(Line::from(vec![
                Span::styled("  $ ", Style::default().fg(C_DIM)),
                Span::styled(
                    cmd_short(&pkg.cmd),
                    Style::default().fg(C_OK).add_modifier(Modifier::BOLD),
                ),
            ]));

            (format!(" {} ", pkg.name), ls)
        }
    };

    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(title)
                .title_style(Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

// ── Controls / footer ────────────────────────────────────────────────────────

fn render_controls(f: &mut Frame, app: &App, area: Rect) {
    let selected = app.selected_count();
    let total    = app.packages.len();
    let bar      = progress_bar(selected, total, 20);

    // Title carries the progress bar + count
    let bar_title = format!(" {} {}/{} packages ", bar, selected, total);
    let bar_style = if selected > 0 {
        Style::default().fg(C_OK).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(C_DIM)
    };

    let k   = |s: &'static str| Span::styled(s, Style::default().fg(C_WARN).add_modifier(Modifier::BOLD));
    let d   = |s: &'static str| Span::styled(s, Style::default().fg(Color::DarkGray));
    let dot = Span::styled("  ·  ", Style::default().fg(C_DIM));

    let line = Line::from(vec![
        Span::raw("  "),
        k("↑↓"), d(" nav"),     dot.clone(),
        k("Spc"), d(" toggle"),  dot.clone(),
        k("A"), d(" all"),       dot.clone(),
        k("N"), d(" none"),      dot.clone(),
        k("PgUp/Dn"), d(" jump"), dot.clone(),
        Span::styled("Enter", Style::default().fg(C_OK).add_modifier(Modifier::BOLD)),
        d(" install"),           dot.clone(),
        Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        d(" quit"),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .title(bar_title)
        .title_style(bar_style);

    f.render_widget(Paragraph::new(line).block(block), area);
}

// ── Confirm screen ────────────────────────────────────────────────────────────

fn render_confirm(f: &mut Frame, app: &App) {
    let area    = f.area();
    let inner_w = area.width.saturating_sub(2) as usize;
    let sep     = "─".repeat(inner_w);
    let selected = app.selected_packages();

    let mut lines: Vec<Line> = vec![];

    if selected.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  No packages selected.",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  Press B or Esc to go back.",
            Style::default().fg(C_DIM),
        )));
    } else {
        // Group packages by install type and render each group
        let type_groups: &[(&str, Color, fn(&InstallCmd) -> bool)] = &[
            ("APT",   Color::Cyan,         |c| matches!(c, InstallCmd::Apt(_))),
            ("SH",    Color::LightGreen,   |c| matches!(c, InstallCmd::Script(_))),
            ("CARGO", Color::LightMagenta, |c| matches!(c, InstallCmd::Cargo(_))),
            ("PIP",   Color::LightBlue,    |c| matches!(c, InstallCmd::Pip(_))),
            ("SNAP",  Color::Yellow,       |c| matches!(c, InstallCmd::Snap(_))),
        ];

        for (group_label, group_color, group_filter) in type_groups {
            let group: Vec<&&Package> = selected.iter().filter(|p| group_filter(&p.cmd)).collect();
            if group.is_empty() {
                continue;
            }

            // Section header
            let head  = format!("  ─── {} ", group_label);
            let hlen  = head.chars().count();
            let fill  = if inner_w > hlen { "─".repeat(inner_w - hlen) } else { String::new() };
            lines.push(Line::from(vec![
                Span::styled(head, Style::default().fg(*group_color).add_modifier(Modifier::BOLD)),
                Span::styled(fill, Style::default().fg(C_DIM)),
            ]));
            lines.push(Line::from(""));

            for pkg in group {
                lines.push(Line::from(vec![
                    Span::styled("   ● ", Style::default().fg(*group_color).add_modifier(Modifier::BOLD)),
                    Span::styled(pkg.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    if pkg.requires_root {
                        Span::styled("  [root]", Style::default().fg(C_ROOT))
                    } else {
                        Span::raw("")
                    },
                ]));
                lines.push(Line::from(vec![
                    Span::raw("       "),
                    Span::styled("$ ", Style::default().fg(C_DIM)),
                    Span::styled(cmd_short(&pkg.cmd), Style::default().fg(C_DIM)),
                ]));
                lines.push(Line::from(""));
            }
        }

        // ── Warnings ──────────────────────────────────────────────────────────
        let mut warned = false;
        let push_sep = |ls: &mut Vec<Line>, first: &mut bool| {
            if !*first { return; }
            *first = false;
            ls.push(Line::from(Span::styled(sep.clone(), Style::default().fg(C_DIM))));
            ls.push(Line::from(""));
        };

        if app.has_selected_cargo() && !app.rust_will_be_installed() {
            push_sep(&mut lines, &mut !warned);
            warned = true;
            lines.push(Line::from(vec![
                Span::styled("  [!] ", Style::default().fg(C_WARN).add_modifier(Modifier::BOLD)),
                Span::styled("Cargo tools selected but Rust is not. Ensure `cargo` is in PATH,", Style::default().fg(C_WARN)),
            ]));
            lines.push(Line::from(Span::styled(
                "       or press B and add Rust to your selection.",
                Style::default().fg(C_DIM),
            )));
            lines.push(Line::from(""));
        }

        if app.has_selected_pip() && !app.python_will_be_installed() {
            push_sep(&mut lines, &mut !warned);
            warned = true;
            lines.push(Line::from(vec![
                Span::styled("  [!] ", Style::default().fg(C_WARN).add_modifier(Modifier::BOLD)),
                Span::styled("Python packages selected but Python is not. Ensure `pip3` is in PATH,", Style::default().fg(C_WARN)),
            ]));
            lines.push(Line::from(Span::styled(
                "       or press B and add Python to your selection.",
                Style::default().fg(C_DIM),
            )));
            lines.push(Line::from(""));
        }

        if selected.iter().any(|p| p.requires_root) {
            push_sep(&mut lines, &mut !warned);
            lines.push(Line::from(vec![
                Span::styled("  [!] ", Style::default().fg(C_WARN).add_modifier(Modifier::BOLD)),
                Span::styled("Packages marked [root] require sudo. Run with: ", Style::default().fg(C_WARN)),
                Span::styled("sudo ./installer", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(""));
        }

        let _ = warned; // suppress unused warning
    }

    // Footer keybinds embedded in the bottom border
    let bottom = " Enter: install  ·  B/Esc: back  ·  ↑↓/j k: scroll  ·  Q: quit ";

    let title = format!(
        " Review Installation  ·  {}/{} packages ",
        app.selected_count(),
        app.packages.len()
    );

    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(title)
                .title_style(Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD))
                .title_bottom(
                    ratatui::text::Line::from(Span::styled(bottom, Style::default().fg(C_DIM)))
                ),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.confirm_scroll, 0));

    f.render_widget(para, area);
}

// ─── Installation ─────────────────────────────────────────────────────────────

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn run_install(packages: Vec<Package>) {
    if packages.is_empty() {
        println!("No packages selected.");
        return;
    }

    let g = "\x1b[1;32m";
    let r = "\x1b[1;31m";
    let y = "\x1b[1;33m";
    let c = "\x1b[1;36m";
    let x = "\x1b[0m";
    let bar = "=".repeat(52);

    println!("\n{c}{bar}{x}");
    println!("{c}  Ubuntu Dev Environment Installer{x}");
    println!("{c}{bar}{x}\n");

    let has_apt_or_script = packages
        .iter()
        .any(|p| matches!(p.cmd, InstallCmd::Apt(_) | InstallCmd::Script(_)));

    if has_apt_or_script {
        println!("{y}-> Updating apt package lists...{x}");
        match Command::new("apt").arg("update").status() {
            Ok(s) if s.success() => println!("{g}   Package lists updated.{x}\n"),
            _ => println!("{y}   apt update failed — continuing anyway.{x}\n"),
        }
    }

    let total = packages.len();
    for (idx, pkg) in packages.iter().enumerate() {
        println!("{c}[{}/{}]{x} {y}Installing: {}{x}", idx + 1, total, pkg.name);

        let real_user = std::env::var("SUDO_USER").unwrap_or_else(|_| {
            std::env::var("USER").unwrap_or_else(|_| "root".to_string())
        });

        let result = match &pkg.cmd {
            InstallCmd::Apt(pkgs) => Command::new("apt")
                .args(["install", "-y"])
                .args(*pkgs)
                .status(),
            InstallCmd::Script(script) => Command::new("sh").args(["-c", script]).status(),
            InstallCmd::Cargo(name) => {
                let real_home = std::env::var("SUDO_USER")
                    .ok()
                    .and_then(|u| {
                        Command::new("sh")
                            .args(["-c", &format!("eval echo ~{}", u)])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    })
                    .unwrap_or_else(|| std::env::var("HOME").unwrap_or_default());
                let cargo_bin = format!("{}/.cargo/bin/cargo", real_home);
                Command::new("sudo")
                    .args(["-u", &real_user, &cargo_bin, "install", name])
                    .status()
            }
            InstallCmd::Pip(pkgs) => Command::new("pip3").arg("install").args(*pkgs).status(),
            InstallCmd::Snap(name) => Command::new("snap").args(["install", name]).status(),
        };

        match result {
            Ok(s) if s.success() => {
                println!("{g}   [ok] {} installed successfully.{x}\n", pkg.name)
            }
            Ok(s) => println!(
                "{r}   [fail] {} exited with code {:?}.{x}\n",
                pkg.name,
                s.code()
            ),
            Err(e) => println!(
                "{r}   [error] Could not launch installer for {}: {}.{x}\n",
                pkg.name, e
            ),
        }
    }

    println!("{g}{bar}{x}");
    println!("{g}  Done! Check output above for any failures.{x}");
    println!("{g}{bar}{x}\n");
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut do_install = false;

    loop {
        terminal.draw(|f| render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.screen {
                Screen::Select => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::PageUp => {
                        for _ in 0..10 {
                            app.move_up();
                        }
                    }
                    KeyCode::PageDown => {
                        for _ in 0..10 {
                            app.move_down();
                        }
                    }
                    KeyCode::Char(' ') => app.toggle(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.select_all(),
                    KeyCode::Char('n') | KeyCode::Char('N') => app.select_none(),
                    KeyCode::Enter => {
                        app.screen = Screen::Confirm;
                        app.confirm_scroll = 0;
                    }
                    _ => {}
                },
                Screen::Confirm => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc => {
                        app.screen = Screen::Select;
                    }
                    KeyCode::Enter => {
                        if app.selected_count() > 0 {
                            do_install = true;
                            break;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.confirm_scroll = app.confirm_scroll.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.confirm_scroll = app.confirm_scroll.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        app.confirm_scroll = app.confirm_scroll.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        app.confirm_scroll = app.confirm_scroll.saturating_add(10);
                    }
                    _ => {}
                },
            }
        }
    }

    let selected: Vec<Package> = if do_install {
        app.selected_packages().into_iter().cloned().collect()
    } else {
        vec![]
    };

    // Restore terminal before printing install output
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if do_install {
        run_install(selected);
    }

    Ok(())
}
