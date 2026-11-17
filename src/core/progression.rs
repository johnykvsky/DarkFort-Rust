use crate::core::character::Character;
use crate::core::dice::DiceRoller;
use crate::core::equipment::{Item, WeaponType};
use crate::core::monsters::{ToughMonsterType, WeakMonsterType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelUpReward {
    Knighted,                // #1: Sir/Lady title
    AttackBonusPlusOne,      // #2: +1 attack
    MaxHpPlusFive,           // #3: Max HP increases to 20
    HerbmasterFivePotions,   // #4: 5 potions
    MightyZweihaender,       // #5: Greatsword (1d6+2)
    HalvedMonsterDamage,     // #6: Choose 1 Weak & 1 Tough monster
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionManager {
    pub scratched_rewards: Vec<LevelUpReward>,
}

impl ProgressionManager {
    pub fn new() -> Self {
        Self {
            scratched_rewards: Vec::new(),
        }
    }
}

impl Default for ProgressionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressionManager {

    /// Check if character can level up via standard conditions.
    pub fn can_level_up_via_exploration(player: &Character) -> bool {
        player.rooms_explored >= 12 && player.points >= 15
    }

    pub fn can_level_up_via_charity(player: &Character) -> bool {
        player.silver >= 40
    }

    /// Check if all 6 rewards have been achieved (Grand Victory / 7th Misery).
    pub fn is_grand_victory(&self) -> bool {
        self.scratched_rewards.len() >= 6
    }

    /// Roll on the Level-Up table (d6), rerolling any already scratched results.
    pub fn roll_new_reward<R: DiceRoller>(&mut self, roller: &mut R) -> Option<LevelUpReward> {
        if self.is_grand_victory() {
            return None;
        }

        loop {
            let roll = roller.roll_d6();
            let reward = match roll {
                1 => LevelUpReward::Knighted,
                2 => LevelUpReward::AttackBonusPlusOne,
                3 => LevelUpReward::MaxHpPlusFive,
                4 => LevelUpReward::HerbmasterFivePotions,
                5 => LevelUpReward::MightyZweihaender,
                _ => LevelUpReward::HalvedMonsterDamage,
            };

            if !self.scratched_rewards.contains(&reward) {
                self.scratched_rewards.push(reward);
                return Some(reward);
            }
        }
    }

    /// Apply the reward to Kargunt.
    pub fn apply_reward(
        &self,
        reward: LevelUpReward,
        player: &mut Character,
        chosen_weak: Option<WeakMonsterType>,
        chosen_tough: Option<ToughMonsterType>,
    ) {
        player.level += 1;

        match reward {
            LevelUpReward::Knighted => {
                player.title = Some("Sir Kargunt".to_string());
            }
            LevelUpReward::AttackBonusPlusOne => {
                player.permanent_attack_bonus += 1;
            }
            LevelUpReward::MaxHpPlusFive => {
                player.max_hp = 20;
                player.hp = (player.hp + 5).min(20);
            }
            LevelUpReward::HerbmasterFivePotions => {
                for _ in 0..5 {
                    player.inventory.push(Item::Potion);
                }
            }
            LevelUpReward::MightyZweihaender => {
                player.inventory.push(Item::Weapon(WeaponType::MightyZweihaender));
            }
            LevelUpReward::HalvedMonsterDamage => {
                if let Some(weak) = chosen_weak {
                    player.halved_weak_monster = Some(weak);
                }
                if let Some(tough) = chosen_tough {
                    player.halved_tough_monster = Some(tough);
                }
            }
        }
    }
}
