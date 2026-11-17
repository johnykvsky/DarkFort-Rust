use crate::core::character::{Character, CharacterVitals};
use crate::core::config::EngineType;
use crate::core::dice::{DiceFormula, DiceRoller};
use crate::core::equipment::{Item, ScrollType, WeaponType};
use crate::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard, ToughMonsterType, WeakMonsterType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerCombatAction {
    Attack,
    CastPalmsOpenSouthernGate,
    DrinkPotion,
    UseCloakOfInvisibility,
    Flee,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundOutcome {
    PlayerHit {
        attack_roll: i32,
        attack_bonus: i32,
        total_attack: i32,
        target_points: i32,
        weapon_damage: i32,
        daemon_damage: i32,
        monster_hp_left: i32,
    },
    PlayerMissMonsterHit {
        attack_roll: i32,
        attack_bonus: i32,
        total_attack: i32,
        target_points: i32,
        is_death_ray: bool,
        raw_monster_damage: i32,
        halved: bool,
        armor_absorbed: i32,
        aegis_absorbed: i32,
        net_player_damage: i32,
        player_hp_left: i32,
    },
    MothershipHit {
        roll: u8,
        target: u8,
        is_crit: bool,
        is_double: bool,
        damage: i32,
        monster_hp_left: i32,
        stress_delta: i32,
    },
    MothershipMissMonsterHit {
        roll: u8,
        target: u8,
        is_crit_fail: bool,
        raw_monster_damage: i32,
        ap_absorbed: i32,
        net_player_damage: i32,
        player_hp_left: i32,
        player_wounds: i32,
        wound_taken: bool,
    },
    DurfHit {
        roll: i32,
        target: i32,
        damage: i32,
        monster_hp_left: i32,
    },
    DurfMissMonsterHit {
        roll: i32,
        target: i32,
        raw_monster_damage: i32,
        shield_absorbed: i32,
        armor_absorbed: i32,
        net_damage: i32,
        wounds_total: i32,
        player_died: bool,
    },
    MorkBorgHit {
        attack_roll: i32,
        is_crit: bool,
        is_fumble: bool,
        damage: i32,
        monster_hp_left: i32,
    },
    MorkBorgMissMonsterHit {
        attack_roll: i32,
        defense_roll: i32,
        is_defense_success: bool,
        raw_monster_damage: i32,
        armor_absorbed: i32,
        net_player_damage: i32,
        player_hp_left: i32,
    },
    AdndHit {
        thac0: i32,
        target_ac: i32,
        roll: i32,
        effective_ac_hit: i32,
        is_crit: bool,
        damage: i32,
        monster_hp_left: i32,
    },
    AdndMissMonsterHit {
        thac0: i32,
        target_ac: i32,
        roll: i32,
        effective_ac_hit: i32,
        monster_thac0: i32,
        player_ac: i32,
        monster_roll: i32,
        monster_hit: bool,
        damage_taken: i32,
        player_hp_left: i32,
    },
    SpellCast {
        spell_name: String,
        damage: i32,
        monster_hp_left: i32,
    },
    PotionDrank {
        healed: i32,
        player_hp_left: i32,
    },
    CloakUsed {
        points_awarded: i32,
    },
    Fled {
        damage_taken: i32,
        player_hp_left: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatVictory {
    pub points_earned: i32,
    pub looted_items: Vec<Item>,
    pub looted_silver: i32,
    pub hazard_triggered: Option<String>,
    pub instant_level_up: bool,
    pub adventure_ended: bool,
}

pub struct CombatEngine;

impl CombatEngine {
    /// Resolve one standard attack round: Player attacks with equipped weapon.
    pub fn resolve_attack_round<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> RoundOutcome {
        match player.engine_type {
            EngineType::Mothership => return Self::resolve_mothership_attack_round(player, monster, roller),
            EngineType::Durf => return Self::resolve_durf_attack_round(player, monster, roller),
            EngineType::MorkBorg => return Self::resolve_mork_borg_attack_round(player, monster, roller),
            EngineType::Adnd2e => return Self::resolve_adnd_attack_round(player, monster, roller),
            EngineType::Classic => {}
        }

        let attack_roll = roller.roll_d6();
        let attack_bonus = player.total_attack_bonus();
        let total_attack = attack_roll + attack_bonus;

        if total_attack >= monster.points_to_hit {
            // Player HIT!
            let weapon_damage = roller.roll_formula(player.weapon.damage_formula());
            let mut daemon_damage = 0;
            if player.active_daemon_fights_left > 0 {
                daemon_damage = roller.roll_d4();
            }

            let total_damage = weapon_damage + daemon_damage;
            monster.hp -= total_damage;

            RoundOutcome::PlayerHit {
                attack_roll,
                attack_bonus,
                total_attack,
                target_points: monster.points_to_hit,
                weapon_damage,
                daemon_damage,
                monster_hp_left: monster.hp,
            }
        } else {
            // Player MISS! Monster strikes back.
            let (raw_monster_damage, is_death_ray) = monster.roll_attack_damage(roller);

            // Check if damage is halved from Level Up #6
            let is_halved = match monster.category {
                MonsterCategory::Weak => {
                    player.halved_weak_monster.is_some() &&
                        player.halved_weak_monster == match monster.name.as_str() {
                            "Blood-drenched Skeleton" => Some(WeakMonsterType::BloodDrenchedSkeleton),
                            "Catacomb Cultist" => Some(WeakMonsterType::CatacombCultist),
                            "Goblin" => Some(WeakMonsterType::Goblin),
                            "Undead Hound" => Some(WeakMonsterType::UndeadHound),
                            _ => None,
                        }
                }
                MonsterCategory::Tough => {
                    player.halved_tough_monster.is_some() &&
                        player.halved_tough_monster == match monster.name.as_str() {
                            "Necro-Sorcerer" => Some(ToughMonsterType::NecroSorcerer),
                            "Small Stone Troll" => Some(ToughMonsterType::SmallStoneTroll),
                            "Medusa" => Some(ToughMonsterType::Medusa),
                            "Ruin Basilisk" => Some(ToughMonsterType::RuinBasilisk),
                            _ => None,
                        }
                }
            };

            let effective_damage = if is_halved {
                (raw_monster_damage + 1) / 2
            } else {
                raw_monster_damage
            };

            let (net_damage, armor_absorbed, aegis_absorbed) =
                player.take_damage(effective_damage, false, roller);

            RoundOutcome::PlayerMissMonsterHit {
                attack_roll,
                attack_bonus,
                total_attack,
                target_points: monster.points_to_hit,
                is_death_ray,
                raw_monster_damage,
                halved: is_halved,
                armor_absorbed,
                aegis_absorbed,
                net_player_damage: net_damage,
                player_hp_left: player.hp,
            }
        }
    }

    /// Cast Palms Open the Southern Gate scroll (1d6+1 damage to monster).
    pub fn cast_palms_scroll<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> Option<RoundOutcome> {
        let scroll_pos = player.inventory.iter().position(|i| match i {
            Item::Scroll(ScrollType::PalmsOpenSouthernGate { uses_remaining }) => *uses_remaining > 0,
            _ => false,
        })?;

        let mut remove = false;
        if let Item::Scroll(ScrollType::PalmsOpenSouthernGate { uses_remaining }) = &mut player.inventory[scroll_pos] {
            *uses_remaining -= 1;
            if *uses_remaining == 0 {
                remove = true;
            }
        }
        if remove {
            player.inventory.remove(scroll_pos);
        }

        let damage = roller.roll_formula(DiceFormula::D6Plus1);
        monster.hp -= damage;

        Some(RoundOutcome::SpellCast {
            spell_name: "Palms Open the Southern Gate".to_string(),
            damage,
            monster_hp_left: monster.hp,
        })
    }

    /// Flee fight: Kargunt takes 1d4 damage, leaving room unexplored.
    pub fn flee<R: DiceRoller>(player: &mut Character, roller: &mut R) -> RoundOutcome {
        let flee_damage = roller.roll_d4();
        let (net_damage, _, _) = player.take_damage(flee_damage, true, roller); // Direct damage
        RoundOutcome::Fled {
            damage_taken: net_damage,
            player_hp_left: player.hp,
        }
    }

    /// Resolve victory when monster is defeated or bypassed with Cloak.
    pub fn resolve_victory<R: DiceRoller>(
        player: &mut Character,
        monster: &Monster,
        roller: &mut R,
    ) -> CombatVictory {
        // Points awarded
        let points = monster.points_awarded;
        player.points += points;

        // Decrement daemon duration if active
        if player.active_daemon_fights_left > 0 {
            player.active_daemon_fights_left -= 1;
        }

        let mut looted_items = Vec::new();
        let mut looted_silver = 0;

        // Resolve Loot
        match monster.loot {
            MonsterLoot::DaggerChance => {
                if roller.check_x_in_6(2) {
                    looted_items.push(Item::Weapon(WeaponType::Dagger));
                }
            }
            MonsterLoot::ScrollChance => {
                if roller.check_x_in_6(2) {
                    looted_items.push(Item::Scroll(ScrollType::roll_random(roller)));
                }
            }
            MonsterLoot::RopeChance => {
                if roller.check_x_in_6(2) {
                    looted_items.push(Item::Rope);
                }
            }
            MonsterLoot::NecroSilver => {
                looted_silver = roller.roll_formula(DiceFormula::ThreeD6);
                player.silver += looted_silver;
            }
            MonsterLoot::MedusaSilver => {
                looted_silver = roller.roll_formula(DiceFormula::D4TimesD6);
                player.silver += looted_silver;
            }
            MonsterLoot::None => {}
        }

        // Add items to inventory
        for item in &looted_items {
            player.inventory.push(item.clone());
        }

        // Resolve post-fight hazard / boon
        let mut hazard_triggered = None;
        let mut instant_level_up = false;
        let mut adventure_ended = false;

        match monster.post_hazard {
            PostFightHazard::MaggotTransformation => {
                // 1-in-6 chance transformed into a maggot
                if roller.check_x_in_6(1) {
                    hazard_triggered = Some("You were transformed into a maggot! Your adventure ends here.".to_string());
                    adventure_ended = true;
                }
            }
            PostFightHazard::Petrification => {
                // 1-in-6 chance petrified
                if roller.check_x_in_6(1) {
                    hazard_triggered = Some("You caught the Medusa's dying gaze and turned into stone! Your adventure ends here.".to_string());
                    adventure_ended = true;
                }
            }
            PostFightHazard::ImmediateLevelUp => {
                // 2-in-6 chance immediately LEVEL UP
                if roller.check_x_in_6(2) {
                    instant_level_up = true;
                }
            }
            PostFightHazard::None => {}
        }

        CombatVictory {
            points_earned: points,
            looted_items,
            looted_silver,
            hazard_triggered,
            instant_level_up,
            adventure_ended,
        }
    }

    /// Resolve one attack round using the Mothership 1e percentile engine.
    fn resolve_mothership_attack_round<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> RoundOutcome {
        let combat_target = if let CharacterVitals::Mothership { combat, .. } = player.vitals {
            (combat + player.weapon.attack_bonus() * 5).clamp(5, 95) as u8
        } else {
            40
        };

        let roll = roller.roll_d100();
        let is_double = (roll % 11 == 0) || roll == 100;
        let is_hit = roll <= combat_target;

        if is_hit {
            let damage = if is_double {
                // Critical Hit on matching double digits: Maximum damage + 2
                match player.weapon.damage_formula() {
                    DiceFormula::D4 => 4 + 2,
                    DiceFormula::D6 => 6 + 2,
                    DiceFormula::TwoD6 => 12 + 2,
                    DiceFormula::D6Plus1 => 7 + 2,
                    DiceFormula::D6Plus2 => 8 + 2,
                    _ => 8,
                }
            } else {
                roller.roll_formula(player.weapon.damage_formula()).max(1)
            };

            let mut stress_delta = 0;
            if is_double {
                if let CharacterVitals::Mothership { stress, .. } = &mut player.vitals {
                    *stress = stress.saturating_sub(1).max(2);
                    stress_delta = -1;
                }
            }

            monster.hp -= damage;

            RoundOutcome::MothershipHit {
                roll,
                target: combat_target,
                is_crit: is_double,
                is_double,
                damage,
                monster_hp_left: monster.hp,
                stress_delta,
            }
        } else {
            let is_crit_fail = is_double;
            if is_crit_fail {
                if let CharacterVitals::Mothership { stress, max_stress, .. } = &mut player.vitals {
                    *stress = (*stress + 1).min(*max_stress);
                }
            }

            let (raw_monster_damage, _) = monster.roll_attack_damage(roller);
            let wounds_before = match &player.vitals {
                CharacterVitals::Mothership { wounds, .. } => *wounds,
                _ => 0,
            };

            let (net_damage, ap_absorbed, _) = player.take_damage(raw_monster_damage, false, roller);

            let wounds_after = match &player.vitals {
                CharacterVitals::Mothership { wounds, .. } => *wounds,
                _ => 0,
            };

            RoundOutcome::MothershipMissMonsterHit {
                roll,
                target: combat_target,
                is_crit_fail,
                raw_monster_damage,
                ap_absorbed,
                net_player_damage: net_damage,
                player_hp_left: player.hp,
                player_wounds: wounds_after,
                wound_taken: wounds_after > wounds_before,
            }
        }
    }

    /// Resolve one attack round using the DURF engine (Ablative armor & HD checks).
    fn resolve_durf_attack_round<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> RoundOutcome {
        let force = match &player.vitals {
            CharacterVitals::Durf { force, .. } => *force as i32,
            _ => 2,
        };

        let d20 = roller.roll_d20();
        let total_attack = d20 + force + player.weapon.attack_bonus();
        let target = 15;

        if total_attack > target {
            // HIT! (DURF: over 15 is success)
            let damage = roller.roll_formula(player.weapon.damage_formula()).max(1);
            monster.hp -= damage;

            RoundOutcome::DurfHit {
                roll: total_attack,
                target,
                damage,
                monster_hp_left: monster.hp,
            }
        } else {
            // MISS! Monster strikes back
            let (raw_monster_damage, _) = monster.roll_attack_damage(roller);
            let (net_damage, absorbed, _) = player.take_damage(raw_monster_damage, false, roller);

            let (total_wounds, player_died) = match &player.vitals {
                CharacterVitals::Durf { wounds, is_dead, .. } => (*wounds, *is_dead),
                _ => (0, !player.is_alive()),
            };

            RoundOutcome::DurfMissMonsterHit {
                roll: total_attack,
                target,
                raw_monster_damage,
                shield_absorbed: 1,
                armor_absorbed: absorbed.saturating_sub(1),
                net_damage,
                wounds_total: total_wounds,
                player_died,
            }
        }
    }

    /// Resolve one attack round using the Full MÖRK BORG engine (d20 DR 12 & active defense).
    fn resolve_mork_borg_attack_round<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> RoundOutcome {
        let (strength, agility) = match &player.vitals {
            CharacterVitals::MorkBorg { strength, agility, .. } => (*strength as i32, *agility as i32),
            _ => (-1, 2),
        };

        let attack_stat = if matches!(player.weapon, WeaponType::Dagger | WeaponType::Sword) {
            agility
        } else {
            strength
        };

        let attack_d20 = roller.roll_d20();
        let is_crit = attack_d20 == 20;
        let is_fumble = attack_d20 == 1;
        let total_attack = attack_d20 + attack_stat + player.weapon.attack_bonus();

        if total_attack >= 12 && !is_fumble {
            // HIT! (DR 12 baseline)
            let raw_damage = roller.roll_formula(player.weapon.damage_formula()).max(1);
            let damage = if is_crit { raw_damage * 2 } else { raw_damage };
            monster.hp -= damage;

            RoundOutcome::MorkBorgHit {
                attack_roll: total_attack,
                is_crit,
                is_fumble,
                damage,
                monster_hp_left: monster.hp,
            }
        } else {
            // MISS or FUMBLE: Monster strikes, Player rolls active Defense (d20 + Agility vs DR 12)
            let defense_d20 = roller.roll_d20();
            let defense_total = defense_d20 + agility;
            let defense_success = defense_total >= 12 && defense_d20 != 1;

            if defense_success {
                RoundOutcome::MorkBorgMissMonsterHit {
                    attack_roll: total_attack,
                    defense_roll: defense_total,
                    is_defense_success: true,
                    raw_monster_damage: 0,
                    armor_absorbed: 0,
                    net_player_damage: 0,
                    player_hp_left: player.hp,
                }
            } else {
                let (raw_monster_damage, _) = monster.roll_attack_damage(roller);
                let (net_damage, armor_absorbed, _) = player.take_damage(raw_monster_damage, false, roller);

                RoundOutcome::MorkBorgMissMonsterHit {
                    attack_roll: total_attack,
                    defense_roll: defense_total,
                    is_defense_success: false,
                    raw_monster_damage,
                    armor_absorbed,
                    net_player_damage: net_damage,
                    player_hp_left: player.hp,
                }
            }
        }
    }

    /// Resolve one attack round using the AD&D 2nd Edition (Infinity Engine) ruleset.
    fn resolve_adnd_attack_round<R: DiceRoller>(
        player: &mut Character,
        monster: &mut Monster,
        roller: &mut R,
    ) -> RoundOutcome {
        let (thac0, player_ac) = match &player.vitals {
            CharacterVitals::Adnd2e { thac0, ac, .. } => (*thac0, *ac),
            _ => (20, 6),
        };

        // Monster AC lookup based on AD&D 2e monster archetypes
        let monster_ac = match monster.name.as_str() {
            "Blood-drenched Skeleton" => 7,
            "Goblin" => 6,
            "Catacomb Cultist" => 8,
            "Undead Hound" => 7,
            "Necro-Sorcerer" => 5,
            "Small Stone Troll" => 4,
            "Medusa" => 5,
            "Ruin Basilisk" => 3,
            _ => if monster.category == MonsterCategory::Weak { 7 } else { 5 },
        };

        let roll_d20 = roller.roll_d20();
        let weapon_bonus = player.weapon.attack_bonus();
        let total_roll = roll_d20 + weapon_bonus;
        let is_crit = roll_d20 == 20;
        let is_fumble = roll_d20 == 1;

        // In AD&D 2e: Effective AC hit = THAC0 - (d20 + attack modifiers)
        // Hit succeeds if effective_ac_hit <= monster_ac
        let effective_ac_hit = thac0 - total_roll;
        let hit_success = (effective_ac_hit <= monster_ac || is_crit) && !is_fumble;

        if hit_success {
            let raw_damage = roller.roll_formula(player.weapon.damage_formula()).max(1);
            let damage = if is_crit { raw_damage * 2 } else { raw_damage };
            monster.hp -= damage;

            RoundOutcome::AdndHit {
                thac0,
                target_ac: monster_ac,
                roll: roll_d20,
                effective_ac_hit,
                is_crit,
                damage,
                monster_hp_left: monster.hp,
            }
        } else {
            // Monster counter-attacks using AD&D 2e monster THAC0
            let monster_thac0 = if monster.category == MonsterCategory::Weak { 19 } else { 16 };
            let monster_roll = roller.roll_d20();
            let monster_effective_ac = monster_thac0 - monster_roll;
            let monster_hit = (monster_effective_ac <= player_ac || monster_roll == 20) && monster_roll != 1;

            let mut damage_taken = 0;
            if monster_hit {
                let (raw_monster_damage, _) = monster.roll_attack_damage(roller);
                let (net_damage, _, _) = player.take_damage(raw_monster_damage, false, roller);
                damage_taken = net_damage;
            }

            RoundOutcome::AdndMissMonsterHit {
                thac0,
                target_ac: monster_ac,
                roll: roll_d20,
                effective_ac_hit,
                monster_thac0,
                player_ac,
                monster_roll,
                monster_hit,
                damage_taken,
                player_hp_left: player.hp,
            }
        }
    }
}
