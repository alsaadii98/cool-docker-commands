//! Built-in themes. Each one carries a palette, a glyph set and a layout.

use super::{Chars, HeaderStyle, Layout, Palette, Rgb, Theme};

const fn rgb(hex: u32) -> Rgb {
    Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

// ── glyph sets ──────────────────────────────────────────────────────────────

/// Rounded unicode: the default look.
pub const UNICODE: Chars = Chars {
    dot_running: "●",
    dot_stopped: "○",
    dot_paused: "⏸",
    dot_restarting: "↻",
    dot_created: "◌",
    dot_removing: "◍",
    dot_dead: "☠",
    dot_unknown: "·",
    ok: "✔",
    fail: "✖",
    pending: "◐",
    tree_branch: "├─ ",
    tree_last: "└─ ",
    tree_stem: "│  ",
    tree_blank: "   ",
    group_mark: "▾",
    arrow: "→",
    sep: "│",
    bullet: "◆",
    server: "▲",
    bar_full: "█",
    bar_partials: ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"],
    bar_alt: "▒",
    bar_empty: "·",
};

/// Pure ASCII — safe over serial consoles, CI logs and busted fonts.
pub const ASCII: Chars = Chars {
    dot_running: "+",
    dot_stopped: "-",
    dot_paused: "=",
    dot_restarting: "~",
    dot_created: "o",
    dot_removing: "x",
    dot_dead: "X",
    dot_unknown: ".",
    ok: "OK",
    fail: "!!",
    pending: "..",
    tree_branch: "|- ",
    tree_last: "`- ",
    tree_stem: "|  ",
    tree_blank: "   ",
    group_mark: ">",
    arrow: "->",
    sep: "|",
    bullet: "*",
    server: "^",
    bar_full: "#",
    bar_partials: ["-", "-", "-", "=", "=", "=", "#", "#"],
    bar_alt: "+",
    bar_empty: ".",
};

/// Square and heavy — dense blocks instead of thin lines.
pub const HEAVY: Chars = Chars {
    dot_running: "■",
    dot_stopped: "□",
    dot_paused: "▮",
    dot_restarting: "▶",
    dot_created: "▫",
    dot_removing: "▨",
    dot_dead: "▩",
    dot_unknown: "▪",
    ok: "✓",
    fail: "✗",
    pending: "◑",
    tree_branch: "┣━ ",
    tree_last: "┗━ ",
    tree_stem: "┃  ",
    tree_blank: "   ",
    group_mark: "▼",
    arrow: "▶",
    sep: "┃",
    bullet: "◼",
    server: "▰",
    bar_full: "█",
    bar_partials: ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"],
    bar_alt: "▓",
    bar_empty: "░",
};

/// Minimal: hairline tree, dotted bars, nothing shouty.
pub const SLIM: Chars = Chars {
    dot_running: "•",
    dot_stopped: "◦",
    dot_paused: "‖",
    dot_restarting: "↺",
    dot_created: "◌",
    dot_removing: "⌫",
    dot_dead: "✕",
    dot_unknown: "·",
    ok: "✓",
    fail: "✗",
    pending: "…",
    tree_branch: "├╴ ",
    tree_last: "╰╴ ",
    tree_stem: "│  ",
    tree_blank: "   ",
    group_mark: "❯",
    arrow: "›",
    sep: "┊",
    bullet: "◇",
    server: "△",
    bar_full: "▬",
    bar_partials: ["▭", "▭", "▭", "▬", "▬", "▬", "▬", "▬"],
    bar_alt: "▭",
    bar_empty: "·",
};

// ── layouts ─────────────────────────────────────────────────────────────────

pub const LAYOUT_DEFAULT: Layout =
    Layout { header: HeaderStyle::Underline, gutter: 2, rule: None, column_sep: None };

pub const LAYOUT_RULED: Layout =
    Layout { header: HeaderStyle::Caps, gutter: 2, rule: Some("─"), column_sep: None };

pub const LAYOUT_GRID: Layout =
    Layout { header: HeaderStyle::Bold, gutter: 2, rule: Some("─"), column_sep: Some("│") };

pub const LAYOUT_QUIET: Layout =
    Layout { header: HeaderStyle::Dim, gutter: 3, rule: None, column_sep: None };

pub const LAYOUT_ASCII: Layout =
    Layout { header: HeaderStyle::Caps, gutter: 2, rule: Some("-"), column_sep: None };

// ── palettes ────────────────────────────────────────────────────────────────

const ONEDARK: Palette = Palette {
    green: rgb(0x98c379),
    red: rgb(0xe06c75),
    yellow: rgb(0xe5c07b),
    orange: rgb(0xd19a66),
    blue: rgb(0x61afef),
    cyan: rgb(0x56b6c2),
    magenta: rgb(0xc678dd),
    gray: rgb(0x6a7382),
    fg: rgb(0xbec4d0),
};

const DRACULA: Palette = Palette {
    green: rgb(0x50fa7b),
    red: rgb(0xff5555),
    yellow: rgb(0xf1fa8c),
    orange: rgb(0xffb86c),
    blue: rgb(0xbd93f9),
    cyan: rgb(0x8be9fd),
    magenta: rgb(0xff79c6),
    gray: rgb(0x6272a4),
    fg: rgb(0xf8f8f2),
};

const NORD: Palette = Palette {
    green: rgb(0xa3be8c),
    red: rgb(0xbf616a),
    yellow: rgb(0xebcb8b),
    orange: rgb(0xd08770),
    blue: rgb(0x81a1c1),
    cyan: rgb(0x88c0d0),
    magenta: rgb(0xb48ead),
    gray: rgb(0x616e88),
    fg: rgb(0xe5e9f0),
};

const GRUVBOX: Palette = Palette {
    green: rgb(0xb8bb26),
    red: rgb(0xfb4934),
    yellow: rgb(0xfabd2f),
    orange: rgb(0xfe8019),
    blue: rgb(0x83a598),
    cyan: rgb(0x8ec07c),
    magenta: rgb(0xd3869b),
    gray: rgb(0x928374),
    fg: rgb(0xebdbb2),
};

const CATPPUCCIN: Palette = Palette {
    green: rgb(0xa6e3a1),
    red: rgb(0xf38ba8),
    yellow: rgb(0xf9e2af),
    orange: rgb(0xfab387),
    blue: rgb(0x89b4fa),
    cyan: rgb(0x94e2d5),
    magenta: rgb(0xcba6f7),
    gray: rgb(0x7f849c),
    fg: rgb(0xcdd6f4),
};

const TOKYONIGHT: Palette = Palette {
    green: rgb(0x9ece6a),
    red: rgb(0xf7768e),
    yellow: rgb(0xe0af68),
    orange: rgb(0xff9e64),
    blue: rgb(0x7aa2f7),
    cyan: rgb(0x7dcfff),
    magenta: rgb(0xbb9af7),
    gray: rgb(0x565f89),
    fg: rgb(0xc0caf5),
};

/// Tuned for light backgrounds, where pastel palettes disappear.
const SOLARIZED_LIGHT: Palette = Palette {
    green: rgb(0x4f7a28),
    red: rgb(0xdc322f),
    yellow: rgb(0xa57705),
    orange: rgb(0xbd3612),
    blue: rgb(0x2176c7),
    cyan: rgb(0x259286),
    magenta: rgb(0xc61c6f),
    gray: rgb(0x8a8a8a),
    fg: rgb(0x35434b),
};

/// No hue at all — separation comes from brightness and weight.
const MONO: Palette = Palette {
    green: rgb(0xe8e8e8),
    red: rgb(0xffffff),
    yellow: rgb(0xd0d0d0),
    orange: rgb(0xc0c0c0),
    blue: rgb(0xe0e0e0),
    cyan: rgb(0xb8b8b8),
    magenta: rgb(0xa8a8a8),
    gray: rgb(0x707070),
    fg: rgb(0xcccccc),
};

const MATRIX: Palette = Palette {
    green: rgb(0x00ff5f),
    red: rgb(0xff0040),
    yellow: rgb(0xafff00),
    orange: rgb(0x5fff87),
    blue: rgb(0x00d75f),
    cyan: rgb(0x00ffaf),
    magenta: rgb(0x87ff5f),
    gray: rgb(0x1f6f3f),
    fg: rgb(0x2fdd6f),
};

// ── the registry ────────────────────────────────────────────────────────────

fn theme(
    name: &str,
    description: &'static str,
    palette: Palette,
    chars: Chars,
    layout: Layout,
) -> Theme {
    Theme { name: name.to_string(), description, palette, chars, layout }
}

pub fn all() -> Vec<Theme> {
    vec![
        theme("default", "one-dark palette, rounded unicode", ONEDARK, UNICODE, LAYOUT_DEFAULT),
        theme("dracula", "high-contrast purples, heavy blocks", DRACULA, HEAVY, LAYOUT_DEFAULT),
        theme("nord", "cool arctic palette, slim glyphs", NORD, SLIM, LAYOUT_QUIET),
        theme("gruvbox", "warm retro palette, ruled headers", GRUVBOX, UNICODE, LAYOUT_RULED),
        theme("catppuccin", "soft pastels, slim glyphs", CATPPUCCIN, SLIM, LAYOUT_DEFAULT),
        theme("tokyonight", "neon night palette, grid rules", TOKYONIGHT, UNICODE, LAYOUT_GRID),
        theme("solarized-light", "for light terminals", SOLARIZED_LIGHT, UNICODE, LAYOUT_RULED),
        theme("mono", "greyscale only, weight over hue", MONO, UNICODE, LAYOUT_DEFAULT),
        theme("matrix", "all green, heavy blocks", MATRIX, HEAVY, LAYOUT_GRID),
        theme("ascii", "no unicode anywhere, plain rules", ONEDARK, ASCII, LAYOUT_ASCII),
    ]
}

pub fn default_theme() -> Theme {
    theme("default", "one-dark palette, rounded unicode", ONEDARK, UNICODE, LAYOUT_DEFAULT)
}

pub fn by_name(name: &str) -> Option<Theme> {
    all().into_iter().find(|t| t.name == name)
}

/// Glyph set by name, for config overrides.
pub fn chars_by_name(name: &str) -> Option<Chars> {
    match name {
        "unicode" => Some(UNICODE),
        "ascii" => Some(ASCII),
        "heavy" => Some(HEAVY),
        "slim" => Some(SLIM),
        _ => None,
    }
}

/// Layout by name, for config overrides.
pub fn layout_by_name(name: &str) -> Option<Layout> {
    match name {
        "default" => Some(LAYOUT_DEFAULT),
        "ruled" => Some(LAYOUT_RULED),
        "grid" => Some(LAYOUT_GRID),
        "quiet" => Some(LAYOUT_QUIET),
        "ascii" => Some(LAYOUT_ASCII),
        _ => None,
    }
}
