use dark_fort::core::character::Character;
use dark_fort::core::dice::MockRoller;
use dark_fort::core::progression::ProgressionManager;
use dark_fort::dungeon::map::DungeonGraph;
use dark_fort::simulation::HeadlessSimulator;
use dark_fort::storage::save_game::{SaveData, StorageManager};
use dark_fort::ui::theme::ThemeMode;

#[test]
fn test_save_and_load_roundtrip() {
    let mut mock = MockRoller::new(vec![2, 1], vec![4]);
    let character = Character::new_kargunt(&mut mock);
    let dungeon = DungeonGraph::new(&mut mock);
    let progression = ProgressionManager::new();

    let save_data = SaveData {
        character: character.clone(),
        dungeon,
        progression,
        theme_mode: ThemeMode::AmberCrt,
        is_scifi: false,
    };

    let temp_save = "target/test_savegame.json";

    // Save
    StorageManager::save(&save_data, temp_save).expect("Save should succeed");
    assert!(StorageManager::exists(temp_save));

    // Load
    let loaded = StorageManager::load(temp_save).expect("Load should succeed");
    assert_eq!(loaded.character.name, character.name);
    assert_eq!(loaded.character.hp, character.hp);
    assert_eq!(loaded.character.silver, character.silver);
    assert_eq!(loaded.theme_mode, ThemeMode::AmberCrt);

    // Delete
    StorageManager::delete(temp_save).expect("Delete should succeed");
    assert!(!StorageManager::exists(temp_save));
}

#[test]
fn test_headless_simulation_runs_cleanly() {
    // Run 50 simulated delves to ensure no panics or deadlocks
    let stats = HeadlessSimulator::run_simulations(50, 42);
    assert_eq!(stats.games_played, 50);
    assert!(stats.total_rooms_explored > 0);
    assert_eq!(stats.victories + stats.deaths, 50);
}

#[test]
fn test_manual_save_only() {
    use dark_fort::app::App;

    let temp_save = "target/test_manual_save.json";
    let _ = StorageManager::delete(temp_save);

    let app = App::new();
    // New app and room entry should NOT auto-save
    assert!(!StorageManager::exists(temp_save));

    // Calling save_game manually will save
    let saved = app.save_game_to(temp_save);
    assert!(saved);
    assert!(StorageManager::exists(temp_save));

    // Cleanup
    let _ = StorageManager::delete(temp_save);
}

#[test]
fn test_load_game_behavior() {
    use dark_fort::app::App;

    let temp_save = "target/test_load_behavior.json";
    let _ = StorageManager::delete(temp_save);

    let mut app = App::new();
    // Attempting to load without an existing save returns Err and does not corrupt state
    let load_res = app.load_game_from(temp_save);
    assert!(load_res.is_err());

    // Modify character and save
    app.character.silver = 99;
    assert!(app.save_game_to(temp_save));

    // Create a new app with default silver
    let mut app2 = App::new();
    assert_ne!(app2.character.silver, 99);

    // Loading restores saved silver
    assert!(app2.load_game_from(temp_save).is_ok());
    assert_eq!(app2.character.silver, 99);

    // Cleanup
    let _ = StorageManager::delete(temp_save);
}
