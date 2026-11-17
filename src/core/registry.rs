use crate::core::dice::{DiceFormula, DiceRoller};
use crate::core::equipment::{Item, ScrollType, WeaponType};
use crate::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponDef {
    pub id: String,
    pub name: String,
    pub damage_formula: DiceFormula,
    #[serde(default)]
    pub attack_bonus: i32,
    pub price: i32,
    #[serde(default)]
    pub is_starting: bool,
    #[serde(default)]
    pub description: String,
}

impl WeaponDef {
    pub fn to_weapon_type(&self) -> WeaponType {
        match self.id.as_str() {
            "warhammer" => WeaponType::Warhammer,
            "dagger" => WeaponType::Dagger,
            "sword" => WeaponType::Sword,
            "flail" => WeaponType::Flail,
            "mighty_zweihaender" => WeaponType::MightyZweihaender,
            "unarmed" => WeaponType::Unarmed,
            _ => WeaponType::Custom {
                id: self.id.clone(),
                name: self.name.clone(),
                damage_formula: self.damage_formula,
                attack_bonus: self.attack_bonus,
                price: self.price,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmourDef {
    pub id: String,
    pub name: String,
    pub absorption_formula: DiceFormula,
    pub price: i32,
    #[serde(default)]
    pub is_starting: bool,
    #[serde(default)]
    pub description: String,
}

impl ArmourDef {
    pub fn to_item(&self) -> Item {
        if self.id == "leather_armor" {
            Item::Armor
        } else {
            Item::CustomArmor {
                id: self.id.clone(),
                name: self.name.clone(),
                absorption_formula: self.absorption_formula,
                price: self.price,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotionDef {
    pub id: String,
    pub name: String,
    pub heal_formula: DiceFormula,
    pub price: i32,
    #[serde(default)]
    pub is_starting: bool,
    #[serde(default)]
    pub description: String,
}

impl PotionDef {
    pub fn to_item(&self) -> Item {
        if self.id == "healing_potion" {
            Item::Potion
        } else {
            Item::CustomPotion {
                id: self.id.clone(),
                name: self.name.clone(),
                heal_formula: self.heal_formula,
                price: self.price,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollKind {
    SummonWeakDaemon,
    PalmsOpenSouthernGate,
    AegisOfSorrow,
    FalseOmen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollDef {
    pub id: String,
    pub name: String,
    pub scroll_kind: ScrollKind,
    pub charges_formula: Option<DiceFormula>,
    pub price: i32,
    #[serde(default)]
    pub is_starting: bool,
    #[serde(default)]
    pub description: String,
}

impl ScrollDef {
    pub fn roll_scroll_type<R: DiceRoller>(&self, roller: &mut R) -> ScrollType {
        let charges = self
            .charges_formula
            .map(|f| roller.roll_formula(f).max(1) as u32)
            .unwrap_or(1);

        match self.scroll_kind {
            ScrollKind::SummonWeakDaemon => ScrollType::SummonWeakDaemon {
                fights_remaining: charges,
            },
            ScrollKind::PalmsOpenSouthernGate => ScrollType::PalmsOpenSouthernGate {
                uses_remaining: charges,
            },
            ScrollKind::AegisOfSorrow => ScrollType::AegisOfSorrow {
                uses_remaining: charges,
            },
            ScrollKind::FalseOmen => ScrollType::FalseOmen,
        }
    }

    pub fn to_item<R: DiceRoller>(&self, roller: &mut R) -> Item {
        Item::Scroll(self.roll_scroll_type(roller))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearKind {
    Rope,
    CloakOfInvisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearDef {
    pub id: String,
    pub name: String,
    pub gear_kind: GearKind,
    pub charges_formula: Option<DiceFormula>,
    pub price: i32,
    #[serde(default)]
    pub is_starting: bool,
    #[serde(default)]
    pub description: String,
}

impl GearDef {
    pub fn to_item<R: DiceRoller>(&self, roller: &mut R) -> Item {
        if self.id == "rope" {
            Item::Rope
        } else if self.id == "cloak_of_invisibility" {
            let uses = self
                .charges_formula
                .map(|f| roller.roll_formula(f).max(1) as u32)
                .unwrap_or(1);
            Item::CloakOfInvisibility {
                uses_remaining: uses,
            }
        } else {
            let uses = self
                .charges_formula
                .map(|f| roller.roll_formula(f).max(1) as u32);
            Item::CustomGear {
                id: self.id.clone(),
                name: self.name.clone(),
                gear_kind: self.gear_kind,
                uses_remaining: uses,
                price: self.price,
            }
        }
    }
}

/// Dynamic equipment registry loaded from data/items/*.json with compiled-in fallbacks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemRegistry {
    pub weapons: Vec<WeaponDef>,
    pub armours: Vec<ArmourDef>,
    pub potions: Vec<PotionDef>,
    pub scrolls: Vec<ScrollDef>,
    pub gear: Vec<GearDef>,
    #[serde(skip)]
    weapons_by_id: HashMap<String, WeaponDef>,
}

impl ItemRegistry {
    pub const DEFAULT_DATA_DIR: &'static str = "data/items";
    pub const SCIFI_DATA_DIR: &'static str = "data/scifi/items";

    /// Load from default directory `data/items`, falling back to compiled-in JSON if not found.
    pub fn load_or_default() -> Self {
        Self::load_from_dir(Self::DEFAULT_DATA_DIR).unwrap_or_else(|_| Self::default_embedded())
    }

    /// Load from sci-fi directory `data/scifi/items`, falling back to compiled-in JSON if not found.
    pub fn load_scifi_or_default() -> Self {
        Self::load_from_dir(Self::SCIFI_DATA_DIR).unwrap_or_else(|_| Self::default_scifi_embedded())
    }

    /// Load from a specified directory on disk.
    pub fn load_from_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let weapons_path = dir.join("weapons.json");
        let armours_path = dir.join("armours.json");
        let potions_path = dir.join("potions.json");
        let scrolls_path = dir.join("scrolls.json");
        let gear_path = dir.join("gear.json");

        let weapons: Vec<WeaponDef> = serde_json::from_str(&fs::read_to_string(&weapons_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let armours: Vec<ArmourDef> = serde_json::from_str(&fs::read_to_string(&armours_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let potions: Vec<PotionDef> = serde_json::from_str(&fs::read_to_string(&potions_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let scrolls: Vec<ScrollDef> = serde_json::from_str(&fs::read_to_string(&scrolls_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let gear: Vec<GearDef> = serde_json::from_str(&fs::read_to_string(&gear_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut registry = Self {
            weapons,
            armours,
            potions,
            scrolls,
            gear,
            weapons_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        Ok(registry)
    }

    /// Embedded fallback compiled directly into the binary.
    pub fn default_embedded() -> Self {
        let weapons: Vec<WeaponDef> =
            serde_json::from_str(include_str!("../../data/items/weapons.json"))
                .expect("Embedded weapons.json must be valid");
        let armours: Vec<ArmourDef> =
            serde_json::from_str(include_str!("../../data/items/armours.json"))
                .expect("Embedded armours.json must be valid");
        let potions: Vec<PotionDef> =
            serde_json::from_str(include_str!("../../data/items/potions.json"))
                .expect("Embedded potions.json must be valid");
        let scrolls: Vec<ScrollDef> =
            serde_json::from_str(include_str!("../../data/items/scrolls.json"))
                .expect("Embedded scrolls.json must be valid");
        let gear: Vec<GearDef> = serde_json::from_str(include_str!("../../data/items/gear.json"))
            .expect("Embedded gear.json must be valid");

        let mut registry = Self {
            weapons,
            armours,
            potions,
            scrolls,
            gear,
            weapons_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        registry
    }

    /// Embedded sci-fi fallback compiled directly into the binary.
    pub fn default_scifi_embedded() -> Self {
        let weapons: Vec<WeaponDef> =
            serde_json::from_str(include_str!("../../data/scifi/items/weapons.json"))
                .expect("Embedded scifi weapons.json must be valid");
        let armours: Vec<ArmourDef> =
            serde_json::from_str(include_str!("../../data/scifi/items/armours.json"))
                .expect("Embedded scifi armours.json must be valid");
        let potions: Vec<PotionDef> =
            serde_json::from_str(include_str!("../../data/scifi/items/potions.json"))
                .expect("Embedded scifi potions.json must be valid");
        let scrolls: Vec<ScrollDef> =
            serde_json::from_str(include_str!("../../data/scifi/items/scrolls.json"))
                .expect("Embedded scifi scrolls.json must be valid");
        let gear: Vec<GearDef> = serde_json::from_str(include_str!("../../data/scifi/items/gear.json"))
            .expect("Embedded scifi gear.json must be valid");

        let mut registry = Self {
            weapons,
            armours,
            potions,
            scrolls,
            gear,
            weapons_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        registry
    }

    pub fn rebuild_indexes(&mut self) {
        self.weapons_by_id.clear();
        for w in &self.weapons {
            self.weapons_by_id.insert(w.id.clone(), w.clone());
        }
    }

    pub fn get_weapon(&self, id: &str) -> Option<&WeaponDef> {
        self.weapons_by_id.get(id)
    }

    /// Roll a starting weapon from all weapons flagged `is_starting: true`.
    pub fn roll_starting_weapon<R: DiceRoller>(&self, roller: &mut R) -> WeaponType {
        let starting: Vec<&WeaponDef> = self.weapons.iter().filter(|w| w.is_starting).collect();
        if starting.is_empty() {
            return WeaponType::Warhammer;
        }
        let roll = roller.roll_d4();
        let idx = ((roll - 1).max(0) as usize) % starting.len();
        starting[idx].to_weapon_type()
    }

    /// Roll starting item (Table B).
    pub fn roll_starting_item<R: DiceRoller>(&self, roller: &mut R) -> Item {
        match roller.roll_d4() {
            1 => {
                if let Some(armor) = self.armours.iter().find(|a| a.is_starting) {
                    armor.to_item()
                } else {
                    Item::Armor
                }
            }
            2 => {
                if let Some(pot) = self.potions.iter().find(|p| p.is_starting) {
                    pot.to_item()
                } else {
                    Item::Potion
                }
            }
            3 => {
                if let Some(scroll) = self.scrolls.iter().find(|s| s.is_starting) {
                    scroll.to_item(roller)
                } else {
                    let fights = roller.roll_d4() as u32;
                    Item::Scroll(ScrollType::SummonWeakDaemon {
                        fights_remaining: fights,
                    })
                }
            }
            _ => {
                if let Some(gear) = self.gear.iter().find(|g| g.is_starting) {
                    gear.to_item(roller)
                } else {
                    let uses = roller.roll_d4() as u32;
                    Item::CloakOfInvisibility {
                        uses_remaining: uses,
                    }
                }
            }
        }
    }

    /// Roll a random item on the Room Item Table (1d6).
    pub fn roll_random_item<R: DiceRoller>(&self, roller: &mut R) -> Item {
        match roller.roll_d6() {
            1 => Item::Weapon(self.roll_starting_weapon(roller)),
            2 => {
                if let Some(pot) = self.potions.first() {
                    pot.to_item()
                } else {
                    Item::Potion
                }
            }
            3 => {
                if let Some(rope) = self.gear.iter().find(|g| g.id == "rope") {
                    rope.to_item(roller)
                } else {
                    Item::Rope
                }
            }
            4 => self.roll_random_scroll(roller),
            5 => {
                if let Some(arm) = self.armours.first() {
                    arm.to_item()
                } else {
                    Item::Armor
                }
            }
            _ => {
                if let Some(cloak) = self.gear.iter().find(|g| g.id == "cloak_of_invisibility") {
                    cloak.to_item(roller)
                } else {
                    let uses = roller.roll_d4() as u32;
                    Item::CloakOfInvisibility {
                        uses_remaining: uses,
                    }
                }
            }
        }
    }

    /// Roll a random scroll on the Scroll Table (1d4).
    pub fn roll_random_scroll<R: DiceRoller>(&self, roller: &mut R) -> Item {
        Item::Scroll(self.roll_random_scroll_type(roller))
    }

    /// Roll a random ScrollType on the Scroll Table (1d4).
    pub fn roll_random_scroll_type<R: DiceRoller>(&self, roller: &mut R) -> ScrollType {
        if self.scrolls.is_empty() {
            return ScrollType::FalseOmen;
        }
        let roll = roller.roll_d4();
        let idx = ((roll - 1).max(0) as usize) % self.scrolls.len();
        self.scrolls[idx].roll_scroll_type(roller)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    pub id: String,
    pub name: String,
    pub category: MonsterCategory,
    pub points_to_hit: i32,
    pub points_awarded: i32,
    pub hp: i32,
    pub damage_formula: DiceFormula,
    #[serde(default)]
    pub loot: MonsterLoot,
    #[serde(default)]
    pub post_hazard: PostFightHazard,
    #[serde(default)]
    pub description: String,
}

impl MonsterDef {
    pub fn to_monster(&self) -> Monster {
        Monster {
            name: self.name.clone(),
            category: self.category,
            points_to_hit: self.points_to_hit,
            points_awarded: self.points_awarded,
            hp: self.hp,
            max_hp: self.hp,
            damage_formula: self.damage_formula,
            attack_count: 0,
            loot: self.loot,
            post_hazard: self.post_hazard,
        }
    }
}

/// Dynamic enemy registry loaded from data/enemies/*.json with compiled-in fallbacks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnemyRegistry {
    pub weak_monsters: Vec<MonsterDef>,
    pub tough_monsters: Vec<MonsterDef>,
    #[serde(default)]
    pub boss_monsters: Vec<MonsterDef>,
    #[serde(skip)]
    weak_by_id: HashMap<String, MonsterDef>,
    #[serde(skip)]
    tough_by_id: HashMap<String, MonsterDef>,
    #[serde(skip)]
    boss_by_id: HashMap<String, MonsterDef>,
}

impl EnemyRegistry {
    pub const DEFAULT_DATA_DIR: &'static str = "data/enemies";
    pub const SCIFI_DATA_DIR: &'static str = "data/scifi/enemies";

    pub fn load_or_default() -> Self {
        Self::load_from_dir(Self::DEFAULT_DATA_DIR).unwrap_or_else(|_| Self::default_embedded())
    }

    pub fn load_scifi_or_default() -> Self {
        Self::load_from_dir(Self::SCIFI_DATA_DIR).unwrap_or_else(|_| Self::default_scifi_embedded())
    }

    pub fn load_from_dir(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let weak_path = dir.join("weak.json");
        let tough_path = dir.join("tough.json");
        let boss_path = dir.join("boss.json");

        let weak_monsters: Vec<MonsterDef> = serde_json::from_str(&fs::read_to_string(&weak_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let tough_monsters: Vec<MonsterDef> = serde_json::from_str(&fs::read_to_string(&tough_path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let boss_monsters: Vec<MonsterDef> = if boss_path.exists() {
            serde_json::from_str(&fs::read_to_string(&boss_path)?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            Vec::new()
        };

        let mut registry = Self {
            weak_monsters,
            tough_monsters,
            boss_monsters,
            weak_by_id: HashMap::new(),
            tough_by_id: HashMap::new(),
            boss_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        Ok(registry)
    }

    pub fn default_embedded() -> Self {
        let weak_monsters: Vec<MonsterDef> =
            serde_json::from_str(include_str!("../../data/enemies/weak.json"))
                .expect("Embedded weak.json must be valid");
        let tough_monsters: Vec<MonsterDef> =
            serde_json::from_str(include_str!("../../data/enemies/tough.json"))
                .expect("Embedded tough.json must be valid");

        let mut registry = Self {
            weak_monsters,
            tough_monsters,
            boss_monsters: Vec::new(),
            weak_by_id: HashMap::new(),
            tough_by_id: HashMap::new(),
            boss_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        registry
    }

    /// Embedded sci-fi alien fallback compiled directly into the binary.
    pub fn default_scifi_embedded() -> Self {
        let weak_monsters: Vec<MonsterDef> =
            serde_json::from_str(include_str!("../../data/scifi/enemies/weak.json"))
                .expect("Embedded scifi weak.json must be valid");
        let tough_monsters: Vec<MonsterDef> =
            serde_json::from_str(include_str!("../../data/scifi/enemies/tough.json"))
                .expect("Embedded scifi tough.json must be valid");
        let boss_monsters: Vec<MonsterDef> =
            serde_json::from_str(include_str!("../../data/scifi/enemies/boss.json"))
                .expect("Embedded scifi boss.json must be valid");

        let mut registry = Self {
            weak_monsters,
            tough_monsters,
            boss_monsters,
            weak_by_id: HashMap::new(),
            tough_by_id: HashMap::new(),
            boss_by_id: HashMap::new(),
        };
        registry.rebuild_indexes();
        registry
    }

    pub fn rebuild_indexes(&mut self) {
        self.weak_by_id.clear();
        for m in &self.weak_monsters {
            self.weak_by_id.insert(m.id.clone(), m.clone());
        }
        self.tough_by_id.clear();
        for m in &self.tough_monsters {
            self.tough_by_id.insert(m.id.clone(), m.clone());
        }
        self.boss_by_id.clear();
        for m in &self.boss_monsters {
            self.boss_by_id.insert(m.id.clone(), m.clone());
        }
    }

    pub fn alien_queen(&self) -> Monster {
        if let Some(b) = self.boss_monsters.iter().find(|m| m.id == "alien_queen") {
            b.to_monster()
        } else {
            Monster {
                name: "The Alien Queen (Broodmother)".to_string(),
                category: MonsterCategory::Tough,
                points_to_hit: 5,
                points_awarded: 25,
                hp: 22,
                max_hp: 22,
                damage_formula: DiceFormula::TwoD6,
                attack_count: 0,
                loot: MonsterLoot::None,
                post_hazard: PostFightHazard::None,
            }
        }
    }

    pub fn get_weak(&self, id: &str) -> Option<&MonsterDef> {
        self.weak_by_id.get(id)
    }

    pub fn get_tough(&self, id: &str) -> Option<&MonsterDef> {
        self.tough_by_id.get(id)
    }

    pub fn roll_weak<R: DiceRoller>(&self, roller: &mut R) -> Monster {
        if self.weak_monsters.is_empty() {
            return Monster {
                name: "Blood-drenched Skeleton".to_string(),
                category: MonsterCategory::Weak,
                points_to_hit: 3,
                points_awarded: 3,
                hp: 6,
                max_hp: 6,
                damage_formula: DiceFormula::D4,
                attack_count: 0,
                loot: MonsterLoot::DaggerChance,
                post_hazard: PostFightHazard::None,
            };
        }
        let roll = roller.roll_d4();
        let idx = ((roll - 1).max(0) as usize) % self.weak_monsters.len();
        self.weak_monsters[idx].to_monster()
    }

    pub fn roll_tough<R: DiceRoller>(&self, roller: &mut R) -> Monster {
        if self.tough_monsters.is_empty() {
            return Monster {
                name: "Necro-Sorcerer".to_string(),
                category: MonsterCategory::Tough,
                points_to_hit: 4,
                points_awarded: 4,
                hp: 8,
                max_hp: 8,
                damage_formula: DiceFormula::D4,
                attack_count: 0,
                loot: MonsterLoot::NecroSilver,
                post_hazard: PostFightHazard::MaggotTransformation,
            };
        }
        let roll = roller.roll_d4();
        let idx = ((roll - 1).max(0) as usize) % self.tough_monsters.len();
        self.tough_monsters[idx].to_monster()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShopItemType {
    Potion,
    RandomScroll,
    #[serde(alias = "scroll")]
    Scroll,
    Armor,
    Weapon,
    Gear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopWare {
    pub key: char,
    pub name: String,
    pub item_type: ShopItemType,
    #[serde(default)]
    pub item_id: Option<String>,
    pub price: i32,
}

impl ShopWare {
    pub fn create_item<R: DiceRoller>(&self, registry: &ItemRegistry, roller: &mut R) -> Item {
        match self.item_type {
            ShopItemType::Potion => {
                if let Some(ref id) = self.item_id {
                    if let Some(pot) = registry.potions.iter().find(|p| &p.id == id) {
                        return pot.to_item();
                    }
                }
                if let Some(pot) = registry.potions.first() {
                    pot.to_item()
                } else {
                    Item::Potion
                }
            }
            ShopItemType::Scroll | ShopItemType::RandomScroll => {
                if let Some(ref id) = self.item_id {
                    if let Some(s) = registry.scrolls.iter().find(|s| &s.id == id) {
                        return s.to_item(roller);
                    }
                }
                registry.roll_random_scroll(roller)
            }
            ShopItemType::Armor => {
                if let Some(ref id) = self.item_id {
                    if let Some(arm) = registry.armours.iter().find(|a| &a.id == id) {
                        return arm.to_item();
                    }
                }
                if let Some(arm) = registry.armours.first() {
                    arm.to_item()
                } else {
                    Item::Armor
                }
            }
            ShopItemType::Weapon => {
                if let Some(ref id) = self.item_id {
                    if let Some(w) = registry.get_weapon(id) {
                        return Item::Weapon(w.to_weapon_type());
                    }
                }
                Item::Weapon(registry.roll_starting_weapon(roller))
            }
            ShopItemType::Gear => {
                if let Some(ref id) = self.item_id {
                    if let Some(g) = registry.gear.iter().find(|g| &g.id == id) {
                        return g.to_item(roller);
                    }
                }
                Item::Rope
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShopConfig {
    pub wares: Vec<ShopWare>,
    #[serde(default)]
    pub prices: HashMap<String, i32>,
}

impl ShopConfig {
    pub const DEFAULT_FILE_PATH: &'static str = "data/shop.json";
    pub const SCIFI_DEFAULT_FILE_PATH: &'static str = "data/scifi/shop.json";

    pub fn load_or_default() -> Self {
        Self::load_from_file(Self::DEFAULT_FILE_PATH).unwrap_or_else(|_| Self::default_embedded())
    }

    pub fn load_scifi_or_default() -> Self {
        Self::load_from_file(Self::SCIFI_DEFAULT_FILE_PATH).unwrap_or_else(|_| Self::default_scifi_embedded())
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    pub fn default_embedded() -> Self {
        serde_json::from_str(include_str!("../../data/shop.json"))
            .expect("Embedded shop.json must be valid")
    }

    pub fn default_scifi_embedded() -> Self {
        serde_json::from_str(include_str!("../../data/scifi/shop.json"))
            .expect("Embedded scifi shop.json must be valid")
    }

    pub fn get_price(&self, id: &str) -> Option<i32> {
        self.prices.get(id).copied()
    }
}
