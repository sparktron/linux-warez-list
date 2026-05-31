#!/usr/bin/env python3
"""Generate HTML mock-ups of the installer-tui for README screenshots.

Package data is sourced live from `./installer --dump-json` so this file
never needs manual updates when packages are added or removed from main.rs.
"""

import html as H
import json
import os
import subprocess
import textwrap

# ── Terminal geometry ──────────────────────────────────────────────────────────
W   = 100   # total columns
LW  = 60    # left panel total width
RW  = 40    # right panel total width
LI  = LW - 2   # left inner  = 58
RI  = RW - 2   # right inner = 38

PREFIX   = 8          # "▶ ● [x] "
SUFFIX   = 9          # " [root]  "
NAME_W   = LI - PREFIX - SUFFIX   # 41

# ── Load package data from the binary ─────────────────────────────────────────

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def _find_installer():
    candidates = [
        os.path.join(REPO_ROOT, "installer"),
        os.path.join(REPO_ROOT, "installer-tui", "target", "release", "installer-tui"),
        os.path.join(REPO_ROOT, "installer-tui", "target", "debug",   "installer-tui"),
    ]
    for p in candidates:
        if os.path.isfile(p) and os.access(p, os.X_OK):
            return p
    raise FileNotFoundError(
        "No installer binary found. Build with `cargo build` in installer-tui/ "
        "or run `cargo build --release` and copy the binary to the repo root as `installer`."
    )

def load_packages():
    binary = _find_installer()
    result = subprocess.run([binary, "--dump-json"], capture_output=True, text=True, check=True)
    return json.loads(result.stdout)

PKGS = load_packages()
TOTAL = len(PKGS)

# ── CSS color class per cmd_type ───────────────────────────────────────────────
DOT_CLS = {"apt": "c", "sh": "lg", "cargo": "lm", "pip": "lb", "snap": "lyl"}

# Cursor package for the select screen mock-up
CURSOR_NAME = "fzf"

# Packages shown in the confirm screen mock-up: all default-selected plus one
# representative package per remaining install type.
_CONFIRM_EXTRAS = {
    "Docker  +  Docker Compose",
    "Starship  (shell prompt)",
    "pytest  +  pytest-mock  +  pytest-cov",
    "Notion  (snap)",
}
CONFIRM_PKGS = [
    p for p in PKGS
    if p["default_selected"] or p["name"] in _CONFIRM_EXTRAS
]

# ── Helpers ────────────────────────────────────────────────────────────────────

def e(t):
    return H.escape(str(t))

def sp(text, cls=""):
    text = str(text)
    return f'<span class="{cls}">{e(text)}</span>' if cls else e(text)

def pad(text, width):
    n = len(text)
    return text + " " * max(0, width - n) if n <= width else text[:width]

def hbar(n):
    return "─" * n

def wrap_desc(desc, width):
    """Wrap a long description string to `width` chars per line."""
    return textwrap.wrap(desc, width=width) or [""]

# ── CSS ────────────────────────────────────────────────────────────────────────

CSS = """
* { margin:0; padding:0; box-sizing:border-box; }
body { background:#111; }
.t {
    display: inline-block;
    background: #0d0d0d;
    font-family: 'Consolas','Courier New',Courier,monospace;
    font-size: 13.5px;
    line-height: 1.4;
    padding: 10px 14px;
    color: #c0c0c0;
    border-radius: 6px;
}
.row { display:block; white-space:pre; }
.hl  { background:#16345F; }

.c   { color:#00d7d7; }
.cb  { color:#00d7d7; font-weight:bold; }
.yb  { color:#ffff00; font-weight:bold; }
.g   { color:#5fff5f; }
.gb  { color:#5fff5f; font-weight:bold; }
.lr  { color:#ff6060; }
.lrb { color:#ff6060; font-weight:bold; }
.lm  { color:#ff87ff; }
.lb  { color:#87d7ff; }
.lyl { color:#ffd75f; }
.lg  { color:#87ff87; }
.dim { color:#5a5a80; }
.wh  { color:#ffffff; }
.wb  { color:#ffffff; font-weight:bold; }
.dg  { color:#585858; }
.inv { background:#00d7d7; color:#000000; font-weight:bold; }
"""

# ── Row builder ────────────────────────────────────────────────────────────────

def row(*spans, hl=False):
    content = "".join(sp(t, c) for t, c in spans)
    cls = "row hl" if hl else "row"
    return f'<div class="{cls}">{content}</div>'

# ── Right-panel helpers ────────────────────────────────────────────────────────

def right_empty():
    return [("│","c"), (" "*RI,""), ("│","c")]

def right_text(text, cls="wh"):
    line = " " + pad(text, RI-1)
    return [("│","c"), (line, cls), ("│","c")]

def right_sep():
    return [("│","c"), (hbar(RI),"dim"), ("│","c")]

def right_field(label, value, val_cls):
    label_s = f"  {label}  "
    val_pad = pad(value, RI - len(label_s))
    return [("│","c"), (label_s,"dim"), (val_pad, val_cls), ("│","c")]

