use crate::core::character::Character;
use crate::core::combat::{CombatEngine, RoundOutcome};
use crate::core::config::EngineType;
use crate::core::dice::{DiceRoller, StandardRoller};
use crate::core::equipment::Item;
use crate::core::monsters::{Monster, ToughMonsterType, WeakMonsterType};
use crate::core::progression::{LevelUpReward, ProgressionManager};
use crate::core::registry::{EnemyRegistry, ItemRegistry, ShopConfig};
use crate::dungeon::map::{DungeonGraph, NavigationEvent};
use crate::dungeon::room::{Direction, RoomContent, RoomStatus};
use crate::ui::renderer::UIRenderer;
use crate::ui::theme::{Theme, ThemeMode};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::thread_rng;
use ratatui::Frame;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Exploring,
    Combat(Monster),
    PitTrapPending,
    SoothsayerPrompt,
    PeddlerShop,
    LevelUpPending,
    QuitConfirmation(Box<AppState>),
    GameOver,
    Victory,
}

pub struct App {
    pub character: Character,
    pub dungeon: DungeonGraph,
    pub progression: ProgressionManager,
    pub registry: ItemRegistry,
    pub enemy_registry: EnemyRegistry,
    pub shop: ShopConfig,
    pub state: AppState,
    pub theme_mode: ThemeMode,
    pub theme: Theme,
    pub is_scifi: bool,
    pub messages: Vec<String>,
    pub should_quit: bool,
    roller: StandardRoller<rand::rngs::ThreadRng>,
}

use crate::storage::save_game::{SaveData, StorageManager, DEFAULT_SAVE_PATH};

impl App {
    pub fn new() -> Self {
        Self::new_with_engine(EngineType::Classic, None)
    }

    pub fn new_with_seed(seed: Option<u64>) -> Self {
        Self::new_with_engine(EngineType::Classic, seed)
    }

    pub fn new_with_engine(engine_type: EngineType, seed: Option<u64>) -> Self {
        Self::new_with_engine_and_theme(engine_type, seed, false, ThemeMode::ClassicWhite)
    }

    pub fn new_with_engine_and_theme(
        engine_type: EngineType,
        seed: Option<u64>,
        is_scifi: bool,
        theme_mode: ThemeMode,
    ) -> Self {
        let mut roller = StandardRoller::new(thread_rng());
        let (registry, enemy_registry, shop, active_theme_mode) = if is_scifi {
            (
                ItemRegistry::load_scifi_or_default(),
                EnemyRegistry::load_scifi_or_default(),
                ShopConfig::load_scifi_or_default(),
                ThemeMode::SciFiVoid,
            )
        } else {
            (
                ItemRegistry::load_or_default(),
                EnemyRegistry::load_or_default(),
                ShopConfig::load_or_default(),
                theme_mode,
            )
        };

        let character = Character::new_with_engine_and_theme(engine_type, is_scifi, &mut roller);
        let dungeon = DungeonGraph::new(&mut roller);
        let theme = Theme::for_mode(active_theme_mode);

        let mut app = Self {
            character,
            dungeon,
            progression: ProgressionManager::new(),
            registry,
            enemy_registry,
            shop,
            state: AppState::Exploring,
            theme_mode: active_theme_mode,
            theme,
            is_scifi,
            messages: Vec::new(),
            should_quit: false,
            roller,
        };

        let seed_note = if let Some(s) = seed { format!(" (Seed: {})", s) } else { "".to_string() };
        if is_scifi {
            app.messages.push(format!("WARNING: USCSS ATROPOS DOCKING COMPLETE [{}]{}...", engine_type.name(), seed_note));
            app.messages.push("LIFE SUPPORT CRITICAL. DESCEND TO DECK 7 AND ELIMINATE THE ALIEN QUEEN.".to_string());
        } else {
            app.messages.push(format!("Welcome to the DARK FORT [{}], Kargunt{}...", engine_type.name(), seed_note));
            app.messages.push("Draw your weapon. Death lurks behind every splintered door.".to_string());
        }
        app.enter_current_room();
        app
    }

    pub fn from_save(data: SaveData) -> Self {
        let roller = StandardRoller::new(thread_rng());
        let is_scifi = data.is_scifi || data.theme_mode == ThemeMode::SciFiVoid;
        let (registry, enemy_registry, shop) = if is_scifi {
            (
                ItemRegistry::load_scifi_or_default(),
                EnemyRegistry::load_scifi_or_default(),
                ShopConfig::load_scifi_or_default(),
            )
        } else {
            (
                ItemRegistry::load_or_default(),
                EnemyRegistry::load_or_default(),
                ShopConfig::load_or_default(),
            )
        };
        let theme = Theme::for_mode(data.theme_mode);
        let mut app = Self {
            character: data.character,
            dungeon: data.dungeon,
            progression: data.progression,
            registry,
            enemy_registry,
            shop,
            state: AppState::Exploring,
            theme_mode: data.theme_mode,
            theme,
            is_scifi,
            messages: Vec::new(),
            should_quit: false,
            roller,
        };
        if is_scifi {
            app.messages.push("Telemetry restored: Resuming sweep of USCSS ATROPOS...".to_string());
        } else {
            app.messages.push("Resuming your perilous delve into the DARK FORT...".to_string());
        }
        if app.dungeon.current_room().status != RoomStatus::Explored {
            app.enter_current_room();
        }
        app
    }

