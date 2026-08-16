//! Themes: palette, glyph set and layout, resolved once at startup.
//!
//! A theme is more than colour — it also decides the state dots, tree stubs,
//! bar characters, table gutter and header treatment, so an ASCII theme really
//! is ASCII and a heavy theme really is heavy.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub mod builtin;

static COLOR: AtomicBool = AtomicBool::new(true);
static ICONS: AtomicU8 = AtomicU8::new(IconSet::Unicode as u8);
static THEME: OnceLock<Theme> = OnceLock::new();

// ── knobs set from the CLI ──────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IconSet {
    None = 0,
    Unicode = 1,
    Nerd = 2,
}

pub fn set_color(on: bool) {
    COLOR.store(on, Ordering::Relaxed);
}

pub fn color_enabled() -> bool {
    COLOR.load(Ordering::Relaxed)
}

pub fn set_icons(set: IconSet) {
    ICONS.store(set as u8, Ordering::Relaxed);
}

pub fn icons() -> IconSet {
    match ICONS.load(Ordering::Relaxed) {
        0 => IconSet::None,
        2 => IconSet::Nerd,
        _ => IconSet::Unicode,
    }
}

/// Install the active theme. First call wins; later calls are ignored.
pub fn set_theme(t: Theme) {
    let _ = THEME.set(t);
}

pub fn theme() -> &'static Theme {
    THEME.get_or_init(builtin::default_theme)
}

/// Palette shorthand — `p().green`.
pub fn p() -> &'static Palette {
    &theme().palette
}

/// Glyph shorthand — `g().tree_branch`.
pub fn g() -> &'static Chars {
    &theme().chars
}

/// Layout shorthand — `l().gutter`.
pub fn l() -> &'static Layout {
    &theme().layout
}

// ── theme data ──────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// Parse `#rrggbb` or `rrggbb`.
    pub fn parse(s: &str) -> Option<Rgb> {
        let h = s.trim().trim_start_matches('#');
        if h.len() != 6 {
            return None;
        }
        let n = u32::from_str_radix(h, 16).ok()?;
        Some(Rgb((n >> 16) as u8, (n >> 8) as u8, n as u8))
    }
}

/// Nine semantic roles. Commands never name a colour, only a role.
#[derive(Copy, Clone, Debug)]
pub struct Palette {
    pub green: Rgb,
    pub red: Rgb,
    pub yellow: Rgb,
    pub orange: Rgb,
    pub blue: Rgb,
    pub cyan: Rgb,
    pub magenta: Rgb,
    pub gray: Rgb,
    pub fg: Rgb,
}

/// Structural glyphs. Icons (image logos, section marks) stay separate because
/// they follow `--icons`, while these follow the theme.
#[derive(Copy, Clone, Debug)]
pub struct Chars {
    pub dot_running: &'static str,
    pub dot_stopped: &'static str,
    pub dot_paused: &'static str,
    pub dot_restarting: &'static str,
    pub dot_created: &'static str,
    pub dot_removing: &'static str,
    pub dot_dead: &'static str,
    pub dot_unknown: &'static str,

    pub ok: &'static str,
    pub fail: &'static str,
    pub pending: &'static str,

    pub tree_branch: &'static str,
    pub tree_last: &'static str,
    pub tree_stem: &'static str,
    pub tree_blank: &'static str,
    pub group_mark: &'static str,

    pub arrow: &'static str,
    pub sep: &'static str,
    pub bullet: &'static str,
    /// Used for proxies / web servers in the image-icon guesser.
    pub server: &'static str,

