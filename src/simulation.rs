use crate::core::character::Character;
use crate::core::combat::CombatEngine;
use crate::core::config::EngineType;
use crate::core::dice::{DiceRoller, StandardRoller};
use crate::core::equipment::Item;
use crate::core::monsters::{Monster, ToughMonsterType, WeakMonsterType};
use crate::core::progression::ProgressionManager;
use crate::dungeon::map::{DungeonGraph, NavigationEvent};
use crate::dungeon::room::{Direction, RoomContent};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SimulationStats {
    pub engine_name: String,
    pub games_played: u32,
    pub victories: u32,
    pub deaths: u32,
    pub total_rooms_explored: u64,
    pub total_silver_collected: u64,
    pub total_levels_gained: u64,
    pub death_causes: HashMap<String, u32>,
}

impl SimulationStats {
    pub fn print_summary(&self) {
        println!("============================================================");
        println!("           DARK FORT MONTE-CARLO SIMULATION REPORT          ");
        if !self.engine_name.is_empty() {
            println!("  Engine:                  {}", self.engine_name);
        }
        println!("============================================================");
        println!("  Total Runs:              {}", self.games_played);
        println!("  Grand Victories (Lvl 6): {} ({:.2}%)", self.victories, (self.victories as f64 / self.games_played as f64) * 100.0);
        println!("  Deaths:                  {} ({:.2}%)", self.deaths, (self.deaths as f64 / self.games_played as f64) * 100.0);
        println!("  Avg Rooms Explored:      {:.2}", self.total_rooms_explored as f64 / self.games_played as f64);
        println!("  Avg Silver at End:       {:.2}s", self.total_silver_collected as f64 / self.games_played as f64);
        println!("  Avg Level Reached:       {:.2}", self.total_levels_gained as f64 / self.games_played as f64);
        println!("------------------------------------------------------------");
        println!("  Causes of Death:");
        let mut causes: Vec<(&String, &u32)> = self.death_causes.iter().collect();
        causes.sort_by(|a, b| b.1.cmp(a.1));
        for (cause, count) in causes {
            println!("   - {:<28}: {:>4} ({:.1}%)", cause, count, (*count as f64 / self.deaths as f64) * 100.0);
        }
        println!("============================================================");
    }
}

pub struct HeadlessSimulator;

impl HeadlessSimulator {
    pub fn run_simulations(runs: u32, base_seed: u64) -> SimulationStats {
        Self::run_simulations_with_engine(runs, base_seed, EngineType::Classic)
    }

    pub fn run_simulations_with_engine(runs: u32, base_seed: u64, engine: EngineType) -> SimulationStats {
        let mut stats = SimulationStats {
            engine_name: engine.name().to_string(),
            ..Default::default()
        };

        for i in 0..runs {
            let seed = base_seed.wrapping_add(i as u64 * 31);
            let rng = StdRng::seed_from_u64(seed);
            let mut roller = StandardRoller::new(rng);

            let result = Self::run_single_delve(&mut roller, engine);
            stats.games_played += 1;
            stats.total_rooms_explored += result.rooms_explored as u64;
            stats.total_silver_collected += result.final_silver as u64;
            stats.total_levels_gained += result.final_level as u64;

            if result.victory {
                stats.victories += 1;
            } else {
                stats.deaths += 1;
                let cause = result.death_reason.unwrap_or_else(|| "Unknown".to_string());
                *stats.death_causes.entry(cause).or_insert(0) += 1;
            }
        }

        stats
    }

