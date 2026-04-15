#!/usr/bin/env python3
"""Generate HTML mock-ups of the installer-tui for README screenshots."""

import html as H
import os

# ── Terminal geometry ──────────────────────────────────────────────────────────
W   = 100   # total columns
LW  = 60    # left panel total width
RW  = 40    # right panel total width
LI  = LW - 2   # left inner  = 58
RI  = RW - 2   # right inner = 38

PREFIX   = 8          # "▶ ● [x] "
SUFFIX   = 9          # " [root]  "
NAME_W   = LI - PREFIX - SUFFIX   # 41

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

# ── Package definitions ────────────────────────────────────────────────────────
# (name, dot_color_cls, selected, requires_root, is_cursor)
# None for dot_color_cls = category header

PKGS = [
    # name                              dot    sel    root   cursor
    ("System Tools",                    None,  None,  None,  False),
    ("build-essential",                 "c",   True,  True,  False),
    ("git",                             "c",   True,  True,  False),
    ("gh  (GitHub CLI)",                "lg",  False, True,  False),
    ("linux-lowlatency  (kernel)",      "c",   False, True,  False),
    ("snapd",                           "lg",  False, True,  False),
    ("curl",                            "c",   True,  True,  False),
    ("wget",                            "c",   True,  True,  False),
    ("unzip",                           "c",   True,  True,  False),
    ("Languages & Runtimes",            None,  None,  None,  False),
    ("Python 3.10  +  pip  +  venv",    "c",   True,  True,  False),
    ("Node.js 20  +  npm",              "lg",  False, True,  False),
    ("Rust  (via rustup)",              "lg",  False, False, False),
    ("CLI Tools",                       None,  None,  None,  False),
    ("ripgrep  (rg)",                   "c",   True,  True,  False),
    ("bat",                             "c",   False, True,  False),
    ("fzf",                             "c",   False, True,  True),   # ← cursor
    ("hstr  (bash history)",            "c",   False, True,  False),
    ("rsync",                           "c",   False, True,  False),
    ("yt-dlp",                          "c",   False, True,  False),
    ("htop",                            "c",   False, True,  False),
    ("tree",                            "c",   False, True,  False),
    ("strace",                          "c",   False, True,  False),
    ("ShellCheck",                      "c",   False, True,  False),
    ("duf",                             "c",   False, True,  False),
    ("ncdu",                            "c",   False, True,  False),
    ("xclip",                           "c",   False, True,  False),
    ("pipx",                            "c",   False, True,  False),
    ("lazygit",                         "lg",  False, True,  False),
    ("bottom  (btm)",                   "lyl", False, True,  False),
    ("Containers",                      None,  None,  None,  False),
    ("Docker  +  Docker Compose",       "lg",  False, True,  False),
    ("Security & Networking",           None,  None,  None,  False),
    ("nmap",                            "c",   False, True,  False),
    ("netcat  (nc)",                    "c",   False, True,  False),
    ("aircrack-ng",                     "c",   False, True,  False),
    ("wifite  +  hcxtools",             "c",   False, True,  False),
    ("Tailscale",                       "lyl", False, True,  False),
    ("NetBird",                         "lyl", False, True,  False),
    ("NordVPN",                         "lyl", False, True,  False),
    ("OpenSSH Server  (sshd)",          "c",   False, True,  False),
    ("net-tools",                       "c",   False, True,  False),
    ("WireGuard Tools  (wg)",           "c",   False, True,  False),
    ("Desktop Applications",            None,  None,  None,  False),
    ("Google Chrome",                   "lg",  False, True,  False),
    ("Signal",                          "lg",  False, True,  False),
    ("Claude  (desktop)",               "lg",  False, True,  False),
    ("GNOME Tweaks",                    "c",   False, True,  False),
    ("Solaar",                          "c",   False, True,  False),
    ("Meld",                            "c",   False, True,  False),
    ("Peek",                            "c",   False, True,  False),
]

# fzf description (cursor package, wrapped to RI-2 = 36 chars per line)
FZFDESC = [
    "General-purpose interactive fuzzy",
    "finder. Pipe any list for instant",
    "filtering — shell history search",
    "(Ctrl+R), file picker, git branch",
    "selector. Shell keybindings",
    "installed automatically.",
    "",
    "$ git log --oneline | fzf",
    "$ cd $(fd -t d | fzf)",
]

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

# ── Left panel rows ────────────────────────────────────────────────────────────

def left_row(pkg):
    name, dot_cls, selected, requires_root, is_cursor = pkg

    if dot_cls is None:
        head = f"  {hbar(3)} {name} "
        fill = hbar(max(0, LI - len(head)))
        return row(("│","c"), (head + fill, "yb"), ("│","c"))

    arrow     = "▶ " if is_cursor else "  "
    arrow_cls = "cb" if is_cursor else "dim"
    dot_col   = "wh" if is_cursor else dot_cls
    check     = "x" if selected else " "
    check_col = ("gb" if is_cursor else "g") if selected else "dg"
    brk_col   = "dim"
    name_col  = "wb" if (is_cursor or selected) else "wh"
    name_d    = pad(name, NAME_W)
    suffix    = " [root]  " if requires_root else "         "
    suf_col   = ("lrb" if is_cursor else "lr") if requires_root else ""

    return row(
        ("│","c"),
        (arrow, arrow_cls),
        ("●", dot_col),
        (" ", ""),
        ("[", brk_col),
        (check, check_col),
        ("] ", brk_col),
        (name_d, name_col),
        (suffix, suf_col),
        ("│","c"),
        hl=is_cursor,
    )

# ── Right panel rows ───────────────────────────────────────────────────────────

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

