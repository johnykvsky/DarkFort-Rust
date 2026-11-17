use crate::core::monsters::{Monster, MonsterCategory};
use crate::dungeon::room::{RoomContent, RoomNode};
use crate::ui::ascii_art;
use crate::ui::theme::{Theme, ThemeMode};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};

pub struct EncounterWidget<'a> {
    room: &'a RoomNode,
    active_monster: Option<&'a Monster>,
    theme: &'a Theme,
}

impl<'a> EncounterWidget<'a> {
    pub fn new(room: &'a RoomNode, active_monster: Option<&'a Monster>, theme: &'a Theme) -> Self {
        Self {
            room,
            active_monster,
            theme,
        }
    }
}

impl<'a> Widget for EncounterWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = if self.theme.mode == ThemeMode::SciFiVoid {
            " [ BIO-SCANNER / COMPARTMENT ] "
        } else {
            " [ ENCOUNTER / CHAMBER ] "
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title(title)
            .title_style(Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 10 || inner.height < 4 {
            return;
        }

        let (art_str, title_str, subtitle_str) = if let Some(m) = self.active_monster {
            let cat_str = match m.category {
                MonsterCategory::Weak => "Weak Monster",
                MonsterCategory::Tough => "TOUGH Monster",
            };
            let sub = format!(
                "{} | Points to hit: {} | Awards: {} pts | HP: {}/{}",
                cat_str, m.points_to_hit, m.points_awarded, m.hp, m.max_hp
            );
            (ascii_art::get_art_for_monster(&m.name), m.name.clone(), sub)
        } else {
            match &self.room.content {
                RoomContent::PitTrap => {
                    if self.theme.mode == ThemeMode::SciFiVoid {
                        (
                            ascii_art::PIT_TRAP_ART,
                            "Hull Breach / Gravity Vent".to_string(),
                            "Roll 1d6 (1-3: 1d6 decompression damage). Thruster pack grants +1 bonus!".to_string(),
                        )
                    } else {
                        (
                            ascii_art::PIT_TRAP_ART,
                            "Treacherous Pit Trap".to_string(),
                            "Roll 1d6 (1-3: 1d6 damage). Carry a rope for +1 bonus!".to_string(),
                        )
                    }
                }
                RoomContent::RiddlingSoothsayer => {
                    if self.theme.mode == ThemeMode::SciFiVoid {
                        (
                            ascii_art::SOOTHSAYER_ART,
                            "M.O.T.H.E.R. AI Mainframe".to_string(),
                            "Cryptographic override! Odd = security clearance cache, Even = neural feedback shock!".to_string(),
                        )
                    } else {
                        (
                            ascii_art::SOOTHSAYER_ART,
                            "Riddling Soothsayer".to_string(),
                            "Answer the riddle! Odd = reward, Even = mental shockwave!".to_string(),
                        )
                    }
                }
                RoomContent::Peddler => {
                    if self.theme.mode == ThemeMode::SciFiVoid {
                        (
                            ascii_art::PEDDLER_ART,
                            "Automated Requisition Kiosk".to_string(),
                            "Emergency supply terminal: Military surplus & stimms for credits.".to_string(),
                        )
                    } else {
                        (
                            ascii_art::PEDDLER_ART,
                            "Peddler from Beyond the Void".to_string(),
                            "Trinkets and steel for blood-soaked coins.".to_string(),
                        )
                    }
                }
                RoomContent::ItemStash(item) => (
                    ascii_art::LOGO_BANNER,
                    if self.theme.mode == ThemeMode::SciFiVoid { "Salvage Locker".to_string() } else { "Abandoned Stash".to_string() },
                    format!("You discovered: {}", item.name()),
                ),
                RoomContent::MysticScroll(scroll) => (
                    ascii_art::SOOTHSAYER_ART,
                    if self.theme.mode == ThemeMode::SciFiVoid { "Data Pad Terminal".to_string() } else { "Dying Mystic".to_string() },
                    format!("Decrypted tactical protocol: {}", scroll.name()),
                ),
                RoomContent::WeakMonsterEncounter(m) => {
                    (ascii_art::get_art_for_monster(&m.name), m.name.clone(), "A weak monster stands guard!".to_string())
                }
                RoomContent::ToughMonsterEncounter(m) => {
                    (ascii_art::get_art_for_monster(&m.name), m.name.clone(), "A formidable tough beast!".to_string())
                }
                RoomContent::Empty => (
                    "",
                    if self.theme.mode == ThemeMode::SciFiVoid {
                        format!("Depressurized {}", self.room.shape.name())
                    } else {
                        format!("Empty {}", self.room.shape.name())
                    },
                    if self.theme.mode == ThemeMode::SciFiVoid {
                        "Atmospheric pressure nominal. Emergency strobes illuminate empty bulkheads.".to_string()
                    } else {
                        "The dust settles in the eerie silence of the catacombs.".to_string()
                    },
                ),
            }
        };

        let mut y = inner.y;
        let x = inner.x;

        // Render Title & Subtitle (with line wrapping so descriptions never truncate)
        buf.set_string(
            x,
            y,
            &title_str,
            Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
        );
        y += 1;

        for sub_line in crate::ui::widgets::log_widget::wrap_text(&subtitle_str, inner.width as usize) {
            if y >= inner.y + inner.height {
                break;
            }
            buf.set_string(
                x,
                y,
                &sub_line,
                Style::default().fg(self.theme.fg),
            );
            y += 1;
        }

        // Divider
        if y < inner.y + inner.height {
            buf.set_string(x, y, "―".repeat(inner.width as usize), Style::default().fg(Color::DarkGray));
            y += 1;
        }

        // If active monster, draw HP bar
        if let Some(m) = self.active_monster {
            let hp_pct = (m.hp as f32 / m.max_hp as f32).clamp(0.0, 1.0);
            let bar_width = (inner.width as usize).saturating_sub(15).min(20);
            let filled = (hp_pct * bar_width as f32).round() as usize;
            let empty = bar_width.saturating_sub(filled);

            let hp_str = format!("Monster HP: [{}{}] {}/{}", "█".repeat(filled), "░".repeat(empty), m.hp, m.max_hp);
            buf.set_string(x, y, &hp_str, Style::default().fg(self.theme.danger).add_modifier(Modifier::BOLD));
            y += 1;
        }

        // Render ASCII art lines
        for line in art_str.lines() {
            if y >= inner.y + inner.height {
                break;
            }
            if !line.trim().is_empty() {
                buf.set_string(x + 2, y, line, Style::default().fg(self.theme.border));
                y += 1;
            }
        }
    }
}
