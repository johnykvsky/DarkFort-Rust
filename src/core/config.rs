use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Supported tabletop RPG engines in Dark Fort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineType {
    Classic,
    Mothership,
    MorkBorg,
    Durf,
    Adnd2e,
}

impl Default for EngineType {
    fn default() -> Self {
        Self::Durf
    }
}

impl EngineType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Classic => "Classic Dark Fort (2018)",
            Self::Mothership => "Mothership 1e (Fantasy Conversion)",
            Self::MorkBorg => "Full MÖRK BORG (d20)",
            Self::Durf => "DURF (Ablative Armor)",
            Self::Adnd2e => "AD&D 2nd Edition (Infinity Engine)",
        }
    }

    pub fn parse_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('_', "-").as_str() {
            "classic" | "dark-fort" | "darkfort" => Some(Self::Classic),
            "mothership" | "mothership-1e" | "mosh" => Some(Self::Mothership),
            "mork-borg" | "morkborg" | "mb" => Some(Self::MorkBorg),
            "durf" => Some(Self::Durf),
            "adnd" | "adnd2e" | "ad&d" | "infinity" | "bg" | "baldurs-gate" => Some(Self::Adnd2e),
            _ => None,
        }
    }
}

/// Global game configuration loadable from config.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    #[serde(default)]
    pub engine: EngineType,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub scifi: bool,
    #[serde(default)]
    pub seed: Option<u64>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            engine: EngineType::Durf,
            theme: None,
            scifi: false,
            seed: None,
        }
    }
}

impl GameConfig {
    pub fn is_scifi(&self) -> bool {
        self.scifi || matches!(self.theme.as_deref(), Some("scifi") | Some("scifi-void") | Some("alien") | Some("sf"))
    }
    /// Load from `config.json` or fallback to `data/config.json`, or default if missing.
    pub fn load_or_default() -> Self {
        if Path::new("config.json").exists() {
            if let Ok(content) = fs::read_to_string("config.json") {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        if Path::new("data/config.json").exists() {
            if let Ok(content) = fs::read_to_string("data/config.json") {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }
}
