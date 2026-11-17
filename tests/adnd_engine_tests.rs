use dark_fort::core::character::{Character, CharacterVitals};
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::config::EngineType;
use dark_fort::core::dice::MockRoller;
use dark_fort::core::equipment::WeaponType;
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_adnd_character_creation() {
    let mut roller = MockRoller::new(vec![2], vec![4, 3, 2]);
    let character = Character::new_with_engine(EngineType::Adnd2e, &mut roller);

    assert_eq!(character.engine_type, EngineType::Adnd2e);
    assert_eq!(character.hp, 6);
    assert_eq!(character.max_hp, 6);

    if let CharacterVitals::Adnd2e {
        thac0,
        ac,
        hp,
        max_hp,
        strength,
        dexterity,
        constitution,
        intelligence,
        wisdom,
        charisma,
        save_death_poison,
        save_petrify,
        save_spell,
    } = character.vitals
    {
        assert_eq!(thac0, 20);
        assert!(ac <= 8); // AC 6 with leather + DEX bonus, or AC 8 without armor
        assert_eq!(hp, 6);
        assert_eq!(max_hp, 6);
        assert_eq!(strength, 14);
        assert_eq!(dexterity, 16);
        assert_eq!(constitution, 14);
        assert_eq!(intelligence, 12);
        assert_eq!(wisdom, 10);
        assert_eq!(charisma, 11);
        assert_eq!(save_death_poison, 13);
        assert_eq!(save_petrify, 12);
        assert_eq!(save_spell, 15);
    } else {
        panic!("Expected AD&D 2e vitals!");
    }
}

#[test]
fn test_adnd_thac0_attack_hit() {
    // Player has THAC0 20.
    // Monster Skeleton has AC 7. Needed roll = 20 - 7 = 13.
    // Player rolls 14 on d20 -> Effective AC hit = 20 - 14 = 6 <= 7 -> HIT!
    let mut roller = MockRoller::new(vec![2, 2, 4], vec![4, 3, 2])
        .with_d20(vec![14]);

    let mut character = Character::new_with_engine(EngineType::Adnd2e, &mut roller);
    character.weapon = WeaponType::Warhammer; // 1d6 damage, 0 bonus

    let mut monster = Monster {
        name: "Blood-drenched Skeleton".to_string(), // AC 7
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
        RoundOutcome::AdndHit {
            thac0,
            target_ac,
            roll,
            effective_ac_hit,
            is_crit,
            damage,
            monster_hp_left,
        } => {
            assert_eq!(thac0, 20);
            assert_eq!(target_ac, 7);
            assert_eq!(roll, 14);
            assert_eq!(effective_ac_hit, 6);
            assert!(!is_crit);
            assert!(damage >= 1);
            assert_eq!(monster_hp_left, 6 - damage);
        }
        _ => panic!("Expected AdndHit!"),
    }
}

#[test]
fn test_adnd_attack_miss_and_monster_misses_player() {
    // Player rolls 6 on d20 -> Effective AC hit = 20 - 6 = 14 > 7 -> MISS!
    // Skeleton counter-attacks: THAC0 19 vs Player AC 6.
    // Needed roll = 19 - 6 = 13.
    // Monster rolls 8 on d20 (< 13) -> MONSTER MISSES!
    let mut roller = MockRoller::new(vec![2, 2], vec![4, 3, 2])
        .with_d20(vec![6, 8]); // 1st d20 = player attack (6), 2nd d20 = monster counter (8)

    let mut character = Character::new_with_engine(EngineType::Adnd2e, &mut roller);
    character.has_armor = true; // AC 6

    let mut monster = Monster {
        name: "Blood-drenched Skeleton".to_string(),
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
        RoundOutcome::AdndMissMonsterHit {
            effective_ac_hit,
            monster_hit,
            damage_taken,
            player_hp_left,
            ..
        } => {
            assert!(effective_ac_hit > 7); // Player missed
            assert!(!monster_hit); // Monster also missed!
            assert_eq!(damage_taken, 0);
            assert_eq!(player_hp_left, 6);
        }
        _ => panic!("Expected AdndMissMonsterHit!"),
    }
}

#[test]
fn test_adnd_attack_miss_and_monster_hits_player() {
    // Player rolls 5 on d20 -> MISS!
    // Skeleton counter-attacks: THAC0 19 vs Player AC 6.
    // Needed roll = 13.
    // Monster rolls 15 on d20 (>= 13) -> MONSTER HITS!
    // Monster deals 3 damage (d6 roll: 3).
    let mut roller = MockRoller::new(vec![2, 2], vec![4, 3, 2, 3])
        .with_d20(vec![5, 15]);

    let mut character = Character::new_with_engine(EngineType::Adnd2e, &mut roller);

    let mut monster = Monster {
        name: "Blood-drenched Skeleton".to_string(),
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
        RoundOutcome::AdndMissMonsterHit {
            monster_hit,
            damage_taken,
            player_hp_left,
            ..
        } => {
            assert!(monster_hit);
            assert_eq!(damage_taken, 3);
            assert_eq!(player_hp_left, 6 - 3);
        }
        _ => panic!("Expected AdndMissMonsterHit!"),
    }
}

#[test]
fn test_adnd_save_and_load_roundtrip() {
    let path = "target/test_adnd_save.json";
    let _ = StorageManager::delete(path);

    let mut roller = MockRoller::new(vec![2], vec![3, 2, 1]);
    let character = Character::new_with_engine(EngineType::Adnd2e, &mut roller);
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

    assert_eq!(loaded.character.engine_type, EngineType::Adnd2e);
    assert_eq!(loaded.character.hp, 6);
    assert_eq!(loaded.character.name, character.name);

    let _ = StorageManager::delete(path);
}