    fn run_single_delve<R: DiceRoller>(roller: &mut R, engine: EngineType) -> DelveResult {
        let mut character = Character::new_with_engine(engine, roller);
        let mut dungeon = DungeonGraph::new(roller);
        let mut progression = ProgressionManager::new();

        let mut max_steps = 2000; // prevent runaway loops

        while max_steps > 0 && character.is_alive() && !progression.is_grand_victory() {
            max_steps -= 1;

            // Resolve current room encounter if unexplored
            let content = dungeon.current_room().content.clone();
            match content {
                RoomContent::WeakMonsterEncounter(mut monster) => {
                    let survived = Self::resolve_ai_combat(&mut character, &mut monster, &mut progression, roller);
                    if !survived {
                        return DelveResult {
                            victory: false,
                            rooms_explored: dungeon.explored_room_count,
                            final_silver: character.silver,
                            final_level: character.level,
                            death_reason: Some(format!("Slain by {}", monster.name)),
                        };
                    }
                    dungeon.mark_current_explored();
                }
                RoomContent::ToughMonsterEncounter(mut monster) => {
                    let survived = Self::resolve_ai_combat(&mut character, &mut monster, &mut progression, roller);
                    if !survived {
                        return DelveResult {
                            victory: false,
                            rooms_explored: dungeon.explored_room_count,
                            final_silver: character.silver,
                            final_level: character.level,
                            death_reason: Some(format!("Slain by {}", monster.name)),
                        };
                    }
                    dungeon.mark_current_explored();
                }
                RoomContent::PitTrap => {
                    let has_rope = character.has_rope();
                    let roll = roller.roll_d6();
                    let effective = if has_rope { roll + 1 } else { roll };
                    if effective <= 3 {
                        let dmg = roller.roll_d6();
                        character.take_damage(dmg, false, roller);
                        if !character.is_alive() {
                            return DelveResult {
                                victory: false,
                                rooms_explored: dungeon.explored_room_count,
                                final_silver: character.silver,
                                final_level: character.level,
                                death_reason: Some("Fell into Pit Trap".to_string()),
                            };
                        }
                    }
                    dungeon.mark_current_explored();
                }
                RoomContent::RiddlingSoothsayer => {
                    let roll = roller.roll_d6();
                    if roll % 2 != 0 {
                        character.silver += 10;
                    } else {
                        let shock_dmg = roller.roll_d4();
                        character.take_damage(shock_dmg, true, roller);
                        if !character.is_alive() {
                            return DelveResult {
                                victory: false,
                                rooms_explored: dungeon.explored_room_count,
                                final_silver: character.silver,
                                final_level: character.level,
                                death_reason: Some("Psychic Soothsayer Shockwave".to_string()),
                            };
                        }
                    }
                    dungeon.mark_current_explored();
                }
                RoomContent::Peddler => {
                    // Buy healing potions if low on potions and have silver
                    while character.silver >= 4 && character.inventory.iter().filter(|i| matches!(i, Item::Potion)).count() < 3 {
                        character.silver -= 4;
                        character.inventory.push(Item::Potion);
                    }
                    dungeon.mark_current_explored();
                }
                RoomContent::ItemStash(item) => {
                    character.inventory.push(item);
                    dungeon.mark_current_explored();
                }
                RoomContent::MysticScroll(scroll) => {
                    character.inventory.push(scroll);
                    dungeon.mark_current_explored();
                }
                RoomContent::Empty => {
                    dungeon.mark_current_explored();
                }
            }

            // Check level up conditions
            character.rooms_explored = dungeon.explored_room_count;
            if ProgressionManager::can_level_up_via_exploration(&character) {
                character.points = 0;
                if let Some(reward) = progression.roll_new_reward(roller) {
                    progression.apply_reward(reward, &mut character, Some(WeakMonsterType::Goblin), Some(ToughMonsterType::SmallStoneTroll));
                }
            } else if ProgressionManager::can_level_up_via_charity(&character) {
                character.silver -= 40;
                if let Some(reward) = progression.roll_new_reward(roller) {
                    progression.apply_reward(reward, &mut character, Some(WeakMonsterType::Goblin), Some(ToughMonsterType::SmallStoneTroll));
                }
            }

            if progression.is_grand_victory() {
                return DelveResult {
                    victory: true,
                    rooms_explored: dungeon.explored_room_count,
                    final_silver: character.silver,
                    final_level: character.level,
                    death_reason: None,
                };
            }

            // AI Decision: Drink potion if wounded (HP <= 7)
            if character.hp <= 7 {
                character.drink_potion(roller);
            }

            // Find unopened door or backtrack
            let unopened = dungeon.current_room().unopened_directions();
            if !unopened.is_empty() {
                let dir = unopened[0];
                let _ = dungeon.traverse_door(dir, None, roller);
            } else {
                // Backtrack to connected room
                let connected_dirs: Vec<Direction> = dungeon.current_room().doors.iter().map(|d| d.direction).collect();
                if let Some(&dir) = connected_dirs.first() {
                    let nav = dungeon.traverse_door(dir, None, roller);
                    if let Ok(NavigationEvent::WanderingMonsterAmbush(mut monster)) = nav {
                        let survived = Self::resolve_ai_combat(&mut character, &mut monster, &mut progression, roller);
                        if !survived {
                            return DelveResult {
                                victory: false,
                                rooms_explored: dungeon.explored_room_count,
                                final_silver: character.silver,
                                final_level: character.level,
                                death_reason: Some(format!("Ambushed by wandering {}", monster.name)),
                            };
                        }
                    }
                } else if !dungeon.has_any_unopened_doors() {
                    return DelveResult {
                        victory: false,
                        rooms_explored: dungeon.explored_room_count,
                        final_silver: character.silver,
                        final_level: character.level,
                        death_reason: Some("Trapped in Dead End".to_string()),
                    };
                }
            }
        }

        DelveResult {
            victory: progression.is_grand_victory(),
            rooms_explored: dungeon.explored_room_count,
            final_silver: character.silver,
            final_level: character.level,
            death_reason: if character.is_alive() { None } else { Some("Succumbed to Catacomb Wounds".to_string()) },
        }
    }

    fn resolve_ai_combat<R: DiceRoller>(
        character: &mut Character,
        monster: &mut Monster,
        progression: &mut ProgressionManager,
        roller: &mut R,
    ) -> bool {
        while monster.hp > 0 && character.is_alive() {
            // Drink potion if critical
            if character.hp <= 5 {
                character.drink_potion(roller);
            }

            // Use offensive spell if available
            if CombatEngine::cast_palms_scroll(character, monster, roller).is_some() {
                if monster.hp <= 0 {
                    break;
                }
                continue;
            }

            // Use cloak if tough monster and available
            if monster.points_to_hit >= 5 && character.has_cloak() && character.use_cloak_charge() {
                break;
            }

            // Attack
            CombatEngine::resolve_attack_round(character, monster, roller);
        }

        if !character.is_alive() {
            return false;
        }

        let victory = CombatEngine::resolve_victory(character, monster, roller);
        if victory.adventure_ended {
            return false; // Maggot or Medusa petrification
        }

        if victory.instant_level_up {
            if let Some(reward) = progression.roll_new_reward(roller) {
                progression.apply_reward(reward, character, Some(WeakMonsterType::BloodDrenchedSkeleton), Some(ToughMonsterType::NecroSorcerer));
            }
        }

        true
    }
}

struct DelveResult {
    pub victory: bool,
    pub rooms_explored: u32,
    pub final_silver: i32,
    pub final_level: u32,
    pub death_reason: Option<String>,
}
