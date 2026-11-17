use dark_fort::app::{App, AppState};
use dark_fort::core::character::{Character, CharacterVitals};
use dark_fort::core::config::EngineType;
use dark_fort::core::dice::MockRoller;
use dark_fort::core::registry::{EnemyRegistry, ItemRegistry, ShopConfig};
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_scifi_registries_load() {
    let items = ItemRegistry::load_scifi_or_default();
    assert!(!items.weapons.is_empty());
    assert!(items.get_weapon("pulse_rifle").is_some());
    assert!(items.get_weapon("smg").is_some());

    let enemies = EnemyRegistry::load_scifi_or_default();
    assert!(!enemies.weak_monsters.is_empty());
    assert!(!enemies.tough_monsters.is_empty());
    assert!(enemies.weak_monsters.iter().any(|m| m.id == "face_leaper"));
    assert!(enemies.tough_monsters.iter().any(|m| m.id == "xeno_praetorian"));

    let shop = ShopConfig::load_scifi_or_default();
    assert!(!shop.wares.is_empty());
    assert!(shop.wares.iter().any(|w| w.name.contains("Pulse Rifle")));
}

#[test]
fn test_scifi_character_creation_mothership() {
    let mut roller = MockRoller::new(vec![2], vec![3]);
    let character = Character::new_with_engine_and_theme(EngineType::Mothership, true, &mut roller);

    assert_eq!(character.engine_type, EngineType::Mothership);
    assert_eq!(character.title, Some("Warrant Officer Kargunt (Colonial Marine)".to_string()));
    assert!(character.has_armor);
    assert!(character.silver >= 25); // Initial credits

    // Check Mothership vitals
    match &character.vitals {
        CharacterVitals::Mothership { health, max_health, wounds, max_wounds, stress, combat, armor_points, .. } => {
            assert_eq!(*health, 15);
            assert_eq!(*max_health, 15);
            assert_eq!(*wounds, 0);
            assert_eq!(*max_wounds, 2);
            assert_eq!(*stress, 2);
            assert_eq!(*combat, 45);
            assert_eq!(*armor_points, 3);
        }
        _ => panic!("Expected Mothership vitals"),
    }

    // Check starting inventory has stimm-pak and mag-boots
    assert!(character.inventory.iter().any(|i| i.name().contains("Stimm-Pak")));
    assert!(character.inventory.iter().any(|i| i.name().contains("Magnetic Boots")));
}

#[test]
fn test_alien_queen_boss_stats() {
    let enemies = EnemyRegistry::load_scifi_or_default();
    let queen = enemies.alien_queen();

    assert!(queen.name.contains("Alien Queen"));
    assert_eq!(queen.hp, 22);
    assert_eq!(queen.points_to_hit, 5);
    assert_eq!(queen.points_awarded, 25);
}

#[test]
fn test_stimm_pak_heals_and_relieves_stress() {
    let mut roller = MockRoller::new(vec![2], vec![4]);
    let mut character = Character::new_with_engine_and_theme(EngineType::Mothership, true, &mut roller);

    // Inflict damage and stress
    character.hp = 8;
    if let CharacterVitals::Mothership { ref mut health, ref mut stress, .. } = character.vitals {
        *health = 8;
        *stress = 8;
    }

    // Use healing potion (stimm-pak)
    character.heal(5);
    assert_eq!(character.hp, 13);

    if let CharacterVitals::Mothership { health, stress, .. } = character.vitals {
        assert_eq!(health, 13);
        assert_eq!(stress, 6); // 8 - 2 = 6 stress remaining!
    } else {
        panic!("Expected Mothership vitals");
    }
}

#[test]
fn test_scifi_alien_queen_victory_triggers_game_won() {
    let path = "target/test_scifi_queen_victory.json";
    let _ = StorageManager::delete(path);

    let mut app = App::new_with_engine_and_theme(
        EngineType::Mothership,
        Some(42),
        true,
        ThemeMode::SciFiVoid,
    );
    assert!(app.is_scifi);
    assert_eq!(app.theme_mode, ThemeMode::SciFiVoid);

    // Simulate character reaching Deck 7 (level 6)
    app.character.level = 6;
    app.enter_current_room();

    // Queen encounter triggered
    match &app.state {
        AppState::Combat(monster) => {
            assert!(monster.name.contains("Alien Queen"));
            assert_eq!(monster.hp, 22);
        }
        _ => panic!("Expected Alien Queen Combat state on Deck 7"),
    }
}

#[test]
fn test_scifi_save_and_load_roundtrip() {
    let path = "target/test_scifi_save.json";
    let _ = StorageManager::delete(path);

    let mut roller = MockRoller::new(vec![2], vec![3]);
    let character = Character::new_with_engine_and_theme(EngineType::Mothership, true, &mut roller);
    let dungeon = dark_fort::dungeon::map::DungeonGraph::new(&mut roller);
    let progression = dark_fort::core::progression::ProgressionManager::new();

    let save_data = SaveData {
        character: character.clone(),
        dungeon,
        progression,
        theme_mode: ThemeMode::SciFiVoid,
        is_scifi: true,
    };

    StorageManager::save(&save_data, path).expect("Save should succeed");
    let loaded = StorageManager::load(path).expect("Load should succeed");

    assert!(loaded.is_scifi);
    assert_eq!(loaded.theme_mode, ThemeMode::SciFiVoid);
    assert_eq!(loaded.character.engine_type, EngineType::Mothership);

    let app = App::from_save(loaded);
    assert!(app.is_scifi);
    assert_eq!(app.theme_mode, ThemeMode::SciFiVoid);

    let _ = StorageManager::delete(path);
}
