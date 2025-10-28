use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Button configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub label: String,
    pub action: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub keybind: Option<char>,
    /// Optional custom icon path (e.g., "/path/to/icon.png")
    #[serde(default)]
    pub icon_path: Option<String>,
    /// Optional custom icon character (Unicode/Nerd Font symbol)
    #[serde(default)]
    pub icon_char: Option<char>,
    /// Optional base button color in hex format (e.g., "#81A1C1")
    #[serde(default)]
    pub color: Option<String>,
    /// Optional hover button color in hex format (e.g., "#5E81AC")
    #[serde(default)]
    pub hover_color: Option<String>,
    /// Whether to show the text label below the icon (default: false)
    #[serde(default)]
    pub show_label: bool,
}

/// Load configuration from JSON file
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Vec<Button>> {
    let content = fs::read_to_string(path)?;
    let buttons: Vec<Button> = serde_json::from_str(&content)?;
    Ok(buttons)
}

/// Parse hex color string to RGBA tuple
pub fn parse_color(color_str: &str) -> (f64, f64, f64, f64) {
    let color_str = color_str.trim_start_matches('#');

    if let Ok(value) = u32::from_str_radix(color_str, 16) {
        let r = ((value >> 16) & 0xFF) as f64 / 255.0;
        let g = ((value >> 8) & 0xFF) as f64 / 255.0;
        let b = (value & 0xFF) as f64 / 255.0;
        (r, g, b, 1.0)
    } else {
        // Fallback to gray
        (0.5, 0.5, 0.5, 1.0)
    }
}

/// Parse hex color with custom alpha
pub fn parse_color_with_alpha(color_str: &str, alpha: f64) -> (f64, f64, f64, f64) {
    let (r, g, b, _) = parse_color(color_str);
    (r, g, b, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let (r, g, b, a) = parse_color("#FF0000");
        assert_eq!(r, 1.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_parse_color_with_alpha() {
        let (r, g, b, a) = parse_color_with_alpha("#00FF00", 0.5);
        assert_eq!(r, 0.0);
        assert_eq!(g, 1.0);
        assert_eq!(b, 0.0);
        assert_eq!(a, 0.5);
    }
}
