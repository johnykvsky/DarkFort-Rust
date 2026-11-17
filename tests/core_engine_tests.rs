use dark_fort::core::character::Character;
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::dice::{DiceFormula, MockRoller};
use dark_fort::core::equipment::{Item, ScrollType, WeaponType};
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard, ToughMonsterType, WeakMonsterType};
use dark_fort::core::progression::ProgressionManager;

#[test]
fn test_character_creation_and_attack_bonus() {
    // d4 rolls: weapon roll 2 (Dagger), item roll 1 (Armor)
    // d6 rolls: silver roll 4 (15 + 4 = 19 silver)
    let mut mock = MockRoller::new(vec![2, 1], vec![4]);
    let kargunt = Character::new_kargunt(&mut mock);

    assert_eq!(kargunt.name, "Kargunt");
    assert_eq!(kargunt.hp, 15);
    assert_eq!(kargunt.max_hp, 15);
    assert_eq!(kargunt.silver, 19);
    assert_eq!(kargunt.weapon, WeaponType::Dagger);
    assert!(kargunt.has_armor);
    assert_eq!(kargunt.total_attack_bonus(), 1); // Dagger has +1 attack bonus
}

#[test]
fn test_armor_absorption() {
    let mut mock = MockRoller::new(vec![2, 1], vec![1]); // Dagger, Armor
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.has_armor = true;

    // Incoming 3 damage, armor absorbs roll of 2 -> net 1 damage taken
    let mut roll_mock = MockRoller::new(vec![2], vec![]);
    let (net_dmg, armor, aegis) = kargunt.take_damage(3, false, &mut roll_mock);
    assert_eq!(armor, 2);
    assert_eq!(aegis, 0);
    assert_eq!(net_dmg, 1);
    assert_eq!(kargunt.hp, 14);
}

#[test]
fn test_potion_healing_bounded_by_max_hp() {
    let mut mock = MockRoller::new(vec![1, 2], vec![1]); // Warhammer, Potion
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.hp = 12;
    assert_eq!(kargunt.inventory.len(), 1);

    // Roll 5 on potion heal -> heals 3 HP to cap at max_hp (15)
    let mut roll_mock = MockRoller::new(vec![], vec![5]);
    let healed = kargunt.drink_potion(&mut roll_mock);
    assert_eq!(healed, Some(5));
    assert_eq!(kargunt.hp, 15);
    assert_eq!(kargunt.inventory.len(), 0); // Consumed
}