def right_cmd(cmd):
    inner = "  $ " + cmd
    pad_s = " " * max(0, RI - len(inner))
    return [("│","c"), ("  $ ","dim"), (cmd,"gb"), (pad_s,""), ("│","c")]

# ── Build select screen ────────────────────────────────────────────────────────

def build_select():
    lines = []

    # Find cursor package and build its right-panel description lines
    cursor_pkg = next((p for p in PKGS if p["name"] == CURSOR_NAME), PKGS[0])
    desc_lines = wrap_desc(cursor_pkg["description"], RI - 2)
    dot_cls     = DOT_CLS[cursor_pkg["cmd_type"]]
    type_label  = cursor_pkg["cmd_type"]
    root_txt    = "yes  (sudo required)" if cursor_pkg["requires_root"] else "no"
    root_cls    = "lr" if cursor_pkg["requires_root"] else "g"

    # Build right-panel rows
    right_rows = (
        [right_empty()]
        + [right_text(l) for l in desc_lines]
        + [right_empty(), right_sep(), right_empty()]
        + [right_field("Type", f"● {type_label}", dot_cls)]
        + [right_field("Root", root_txt, root_cls)]
        + [right_empty(), right_sep(), right_empty()]
        + [right_cmd(f"{type_label} install -y {cursor_pkg['name']}")]
        + [right_empty()] * 20
    )

    # Count default-selected
    n_selected = sum(1 for p in PKGS if p["default_selected"])
    selected_lbl = f"{n_selected}/{TOTAL} selected  "

    # Title bar
    label = " ubuntu-installer "
    lines.append(row(("╭","c"), (label,"inv"), (hbar(W-2-len(label)),"c"), ("╮","c")))

    lh = "  Ubuntu Dev Environment Installer"
    lines.append(row(
        ("│","c"), (lh,"cb"),
        (" "*(W-2-len(lh)-len(selected_lbl)), ""),
        (selected_lbl,"gb"), ("│","c")
    ))
    lines.append(row(
        ("│","c"), ("  ",""),
        ("Space","yb"), (" toggle  ·  ","dim"),
        ("A","yb"), (" all  ·  ","dim"),
        ("N","yb"), (" none  ·  ","dim"),
        ("Enter","gb"), (" review  ·  ","dim"),
        ("Q","lr"), (" quit","dim"),
        (" "*24,""), ("│","c"),
    ))
    lines.append(row(("╰"+hbar(W-2)+"╯","c")))

    # Panel top borders
    pt = f" Packages ({TOTAL} total) "
    dt = f" {cursor_pkg['name']} "
    lines.append(row(
        ("╭","c"),(pt,"cb"),(hbar(LW-2-len(pt)),"c"),("╮","c"),
        ("╭","c"),(dt,"cb"),(hbar(RW-2-len(dt)),"c"),("╮","c"),
    ))

    # Content rows — iterate flat list (categories + packages)
    ri = 0
    seen_cats = set()
    for pkg in PKGS:
        cat = pkg["category"]
        if cat not in seen_cats:
            seen_cats.add(cat)
            head = f"  {hbar(3)} {cat} "
            fill = hbar(max(0, LI - len(head)))
            l_spans = [("│","c"), (head+fill,"yb"), ("│","c")]
            rr = right_rows[ri] if ri < len(right_rows) else right_empty()
            all_spans = l_spans + rr
            content = "".join(sp(t,c) for t,c in all_spans)
            lines.append(f'<div class="row">{content}</div>')
            ri += 1

        is_cursor = pkg["name"] == CURSOR_NAME
        selected  = pkg["default_selected"]
        requires_root = pkg["requires_root"]
        dcls = DOT_CLS[pkg["cmd_type"]]

        arrow     = "▶ " if is_cursor else "  "
        arrow_cls = "cb" if is_cursor else "dim"
        dot_col   = "wh" if is_cursor else dcls
        check     = "x" if selected else " "
        check_col = ("gb" if is_cursor else "g") if selected else "dg"
        brk_col   = "dim"
        name_col  = "wb" if (is_cursor or selected) else "wh"
        name_d    = pad(pkg["name"], NAME_W)
        suffix    = " [root]  " if requires_root else "         "
        suf_col   = ("lrb" if is_cursor else "lr") if requires_root else ""
        l_spans   = [
            ("│","c"), (arrow,arrow_cls), ("●",dot_col), (" ",""),
            ("[",brk_col), (check,check_col), ("] ",brk_col),
            (name_d,name_col), (suffix,suf_col), ("│","c"),
        ]
        rr = right_rows[ri] if ri < len(right_rows) else right_empty()
        all_spans = l_spans + rr
        content = "".join(sp(t,c) for t,c in all_spans)
        cls = "row hl" if is_cursor else "row"
        lines.append(f'<div class="{cls}">{content}</div>')
        ri += 1

    # Panel bottom borders
    leg = " ● apt  ● sh  ● cargo  ● pip  ● snap "
    lines.append(row(
        ("╰","c"),(leg,"dim"),(hbar(LW-2-len(leg)),"c"),("╯","c"),
        ("╰","c"),(hbar(RI),"c"),("╯","c"),
    ))

    # Controls
    bar = "[████░░░░░░░░░░░░░░░░]"
    ct  = f" {bar} {n_selected}/{TOTAL} packages "
    lines.append(row(("╭","c"),(ct,"gb"),(hbar(W-2-len(ct)),"c"),("╮","c")))
    lines.append(row(
        ("│","c"), ("  ",""),
        ("↑↓","yb"),(" nav  ·  ","dim"),
        ("Spc","yb"),(" toggle  ·  ","dim"),
        ("A","yb"),(" all  ·  ","dim"),
        ("N","yb"),(" none  ·  ","dim"),
        ("PgUp/Dn","yb"),(" jump  ·  ","dim"),
        ("Enter","gb"),(" install  ·  ","dim"),
        ("Q","lr"),(" quit","dim"),
        ("   ",""), ("│","c"),
    ))
    lines.append(row(("╰"+hbar(W-2)+"╯","c")))

    return lines