    pub fn save_game(&self) -> bool {
        self.save_game_to(DEFAULT_SAVE_PATH)
    }

    pub fn save_game_to(&self, path: impl AsRef<std::path::Path>) -> bool {
        if self.state != AppState::GameOver && self.state != AppState::Victory {
            let data = SaveData {
                character: self.character.clone(),
                dungeon: self.dungeon.clone(),
                progression: self.progression.clone(),
                theme_mode: self.theme_mode,
                is_scifi: self.is_scifi,
            };
            StorageManager::save(&data, path).is_ok()
        } else {
            false
        }
    }

    pub fn load_game(&mut self) -> Result<(), &'static str> {
        self.load_game_from(DEFAULT_SAVE_PATH)
    }

    pub fn load_game_from(&mut self, path: impl AsRef<std::path::Path>) -> Result<(), &'static str> {
        let path = path.as_ref();
        if !StorageManager::exists(path) {
            return Err("No save game found");
        }
        let data = StorageManager::load(path)
            .map_err(|_| "Failed to load save file")?;

        self.character = data.character;
        self.dungeon = data.dungeon;
        self.progression = data.progression;
        self.theme_mode = data.theme_mode;
        self.theme = Theme::for_mode(data.theme_mode);
        self.messages.push("Delve save loaded successfully.".to_string());
        if self.dungeon.current_room().status != RoomStatus::Explored {
            self.enter_current_room();
        } else {
            self.state = AppState::Exploring;
        }
        Ok(())
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.theme = Theme::for_mode(mode);
    }

    /// Handle room contents upon entering
    pub fn enter_current_room(&mut self) {
        if self.is_scifi && self.character.level >= 6 {
            let queen = self.enemy_registry.alien_queen();
            self.messages.push("CRITICAL ALERT: DECK 7 HIVE CORE DETECTED!".to_string());
            self.messages.push("THE ALIEN QUEEN (BROODMOTHER) TOWERS OVER HER CLUTCH OF HATCHING EGGS!".to_string());
            self.state = AppState::Combat(queen);
            return;
        }

        let content = self.dungeon.current_room().content.clone();
        match content {
            RoomContent::WeakMonsterEncounter(monster) => {
                let monster = if self.is_scifi {
                    self.enemy_registry.roll_weak(&mut self.roller)
                } else {
                    monster
                };
                if self.is_scifi {
                    self.messages.push(format!("MOTION DETECTED: An alien {} drops from an overhead vent!", monster.name));
                } else {
                    self.messages.push(format!("A {} stands guard! Prepare for battle!", monster.name));
                }
                self.state = AppState::Combat(monster);
            }
            RoomContent::ToughMonsterEncounter(monster) => {
                let monster = if self.is_scifi {
                    self.enemy_registry.roll_tough(&mut self.roller)
                } else {
                    monster
                };
                if self.is_scifi {
                    self.messages.push(format!("PROXIMITY ALARM! A lethal {} stalks into the corridor!", monster.name));
                } else {
                    self.messages.push(format!("Terror strikes! You face a formidable {}!", monster.name));
                }
                self.state = AppState::Combat(monster);
            }
            RoomContent::PitTrap => {
                if self.is_scifi {
                    self.messages.push("WARNING: Atmospheric seal failure! A DEPRESSURIZATION RIFT tears open!".to_string());
                } else {
                    self.messages.push("A deceptive pit trap opens beneath your boots!".to_string());
                }
                self.state = AppState::PitTrapPending;
            }
            RoomContent::RiddlingSoothsayer => {
                if self.is_scifi {
                    self.messages.push("A malfunctioning holographic terminal of M.O.T.H.E.R. ship AI flickers online.".to_string());
                } else {
                    self.messages.push("A hooded soothsayer whispers dark riddles from the void.".to_string());
                }
                self.state = AppState::SoothsayerPrompt;
            }
            RoomContent::Peddler => {
                if self.is_scifi {
                    self.messages.push("An automated emergency requisition kiosk hums to life:".to_string());
                    for w in &self.shop.wares {
                        self.messages.push(format!("  [{}] {} ({}cr)", w.key, w.name, w.price));
                    }
                } else {
                    self.messages.push("A peddler from beyond the void offers steel and trinkets:".to_string());
                    for w in &self.shop.wares {
                        self.messages.push(format!("  [{}] {} ({}s)", w.key, w.name, w.price));
                    }
                }
                self.state = AppState::PeddlerShop;
            }
            RoomContent::ItemStash(item) => {
                let item = if self.is_scifi {
                    self.registry.roll_random_item(&mut self.roller)
                } else {
                    item
                };
                if self.is_scifi {
                    self.messages.push(format!("You breach an emergency locker and salvage: {}!", item.name()));
                } else {
                    self.messages.push(format!("You discover an abandoned item: {}!", item.name()));
                }
                self.character.inventory.push(item);
                self.dungeon.mark_current_explored();
                self.state = AppState::Exploring;
            }
            RoomContent::MysticScroll(scroll) => {
                let scroll = if self.is_scifi {
                    self.registry.roll_random_scroll(&mut self.roller)
                } else {
                    scroll
                };
                if self.is_scifi {
                    self.messages.push(format!("You recover an encrypted data slate: {}!", scroll.name()));
                } else {
                    self.messages.push(format!("A dying mystic imparts a scroll: {}!", scroll.name()));
                }
                self.character.inventory.push(scroll);
                self.dungeon.mark_current_explored();
                self.state = AppState::Exploring;
            }
            RoomContent::Empty => {
                if self.is_scifi {
                    self.messages.push("The compartment is silent, save for the hum of failing life support.".to_string());
                } else {
                    self.messages.push("The chamber is quiet and desolate.".to_string());
                }
                self.dungeon.mark_current_explored();
                self.state = AppState::Exploring;
            }
        }
    }

    /// Main draw method dispatched to Ratatui
    pub fn draw(&self, f: &mut Frame) {
        match self.state {
            AppState::GameOver => {
                let currency_sym = if self.is_scifi { "cr" } else { "s" };
                let score = format!(
                    "Level/Deck: {} | Rooms Explored: {} | Points Earned: {} | Currency: {}{}",
                    self.character.level, self.dungeon.explored_room_count, self.character.points, self.character.silver, currency_sym
                );
                UIRenderer::render_death(f, &self.theme, &score);
            }
            AppState::Victory => {
                let score = if self.is_scifi {
                    format!(
                        "DECK 7 PURGED! Alien Queen Slayed! Compartments Swept: {} | Final Credits: {}cr",
                        self.dungeon.explored_room_count, self.character.silver
                    )
                } else {
                    format!(
                        "Sir Kargunt achieved Level 6! Rooms Explored: {} | Final Silver: {}s",
                        self.dungeon.explored_room_count, self.character.silver
                    )
                };
                UIRenderer::render_victory(f, &self.theme, &score);
            }
            _ => {
                let underlying_state = match &self.state {
                    AppState::QuitConfirmation(prev) => prev.as_ref(),
                    other => other,
                };

                let active_monster = match underlying_state {
                    AppState::Combat(m) => Some(m),
                    _ => None,
                };

                let peddler_prompt_storage;
                let (prompt, hotkeys) = match &self.state {
                    AppState::QuitConfirmation(_) => (
                        "ABANDON DELVE? [Y] Quit | [S] Save | [L] Load | [N/Esc] Return",
                        &[
                            ("Y", "Quit (no save)"),
                            ("S", "Save & Return"),
                            ("L", "Load & Return"),
                            ("N/Esc", "Return"),
                        ][..],
                    ),
                    _ => match underlying_state {
                        AppState::Exploring => (
                            "Choose a door direction or action:",
                            &[
                                ("W/↑", "North"),
                                ("D/→", "East"),
                                ("S/↓", "South"),
                                ("A/←", "West"),
                                ("P", "Potion"),
                                ("T", "Theme"),
                                ("Q/Esc", "Quit"),
                            ][..],
                        ),
                        AppState::Combat(_) => (
                            "Combat in progress! What do you do?",
                            &[
                                ("A", "Attack"),
                                ("S", "Spell"),
                                ("P", "Potion"),
                                ("C", "Cloak"),
                                ("F", "Flee"),
                                ("Q/Esc", "Quit"),
                            ][..],
                        ),
                        AppState::PitTrapPending => (
                            if self.is_scifi {
                                "Decompression Rift: Press [Space] to activate Mag-Boots & leap!"
                            } else {
                                "Pit Trap: Press [Space] to leap and roll for agility!"
                            },
                            &[("[Space]", "Leap / Roll d6"), ("P", "Potion"), ("Q/Esc", "Quit")][..],
                        ),
                        AppState::SoothsayerPrompt => (
                            if self.is_scifi {
                                "M.O.T.H.E.R. AI Console: Press [Space] to input cryptographic override!"
                            } else {
                                "Soothsayer Riddle: Press [Space] to speak your answer!"
                            },
                            &[("[Space]", "Input Override"), ("Q/Esc", "Quit")][..],
                        ),
                        AppState::PeddlerShop => {
                            let mut parts = Vec::new();
                            let sym = if self.is_scifi { "cr" } else { "s" };
                            for w in &self.shop.wares {
                                parts.push(format!("[{}] {} ({}{})", w.key, w.name, w.price, sym));
                            }
                            parts.push("[X] Exit".to_string());
                            let shop_title = if self.is_scifi { "Requisition Kiosk" } else { "Peddler Shop" };
                            peddler_prompt_storage = format!("{}: {}", shop_title, parts.join(" | "));
                            (
                                peddler_prompt_storage.as_str(),
                                &[("1", "Item 1"), ("2", "Item 2"), ("3", "Item 3"), ("4", "Item 4"), ("X", "Leave")][..],
                            )
                        }
                        AppState::LevelUpPending => (
                            if self.is_scifi {
                                "DECK CLEARANCE PROMOTION! Press [Space] to claim ship upgrade!"
                            } else {
                                "LEVEL UP! Press [Space] to roll your destiny reward!"
                            },
                            &[("[Space]", "Roll Reward")][..],
                        ),
                        _ => ("", &[][..]),
                    },
                };

                UIRenderer::render_game(
                    f,
                    &self.dungeon,
                    &self.character,
                    active_monster,
                    &self.messages,
                    prompt,
                    hotkeys,
                    &self.theme,
                );

                if matches!(self.state, AppState::QuitConfirmation(_)) {
                    UIRenderer::render_quit_dialog(f, &self.theme);
                }
            }
        }
    }

    /// Process keyboard inputs
    pub fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    return Ok(());
                }

                // Handle screens
                let current_state = self.state.clone();
                match current_state {
                    AppState::GameOver | AppState::Victory => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                            KeyCode::Char('r') => self.restart(),
                            _ => {}
                        }
                        return Ok(());
                    }
                    AppState::QuitConfirmation(prev_state) => {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                // Quit without saving
                                self.should_quit = true;
                            }
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                // Save game and return to game
                                if self.save_game() {
                                    self.messages.push("Delve saved successfully.".to_string());
                                } else {
                                    self.messages.push("Failed to save delve.".to_string());
                                }
                                self.state = *prev_state;
                                self.messages.push("Delve resumed.".to_string());
                            }
                            KeyCode::Char('l') | KeyCode::Char('L') => {
                                // Load save and return to game (check if save exists before loading)
                                if !StorageManager::exists(DEFAULT_SAVE_PATH) {
                                    self.state = *prev_state;
                                    self.messages.push("No save game found. Delve resumed.".to_string());
                                } else {
                                    match self.load_game() {
                                        Ok(()) => {
                                            self.messages.push("Delve resumed.".to_string());
                                        }
                                        Err(err) => {
                                            self.state = *prev_state;
                                            self.messages.push(format!("{}. Delve resumed.", err));
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                // Return to game
                                self.state = *prev_state;
                                self.messages.push("Delve resumed.".to_string());
                            }
                            _ => {}
                        }
                        return Ok(());
                    }
                    _ => {
                        // In all other gameplay states, Q or Esc prompts for confirmation
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            self.state = AppState::QuitConfirmation(Box::new(current_state));
                            return Ok(());
                        }
                    }
                }
                match current_state {
                    AppState::Exploring => match key.code {
                        KeyCode::Char('w') | KeyCode::Up | KeyCode::Char('k') => {
                            self.try_move(Direction::North);
                        }
                        KeyCode::Char('s') | KeyCode::Down | KeyCode::Char('j') => {
                            self.try_move(Direction::South);
                        }
                        KeyCode::Char('d') | KeyCode::Right | KeyCode::Char('l') => {
                            self.try_move(Direction::East);
                        }
                        KeyCode::Char('a') | KeyCode::Left | KeyCode::Char('h') => {
                            self.try_move(Direction::West);
                        }
                        KeyCode::Char('p') => {
                            self.drink_potion();
                        }
                        KeyCode::Char('t') => {
                            self.cycle_theme();
                        }
                        _ => {}
                    },
                    AppState::Combat(monster) => match key.code {
                        KeyCode::Char('a') | KeyCode::Enter => {
                            self.resolve_combat_attack(monster);
                        }
                        KeyCode::Char('s') => {
                            self.cast_combat_spell(monster);
                        }
                        KeyCode::Char('p') => {
                            self.drink_potion();
                        }
                        KeyCode::Char('c') => {
                            self.use_cloak(monster);
                        }
                        KeyCode::Char('f') => {
                            self.resolve_flee();
                        }
                        _ => {}
                    },
                    AppState::PitTrapPending => {
                        if key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter {
                            self.resolve_pit_trap();
                        }
                    }
                    AppState::SoothsayerPrompt => {
                        if key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter {
                            self.resolve_soothsayer();
                        }
                    }
                    AppState::PeddlerShop => match key.code {
                        KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Esc | KeyCode::Char(' ') => {
                            if self.is_scifi {
                                self.messages.push("You log out of the requisition kiosk terminal.".to_string());
                            } else {
                                self.messages.push("You step away from the peddler's wares.".to_string());
                            }
                            self.dungeon.mark_current_explored();
                            self.check_level_up();
                        }
                        KeyCode::Char(c) => {
                            if let Some(ware) = self.shop.wares.iter().find(|w| w.key == c).cloned() {
                                let item = ware.create_item(&self.registry, &mut self.roller);
                                self.buy_peddler_item(item, ware.price);
                            }
                        }
                        _ => {}
                    },
                    AppState::LevelUpPending
                        if key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter =>
                    {
                        self.resolve_level_up();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn try_move(&mut self, dir: Direction) {
        match self.dungeon.traverse_door(dir, None, &mut self.roller) {
            Ok(NavigationEvent::EnteredNewRoom(id)) => {
                let room_label = if self.is_scifi { format!("Compartment #{}", id.0) } else { format!("Room #{}", id.0) };
                self.messages.push(format!("You cycle the bulkhead hatch and enter {}...", room_label));
                self.enter_current_room();
            }
            Ok(NavigationEvent::BacktrackedToRoom(id)) => {
                let room_label = if self.is_scifi { format!("Compartment #{}", id.0) } else { format!("Room #{}", id.0) };
                self.messages.push(format!("You re-enter {}...", room_label));
                self.state = AppState::Exploring;
            }
            Ok(NavigationEvent::WanderingMonsterAmbush(weak_monster)) => {
                let monster = if self.is_scifi {
                    self.enemy_registry.roll_weak(&mut self.roller)
                } else {
                    weak_monster
                };
                if self.is_scifi {
                    self.messages.push(format!("VENT AMBUSH! An alien {} drops from an overhead conduit!", monster.name));
                } else {
                    self.messages.push(format!("Ambush! A wandering {} lunges from the dark!", monster.name));
                }
                self.state = AppState::Combat(monster);
            }
            Ok(NavigationEvent::DeadEndNoDoorsLeft) => {
                if self.is_scifi {
                    self.messages.push("Dead end! All bulkheads on this deck are locked down. Your mission ends here.".to_string());
                } else {
                    self.messages.push("Dead end! All pathways in the dark fort have closed. Your delve ends here.".to_string());
                }
                self.state = AppState::GameOver;
            }
            Err(e) => {
                self.messages.push(format!("Cannot move: {}", e));
            }
        }
    }

    fn resolve_combat_attack(&mut self, mut monster: Monster) {
        let outcome = CombatEngine::resolve_attack_round(&mut self.character, &mut monster, &mut self.roller);

        match outcome {
            RoundOutcome::PlayerHit {
                total_attack,
                target_points,
                weapon_damage,
                daemon_damage,
                monster_hp_left,
                ..
            } => {
                let daemon_msg = if daemon_damage > 0 {
                    format!(" (Daemon assists +{} dmg!)", daemon_damage)
                } else {
                    "".to_string()
                };
                self.messages.push(format!(
                    "HIT! Attack {} vs {} points! You deal {}{} damage.",
                    total_attack, target_points, weapon_damage, daemon_msg
                ));

                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} collapses into a bloody heap!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::PlayerMissMonsterHit {
                total_attack,
                target_points,
                is_death_ray,
                net_player_damage,
                player_hp_left,
                armor_absorbed,
                ..
            } => {
                let ray_msg = if is_death_ray { " [DEATH RAY]!" } else { "" };
                let armor_msg = if armor_absorbed > 0 {
                    format!(" (Armor absorbed {} dmg)", armor_absorbed)
                } else {
                    "".to_string()
                };

                self.messages.push(format!(
                    "MISS! Attack {} vs {}. The {} strikes back{} dealing {} damage{}!",
                    total_attack, target_points, monster.name, ray_msg, net_player_damage, armor_msg
                ));

                if player_hp_left <= 0 {
                    self.messages.push("Your vision blurs as the darkness consumes you...".to_string());
                    let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
                    self.state = AppState::GameOver;
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::MothershipHit {
                roll,
                target,
                is_crit,
                damage,
                monster_hp_left,
                stress_delta,
                ..
            } => {
                let crit_msg = if is_crit { " [CRITICAL SUCCESS - DOUBLE DIGITS]!" } else { "" };
                let stress_msg = if stress_delta < 0 { " (Composure gained: -1 Stress)" } else { "" };
                self.messages.push(format!(
                    "HIT! Roll [{}] <= {}%{}! You deal {} damage!{}",
                    roll, target, crit_msg, damage, stress_msg
                ));

                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} collapses into a bloody heap!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::MothershipMissMonsterHit {
                roll,
                target,
                is_crit_fail,
                net_player_damage,
                player_hp_left: _,
                player_wounds,
                wound_taken,
                ap_absorbed,
                ..
            } => {
                let crit_msg = if is_crit_fail { " [CATASTROPHIC FUMBLE - DOUBLE DIGITS]! (+1 Stress)" } else { "" };
                let ap_msg = if ap_absorbed > 0 {
                    format!(" (AP absorbed {} dmg)", ap_absorbed)
                } else {
                    "".to_string()
                };

                self.messages.push(format!(
                    "MISS! Roll [{}] > {}%{}! The {} strikes back dealing {} damage{}!",
                    roll, target, crit_msg, monster.name, net_player_damage, ap_msg
                ));

                if wound_taken {
                    self.messages.push(format!("CRITICAL TRAUMA! You suffer Wound #{}! (Health reset to Max)", player_wounds));
                }

                if !self.character.is_alive() {
                    self.messages.push("Your broken body succumbs to mortal wounds...".to_string());
                    let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
                    self.state = AppState::GameOver;
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::DurfHit {
                roll,
                target,
                damage,
                monster_hp_left,
            } => {
                self.messages.push(format!(
                    "HIT! Action Roll [{}] > {}! You deal {} damage.",
                    roll, target, damage
                ));
                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} is destroyed by your relentless blow!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::DurfMissMonsterHit {
                roll,
                target,
                raw_monster_damage: _,
                shield_absorbed,
                armor_absorbed,
                net_damage,
                wounds_total,
                player_died,
            } => {
                let soak_msg = format!(" (Shield -{}, Armor -{} absorbed)", shield_absorbed, armor_absorbed);
                self.messages.push(format!(
                    "MISS! Roll [{}] <= {}! The {} strikes dealing {} damage{}!",
                    roll, target, monster.name, net_damage, soak_msg
                ));

                if wounds_total > 0 {
                    self.messages.push(format!("WOUNDS SUFFERED: Total {}! Rolled Survival Hit Dice...", wounds_total));
                }

                if player_died {
                    self.messages.push("MORTALITY CHECK FAILED! The wounds prove fatal...".to_string());
                    let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
                    self.state = AppState::GameOver;
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::MorkBorgHit {
                attack_roll,
                is_crit,
                is_fumble,
                damage,
                monster_hp_left,
            } => {
                let crit_msg = if is_crit { " [NAT 20 CRITICAL!]" } else if is_fumble { " [NAT 1 FUMBLE!]" } else { "" };
                self.messages.push(format!(
                    "HIT! Attack Roll [{}] vs DR 12{}! You deal {} damage.",
                    attack_roll, crit_msg, damage
                ));
                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} collapses into ash and bone!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::MorkBorgMissMonsterHit {
                attack_roll,
                defense_roll,
                is_defense_success,
                net_player_damage,
                armor_absorbed,
                player_hp_left,
                ..
            } => {
                if is_defense_success {
                    self.messages.push(format!(
                        "Attack [{}] missed, but Defense [{}] >= DR 12: You parry the {}'s strike cleanly!",
                        attack_roll, defense_roll, monster.name
                    ));
                    self.state = AppState::Combat(monster);
                } else {
                    let armor_msg = if armor_absorbed > 0 {
                        format!(" (Tier 1 Armor absorbed {} dmg)", armor_absorbed)
                    } else {
                        "".to_string()
                    };
                    self.messages.push(format!(
                        "Defense [{}] < DR 12! The {} strikes dealing {} damage{}!",
                        defense_roll, monster.name, net_player_damage, armor_msg
                    ));
                    if player_hp_left <= 0 {
                        self.messages.push("Your flesh gives way. Nechrubel smiles upon your death...".to_string());
                        let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
                        self.state = AppState::GameOver;
                    } else {
                        self.state = AppState::Combat(monster);
                    }
                }
            }
            RoundOutcome::AdndHit {
                thac0,
                target_ac,
                roll,
                effective_ac_hit,
                is_crit,
                damage,
                monster_hp_left,
            } => {
                let crit_msg = if is_crit { " [NAT 20 CRITICAL!]" } else { "" };
                self.messages.push(format!(
                    "HIT! THAC0 {} vs AC {}: Rolled [{}] (Hits AC {}){}! You deal {} damage.",
                    thac0, target_ac, roll, effective_ac_hit, crit_msg, damage
                ));
                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} is slain!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
            RoundOutcome::AdndMissMonsterHit {
                thac0,
                target_ac,
                roll,
                effective_ac_hit,
                monster_thac0,
                player_ac,
                monster_roll,
                monster_hit,
                damage_taken,
                player_hp_left,
            } => {
                self.messages.push(format!(
                    "MISS! THAC0 {} vs AC {}: Rolled [{}] (Hits AC {}, needed <= {}).",
                    thac0, target_ac, roll, effective_ac_hit, target_ac
                ));

                if monster_hit {
                    self.messages.push(format!(
                        "The {} counterattacks (THAC0 {} vs AC {}): Rolled [{}] -> HITS for {} damage!",
                        monster.name, monster_thac0, player_ac, monster_roll, damage_taken
                    ));
                    if player_hp_left <= 0 {
                        self.messages.push("You fall to 0 HP... Your party's journey ends here.".to_string());
                        let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
                        self.state = AppState::GameOver;
                    } else {
                        self.state = AppState::Combat(monster);
                    }
                } else {
                    self.messages.push(format!(
                        "The {} counterattacks (THAC0 {} vs AC {}): Rolled [{}] -> MISSES your AC {}!",
                        monster.name, monster_thac0, player_ac, monster_roll, player_ac
                    ));
                    self.state = AppState::Combat(monster);
                }
            }
            _ => {}
        }
    }

    fn cast_combat_spell(&mut self, mut monster: Monster) {
        if let Some(outcome) = CombatEngine::cast_palms_scroll(&mut self.character, &mut monster, &mut self.roller) {
            if let RoundOutcome::SpellCast { damage, monster_hp_left, .. } = outcome {
                self.messages.push(format!("You invoke Palms Open the Southern Gate! Searing hellfire deals {} damage!", damage));
                if monster_hp_left <= 0 {
                    self.messages.push(format!("The {} is incinerated by arcane fury!", monster.name));
                    self.resolve_victory(&monster);
                } else {
                    self.state = AppState::Combat(monster);
                }
            }
        } else {
            self.messages.push("You have no offensive scrolls prepared!".to_string());
        }
    }

    fn use_cloak(&mut self, monster: Monster) {
        if self.character.use_cloak_charge() {
            self.messages.push("You wrap the Cloak of Invisibility tight, slipping past unseen!".to_string());
            self.resolve_victory(&monster);
        } else {
            self.messages.push("You don't have a charged Cloak of Invisibility!".to_string());
        }
    }

    fn resolve_flee(&mut self) {
        let outcome = CombatEngine::flee(&mut self.character, &mut self.roller);
        if let RoundOutcome::Fled { damage_taken, player_hp_left } = outcome {
            self.messages.push(format!("You panic and flee, suffering {} damage in your retreat!", damage_taken));
            if player_hp_left <= 0 {
                self.state = AppState::GameOver;
            } else {
                self.dungeon.mark_current_fled();
                self.state = AppState::Exploring;
            }
        }
    }

    fn resolve_victory(&mut self, monster: &Monster) {
        if self.is_scifi && (monster.name.contains("Alien Queen") || monster.name.contains("Broodmother")) {
            self.messages.push("CRITICAL ALERT: THE ALIEN QUEEN IS SLAIN! Hive bio-signals flatline!".to_string());
            self.messages.push("Emergency reactor self-destruct initiated: 5... 4... 3... 2... 1...".to_string());
            self.messages.push("You launch the escape shuttle into the black void as USCSS Atropos vanishes in nuclear fire!".to_string());
            self.messages.push("MISSION ACCOMPLISHED — GRAND SURVIVAL VICTORY!".to_string());
            let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
            self.state = AppState::Victory;
            return;
        }

        let victory = CombatEngine::resolve_victory(&mut self.character, monster, &mut self.roller);
        self.messages.push(format!("Victory! Slew {} (+{} points).", monster.name, victory.points_earned));

        if victory.looted_silver > 0 {
            let currency_name = if self.is_scifi { "corporate credits" } else { "silver coins" };
            self.messages.push(format!("You plunder {} {} from the carcass!", victory.looted_silver, currency_name));
        }

        for item in victory.looted_items {
            self.messages.push(format!("Loot found: {}!", item.name()));
        }

        if victory.adventure_ended {
            self.messages.push(victory.hazard_triggered.unwrap_or_else(|| "An eldritch curse claims you.".to_string()));
            let _ = StorageManager::delete(DEFAULT_SAVE_PATH);
            self.state = AppState::GameOver;
            return;
        }

        self.dungeon.mark_current_explored();

        if victory.instant_level_up {
            if self.is_scifi {
                self.messages.push("Bio-sample extracted! DECK CLEARANCE PROMOTION READY!".to_string());
            } else {
                self.messages.push("The Basilisk's blood awakens an ancient power! INSTANT LEVEL UP!".to_string());
            }
            self.state = AppState::LevelUpPending;
        } else {
            self.check_level_up();
        }
    }

    fn resolve_pit_trap(&mut self) {
        let has_rope = self.character.has_rope();
        let roll = self.roller.roll_d6();
        let effective_roll = if has_rope { roll + 1 } else { roll };

        let rope_note = if has_rope {
            if self.is_scifi { " (+1 mag-boots bonus)" } else { " (+1 rope bonus)" }
        } else {
            ""
        };
        self.messages.push(format!("Hazard check: Rolled {}{} = {}.", roll, rope_note, effective_roll));

        if effective_roll <= 3 {
            let dmg_roll = self.roller.roll_d6();
            let (net_dmg, armor, _) = self.character.take_damage(dmg_roll, false, &mut self.roller);
            let armor_str = if armor > 0 { format!(" (Armor absorbed {} dmg)", armor) } else { "".to_string() };
            if self.is_scifi {
                self.messages.push(format!("CRASH! You are sucked into the decompression vent, taking {} damage{}!", net_dmg, armor_str));
            } else {
                self.messages.push(format!("CRASH! You fall into spikes, suffering {} damage{}!", net_dmg, armor_str));
            }

            if !self.character.is_alive() {
                if self.is_scifi {
                    self.messages.push("The void claims another reckless operative...".to_string());
                } else {
                    self.messages.push("The pit claims another careless rogue...".to_string());
                }
                self.state = AppState::GameOver;
                return;
            }
        } else {
            if self.is_scifi {
                self.messages.push("With quick reflexes and magnetic boots, you anchor safely across the breach!".to_string());
            } else {
                self.messages.push("With agile reflexes, you leap safely across the abyss!".to_string());
            }
        }

        self.dungeon.mark_current_explored();
        self.check_level_up();
    }

    fn resolve_soothsayer(&mut self) {
        let roll = self.roller.roll_d6();
        if roll % 2 != 0 {
            // Odd: solved riddle / override
            let bonus_amount = 10;
            self.character.silver += bonus_amount;
            if self.is_scifi {
                self.messages.push(format!("Override roll: {} (Odd)! M.O.T.H.E.R. terminal accepts passcode and dispenses {} credits!", roll, bonus_amount));
            } else {
                self.messages.push(format!("You rolled {} (Odd)! The soothsayer smiles and grants you {} silver!", roll, bonus_amount));
            }
        } else {
            // Even: shockwave deals 1d4 damage ignoring armor
            let shockwave_dmg = self.roller.roll_d4();
            let (net_dmg, _, _) = self.character.take_damage(shockwave_dmg, true, &mut self.roller);
            if self.is_scifi {
                self.messages.push(format!(
                    "Override roll: {} (Even)! A high-voltage EMP feedback surge shocks you for {} damage!",
                    roll, net_dmg
                ));
            } else {
                self.messages.push(format!(
                    "You rolled {} (Even)! A mind-shattering psychic shockwave tears into you for {} damage!",
                    roll, net_dmg
                ));
            }
            if !self.character.is_alive() {
                if self.is_scifi {
                    self.messages.push("Your neural implants melt from electrical feedback.".to_string());
                } else {
                    self.messages.push("Your mind shatters into a thousand screaming shards.".to_string());
                }
                self.state = AppState::GameOver;
                return;
            }
        }

        self.dungeon.mark_current_explored();
        self.check_level_up();
    }

    fn buy_peddler_item(&mut self, item: Item, cost: i32) {
        let currency = if self.is_scifi { "credits" } else { "silver" };
        let sym = if self.is_scifi { "cr" } else { "s" };
        if self.character.silver >= cost {
            self.character.silver -= cost;
            self.messages.push(format!("Purchased {} for {} {}.", item.name(), cost, currency));
            match &item {
                Item::Armor => {
                    self.character.has_armor = true;
                    self.character.armor_formula = Some(crate::core::dice::DiceFormula::D4);
                }
                Item::CustomArmor { absorption_formula, .. } => {
                    self.character.has_armor = true;
                    self.character.armor_formula = Some(*absorption_formula);
                }
                _ => {
                    self.character.inventory.push(item);
                }
            }
        } else {
            self.messages.push(format!("Not enough {}! You need {}{} but only have {}{}.", currency, cost, sym, self.character.silver, sym));
        }
    }

    fn drink_potion(&mut self) {
        if let Some(healed) = self.character.drink_potion(&mut self.roller) {
            if self.is_scifi {
                self.messages.push(format!("You inject a combat stimm-pak, recovering {} HP and calming nerves!", healed));
            } else {
                self.messages.push(format!("You quaff a bitter healing draught, recovering {} HP!", healed));
            }
        } else {
            if self.is_scifi {
                self.messages.push("No stimm-paks left in your pack!".to_string());
            } else {
                self.messages.push("You have no healing potions in your sack!".to_string());
            }
        }
    }

    fn check_level_up(&mut self) {
        self.character.rooms_explored = self.dungeon.explored_room_count;

        if ProgressionManager::can_level_up_via_exploration(&self.character) {
            let label = if self.is_scifi { "DECK ELEVATOR CLEARANCE UNLOCKED!" } else { "LEVEL UP READY!" };
            self.messages.push(format!("Exploration milestone reached (12 compartments & 15 points)! {}", label));
            self.character.points = 0;
            self.state = AppState::LevelUpPending;
        } else if ProgressionManager::can_level_up_via_charity(&self.character) {
            let label = if self.is_scifi { "40 credits deposited to override ship elevator clearance!" } else { "Amassed 40 silver to aid ruined Wästland homes!" };
            self.messages.push(format!("{} LEVEL UP READY!", label));
            self.character.silver -= 40;
            self.state = AppState::LevelUpPending;
        } else {
            self.state = AppState::Exploring;
        }
    }

    fn resolve_level_up(&mut self) {
        if let Some(reward) = self.progression.roll_new_reward(&mut self.roller) {
            self.progression.apply_reward(
                reward,
                &mut self.character,
                Some(WeakMonsterType::Goblin),
                Some(ToughMonsterType::SmallStoneTroll),
            );

            let reward_desc = if self.is_scifi {
                match reward {
                    LevelUpReward::Knighted => "Promoted! Executive Deck Officer clearance granted!",
                    LevelUpReward::AttackBonusPlusOne => "Targeting Uplink! Permanently gain +1 to firearm hit rolls!",
                    LevelUpReward::MaxHpPlusFive => "Dermal Nanite Mesh! Max Health increases to 20!",
                    LevelUpReward::HerbmasterFivePotions => "Medical Bay Requisition! Received 5 Combat Stimm-Paks!",
                    LevelUpReward::MightyZweihaender => "Armory Access! Found a Plasma Flamethrower (2d8 damage)!",
                    LevelUpReward::HalvedMonsterDamage => "Bio-hazard Innoculation! Alien acid damage permanently halved!",
                }
            } else {
                match reward {
                    LevelUpReward::Knighted => "Knighted! You may call yourself Sir Kargunt!",
                    LevelUpReward::AttackBonusPlusOne => "Weapon Mastery! Permanently gain +1 to attack rolls!",
                    LevelUpReward::MaxHpPlusFive => "Vigor! Max HP increases to 20!",
                    LevelUpReward::HerbmasterFivePotions => "Herbmaster's Favor! Received 5 healing potions!",
                    LevelUpReward::MightyZweihaender => "Found a Mighty Zweihänder (1d6+2 damage)!",
                    LevelUpReward::HalvedMonsterDamage => "Slayer's Aegis! Chosen monster damage permanently halved!",
                }
            };

            let level_label = if self.is_scifi {
                format!("Deck {}", (self.character.level + 1).min(7))
            } else {
                format!("Level {}", self.character.level)
            };
            self.messages.push(format!("{} attained: {}", level_label, reward_desc));

            if self.is_scifi {
                if self.character.level >= 6 {
                    self.messages.push("DECK 7 CLEARANCE UNLOCKED! THE ELEVATOR DESCENDS TO THE QUEEN'S HIVE!".to_string());
                    self.messages.push("Proceed through the next bulkhead to face THE ALIEN QUEEN (BROODMOTHER)!".to_string());
                }
                self.state = AppState::Exploring;
            } else if self.progression.is_grand_victory() {
                self.messages.push("ALL 6 BLESSINGS CLAIMED! The catacomb bows to your might!".to_string());
                self.state = AppState::Victory;
            } else {
                self.state = AppState::Exploring;
            }
        }
    }

    fn cycle_theme(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::ClassicWhite => ThemeMode::AmberCrt,
            ThemeMode::AmberCrt => ThemeMode::GreenCrt,
            ThemeMode::GreenCrt => ThemeMode::MorkBorg,
            ThemeMode::MorkBorg => ThemeMode::SciFiVoid,
            ThemeMode::SciFiVoid => ThemeMode::ClassicWhite,
        };
        self.theme = Theme::for_mode(self.theme_mode);
        self.is_scifi = self.theme_mode == ThemeMode::SciFiVoid;
        if self.is_scifi {
            self.registry = ItemRegistry::load_scifi_or_default();
            self.enemy_registry = EnemyRegistry::load_scifi_or_default();
            self.shop = ShopConfig::load_scifi_or_default();
        } else {
            self.registry = ItemRegistry::load_or_default();
            self.enemy_registry = EnemyRegistry::load_or_default();
            self.shop = ShopConfig::load_or_default();
        }
        self.messages.push(format!("Visual theme switched to {:?}", self.theme_mode));
    }

    fn restart(&mut self) {
        *self = Self::new();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
