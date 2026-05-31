# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Read the full agent rules: @AGENTS.md

## What this is

Curated Ubuntu 22.04 LTS (x86-64) dev-environment installer. Two install paths
that are **hand-maintained mirrors**, not generated from one source: a Rust
ratatui TUI (`installer-tui/src/main.rs`) and a headless bash script
(`install-all.sh`).

## Release notes and changelog

`CHANGELOG.md` at the repo root is the authoritative record of versioned
changes. When making a significant change (new packages, bug fixes, architecture
changes), add an entry under the current version. See AGENTS.md for the full
file map and conventions.

## Critical rules (don't get these wrong)

- **Sync rule.** Every package change must update all of these in lockstep:
  `installer-tui/src/main.rs` (`build_data()`), `install-all.sh`, `README.md`
  (table + total count), `LINUX_WAREZ_LIST.md`. `docs/gen_screenshots.py`
  **does not need editing** for package changes — it reads live data from
  `./installer --dump-json`. See AGENTS.md "The Sync Rule".

- **Rebuild + copy the binary after any `main.rs` change.** The committed
  `installer` at repo root is a pre-built binary that must match source:
  ```bash
  cd installer-tui && cargo build --release
  /usr/bin/cp target/release/installer-tui ../installer && chmod +x ../installer
  ```
  Use `/usr/bin/cp` — the shell aliases `cp` to `cp -i`, which prompts.

- **Mythos AV stack pins.** This machine also runs `~/mythos`. Don't break
  version pins (Clang 14, clang-format-12, SQLAlchemy==2.0.19, requests==2.31.0,
  FFmpeg 4.4.x from apt only, CMake 3.18.1 from source, linux-lowlatency). Check
  `~/mythos/third_party/rules_python/requirements.txt` before adding/upgrading
  pip packages. See AGENTS.md "Mythos AV Stack Compatibility".

- **Everything runs non-interactively.** Both installers run unattended (`apt -y`,
  etc.). Any new `Script` command must not prompt.

- **Screenshot generator is self-updating.** After rebuilding `installer`,
  regenerate screenshots by running `python3 docs/gen_screenshots.py`. No
  manual edits to that file are needed for package additions.

## Style

- Rust: standard `rustfmt`. Bash: 2-space indent, use the `log`/`warn`/`error`
  helpers, and keep the emoji in section headers and helpers — they're intentional.