#[test]
fn test_combat_hit_and_slay_skeleton() {
    let mut mock = MockRoller::new(vec![3, 1], vec![3]); // Sword (+1 att), Armor
    let mut kargunt = Character::new_kargunt(&mut mock);

    let mut skeleton = Monster {
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

    // Attack roll: roll 3 + 1 bonus = 4 >= 3 -> HIT!
    // Sword damage roll: 6
    let mut combat_mock = MockRoller::new(vec![], vec![3, 6]);
    let outcome = CombatEngine::resolve_attack_round(&mut kargunt, &mut skeleton, &mut combat_mock);

    match outcome {
        RoundOutcome::PlayerHit {
            attack_roll,
            attack_bonus,
            total_attack,
            weapon_damage,
            monster_hp_left,
            ..
        } => {
            assert_eq!(attack_roll, 3);
            assert_eq!(attack_bonus, 1);
            assert_eq!(total_attack, 4);
            assert_eq!(weapon_damage, 6);
            assert_eq!(monster_hp_left, 0);
        }
        _ => panic!("Expected PlayerHit"),
    }

    // Victory: loot check roll 2 (2-in-6 -> dagger dropped!)
    let mut victory_mock = MockRoller::new(vec![], vec![2]);
    let victory = CombatEngine::resolve_victory(&mut kargunt, &skeleton, &mut victory_mock);
    assert_eq!(victory.points_earned, 3);
    assert_eq!(kargunt.points, 3);
    assert_eq!(victory.looted_items, vec![Item::Weapon(WeaponType::Dagger)]);
}

#[test]
fn test_combat_miss_and_monster_counterattack() {
    let mut mock = MockRoller::new(vec![1, 1], vec![3]); // Warhammer (+0 att), Armor
    let mut kargunt = Character::new_kargunt(&mut mock);

    let mut hound = Monster {
        name: "Undead Hound".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 4,
        points_awarded: 4,
        hp: 6,
        max_hp: 6,
        damage_formula: DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    // Attack roll: roll 2 + 0 bonus = 2 < 4 -> MISS!
    // Hound deals 4 damage (d4 roll 4)
    // Armor absorbs 2 damage (d4 roll 2)
    // Net player damage: 2 (15 -> 13)
    let mut combat_mock = MockRoller::new(vec![4, 2], vec![2]);
    let outcome = CombatEngine::resolve_attack_round(&mut kargunt, &mut hound, &mut combat_mock);

    match outcome {
        RoundOutcome::PlayerMissMonsterHit {
            attack_roll,
            raw_monster_damage,
            armor_absorbed,
            net_player_damage,
            player_hp_left,
            ..
        } => {
            assert_eq!(attack_roll, 2);
            assert_eq!(raw_monster_damage, 4);
            assert_eq!(armor_absorbed, 2);
            assert_eq!(net_player_damage, 2);
            assert_eq!(player_hp_left, 13);
        }
        _ => panic!("Expected PlayerMissMonsterHit"),
    }
}

#[test]
fn test_daemon_assist_and_counter_decrement() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.active_daemon_fights_left = 2;

    let mut goblin = Monster {
        name: "Goblin".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 3,
        points_awarded: 3,
        hp: 5,
        max_hp: 5,
        damage_formula: DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    // Player attack roll: 4 (HIT)
    // Warhammer damage: 2
    // Daemon damage: d4 roll 3 -> total 5 damage! Slays goblin in 1 hit!
    let mut combat_mock = MockRoller::new(vec![3], vec![4, 2]);
    let outcome = CombatEngine::resolve_attack_round(&mut kargunt, &mut goblin, &mut combat_mock);

    match outcome {
        RoundOutcome::PlayerHit {
            weapon_damage,
            daemon_damage,
            monster_hp_left,
            ..
        } => {
            assert_eq!(weapon_damage, 2);
            assert_eq!(daemon_damage, 3);
            assert_eq!(monster_hp_left, 0);
        }
        _ => panic!("Expected PlayerHit"),
    }

    // Resolving victory should decrement active_daemon_fights_left from 2 to 1
    let mut vic_mock = MockRoller::new(vec![], vec![]);
    CombatEngine::resolve_victory(&mut kargunt, &goblin, &mut vic_mock);
    assert_eq!(kargunt.active_daemon_fights_left, 1);
}

#[test]
fn test_palms_scroll_casting() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.inventory.push(Item::Scroll(ScrollType::PalmsOpenSouthernGate { uses_remaining: 1 }));

    let mut troll = Monster {
        name: "Small Stone Troll".to_string(),
        category: MonsterCategory::Tough,
        points_to_hit: 5,
        points_awarded: 7,
        hp: 9,
        max_hp: 9,
        damage_formula: DiceFormula::D6Plus1,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    // Spell damage: 1d6+1 (roll 5 -> 6 damage)
    let mut spell_mock = MockRoller::new(vec![], vec![5]);
    let outcome = CombatEngine::cast_palms_scroll(&mut kargunt, &mut troll, &mut spell_mock).unwrap();

    match outcome {
        RoundOutcome::SpellCast { damage, monster_hp_left, .. } => {
            assert_eq!(damage, 6);
            assert_eq!(monster_hp_left, 3);
        }
        _ => panic!("Expected SpellCast"),
    }

    // Since it had 1 use, it should now be removed from inventory
    assert!(kargunt.inventory.is_empty());
}

#[test]
fn test_cloak_of_invisibility_usage() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.inventory.push(Item::CloakOfInvisibility { uses_remaining: 2 });

    assert!(kargunt.has_cloak());
    assert!(kargunt.use_cloak_charge());

    // Remaining uses should now be 1
    match &kargunt.inventory[0] {
        Item::CloakOfInvisibility { uses_remaining } => assert_eq!(*uses_remaining, 1),
        _ => panic!("Expected CloakOfInvisibility"),
    }

    assert!(kargunt.use_cloak_charge());
    match &kargunt.inventory[0] {
        Item::CloakOfInvisibility { uses_remaining } => assert_eq!(*uses_remaining, 0),
        _ => panic!("Expected CloakOfInvisibility"),
    }

    // Now has 0 uses, cannot use again
    assert!(!kargunt.has_cloak());
    assert!(!kargunt.use_cloak_charge());
}

#[test]
fn test_fleeing_damage() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Flee deals 1d4 damage (roll 3)
    let mut flee_mock = MockRoller::new(vec![3], vec![]);
    let outcome = CombatEngine::flee(&mut kargunt, &mut flee_mock);

    match outcome {
        RoundOutcome::Fled { damage_taken, player_hp_left } => {
            assert_eq!(damage_taken, 3);
            assert_eq!(player_hp_left, 12);
        }
        _ => panic!("Expected Fled"),
    }
}