# Build right panel lines
RIGHT_ROWS = (
    [right_empty()]
    + [right_text(l) for l in FZFDESC]
    + [right_empty(), right_sep(), right_empty()]
    + [right_field("Type","● apt", "cb")]
    + [right_field("Root","yes  (sudo required)", "lr")]
    + [right_empty(), right_sep(), right_empty()]
    + [right_cmd("apt install -y fzf")]
    + [right_empty()] * 20
)

# ── Build selection screen ─────────────────────────────────────────────────────

def build_select():
    lines = []

    # Title bar
    label = " ubuntu-installer "
    lines.append(row(("╭","c"), (label,"inv"), (hbar(W-2-len(label)),"c"), ("╮","c")))

    lh = "  Ubuntu Dev Environment Installer"
    rh = "8/84 selected  "
    lines.append(row(
        ("│","c"), (lh,"cb"),
        (" "*(W-2-len(lh)-len(rh)), ""),
        (rh,"gb"), ("│","c")
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
    pt = " Packages (84 total) "
    dt = " fzf "
    lines.append(row(
        ("╭","c"),(pt,"cb"),(hbar(LW-2-len(pt)),"c"),("╮","c"),
        ("╭","c"),(dt,"cb"),(hbar(RW-2-len(dt)),"c"),("╮","c"),
    ))

    # Content rows
    ri = 0
    for pkg in PKGS:
        name, dot_cls, selected, requires_root, cursor = pkg
        if dot_cls is None:
            head = f"  {hbar(3)} {name} "
            fill = hbar(max(0, LI - len(head)))
            l_spans = [("│","c"), (head+fill,"yb"), ("│","c")]
        else:
            arrow     = "▶ " if cursor else "  "
            arrow_cls = "cb" if cursor else "dim"
            dot_col   = "wh" if cursor else dot_cls
            check     = "x" if selected else " "
            check_col = ("gb" if cursor else "g") if selected else "dg"
            brk_col   = "dim"
            name_col  = "wb" if (cursor or selected) else "wh"
            name_d    = pad(name, NAME_W)
            suffix    = " [root]  " if requires_root else "         "
            suf_col   = ("lrb" if cursor else "lr") if requires_root else ""
            l_spans = [
                ("│","c"), (arrow,arrow_cls), ("●",dot_col), (" ",""),
                ("[",brk_col), (check,check_col), ("] ",brk_col),
                (name_d,name_col), (suffix,suf_col), ("│","c"),
            ]
        rr = RIGHT_ROWS[ri] if ri < len(RIGHT_ROWS) else right_empty()
        all_spans = l_spans + rr
        content = "".join(sp(t,c) for t,c in all_spans)
        cls = "row hl" if cursor else "row"
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
    ct  = f" {bar} 8/84 packages "
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

CONFIRM_PKGS = [
    ("build-essential",        "c",   "apt"),
    ("git",                    "c",   "apt"),
    ("Python 3.10 + pip",      "c",   "apt"),
    ("ripgrep  (rg)",          "c",   "apt"),
    ("nmap",                   "c",   "apt"),
    ("gh  (GitHub CLI)",       "lg",  "sh"),
    ("Docker + Compose",       "lg",  "sh"),
    ("Google Chrome",          "lg",  "sh"),
    ("Signal",                 "lg",  "sh"),
    ("Starship",               "lm",  "cargo"),
    ("Just  (task runner)",    "lm",  "cargo"),
    ("pytest + mock + cov",    "lb",  "pip"),
    ("SQLAlchemy",             "lb",  "pip"),
    ("Discord",                "lyl", "snap"),
    ("Tailscale",              "lyl", "snap"),
    ("NordVPN",                "lyl", "snap"),
]

CMDS = {
    "apt":   ("apt install -y ...",  "c"),
    "sh":    ("curl ... | sh",       "lg"),
    "cargo": ("cargo install ...",   "lm"),
    "pip":   ("pip3 install ...",    "lb"),
    "snap":  ("snap install ...",    "lyl"),
}

def build_confirm():
    lines = []
    IW = W - 2  # inner = 98

    title = " Review Installation  ·  16/84 packages "
    lines.append(row(("╭","c"),(title,"cb"),(hbar(W-2-len(title)),"c"),("╮","c")))
    lines.append(row(("│","c"),(" "*IW,""),("│","c")))

    groups = {}
    order  = []
    for name, dot_cls, typ in CONFIRM_PKGS:
        if typ not in groups:
            groups[typ] = []
            order.append(typ)
        groups[typ].append((name, dot_cls))

    type_labels = {"apt":"APT","sh":"SH","cargo":"CARGO","pip":"PIP","snap":"SNAP"}
    type_colors = {"apt":"c","sh":"lg","cargo":"lm","pip":"lb","snap":"lyl"}

    for typ in order:
        pkgs = groups[typ]
        col  = type_colors[typ]
        lbl  = type_labels[typ]
        head = f"  {hbar(3)} {lbl} "
        fill = hbar(IW - len(head))
        lines.append(row(("│","c"),(head,col),(fill,"dim"),("│","c")))
        lines.append(row(("│","c"),(" "*IW,""),("│","c")))

        cmd_txt, _ = CMDS[typ]
        for pname, pdot in pkgs:
            ppad  = " " * max(0, IW - 5 - len(pname) - 8)
            root  = "[root]  " if typ in ("apt","sh","snap") else "        "
            lines.append(row(
                ("│","c"),("   ",""),("● ",col),(pname,"wb"),(ppad,""),(root,"lr"),("│","c")
            ))
            cline = f"       $ {cmd_txt}"
            cpad  = " " * max(0, IW - len(cline))
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

print("Done →", out)
