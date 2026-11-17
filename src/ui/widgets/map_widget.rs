use crate::dungeon::map::DungeonGraph;
use crate::dungeon::room::{Direction, DoorStatus};
use crate::ui::theme::{Theme, ThemeMode};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};
use std::collections::HashMap;

pub struct MapWidget<'a> {
    dungeon: &'a DungeonGraph,
    theme: &'a Theme,
}

impl<'a> MapWidget<'a> {
    pub fn new(dungeon: &'a DungeonGraph, theme: &'a Theme) -> Self {
        Self { dungeon, theme }
    }
}

impl<'a> Widget for MapWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = if self.theme.mode == ThemeMode::SciFiVoid {
            " [ SHIP DECK SCHEMATIC ] "
        } else {
            " [ DUNGEON MAP ] "
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title(title)
            .title_style(Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 5 || inner.height < 3 {
            return;
        }

        // Center map around current room or dungeon bounding box
        let current_coords = self.dungeon.current_room().coordinates;
        let center_x = inner.x + inner.width / 2;
        let center_y = inner.y + inner.height / 2;

        // Scale factors: horizontal rooms spaced by 8 chars, vertical by 4 chars
        let scale_x: i32 = 8;
        let scale_y: i32 = 4;

        let mut rendered_doors = HashMap::new();

        for room in self.dungeon.rooms.values() {
            let rel_x = (room.coordinates.0 - current_coords.0) * scale_x;
            let rel_y = (room.coordinates.1 - current_coords.1) * scale_y;

            let screen_x = center_x as i32 + rel_x;
            let screen_y = center_y as i32 + rel_y;

            let is_current = room.id == self.dungeon.current_room_id;
            let label = if room.is_entrance {
                if is_current { "[@E]" } else { "[ E]" }
            } else if is_current {
                "[@]"
            } else {
                "[#]"
            };

            let room_style = if is_current {
                Style::default()
                    .fg(self.theme.map_player)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.theme.map_explored)
            };

            // Draw room label if inside inner view bounds
            if screen_x > inner.x as i32
                && (screen_x + label.len() as i32) < (inner.x + inner.width) as i32
                && screen_y > inner.y as i32
                && screen_y < (inner.y + inner.height) as i32
            {
                buf.set_string(screen_x as u16, screen_y as u16, label, room_style);
            }

            // Draw doors / corridors
            for door in &room.doors {
                let d_delta = door.direction.delta();
                match door.status {
                    DoorStatus::Connected(target_id) => {
                        // Avoid drawing duplicate corridors
                        let pair = if room.id.0 < target_id.0 {
                            (room.id, target_id)
                        } else {
                            (target_id, room.id)
                        };

                        if rendered_doors.insert(pair, true).is_none() {
                            let corridor_x = screen_x + d_delta.0 * (scale_x / 2);
                            let corridor_y = screen_y + d_delta.1 * (scale_y / 2);

                            let sym = match door.direction {
                                Direction::North | Direction::South => "|",
                                Direction::East | Direction::West => "---",
                            };

                            if corridor_x > inner.x as i32
                                && corridor_x < (inner.x + inner.width) as i32
                                && corridor_y > inner.y as i32
                                && corridor_y < (inner.y + inner.height) as i32
                            {
                                buf.set_string(
                                    corridor_x as u16,
                                    corridor_y as u16,
                                    sym,
                                    Style::default().fg(Color::DarkGray),
                                );
                            }
                        }
                    }
                    DoorStatus::Unopened => {
                        let door_x = screen_x + d_delta.0 * 3;
                        let door_y = screen_y + d_delta.1 * 2;

                        if door_x > inner.x as i32
                            && door_x < (inner.x + inner.width) as i32
                            && door_y > inner.y as i32
                            && door_y < (inner.y + inner.height) as i32
                        {
                            buf.set_string(
                                door_x as u16,
                                door_y as u16,
                                "?",
                                Style::default().fg(self.theme.map_door).add_modifier(Modifier::BOLD),
                            );
                        }
                    }
                }
            }
        }
    }
}
