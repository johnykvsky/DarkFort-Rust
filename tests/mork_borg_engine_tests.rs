use dark_fort::core::character::{Character, CharacterVitals};
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::config::EngineType;
use dark_fort::core::dice::MockRoller;
use dark_fort::core::equipment::WeaponType;
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_mork_borg_character_creation() {
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2]);
    let character = Character::new_with_engine(EngineType::MorkBorg, &mut roller);

    assert_eq!(character.engine_type, EngineType::MorkBorg);
    assert_eq!(character.hp, 8);
    assert_eq!(character.max_hp, 8);

    if let CharacterVitals::MorkBorg {
        hp,
        max_hp,
        omens,
        strength,
        agility,
        presence,
        toughness,
        ..
    } = character.vitals
    {
        assert_eq!(hp, 8);
        assert_eq!(max_hp, 8);
        assert_eq!(omens, 2);
        assert_eq!(strength, -1);
        assert_eq!(agility, 2);
        assert_eq!(presence, 0);
        assert_eq!(toughness, 1);
    } else {
        panic!("Expected MÖRK BORG vitals!");
    }
}

#[test]
fn test_mork_borg_attack_hit_dr12() {
    // Player uses Dagger (Agility +2)
    // Attack d20 rolls 11 -> 11 + 2 = 13 >= 12 -> HIT!
    let mut roller = MockRoller::new(vec![3], vec![4, 3, 2, 4])
        .with_d20(vec![11]);

    let mut character = Character::new_with_engine(EngineType::MorkBorg, &mut roller);
    character.weapon = WeaponType::Dagger; // Agility finesse weapon

    let mut monster = Monster {
        name: "Catacomb Cultist".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 6,
        max_hp: 6,
        damage_formula: dark_fort::core::dice::DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::MorkBorgHit {
            attack_roll,
            is_crit,
            damage,
            monster_hp_left,
            ..
        } => {
            assert!(attack_roll >= 12);
            assert!(!is_crit);
            assert!(damage >= 1);
            assert_eq!(monster_hp_left, 6 - damage);
        }
        _ => panic!("Expected MorkBorgHit outcome!"),
    }
}

#[test]
fn test_mork_borg_active_defense_success() {
    // Attack d20 rolls 8 -> 8 + 2 (Dagger) = 10 < 12 -> MISS!
    // Active Defense d20 rolls 12 -> 12 + 2 (Agility) = 14 >= 12 -> EVADED!
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2])
        .with_d20(vec![8, 12]);

    let mut character = Character::new_with_engine(EngineType::MorkBorg, &mut roller);
    character.weapon = WeaponType::Dagger;

    let mut monster = Monster {
        name: "Blood-drenched Skeleton".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 5,
        max_hp: 5,
        damage_formula: dark_fort::core::dice::DiceFormula::D6,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::MorkBorgMissMonsterHit {
            defense_roll,
            is_defense_success,
            net_player_damage,
            ..
        } => {
            assert!(defense_roll >= 12);
            assert!(is_defense_success);
            assert_eq!(net_player_damage, 0); // No damage taken on successful defense!
            assert_eq!(character.hp, 8);
        }
        _ => panic!("Expected MorkBorgMissMonsterHit!"),
    }
}

#[test]
fn test_mork_borg_active_defense_failure_and_armor_soak() {
    // Attack d20 rolls 7 -> 7 + 2 = 9 < 12 -> MISS!
    // Defense d20 rolls 6 -> 6 + 2 = 8 < 12 -> DEFENSE FAILED!
    // Monster deals 4 damage (d4 roll: 4).
    // Character has Tier 1 armor -> absorbs 1d2 damage.
    // 1st d4 (1) = starting weapon, 2nd d4 (2) = starting item
    // 3rd d4 (4) = monster damage
    let mut roller = MockRoller::new(vec![1, 2, 4], vec![4, 3, 2])
        .with_d20(vec![7, 6]);

    let mut character = Character::new_with_engine(EngineType::MorkBorg, &mut roller);
    character.has_armor = true;

    let mut monster = Monster {
        name: "Goblin".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 4,
        points_awarded: 1,
        hp: 4,
        max_hp: 4,
        damage_formula: dark_fort::core::dice::DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::MorkBorgMissMonsterHit {
            defense_roll,
            is_defense_success,
            raw_monster_damage,
            armor_absorbed,
            net_player_damage,
            player_hp_left,
            ..
        } => {
            assert!(defense_roll < 12);
            assert!(!is_defense_success);
            assert_eq!(raw_monster_damage, 4);
            assert!(armor_absorbed >= 1);
            assert_eq!(net_player_damage, 4 - armor_absorbed);
            assert_eq!(player_hp_left, 8 - net_player_damage);
        }
        _ => panic!("Expected MorkBorgMissMonsterHit!"),
    }
}

#[test]
fn test_mork_borg_save_and_load_roundtrip() {
    let path = "target/test_mork_borg_save.json";
    let _ = StorageManager::delete(path);

    let mut roller = MockRoller::new(vec![2], vec![3, 2, 1]);
    let character = Character::new_with_engine(EngineType::MorkBorg, &mut roller);
    let dungeon = dark_fort::dungeon::map::DungeonGraph::new(&mut roller);
    let progression = dark_fort::core::progression::ProgressionManager::new();

    let save_data = SaveData {
        character: character.clone(),
        dungeon,
        progression,
        theme_mode: ThemeMode::MorkBorg,
        is_scifi: false,
    };

    StorageManager::save(&save_data, path).expect("Save should succeed");
    let loaded = StorageManager::load(path).expect("Load should succeed");

    assert_eq!(loaded.character.engine_type, EngineType::MorkBorg);
    assert_eq!(loaded.character.hp, 8);
    assert_eq!(loaded.character.name, character.name);

    let _ = StorageManager::delete(path);
}
