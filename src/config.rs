use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub terminal: String,
    pub launcher: String,
    pub file_manager: String,
    pub browser: String,
    pub wallpaper: String,
    pub panel_position: String,
    pub layout: String,
    pub autostart: Vec<String>,
    pub panel_color: u64,
    pub accent_color: u64,
    pub background_color: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            terminal: String::new(),
            launcher: String::new(),
            file_manager: String::new(),
            browser: String::new(),
            wallpaper: String::new(),
            panel_position: "top".into(),
            layout: "floating".into(),
            autostart: Vec::new(),
            panel_color: 0x222534,
            accent_color: 0xffb84d,
            background_color: 0x171923,
        }
    }
}

impl Config {
    pub fn path() -> PathBuf {
        if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
            return PathBuf::from(config_home).join("lde/config");
        }
        env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config/lde/config")
    }

    pub fn load() -> Self {
        let mut config = Self::default();
        if let Ok(contents) = fs::read_to_string(Self::path()) {
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') { continue; }
                let Some((key, raw)) = line.split_once('=') else { continue };
                let value = raw.trim().trim_matches('"').trim_matches('\'').to_string();
                match key.trim() {
                    "terminal" => config.terminal = value,
                    "launcher" => config.launcher = value,
                    "file_manager" => config.file_manager = value,
                    "browser" => config.browser = value,
                    "wallpaper" => config.wallpaper = value,
                    "panel_position" if value == "top" || value == "bottom" => config.panel_position = value,
                    "layout" if value == "floating" || value == "tiled" => config.layout = value,
                    "panel_color" => if let Some(color) = parse_color(&value) { config.panel_color = color },
                    "accent_color" => if let Some(color) = parse_color(&value) { config.accent_color = color },
                    "background_color" => if let Some(color) = parse_color(&value) { config.background_color = color },
                    "autostart" if !value.is_empty() => config.autostart.push(value),
                    _ => {}
                }
            }
        }
        config
    }

    pub fn ensure_file() -> Result<PathBuf, String> {
        let path = Self::path();
        if !path.exists() {
            let parent = path.parent().ok_or_else(|| "Invalid LDE config path".to_string())?;
            fs::create_dir_all(parent).map_err(|e| format!("Cannot create {}: {e}", parent.display()))?;
            fs::write(&path, DEFAULT_CONFIG).map_err(|e| format!("Cannot write {}: {e}", path.display()))?;
        }
        Ok(path)
    }
}

fn parse_color(value: &str) -> Option<u64> {
    let digits = value.strip_prefix("0x").or_else(|| value.strip_prefix('#')).unwrap_or(value);
    u64::from_str_radix(digits, 16).ok().filter(|n| *n <= 0xffffff)
}

const DEFAULT_CONFIG: &str = r#"# LDE user configuration (~/.config/lde/config)
# Values for application commands are shell command lines run as your user.
# Use only commands you trust. Blank values use LDE's built-in app discovery.
terminal =
launcher =
file_manager =
browser =

# Optional wallpaper command backend: feh is preferred, nitrogen is also tried.
wallpaper =

# Choose top or bottom panel; choose floating or tiled default layout.
panel_position = top
layout = floating

# Hex RGB colors (examples: 0x222534 or #ffb84d).
panel_color = 0x222534
accent_color = 0xffb84d
background_color = 0x171923

# Optional services are separate programs, keeping the LDE core small.
# Add one command per line, for example:
# autostart = dunst
# autostart = xfce4-power-manager
# autostart = pcmanfm --desktop
"#;

#[cfg(test)]
mod tests {
    use super::parse_color;

    #[test]
    fn parses_rgb_forms_and_rejects_invalid_values() {
        assert_eq!(parse_color("0xffb84d"), Some(0xffb84d));
        assert_eq!(parse_color("#171923"), Some(0x171923));
        assert_eq!(parse_color("ffffff"), Some(0xffffff));
        assert_eq!(parse_color("#gggggg"), None);
        assert_eq!(parse_color("0x1000000"), None);
    }
}