# ── Build confirm screen ───────────────────────────────────────────────────────

CMDS = {
    "apt":   "apt install -y ...",
    "sh":    "curl ... | sh",
    "cargo": "cargo install ...",
    "pip":   "pip3 install ...",
    "snap":  "snap install ...",
}

TYPE_LABELS = {"apt":"APT","sh":"SH","cargo":"CARGO","pip":"PIP","snap":"SNAP"}
TYPE_COLORS = {"apt":"c","sh":"lg","cargo":"lm","pip":"lb","snap":"lyl"}

def build_confirm():
    lines = []
    IW = W - 2  # inner = 98
    n_confirm = len(CONFIRM_PKGS)

    title = f" Review Installation  ·  {n_confirm}/{TOTAL} packages "
    lines.append(row(("╭","c"),(title,"cb"),(hbar(W-2-len(title)),"c"),("╮","c")))
    lines.append(row(("│","c"),(" "*IW,""),("│","c")))

    # Group by cmd_type preserving order
    groups: dict[str, list] = {}
    order:  list[str] = []
    for pkg in CONFIRM_PKGS:
        t = pkg["cmd_type"]
        if t not in groups:
            groups[t] = []
            order.append(t)
        groups[t].append(pkg)

    for typ in order:
        pkgs = groups[typ]
        col  = TYPE_COLORS[typ]
        lbl  = TYPE_LABELS[typ]
        head = f"  {hbar(3)} {lbl} "
        fill = hbar(IW - len(head))
        lines.append(row(("│","c"),(head,col),(fill,"dim"),("│","c")))
        lines.append(row(("│","c"),(" "*IW,""),("│","c")))

        cmd_txt = CMDS[typ]
        for pkg in pkgs:
            pname  = pkg["name"]
            root   = "[root]  " if pkg["requires_root"] else "        "
            ppad   = " " * max(0, IW - 5 - len(pname) - 8)
            lines.append(row(
                ("│","c"),("   ",""),("● ",col),(pname,"wb"),(ppad,""),(root,"lr"),("│","c")
            ))
            cpad = " " * max(0, IW - len(f"       $ {cmd_txt}"))
            lines.append(row(("│","c"),("       $ ","dim"),(cmd_txt,"dim"),(cpad,""),("│","c")))
            lines.append(row(("│","c"),(" "*IW,""),("│","c")))

    lines.append(row(("│","c"),(hbar(IW),"dim"),("│","c")))
    lines.append(row(("│","c"),(" "*IW,""),("│","c")))
    wline_txt = "Packages marked [root] require sudo.  Run with: "
    wline_cmd = "sudo ./installer"
    wpad_n = IW - 8 - len(wline_txt) - len(wline_cmd)
    lines.append(row(
        ("│","c"),("  [!] ","yb"),
        (wline_txt,"yb"),
        (wline_cmd,"wb"),
        (" "*max(0,wpad_n),""),("│","c")))
    lines.append(row(("│","c"),(" "*IW,""),("│","c")))

    bot = " Enter: install  ·  B/Esc: back  ·  ↑↓/j k: scroll  ·  Q: quit "
    lines.append(row(("╰","c"),(bot,"dim"),(hbar(W-2-len(bot)),"c"),("╯","c")))

    return lines

# ── Render to HTML ─────────────────────────────────────────────────────────────

def render(lines):
    return "\n".join(lines)

def page(body, title=""):
    return f"""<!DOCTYPE html>
<html><head>
<meta charset="UTF-8">
<title>{title}</title>
<style>{CSS}</style>
</head>
<body><div class="t">
{body}
</div></body></html>"""

out = os.path.dirname(os.path.abspath(__file__))

select_html  = page(render(build_select()),  "installer-tui — select")
confirm_html = page(render(build_confirm()), "installer-tui — confirm")

with open(f"{out}/screenshot-select.html",  "w") as f: f.write(select_html)
with open(f"{out}/screenshot-confirm.html", "w") as f: f.write(confirm_html)

print(f"Done → {out}  ({TOTAL} packages, {len(CONFIRM_PKGS)} on confirm screen)")