    pub bar_full: &'static str,
    pub bar_partials: [&'static str; 8],
    pub bar_alt: &'static str,
    pub bar_empty: &'static str,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HeaderStyle {
    /// Bold + underline (the default).
    Underline,
    Bold,
    /// Dim lowercase — quietest option.
    Dim,
    /// Bold UPPERCASE without underline.
    Caps,
}

#[derive(Copy, Clone, Debug)]
pub struct Layout {
    pub header: HeaderStyle,
    /// Spaces between columns.
    pub gutter: usize,
    /// Character repeated under the header row, if any.
    pub rule: Option<&'static str>,
    /// Character drawn between columns, if any.
    pub column_sep: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub description: &'static str,
    pub palette: Palette,
    pub chars: Chars,
    pub layout: Layout,
}

// ── painting ────────────────────────────────────────────────────────────────

/// Paint `s` with a foreground colour.
pub fn c(s: &str, rgb: Rgb) -> String {
    if !color_enabled() {
        return s.to_string();
    }
    format!("\x1b[38;2;{};{};{}m{s}\x1b[0m", rgb.0, rgb.1, rgb.2)
}

/// Paint `s` bold + coloured.
pub fn cb(s: &str, rgb: Rgb) -> String {
    if !color_enabled() {
        return s.to_string();
    }
    format!("\x1b[1;38;2;{};{};{}m{s}\x1b[0m", rgb.0, rgb.1, rgb.2)
}

pub fn bold(s: &str) -> String {
    if !color_enabled() {
        return s.to_string();
    }
    format!("\x1b[1m{s}\x1b[0m")
}

pub fn dim(s: &str) -> String {
    c(s, p().gray)
}

/// Header style used by every table, per the theme's [`HeaderStyle`].
pub fn header(s: &str) -> String {
    let style = l().header;
    let text = match style {
        HeaderStyle::Dim => s.to_lowercase(),
        HeaderStyle::Caps => s.to_uppercase(),
        _ => s.to_string(),
    };
    if !color_enabled() {
        return text;
    }
    let g = p().gray;
    let sgr = match style {
        HeaderStyle::Underline => "1;4;",
        HeaderStyle::Bold | HeaderStyle::Caps => "1;",
        HeaderStyle::Dim => "",
    };
    format!("\x1b[{sgr}38;2;{};{};{}m{text}\x1b[0m", g.0, g.1, g.2)
}

/// Colour + glyph for a container state.
pub fn state_style(state: &str) -> (Rgb, &'static str) {
    let (p, g) = (p(), g());
    match state {
        "running" => (p.green, icon(g.dot_running, "\u{f0e7}")),
        "paused" => (p.yellow, icon(g.dot_paused, "\u{f04c}")),
        "restarting" => (p.yellow, icon(g.dot_restarting, "\u{f021}")),
        "created" => (p.blue, icon(g.dot_created, "\u{f055}")),
        "removing" => (p.orange, icon(g.dot_removing, "\u{f014}")),
        "exited" => (p.gray, icon(g.dot_stopped, "\u{f04d}")),
        "dead" => (p.red, icon(g.dot_dead, "\u{f071}")),
        _ => (p.gray, icon(g.dot_unknown, "\u{f059}")),
    }
}

/// Colour + glyph for a healthcheck status.
pub fn health_style(health: &str) -> Option<(Rgb, &'static str)> {
    let (p, g) = (p(), g());
    match health {
        "healthy" => Some((p.green, icon(g.ok, "\u{f058}"))),
        "unhealthy" => Some((p.red, icon(g.fail, "\u{f057}"))),
        "starting" => Some((p.yellow, icon(g.pending, "\u{f251}"))),
        _ => None,
    }
}

/// Pick the theme glyph, or its Nerd Font counterpart when icons are nerd.
/// `--icons none` drops decorative marks entirely.
pub fn icon(themed: &'static str, nerd: &'static str) -> &'static str {
    match icons() {
        IconSet::Nerd => nerd,
        IconSet::Unicode => themed,
        IconSet::None => "",
    }
}

/// Icon guessed from an image name — the small touch that makes `ps` scannable.
pub fn image_icon(image: &str) -> &'static str {
    let i = image.to_ascii_lowercase();
    let has = |n: &str| i.contains(n);
    let g = g();
    if has("postgres") || has("mysql") || has("mariadb") || has("mssql") {
        icon(g.bullet, "\u{f1c0}")
    } else if has("redis") || has("memcached") || has("valkey") {
        icon(g.bullet, "\u{f0e7}")
    } else if has("mongo") || has("couch") || has("cassandra") || has("elastic") {
        icon(g.bullet, "\u{f1c0}")
    } else if has("nginx") || has("traefik") || has("caddy") || has("haproxy") || has("envoy") {
        icon(g.server, "\u{f0ac}")
    } else if has("node") || has("bun") || has("deno") {
        icon(g.group_mark, "\u{e718}")
    } else if has("python") || has("django") {
        icon(g.group_mark, "\u{e73c}")
    } else if has("golang") || has("/go") {
        icon(g.group_mark, "\u{e627}")
    } else if has("rust") {
        icon(g.group_mark, "\u{e7a8}")
    } else if has("rabbit") || has("kafka") || has("nats") || has("mqtt") {
        icon(g.arrow, "\u{f0e0}")
    } else if has("grafana") || has("prometheus") || has("loki") || has("jaeger") {
        icon(g.bullet, "\u{f080}")
    } else if has("alpine") || has("ubuntu") || has("debian") || has("busybox") {
        icon(g.dot_stopped, "\u{f17c}")
    } else {
        icon(g.dot_unknown, "\u{f21b}")
    }
}

/// Size gradient — small is calm, huge is loud.
pub fn size_color(bytes: u64) -> Rgb {
    const MB: u64 = 1000 * 1000;
    let p = p();
    match bytes {
        0..=9_999_999 => p.green,
        b if b < 100 * MB => p.cyan,
        b if b < 500 * MB => p.yellow,
        b if b < 1000 * MB => p.orange,
        _ => p.red,
    }
}

/// Age gradient — fresh is bright, ancient fades out.
pub fn age_color(secs: i64) -> Rgb {
    const H: i64 = 3600;
    let p = p();
    match secs {
        s if s < H => p.blue,
        s if s < 24 * H => p.cyan,
        s if s < 7 * 24 * H => p.fg,
        _ => p.gray,
    }
}

/// Load gradient shared by cpu/mem bars.
pub fn load_color(pct: f64) -> Rgb {
    let p = p();
    match pct {
        v if v < 25.0 => p.green,
        v if v < 60.0 => p.cyan,
        v if v < 85.0 => p.yellow,
        _ => p.red,
    }
}

/// Stable per-name colour, used to tint interleaved log streams.
pub fn hash_color(name: &str) -> Rgb {
    let p = p();
    let wheel = [p.green, p.blue, p.magenta, p.cyan, p.yellow, p.orange, p.red, p.fg];
    let mut h: u32 = 2166136261;
    for b in name.as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(16777619);
    }
    wheel[(h % wheel.len() as u32) as usize]
}
