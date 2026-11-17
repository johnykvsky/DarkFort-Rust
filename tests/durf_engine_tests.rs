use dark_fort::core::character::{Character, CharacterVitals};
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::config::EngineType;
use dark_fort::core::dice::MockRoller;
use dark_fort::core::equipment::WeaponType;
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_durf_character_creation() {
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2]);
    let character = Character::new_with_engine(EngineType::Durf, &mut roller);

    assert_eq!(character.engine_type, EngineType::Durf);
    assert!(character.is_alive());

    if let CharacterVitals::Durf {
        armor_pool,
        max_armor,
        wounds,
        hit_dice,
        force,
        finesse,
        will,
        shield_soak,
        is_dead,
    } = character.vitals
    {
        assert!(armor_pool >= 1);
        assert_eq!(max_armor, armor_pool);
        assert_eq!(wounds, 0);
        assert_eq!(hit_dice, 1);
        assert_eq!(force, 2);
        assert_eq!(finesse, 1);
        assert_eq!(will, 0);
        assert_eq!(shield_soak, 1);
        assert!(!is_dead);
    } else {
        panic!("Expected DURF vitals!");
    }
}

#[test]
fn test_durf_successful_attack() {
    // Action roll: d20 + Force (2) > 15
    // d20 roll of 14 + 2 = 16 > 15 -> HIT!
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2])
        .with_d20(vec![14]);

    let mut character = Character::new_with_engine(EngineType::Durf, &mut roller);
    character.weapon = WeaponType::Warhammer; // 1d6 damage

    let mut monster = Monster {
        name: "Catacomb Cultist".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 6,
        max_hp: 6,
        damage_formula: dark_fort::core::dice::DiceFormula::D6,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::DurfHit {
            roll,
            target,
            damage,
            monster_hp_left,
        } => {
            assert_eq!(roll, 16);
            assert_eq!(target, 15);
            assert!(damage >= 1);
            assert_eq!(monster_hp_left, 6 - damage);
        }
        _ => panic!("Expected DurfHit outcome!"),
    }
}

#[test]
fn test_durf_armor_soaks_and_avoids_wounds() {
    // Attack rolls: 8 + 2 = 10 <= 15 -> MISS!
    // Monster strikes back for 3 damage.
    // Shield soaks 1 -> 2 damage.
    // Armor pool (3) soaks 2 -> 1 armor left, 0 wounds!
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2, 3])
        .with_d20(vec![8]);

    let mut character = Character::new_with_engine(EngineType::Durf, &mut roller);
    character.has_armor = true; // Armor pool 3
    if let CharacterVitals::Durf { armor_pool, max_armor, .. } = &mut character.vitals {
        *armor_pool = 3;
        *max_armor = 3;
    }

    let mut monster = Monster {
        name: "Blood-drenched Skeleton".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 5,
        max_hp: 5,
        damage_formula: dark_fort::core::dice::DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::DurfMissMonsterHit {
            shield_absorbed,
            armor_absorbed,
            wounds_total,
            player_died,
            ..
        } => {
            assert_eq!(shield_absorbed, 1);
            assert!(armor_absorbed >= 1);
            assert_eq!(wounds_total, 0);
            assert!(!player_died);
            assert!(character.is_alive());
        }
        _ => panic!("Expected DurfMissMonsterHit!"),
    }
}

#[test]
fn test_durf_mortality_check_lethal() {
    // 1st d4 (1) = weapon, 2nd d4 (2) = potion item, 3rd d4 (4) = monster damage!
    // 1st d6 (6) = silver, 2nd d6 (1) = survival HD check (rolls 1 <= 3 wounds -> death!)
    let mut roller = MockRoller::new(vec![1, 2, 4], vec![6, 1])
        .with_d20(vec![5]);

    let mut character = Character::new_with_engine(EngineType::Durf, &mut roller);
    if let CharacterVitals::Durf { armor_pool, .. } = &mut character.vitals {
        *armor_pool = 0;
    }

    let mut monster = Monster {
        name: "Small Stone Troll".to_string(),
        category: MonsterCategory::Tough,
        points_to_hit: 8,
        points_awarded: 2,
        hp: 12,
        max_hp: 12,
        damage_formula: dark_fort::core::dice::DiceFormula::D4,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    let outcome = CombatEngine::resolve_attack_round(&mut character, &mut monster, &mut roller);

    match outcome {
        RoundOutcome::DurfMissMonsterHit {
            wounds_total,
            player_died,
            ..
        } => {
            assert!(wounds_total >= 1);
            assert!(player_died);
            assert!(!character.is_alive());
        }
        _ => panic!("Expected DurfMissMonsterHit!"),
    }
}

#[test]
fn test_durf_save_and_load_roundtrip() {
    let path = "target/test_durf_save.json";
    let _ = StorageManager::delete(path);

    let mut roller = MockRoller::new(vec![2], vec![3, 2, 1]);
    let character = Character::new_with_engine(EngineType::Durf, &mut roller);
    let dungeon = dark_fort::dungeon::map::DungeonGraph::new(&mut roller);
    let progression = dark_fort::core::progression::ProgressionManager::new();

    let save_data = SaveData {
        character: character.clone(),
        dungeon,
        progression,
        theme_mode: ThemeMode::ClassicWhite,
        is_scifi: false,
    };

    StorageManager::save(&save_data, path).expect("Save should succeed");
    let loaded = StorageManager::load(path).expect("Load should succeed");

    assert_eq!(loaded.character.engine_type, EngineType::Durf);
    assert_eq!(loaded.character.name, character.name);

    let _ = StorageManager::delete(path);
}
