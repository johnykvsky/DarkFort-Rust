use crate::core::character::{Character, CharacterVitals};
use crate::core::config::EngineType;
use crate::ui::theme::{Theme, ThemeMode};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};

pub struct StatsWidget<'a> {
    character: &'a Character,
    theme: &'a Theme,
}

impl<'a> StatsWidget<'a> {
    pub fn new(character: &'a Character, theme: &'a Theme) -> Self {
        Self { character, theme }
    }
}

impl<'a> Widget for StatsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_engine = if self.theme.mode == ThemeMode::SciFiVoid {
            " [ USCSS ATROPOS — DERELICT SHIP MANIFEST ] "
        } else {
            match self.character.engine_type {
                EngineType::Mothership => " [ CHARACTER — MOTHERSHIP 1e ] ",
                EngineType::MorkBorg => " [ CHARACTER — MÖRK BORG ] ",
                EngineType::Durf => " [ CHARACTER — DURF ] ",
                EngineType::Adnd2e => " [ CHARACTER — AD&D 2e (INFINITY ENGINE) ] ",
                EngineType::Classic => " [ CHARACTER ] ",
            }
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title(title_engine)
            .title_style(Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 10 || inner.height < 5 {
            return;
        }

        let mut y = inner.y;
        let x = inner.x;
        let max_x = inner.x + inner.width;

        // Name & Title
        let full_name = match &self.character.title {
            Some(title) => {
                if self.theme.mode == ThemeMode::SciFiVoid {
                    format!("{} (Deck {}/7)", title, (self.character.level + 1).min(7))
                } else {
                    format!("{} (Level {})", title, self.character.level)
                }
            }
            None => {
                if self.theme.mode == ThemeMode::SciFiVoid {
                    format!("{} (Warrant Officer, Deck {}/7)", self.character.name, (self.character.level + 1).min(7))
                } else {
                    format!("{} (The Catacomb Rogue, Lvl {})", self.character.name, self.character.level)
                }
            }
        };
        buf.set_string(x, y, &full_name, Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD));
        y += 1;

        // Vitals Section: Engine-Specific Rendering
        match &self.character.vitals {
            CharacterVitals::Mothership {
                health,
                max_health,
                wounds,
                max_wounds,
                stress,
                max_stress,
                combat,
                strength,
                agility,
                occult,
                armor_points,
            } => {
                // Health Bar with AP flat soak
                if y < inner.y + inner.height {
                    let hp_pct = (*health as f32 / *max_health as f32).clamp(0.0, 1.0);
                    let bar_width: usize = 12;
                    let filled = (hp_pct * bar_width as f32).round() as usize;
                    let empty = bar_width.saturating_sub(filled);

                    let hp_color = if hp_pct > 0.6 {
                        self.theme.hp_high
                    } else if hp_pct > 0.3 {
                        self.theme.hp_mid
                    } else {
                        self.theme.hp_low
                    };

                    let bar_str = format!(
                        "[{}{}] {}/{} HP  |  AP: {}",
                        "█".repeat(filled),
                        "░".repeat(empty),
                        health,
                        max_health,
                        armor_points
                    );
                    buf.set_string(x, y, &bar_str, Style::default().fg(hp_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }

                // Wounds & Stress
                if y < inner.y + inner.height {
                    let wounds_str = format!("Wounds: {}/{}  |  Stress: {}/{}", wounds, max_wounds, stress, max_stress);
                    let stress_color = if *stress >= 10 { self.theme.danger } else { self.theme.warning };
                    buf.set_string(x, y, &wounds_str, Style::default().fg(stress_color));
                    y += 1;
                }

                // Percentile Stats
                if y < inner.y + inner.height {
                    let stats_str = format!(
                        "STR: {}%  AGI: {}%  OCC: {}%  COM: {}%",
                        strength, agility, occult, combat
                    );
                    buf.set_string(x, y, &stats_str, Style::default().fg(self.theme.fg));
                    y += 1;
                }
            }
            CharacterVitals::Durf {
                armor_pool,
                max_armor,
                wounds,
                hit_dice,
                force,
                finesse,
                will,
                shield_soak,
                ..
            } => {
                // Ablative Armor Bar
                if y < inner.y + inner.height {
                    let armor_pct = if *max_armor > 0 {
                        (*armor_pool as f32 / *max_armor as f32).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let bar_width: usize = 12;
                    let filled = (armor_pct * bar_width as f32).round() as usize;
                    let empty = bar_width.saturating_sub(filled);

                    let armor_color = if *armor_pool > 1 {
                        self.theme.hp_high
                    } else if *armor_pool == 1 {
                        self.theme.hp_mid
                    } else {
                        self.theme.hp_low
                    };

                    let bar_str = format!(
                        "[{}{}] {}/{} ARMOR | SHIELD: -{}",
                        "█".repeat(filled),
                        "░".repeat(empty),
                        armor_pool,
                        max_armor,
                        shield_soak
                    );
                    buf.set_string(x, y, &bar_str, Style::default().fg(armor_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }

                // Wounds & Hit Dice Survival
                if y < inner.y + inner.height {
                    let wound_color = if *wounds == 0 { self.theme.success } else { self.theme.danger };
                    let wounds_str = format!("Wounds: {}  |  Survival Dice: {}d6 (Mortality Check)", wounds, hit_dice);
                    buf.set_string(x, y, &wounds_str, Style::default().fg(wound_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }

                // Attributes (Force, Finesse, Will)
                if y < inner.y + inner.height {
                    let stats_str = format!("FORCE: {:+}  FINESSE: {:+}  WILL: {:+}", force, finesse, will);
                    buf.set_string(x, y, &stats_str, Style::default().fg(self.theme.fg));
                    y += 1;
                }
            }
            CharacterVitals::MorkBorg {
                hp,
                max_hp,
                omens,
                max_omens,
                strength,
                agility,
                presence,
                toughness,
                armor_tier,
            } => {
                // HP Bar
                if y < inner.y + inner.height {
                    let hp_pct = (*hp as f32 / *max_hp as f32).clamp(0.0, 1.0);
                    let bar_width: usize = 12;
                    let filled = (hp_pct * bar_width as f32).round() as usize;
                    let empty = bar_width.saturating_sub(filled);

                    let hp_color = if hp_pct > 0.6 {
                        self.theme.hp_high
                    } else if hp_pct > 0.3 {
                        self.theme.hp_mid
                    } else {
                        self.theme.hp_low
                    };

                    let bar_str = format!(
                        "[{}{}] {}/{} HP  |  Tier {} (1d2 Soak)",
                        "█".repeat(filled),
                        "░".repeat(empty),
                        hp,
                        max_hp,
                        armor_tier
                    );
                    buf.set_string(x, y, &bar_str, Style::default().fg(hp_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }

                // Omens & DR
                if y < inner.y + inner.height {
                    let omens_str = format!("Omens: {}/{} (Free Reroll/Max Dmg)  |  Target: DR 12", omens, max_omens);
                    buf.set_string(x, y, &omens_str, Style::default().fg(self.theme.warning));
                    y += 1;
                }

                // 4 Core Attributes
                if y < inner.y + inner.height {
                    let stats_str = format!(
                        "STR: {:+}  AGI: {:+}  PRES: {:+}  TOUGH: {:+}",
                        strength, agility, presence, toughness
                    );
                    buf.set_string(x, y, &stats_str, Style::default().fg(self.theme.fg));
                    y += 1;
                }
            }
            CharacterVitals::Adnd2e {
                thac0,
                ac,
                hp,
                max_hp,
                strength,
                dexterity,
                constitution,
                intelligence,
                wisdom,
                charisma: _,
                save_death_poison,
                save_spell,
                ..
            } => {
                // HP Bar with AC & THAC0
                if y < inner.y + inner.height {
                    let hp_pct = (*hp as f32 / *max_hp as f32).clamp(0.0, 1.0);
                    let bar_width: usize = 12;
                    let filled = (hp_pct * bar_width as f32).round() as usize;
                    let empty = bar_width.saturating_sub(filled);

                    let hp_color = if hp_pct > 0.6 {
                        self.theme.hp_high
                    } else if hp_pct > 0.3 {
                        self.theme.hp_mid
                    } else {
                        self.theme.hp_low
                    };

                    let bar_str = format!(
                        "[{}{}] {}/{} HP  |  AC: {}  |  THAC0: {}",
                        "█".repeat(filled),
                        "░".repeat(empty),
                        hp,
                        max_hp,
                        ac,
                        thac0
                    );
                    buf.set_string(x, y, &bar_str, Style::default().fg(hp_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }

                // Saves & Combat Info
                if y < inner.y + inner.height {
                    let saves_str = format!("SAVES: Death/Poison {} | Spells {}", save_death_poison, save_spell);
                    buf.set_string(x, y, &saves_str, Style::default().fg(self.theme.warning));
                    y += 1;
                }

                // Classic 6 Ability Scores
                if y < inner.y + inner.height {
                    let stats_str = format!(
                        "STR: {}  DEX: {}  CON: {}  INT: {}  WIS: {}",
                        strength, dexterity, constitution, intelligence, wisdom
                    );
                    buf.set_string(x, y, &stats_str, Style::default().fg(self.theme.fg));
                    y += 1;
                }
            }
            _ => {
                // Classic HP Bar
                if y < inner.y + inner.height {
                    let hp_pct = (self.character.hp as f32 / self.character.max_hp as f32).clamp(0.0, 1.0);
                    let bar_width: usize = 15;
                    let filled = (hp_pct * bar_width as f32).round() as usize;
                    let empty = bar_width.saturating_sub(filled);

                    let hp_color = if hp_pct > 0.6 {
                        self.theme.hp_high
                    } else if hp_pct > 0.3 {
                        self.theme.hp_mid
                    } else {
                        self.theme.hp_low
                    };

                    let bar_str = format!("[{}{}] {}/{} HP", "█".repeat(filled), "░".repeat(empty), self.character.hp, self.character.max_hp);
                    buf.set_string(x, y, &bar_str, Style::default().fg(hp_color).add_modifier(Modifier::BOLD));
                    y += 1;
                }
            }
        }

        // Silver & Points
        if y < inner.y + inner.height {
            let currency_label = if self.theme.mode == ThemeMode::SciFiVoid { "Credits" } else { "Silver" };
            let currency_sym = if self.theme.mode == ThemeMode::SciFiVoid { "cr" } else { "s" };
            let stats_line = format!(
                "{}: {}{}  |  Points: {}/15  |  Explored: {}/12",
                currency_label, self.character.silver, currency_sym, self.character.points, self.character.rooms_explored
            );
            buf.set_string(x, y, &stats_line, Style::default().fg(self.theme.fg));
            y += 1;
        }

        // Divider
        if y < inner.y + inner.height {
            buf.set_string(x, y, "―".repeat(inner.width as usize), Style::default().fg(Color::DarkGray));
            y += 1;
        }

        // Equipment
        if y < inner.y + inner.height {
            let bonus = self.character.total_attack_bonus();
            let bonus_str = if bonus > 0 { format!(" (+{} att)", bonus) } else { "".to_string() };
            let eq_str = format!("Weapon: {}{}", self.character.weapon.name(), bonus_str);
            buf.set_string(x, y, &eq_str, Style::default().fg(self.theme.warning));
            y += 1;
        }

        if y < inner.y + inner.height {
            let armor_str = if self.character.has_armor {
                "Armor:  Leather Armor (-1d4 damage absorbed)"
            } else {
                "Armor:  None (Unprotected)"
            };
            buf.set_string(x, y, armor_str, Style::default().fg(self.theme.fg));
            y += 1;
        }

        // Active Buffs
        if self.character.active_daemon_fights_left > 0 && y < inner.y + inner.height {
            let daemon_str = format!("* Daemon Ally: {} fights remaining (1d4 dmg)", self.character.active_daemon_fights_left);
            buf.set_string(x, y, &daemon_str, Style::default().fg(self.theme.success));
            y += 1;
        }

        if self.character.active_aegis_uses_left > 0 && y < inner.y + inner.height {
            let aegis_str = format!("* Aegis of Sorrow: {} uses (-1d4 dmg)", self.character.active_aegis_uses_left);
            buf.set_string(x, y, &aegis_str, Style::default().fg(self.theme.success));
            y += 1;
        }

        // Divider
        if y < inner.y + inner.height {
            buf.set_string(x, y, "―".repeat(inner.width as usize), Style::default().fg(Color::DarkGray));
            y += 1;
        }

        // Inventory
        if y < inner.y + inner.height {
            buf.set_string(x, y, "Inventory:", Style::default().fg(self.theme.title));
            y += 1;
        }

        if self.character.inventory.is_empty() && y < inner.y + inner.height {
            buf.set_string(x + 2, y, "(empty sack)", Style::default().fg(Color::DarkGray));
        } else {
            for (idx, item) in self.character.inventory.iter().enumerate() {
                if y >= inner.y + inner.height {
                    break;
                }
                let item_line = format!(" [{}] {}", idx + 1, item.name());
                let truncated = if item_line.len() > (max_x - x) as usize {
                    &item_line[..((max_x - x) as usize).saturating_sub(1)]
                } else {
                    &item_line
                };
                buf.set_string(x, y, truncated, Style::default().fg(self.theme.fg));
                y += 1;
            }
        }
    }
}