#[test]
fn test_necro_sorcerer_alternating_death_ray() {
    let mut necro = Monster {
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

    let mut mock = MockRoller::new(vec![3], vec![5]);
    // 1st attack: normal d4 (roll 3)
    let (dmg1, is_death_ray1) = necro.roll_attack_damage(&mut mock);
    assert_eq!(dmg1, 3);
    assert!(!is_death_ray1);

    // 2nd attack: death ray d6 (roll 5)
    let (dmg2, is_death_ray2) = necro.roll_attack_damage(&mut mock);
    assert_eq!(dmg2, 5);
    assert!(is_death_ray2);
}

#[test]
fn test_necro_sorcerer_maggot_hazard_triggers() {
    let necro = Monster {
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

    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Loot 3d6 silver: rolls 3, 4, 5 = 12 silver
    // Maggot check: 1-in-6 -> roll 1 triggers hazard!
    let mut vic_mock = MockRoller::new(vec![], vec![3, 4, 5, 1]);
    let victory = CombatEngine::resolve_victory(&mut kargunt, &necro, &mut vic_mock);

    assert_eq!(victory.looted_silver, 12);
    assert!(victory.adventure_ended);
    assert!(victory.hazard_triggered.unwrap().contains("maggot"));
}

#[test]
fn test_medusa_petrification_and_loot() {
    let medusa = Monster {
        name: "Medusa".to_string(),
        category: MonsterCategory::Tough,
        points_to_hit: 4,
        points_awarded: 4,
        hp: 10,
        max_hp: 10,
        damage_formula: DiceFormula::D6,
        attack_count: 0,
        loot: MonsterLoot::MedusaSilver,
        post_hazard: PostFightHazard::Petrification,
    };

    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Silver loot: d4 * d6 (d4 roll 3, d6 roll 4 = 12 silver)
    // Post hazard check: 1-in-6 -> roll 6 does NOT trigger petrification
    let mut vic_mock = MockRoller::new(vec![3], vec![4, 6]);
    let victory = CombatEngine::resolve_victory(&mut kargunt, &medusa, &mut vic_mock);

    assert_eq!(victory.looted_silver, 12);
    assert!(!victory.adventure_ended);
    assert!(victory.hazard_triggered.is_none());
}

#[test]
fn test_ruin_basilisk_immediate_level_up() {
    let basilisk = Monster {
        name: "Ruin Basilisk".to_string(),
        category: MonsterCategory::Tough,
        points_to_hit: 4,
        points_awarded: 4,
        hp: 11,
        max_hp: 11,
        damage_formula: DiceFormula::D6,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::ImmediateLevelUp,
    };

    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Level up check: 2-in-6 -> roll 2 triggers instant level up!
    let mut vic_mock = MockRoller::new(vec![], vec![2]);
    let victory = CombatEngine::resolve_victory(&mut kargunt, &basilisk, &mut vic_mock);

    assert!(victory.instant_level_up);
}

#[test]
fn test_halved_monster_damage_level6_perk() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]); // unarmed/warhammer, armor
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.has_armor = false; // test pure damage halving
    kargunt.halved_weak_monster = Some(WeakMonsterType::BloodDrenchedSkeleton);

    let mut skeleton = Monster {
        name: "Blood-drenched Skeleton".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 3,
        points_awarded: 3,
        hp: 6,
        max_hp: 6,
        damage_formula: DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    // Attack roll: roll 1 (miss)
    // Skeleton deals d4 roll 4 -> halved to (4+1)/2 = 2 damage!
    // Player hp drops from 15 to 13
    let mut combat_mock = MockRoller::new(vec![4], vec![1]);
    let outcome = CombatEngine::resolve_attack_round(&mut kargunt, &mut skeleton, &mut combat_mock);

    match outcome {
        RoundOutcome::PlayerMissMonsterHit {
            halved,
            net_player_damage,
            player_hp_left,
            ..
        } => {
            assert!(halved);
            assert_eq!(net_player_damage, 2);
            assert_eq!(player_hp_left, 13);
        }
        _ => panic!("Expected PlayerMissMonsterHit"),
    }
}

#[test]
fn test_progression_level_up_scratch_off_and_grand_victory() {
    let mut manager = ProgressionManager::new();
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Charity condition
    kargunt.silver = 45;
    assert!(ProgressionManager::can_level_up_via_charity(&kargunt));

    // Scratch off all 6 rewards in sequence
    let mut rewards_mock = MockRoller::new(vec![], vec![1, 2, 3, 4, 5, 6]);
    for i in 1..=6 {
        assert!(!manager.is_grand_victory());
        let reward = manager.roll_new_reward(&mut rewards_mock).unwrap();
        manager.apply_reward(reward, &mut kargunt, Some(WeakMonsterType::Goblin), Some(ToughMonsterType::Medusa));
        assert_eq!(kargunt.level, i);
    }

    assert!(manager.is_grand_victory());
    assert!(manager.roll_new_reward(&mut rewards_mock).is_none());
}
