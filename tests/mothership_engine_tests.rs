use dark_fort::core::character::{Character, CharacterVitals};
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::config::{EngineType, GameConfig};
use dark_fort::core::dice::MockRoller;
use dark_fort::core::equipment::WeaponType;
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_engine_type_parsing() {
    assert_eq!(EngineType::parse_str("classic"), Some(EngineType::Classic));
    assert_eq!(EngineType::parse_str("mothership"), Some(EngineType::Mothership));
    assert_eq!(EngineType::parse_str("mork-borg"), Some(EngineType::MorkBorg));
    assert_eq!(EngineType::parse_str("durf"), Some(EngineType::Durf));
    assert_eq!(EngineType::parse_str("unknown"), None);
}

#[test]
fn test_game_config_loading() {
    let config = GameConfig::load_or_default();
    // Default config.json now defaults to DURF
    assert_eq!(config.engine, EngineType::Durf);
}

#[test]
fn test_mothership_character_creation() {
    let mut roller = MockRoller::new(vec![2], vec![4]);
    let character = Character::new_with_engine(EngineType::Mothership, &mut roller);

    assert_eq!(character.engine_type, EngineType::Mothership);
    assert_eq!(character.hp, 14);
    assert_eq!(character.max_hp, 14);

    if let CharacterVitals::Mothership {
        health,
        max_health,
        wounds,
        max_wounds,
        stress,
        combat,
        strength,
        agility,
        occult,
        ..
    } = character.vitals
    {
        assert_eq!(health, 14);
        assert_eq!(max_health, 14);
        assert_eq!(wounds, 0);
        assert_eq!(max_wounds, 2);
        assert_eq!(stress, 2);
        assert_eq!(combat, 40);
        assert_eq!(strength, 35);
        assert_eq!(agility, 45);
        assert_eq!(occult, 25);
    } else {
        panic!("Expected Mothership vitals!");
    }
}

#[test]
fn test_mothership_critical_hit_and_stress_relief() {
    let mut roller = MockRoller::new(vec![2], vec![4])
        .with_d100(vec![22]); // 22 <= 40 (matching double digits = Critical Success!)

    let mut character = Character::new_with_engine(EngineType::Mothership, &mut roller);
    character.weapon = WeaponType::Warhammer; // 1d6 damage formula -> crit deals 6 + 2 = 8

    let mut monster = Monster {
        name: "Citadel Skeleton".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 10,
        max_hp: 10,
        damage_formula: dark_fort::core::dice::DiceFormula::D6,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::MothershipHit {
            roll,
            target,
            is_crit,
            damage,
            monster_hp_left,
            stress_delta,
            ..
        } => {
            assert_eq!(roll, 22);
            assert_eq!(target, 40);
            assert!(is_crit);
            assert_eq!(damage, 8); // Max Warhammer 6 + 2 = 8
            assert_eq!(monster_hp_left, 2);
            assert_eq!(stress_delta, -1);
        }
        _ => panic!("Expected MothershipHit!"),
    }
}

#[test]
fn test_mothership_crit_failure_and_wound_taken() {
    let mut roller = MockRoller::new(vec![3], vec![6, 6])
        .with_d100(vec![77]); // 77 > 40 (matching double digits = Critical Failure!)

    let mut character = Character::new_with_engine(EngineType::Mothership, &mut roller);
    character.has_armor = true; // AP 3 flat soak
    character.hp = 2; // Low health to test wound trigger

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
        RoundOutcome::MothershipMissMonsterHit {
            roll,
            is_crit_fail,
            raw_monster_damage,
            ap_absorbed,
            net_player_damage,
            player_wounds,
            wound_taken,
            ..
        } => {
            assert_eq!(roll, 77);
            assert!(is_crit_fail);
            assert_eq!(raw_monster_damage, 6);
            assert_eq!(ap_absorbed, 3); // AP 3 absorbed
            assert_eq!(net_player_damage, 3); // 6 - 3 = 3
            assert!(wound_taken); // Dropped below 0 -> Wound taken!
            assert_eq!(player_wounds, 1);
            assert_eq!(character.hp, 14); // Health reset to max_health
            assert!(character.is_alive());
        }
        _ => panic!("Expected MothershipMissMonsterHit!"),
    }
}

#[test]
fn test_mothership_second_wound_is_lethal() {
    let mut roller = MockRoller::new(vec![1], vec![6, 6, 6, 6, 6])
        .with_d100(vec![85]); // Regular miss

    let mut character = Character::new_with_engine(EngineType::Mothership, &mut roller);
    character.has_armor = false;
    // Set character to 1 wound already
    if let CharacterVitals::Mothership { wounds, .. } = &mut character.vitals {
        *wounds = 1;
    }
    character.hp = 4;

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
        RoundOutcome::MothershipMissMonsterHit {
            player_wounds,
            wound_taken,
            ..
        } => {
            assert!(wound_taken);
            assert_eq!(player_wounds, 2); // 2nd wound taken!
            assert!(!character.is_alive()); // Dead!
            assert_eq!(character.hp, 0);
        }
        _ => panic!("Expected MothershipMissMonsterHit!"),
    }
}

#[test]
fn test_mothership_save_and_load_roundtrip() {
    let path = "target/test_mothership_save.json";
    let _ = StorageManager::delete(path);

    let mut roller = MockRoller::new(vec![2], vec![3]);
    let character = Character::new_with_engine(EngineType::Mothership, &mut roller);
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

    assert_eq!(loaded.character.engine_type, EngineType::Mothership);
    assert_eq!(loaded.character.hp, 14);
    assert_eq!(loaded.character.name, character.name);

    let _ = StorageManager::delete(path);
}
