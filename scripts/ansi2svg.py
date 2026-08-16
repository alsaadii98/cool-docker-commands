#!/usr/bin/env python3
"""Render ANSI-coloured terminal output to a standalone SVG.

Used to generate the screenshots in README.md and the website from real `dok`
output, so the docs can never drift from what the tool actually prints.

    dok ps -a --color=always | scripts/ansi2svg.py --out docs/img/ps.svg --title "dok ps -a"
"""

import argparse
import html
import re
import sys

SGR = re.compile(r"\x1b\[([0-9;]*)m")
OTHER_ESC = re.compile(r"\x1b\[[0-9;?]*[A-Za-z]")

# Type-safe defaults tuned for a 14px monospace stack.
CHAR_W = 8.4
LINE_H = 20.0
PAD_X = 18.0
PAD_Y = 16.0
TITLEBAR = 34.0

BG = "#11141b"
FG = "#bec4d0"
DOTS = ["#ff5f57", "#febc2e", "#28c840"]

# The 16 ANSI colours, for output that does not use truecolor.
BASIC = {
    30: "#3b4048", 31: "#e06c75", 32: "#98c379", 33: "#e5c07b",
    34: "#61afef", 35: "#c678dd", 36: "#56b6c2", 37: "#bec4d0",
    90: "#6a7382", 91: "#e06c75", 92: "#98c379", 93: "#e5c07b",
    94: "#61afef", 95: "#c678dd", 96: "#56b6c2", 97: "#ffffff",
}


class Style:
    __slots__ = ("fg", "bold", "underline")

    def __init__(self):
        self.fg = None
        self.bold = False
        self.underline = False

    def copy(self):
        s = Style()
        s.fg, s.bold, s.underline = self.fg, self.bold, self.underline
        return s

    def same(self, other):
        return (self.fg, self.bold, self.underline) == (other.fg, other.bold, other.underline)


def apply(style, params):
    """Apply one SGR sequence's parameters to `style`."""
    codes = [int(p) if p else 0 for p in params.split(";")] if params else [0]
    i = 0
    while i < len(codes):
        c = codes[i]
        if c == 0:
            style.fg, style.bold, style.underline = None, False, False
        elif c == 1:
            style.bold = True
        elif c == 4:
            style.underline = True
        elif c in (22, 21):
            style.bold = False
        elif c == 24:
            style.underline = False
        elif c == 39:
            style.fg = None
        elif c in BASIC:
            style.fg = BASIC[c]
        elif c == 38 and i + 4 < len(codes) and codes[i + 1] == 2:
            style.fg = "#{:02x}{:02x}{:02x}".format(*codes[i + 2 : i + 5])
            i += 4
        elif c == 38 and i + 2 < len(codes) and codes[i + 1] == 5:
            style.fg = xterm256(codes[i + 2])
            i += 2
        i += 1


def xterm256(n):
    if n < 16:
        return BASIC.get(30 + n if n < 8 else 82 + n, FG)
    if n < 232:
        n -= 16
        levels = [0, 95, 135, 175, 215, 255]
        return "#{:02x}{:02x}{:02x}".format(
            levels[n // 36], levels[(n // 6) % 6], levels[n % 6]
        )
    v = 8 + (n - 232) * 10
    return "#{:02x}{:02x}{:02x}".format(v, v, v)


def parse(text):
    """Split ANSI text into lines of (text, style) runs."""
    lines = []
    style = Style()
    for raw in text.replace("\r\n", "\n").split("\n"):
        raw = OTHER_ESC.sub(lambda m: m.group(0) if m.group(0).endswith("m") else "", raw)
        runs, pos = [], 0
        for m in SGR.finditer(raw):
            chunk = raw[pos : m.start()]
            if chunk:
                runs.append((chunk, style.copy()))
            apply(style, m.group(1))
            pos = m.end()
        tail = raw[pos:]
        if tail:
            runs.append((tail, style.copy()))
        lines.append(runs)
    # Trailing blank lines add nothing but height.
    while lines and not any(t.strip() for t, _ in lines[-1]):
        lines.pop()
    return lines


def render(lines, title, font):
    cols = max((sum(len(t) for t, _ in runs) for runs in lines), default=0)
    width = PAD_X * 2 + cols * CHAR_W
    height = TITLEBAR + PAD_Y * 2 + len(lines) * LINE_H

    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0f}" height="{height:.0f}" '
        f'viewBox="0 0 {width:.0f} {height:.0f}" font-family="{html.escape(font)}" '
        f'font-size="14">',
        f'<rect width="{width:.0f}" height="{height:.0f}" rx="10" fill="{BG}"/>',
        f'<rect width="{width:.0f}" height="{TITLEBAR:.0f}" rx="10" fill="#181c25"/>',
        f'<rect y="{TITLEBAR - 10:.0f}" width="{width:.0f}" height="10" fill="#181c25"/>',
    ]
    for i, colour in enumerate(DOTS):
        out.append(f'<circle cx="{20 + i * 18}" cy="{TITLEBAR / 2:.0f}" r="6" fill="{colour}"/>')
    if title:
        out.append(
            f'<text x="{width / 2:.0f}" y="{TITLEBAR / 2 + 4:.0f}" fill="#6a7382" '
            f'font-size="12" text-anchor="middle">{html.escape(title)}</text>'
        )

    for row, runs in enumerate(lines):
        y = TITLEBAR + PAD_Y + row * LINE_H + 13
        spans, col = [], 0
        for text, style in runs:
            if not text:
                continue
            x = PAD_X + col * CHAR_W
            col += len(text)
            if not text.strip():
                continue
            attrs = [f'x="{x:.1f}"', f'fill="{style.fg or FG}"']
            if style.bold:
                attrs.append('font-weight="bold"')
            if style.underline:
                attrs.append('text-decoration="underline"')
            spans.append(f'<tspan {" ".join(attrs)}>{html.escape(text)}</tspan>')
        if spans:
            out.append(f'<text y="{y:.1f}" xml:space="preserve">{"".join(spans)}</text>')

    out.append("</svg>")
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="path to write the SVG to")
    ap.add_argument("--title", default="", help="text shown in the window title bar")
    ap.add_argument(
        "--font",
        default="JetBrains Mono, SFMono-Regular, Menlo, Consolas, monospace",
        help="font stack used in the SVG",
    )
    args = ap.parse_args()

    text = sys.stdin.read()
    svg = render(parse(text), args.title, args.font)
    with open(args.out, "w", encoding="utf-8") as fh:
        fh.write(svg + "\n")
    print(f"wrote {args.out}")


if __name__ == "__main__":
    main()
