use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    ClassicWhite,
    AmberCrt,
    GreenCrt,
    MorkBorg,
    SciFiVoid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub bg: Color,
    pub fg: Color,
    pub border: Color,
    pub title: Color,
    pub accent: Color,
    pub danger: Color,
    pub warning: Color,
    pub success: Color,
    pub hp_high: Color,
    pub hp_mid: Color,
    pub hp_low: Color,
    pub map_player: Color,
    pub map_explored: Color,
    pub map_door: Color,
    pub quit_border: Color,
    pub quit_fg: Color,
}

impl Theme {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::ClassicWhite => Self {
                mode,
                bg: Color::Black,
                fg: Color::Rgb(220, 220, 220),
                border: Color::Rgb(200, 200, 200),
                title: Color::White,
                accent: Color::White,
                danger: Color::Rgb(255, 110, 110),
                warning: Color::Rgb(240, 240, 160),
                success: Color::White,
                hp_high: Color::White,
                hp_mid: Color::Rgb(180, 180, 180),
                hp_low: Color::Rgb(255, 110, 110),
                map_player: Color::White,
                map_explored: Color::Rgb(140, 140, 140),
                map_door: Color::White,
                // For white theme, quit confirmation is colored amber for high visibility
                quit_border: Color::Rgb(255, 176, 0),
                quit_fg: Color::Rgb(255, 191, 0),
            },
            ThemeMode::AmberCrt => Self {
                mode,
                bg: Color::Black,
                fg: Color::Rgb(255, 176, 0),
                border: Color::Rgb(255, 176, 0),
                title: Color::Rgb(255, 191, 0),
                accent: Color::Rgb(255, 215, 0),
                danger: Color::Rgb(255, 140, 0),
                warning: Color::Rgb(255, 176, 0),
                success: Color::Rgb(255, 215, 0),
                hp_high: Color::Rgb(255, 215, 0),
                hp_mid: Color::Rgb(255, 176, 0),
                hp_low: Color::Rgb(200, 100, 0),
                map_player: Color::Rgb(255, 255, 255),
                map_explored: Color::Rgb(255, 176, 0),
                map_door: Color::Rgb(255, 215, 0),
                // For other themes, quit confirmation is colored white
                quit_border: Color::White,
                quit_fg: Color::White,
            },
            ThemeMode::GreenCrt => Self {
                mode,
                bg: Color::Black,
                fg: Color::Rgb(51, 255, 51),
                border: Color::Rgb(51, 255, 51),
                title: Color::Rgb(102, 255, 102),
                accent: Color::Rgb(153, 255, 153),
                danger: Color::Rgb(0, 204, 0),
                warning: Color::Rgb(102, 255, 102),
                success: Color::Rgb(51, 255, 51),
                hp_high: Color::Rgb(153, 255, 153),
                hp_mid: Color::Rgb(51, 255, 51),
                hp_low: Color::Rgb(0, 153, 0),
                map_player: Color::Rgb(255, 255, 255),
                map_explored: Color::Rgb(51, 255, 51),
                map_door: Color::Rgb(102, 255, 102),
                // For other themes, quit confirmation is colored white
                quit_border: Color::White,
                quit_fg: Color::White,
            },
            ThemeMode::MorkBorg => Self {
                mode,
                bg: Color::Rgb(12, 12, 12),
                fg: Color::Rgb(224, 224, 224),
                border: Color::Rgb(255, 235, 59),      // Doom Yellow
                title: Color::Rgb(255, 235, 59),       // Doom Yellow
                accent: Color::Rgb(255, 23, 68),       // Blood Red
                danger: Color::Rgb(213, 0, 0),         // Deep Red
                warning: Color::Rgb(255, 152, 0),      // Ochre
                success: Color::Rgb(0, 230, 118),      // Toxic Green
                hp_high: Color::Rgb(0, 230, 118),
                hp_mid: Color::Rgb(255, 235, 59),
                hp_low: Color::Rgb(255, 23, 68),
                map_player: Color::Rgb(255, 235, 59),
                map_explored: Color::Rgb(160, 160, 160),
                map_door: Color::Rgb(255, 152, 0),
                // For other themes, quit confirmation is colored white
                quit_border: Color::White,
                quit_fg: Color::White,
            },
            ThemeMode::SciFiVoid => Self {
                mode,
                bg: Color::Rgb(5, 8, 17),
                fg: Color::Rgb(210, 240, 255),
                border: Color::Rgb(0, 210, 255),
                title: Color::Rgb(0, 245, 255),
                accent: Color::Rgb(255, 145, 0),
                danger: Color::Rgb(255, 30, 80),
                warning: Color::Rgb(255, 185, 0),
                success: Color::Rgb(0, 230, 118),
                hp_high: Color::Rgb(0, 230, 118),
                hp_mid: Color::Rgb(0, 210, 255),
                hp_low: Color::Rgb(255, 30, 80),
                map_player: Color::Rgb(0, 245, 255),
                map_explored: Color::Rgb(40, 90, 130),
                map_door: Color::Rgb(0, 210, 255),
                quit_border: Color::Rgb(255, 145, 0),
                quit_fg: Color::Rgb(255, 200, 50),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_confirmation_colors() {
        let white_theme = Theme::for_mode(ThemeMode::ClassicWhite);
        assert_eq!(white_theme.quit_border, Color::Rgb(255, 176, 0));
        assert_eq!(white_theme.quit_fg, Color::Rgb(255, 191, 0));

        let amber_theme = Theme::for_mode(ThemeMode::AmberCrt);
        assert_eq!(amber_theme.quit_border, Color::White);
        assert_eq!(amber_theme.quit_fg, Color::White);

        let green_theme = Theme::for_mode(ThemeMode::GreenCrt);
        assert_eq!(green_theme.quit_border, Color::White);
        assert_eq!(green_theme.quit_fg, Color::White);

        let mork_theme = Theme::for_mode(ThemeMode::MorkBorg);
        assert_eq!(mork_theme.quit_border, Color::White);
        assert_eq!(mork_theme.quit_fg, Color::White);
    }
}
