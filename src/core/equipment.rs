use crate::core::dice::{DiceFormula, DiceRoller};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Warhammer,
    Dagger,
    Sword,
    Flail,
    MightyZweihaender,
    Unarmed,
    Custom {
        id: String,
        name: String,
        damage_formula: DiceFormula,
        attack_bonus: i32,
        price: i32,
    },
}

impl WeaponType {
    pub fn name(&self) -> &str {
        match self {
            WeaponType::Warhammer => "Warhammer",
            WeaponType::Dagger => "Dagger",
            WeaponType::Sword => "Sword",
            WeaponType::Flail => "Flail",
            WeaponType::MightyZweihaender => "Mighty Zweihänder",
            WeaponType::Unarmed => "Fists (Unarmed)",
            WeaponType::Custom { name, .. } => name.as_str(),
        }
    }

    pub fn damage_formula(&self) -> DiceFormula {
        match self {
            WeaponType::Warhammer => DiceFormula::D6,
            WeaponType::Dagger => DiceFormula::D4,
            WeaponType::Sword => DiceFormula::D6,
            WeaponType::Flail => DiceFormula::D6Plus1,
            WeaponType::MightyZweihaender => DiceFormula::D6Plus2,
            WeaponType::Unarmed => DiceFormula::D4Minus1,
            WeaponType::Custom { damage_formula, .. } => *damage_formula,
        }
    }

    /// Underlined weapons (Dagger, Sword) grant +1 to attack rolls, not damage.
    pub fn attack_bonus(&self) -> i32 {
        match self {
            WeaponType::Dagger | WeaponType::Sword => 1,
            WeaponType::Custom { attack_bonus, .. } => *attack_bonus,
            _ => 0,
        }
    }

    pub fn price(&self) -> i32 {
        match self {
            WeaponType::Dagger => 6,
            WeaponType::Warhammer => 9,
            WeaponType::Sword => 12,
            WeaponType::Flail => 15,
            WeaponType::MightyZweihaender => 25,
            WeaponType::Unarmed => 0,
            WeaponType::Custom { price, .. } => *price,
        }
    }

    /// Roll initial starting weapon from registry (Table A).
    pub fn roll_starting<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::ItemRegistry::load_or_default().roll_starting_weapon(roller)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollType {
    SummonWeakDaemon { fights_remaining: u32 },
    PalmsOpenSouthernGate { uses_remaining: u32 },
    AegisOfSorrow { uses_remaining: u32 },
    FalseOmen,
}

impl ScrollType {
    pub fn name(&self) -> &'static str {
        match self {
            ScrollType::SummonWeakDaemon { .. } => "Scroll of Summon Weak Daemon",
            ScrollType::PalmsOpenSouthernGate { .. } => "Scroll: Palms Open the Southern Gate",
            ScrollType::AegisOfSorrow { .. } => "Scroll: Aegis of Sorrow",
            ScrollType::FalseOmen => "Scroll: False Omen",
        }
    }

    pub fn description(&self) -> String {
        match self {
            ScrollType::SummonWeakDaemon { fights_remaining } => {
                format!("Summons a weak daemon (deals 1d4 dmg/fight). {} fights left.", fights_remaining)
            }
            ScrollType::PalmsOpenSouthernGate { uses_remaining } => {
                format!("Deals 1d6+1 damage. {} uses left.", uses_remaining)
            }
            ScrollType::AegisOfSorrow { uses_remaining } => {
                format!("Absorbs 1d4 damage when hit. {} uses left.", uses_remaining)
            }
            ScrollType::FalseOmen => {
                "Either choose a room table result or reroll any die (1 use).".to_string()
            }
        }
    }

    pub fn roll_random<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::ItemRegistry::load_or_default().roll_random_scroll_type(roller)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Item {
    Weapon(WeaponType),
    Armor, // -1d4 damage absorbed
    Potion, // heals 1d6 HP
    Rope, // +1 on pit trap roll
    CloakOfInvisibility { uses_remaining: u32 }, // avoid 1d4 fights
    Scroll(ScrollType),
    CustomArmor {
        id: String,
        name: String,
        absorption_formula: DiceFormula,
        price: i32,
    },
    CustomPotion {
        id: String,
        name: String,
        heal_formula: DiceFormula,
        price: i32,
    },
    CustomGear {
        id: String,
        name: String,
        gear_kind: crate::core::registry::GearKind,
        uses_remaining: Option<u32>,
        price: i32,
    },
}

impl Item {
    pub fn name(&self) -> String {
        match self {
            Item::Weapon(w) => w.name().to_string(),
            Item::Armor => "Armor (-1d4 damage)".to_string(),
            Item::Potion => "Healing Potion (heals 1d6 hp)".to_string(),
            Item::Rope => "Rope (+1 on trap rolls)".to_string(),
            Item::CloakOfInvisibility { uses_remaining } => {
                format!("Cloak of Invisibility ({} uses left)", uses_remaining)
            }
            Item::Scroll(s) => s.name().to_string(),
            Item::CustomArmor { name, .. } => name.clone(),
            Item::CustomPotion { name, .. } => name.clone(),
            Item::CustomGear { name, uses_remaining, .. } => {
                if let Some(uses) = uses_remaining {
                    format!("{} ({} uses left)", name, uses)
                } else {
                    name.clone()
                }
            }
        }
    }

    pub fn price(&self) -> i32 {
        match self {
            Item::Weapon(w) => w.price(),
            Item::Armor => 10,
            Item::Potion => 4,
            Item::Rope => 5,
            Item::CloakOfInvisibility { .. } => 15,
            Item::Scroll(_) => 7,
            Item::CustomArmor { price, .. } => *price,
            Item::CustomPotion { price, .. } => *price,
            Item::CustomGear { price, .. } => *price,
        }
    }

    /// Roll on the Room Item Table.
    pub fn roll_random_item<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::ItemRegistry::load_or_default().roll_random_item(roller)
    }

    /// Roll initial starting Item (Table B).
    pub fn roll_starting<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::ItemRegistry::load_or_default().roll_starting_item(roller)
    }
}
