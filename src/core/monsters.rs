use crate::core::dice::{DiceFormula, DiceRoller};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeakMonsterType {
    BloodDrenchedSkeleton,
    CatacombCultist,
    Goblin,
    UndeadHound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToughMonsterType {
    NecroSorcerer,
    SmallStoneTroll,
    Medusa,
    RuinBasilisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterCategory {
    Weak,
    Tough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MonsterLoot {
    #[default]
    None,
    DaggerChance,       // 2-in-6
    ScrollChance,       // 2-in-6
    RopeChance,         // 2-in-6
    NecroSilver,        // 3d6 silver
    MedusaSilver,       // 1d4 * 1d6 silver
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PostFightHazard {
    #[default]
    None,
    MaggotTransformation, // 1-in-6: Adventure ends
    Petrification,        // 1-in-6: Adventure ends
    ImmediateLevelUp,     // 2-in-6: Level up immediately
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Monster {
    pub name: String,
    pub category: MonsterCategory,
    pub points_to_hit: i32,
    pub points_awarded: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub damage_formula: DiceFormula,
    pub attack_count: u32, // For tracking alternating death-ray attacks
    pub loot: MonsterLoot,
    pub post_hazard: PostFightHazard,
}

impl Monster {
    pub fn roll_weak<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::EnemyRegistry::load_or_default().roll_weak(roller)
    }

    pub fn roll_tough<R: DiceRoller>(roller: &mut R) -> Self {
        crate::core::registry::EnemyRegistry::load_or_default().roll_tough(roller)
    }

    /// Roll monster's attack damage. Handles Necro-Sorcerer alternating death-ray (every other attack is d6 death ray).
    pub fn roll_attack_damage<R: DiceRoller>(&mut self, roller: &mut R) -> (i32, bool) {
        self.attack_count += 1;
        if self.name == "Necro-Sorcerer" && self.attack_count.is_multiple_of(2) {
            // Every other attack is a death-ray dealing 1d6 damage
            (roller.roll_d6(), true)
        } else {
            (roller.roll_formula(self.damage_formula), false)
        }
    }
}
