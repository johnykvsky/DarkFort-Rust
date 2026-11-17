use crate::core::character::Character;
use crate::core::progression::ProgressionManager;
use crate::dungeon::map::DungeonGraph;
use crate::ui::theme::ThemeMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

pub const DEFAULT_SAVE_PATH: &str = "savegame.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub character: Character,
    pub dungeon: DungeonGraph,
    pub progression: ProgressionManager,
    pub theme_mode: ThemeMode,
    #[serde(default)]
    pub is_scifi: bool,
}

pub struct StorageManager;

impl StorageManager {
    pub fn save(data: &SaveData, path: impl AsRef<Path>) -> io::Result<()> {
        let json = serde_json::to_string_pretty(data)
            .map_err(io::Error::other)?;
        fs::write(path, json)
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<SaveData> {
        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    pub fn delete(path: impl AsRef<Path>) -> io::Result<()> {
        if Self::exists(&path) {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }
}
