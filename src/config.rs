//! Config file handling: `~/.config/dok/config.toml`.
//!
//! ```toml
//! theme = "nord"
//! icons = "nerd"
//!
//! [themes.mine]
//! base = "gruvbox"      # start from a built-in
//! glyphs = "heavy"      # unicode | ascii | heavy | slim
//! layout = "grid"       # default | ruled | grid | quiet | ascii
//! header = "caps"       # underline | bold | dim | caps
//! gutter = 3
//! green = "#b8bb26"     # any palette role can be overridden
//! ```

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::theme::{HeaderStyle, Palette, Rgb, Theme, builtin};

#[derive(Deserialize, Default)]
pub struct Config {
    pub theme: Option<String>,
    pub icons: Option<String>,
    #[serde(default)]
    pub themes: HashMap<String, CustomTheme>,
}

#[derive(Deserialize, Default, Clone)]
pub struct CustomTheme {
    pub base: Option<String>,
    pub glyphs: Option<String>,
    pub layout: Option<String>,
    pub header: Option<String>,
    pub gutter: Option<usize>,
    pub rule: Option<String>,
    pub column_sep: Option<String>,

    pub green: Option<String>,
    pub red: Option<String>,
    pub yellow: Option<String>,
    pub orange: Option<String>,
    pub blue: Option<String>,
    pub cyan: Option<String>,
    pub magenta: Option<String>,
    pub gray: Option<String>,
    pub fg: Option<String>,
}

pub fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("dok").join("config.toml"))
}

/// Missing file is fine; malformed file is an error worth reporting.
pub fn load() -> Result<Config> {
    let Some(path) = config_path() else { return Ok(Config::default()) };
    if !path.exists() {
        return Ok(Config::default());
    }
    let text =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

/// Resolve a theme name against the built-ins and the config's custom themes.
pub fn resolve_theme(cfg: &Config, name: &str) -> Result<Theme> {
    if let Some(custom) = cfg.themes.get(name) {
        return build_custom(name, custom);
    }
    builtin::by_name(name).ok_or_else(|| {
        let known: Vec<String> = builtin::all()
            .iter()
            .map(|t| t.name.clone())
            .chain(cfg.themes.keys().cloned())
            .collect();
        anyhow!("unknown theme `{name}` — try one of: {}", known.join(", "))
    })
}

fn build_custom(name: &str, custom: &CustomTheme) -> Result<Theme> {
    let base_name = custom.base.as_deref().unwrap_or("default");
    let base = builtin::by_name(base_name)
        .ok_or_else(|| anyhow!("theme `{name}` has unknown base `{base_name}`"))?;

    let mut t = Theme { name: name.to_string(), description: "custom", ..base };

    if let Some(g) = &custom.glyphs {
        t.chars = builtin::chars_by_name(g)
            .ok_or_else(|| anyhow!("theme `{name}`: unknown glyph set `{g}`"))?;
    }
    if let Some(lay) = &custom.layout {
        t.layout = builtin::layout_by_name(lay)
            .ok_or_else(|| anyhow!("theme `{name}`: unknown layout `{lay}`"))?;
    }
    if let Some(h) = &custom.header {
        t.layout.header = match h.as_str() {
            "underline" => HeaderStyle::Underline,
            "bold" => HeaderStyle::Bold,
            "dim" => HeaderStyle::Dim,
            "caps" => HeaderStyle::Caps,
            other => return Err(anyhow!("theme `{name}`: unknown header style `{other}`")),
        };
    }
    if let Some(gut) = custom.gutter {
        t.layout.gutter = gut.clamp(1, 8);
    }
    // Strings from config must outlive the process; they are leaked once at
    // startup, which is exactly as long as the theme lives.
    if let Some(r) = &custom.rule {
        t.layout.rule = if r.is_empty() { None } else { Some(leak(r)) };
    }
    if let Some(sep) = &custom.column_sep {
        t.layout.column_sep = if sep.is_empty() { None } else { Some(leak(sep)) };
    }

    t.palette = override_palette(name, t.palette, custom)?;
    Ok(t)
}

fn override_palette(name: &str, mut p: Palette, custom: &CustomTheme) -> Result<Palette> {
    let set = |slot: &mut Rgb, value: &Option<String>, role: &str| -> Result<()> {
        if let Some(hex) = value {
            *slot = Rgb::parse(hex)
                .ok_or_else(|| anyhow!("theme `{name}`: `{role}` is not a #rrggbb colour"))?;
        }
        Ok(())
    };
    set(&mut p.green, &custom.green, "green")?;
    set(&mut p.red, &custom.red, "red")?;
    set(&mut p.yellow, &custom.yellow, "yellow")?;
    set(&mut p.orange, &custom.orange, "orange")?;
    set(&mut p.blue, &custom.blue, "blue")?;
    set(&mut p.cyan, &custom.cyan, "cyan")?;
    set(&mut p.magenta, &custom.magenta, "magenta")?;
    set(&mut p.gray, &custom.gray, "gray")?;
    set(&mut p.fg, &custom.fg, "fg")?;
    Ok(p)
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

/// Every theme name available to `--theme`, built-in first.
pub fn theme_names(cfg: &Config) -> Vec<String> {
    let mut names: Vec<String> = builtin::all().iter().map(|t| t.name.clone()).collect();
    let mut custom: Vec<String> = cfg.themes.keys().cloned().collect();
    custom.sort();
    names.extend(custom);
    names
}
