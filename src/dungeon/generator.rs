use crate::core::dice::DiceRoller;
use crate::core::equipment::{Item, ScrollType};
use crate::core::monsters::Monster;
use crate::dungeon::room::{Direction, Door, DoorStatus, RoomContent, RoomId, RoomNode, RoomShape, RoomStatus};
use std::collections::HashSet;

pub struct DungeonGenerator;

impl DungeonGenerator {
    /// Generate the Entrance Room (Room 0).
    /// Roll 1d4 for doors and entrance encounter.
    pub fn generate_entrance<R: DiceRoller>(roller: &mut R) -> RoomNode {
        let d4_roll = roller.roll_d4();

        let content = match d4_roll {
            1 => RoomContent::ItemStash(Item::roll_random_item(roller)),
            2 => RoomContent::WeakMonsterEncounter(Monster::roll_weak(roller)),
            3 => RoomContent::MysticScroll(Item::Scroll(ScrollType::roll_random(roller))),
            _ => RoomContent::Empty,
        };

        // Entrance has d4 doors
        let mut available_dirs = Direction::all().to_vec();
        let door_count = d4_roll as usize;
        let mut doors = Vec::new();

        for _ in 0..door_count {
            if !available_dirs.is_empty() {
                let dir = available_dirs.remove(0);
                doors.push(Door {
                    direction: dir,
                    status: DoorStatus::Unopened,
                });
            }
        }

        RoomNode {
            id: RoomId(0),
            is_entrance: true,
            shape: RoomShape::Square,
            coordinates: (0, 0),
            doors,
            status: RoomStatus::Unexplored,
            content,
        }
    }

    /// Generate a new room connected from `from_room_id` at `from_coords` along `entry_dir`.
    /// `occupied_coords`: set of (x, y) coordinates already having rooms.
    /// `force_content_roll`: allows `Scroll::FalseOmen` to override room content roll!
    pub fn generate_next_room<R: DiceRoller>(
        new_id: RoomId,
        from_room_id: RoomId,
        from_coords: (i32, i32),
        entry_dir: Direction,
        occupied_coords: &HashSet<(i32, i32)>,
        force_content_roll: Option<i32>,
        roller: &mut R,
    ) -> RoomNode {
        let delta = entry_dir.delta();
        let new_coords = (from_coords.0 + delta.0, from_coords.1 + delta.1);

        let shape = RoomShape::roll(roller);

        // Determine content from Room Table (1d6)
        let room_roll = force_content_roll.unwrap_or_else(|| roller.roll_d6());
        let content = match room_roll {
            1 => RoomContent::Empty,
            2 => RoomContent::PitTrap,
            3 => RoomContent::RiddlingSoothsayer,
            4 => RoomContent::WeakMonsterEncounter(Monster::roll_weak(roller)),
            5 => RoomContent::ToughMonsterEncounter(Monster::roll_tough(roller)),
            _ => RoomContent::Peddler,
        };

        // Roll d4 for doors leading further:
        // 1: No door (dead end)
        // 2: 1 door
        // 3-4: 2 doors
        let door_roll = roller.roll_d4();
        let further_doors_needed = match door_roll {
            1 => 0,
            2 => 1,
            _ => 2,
        };

        let back_dir = entry_dir.opposite();
        let mut doors = Vec::new();

        // 1. Always connect back to the originating room
        doors.push(Door {
            direction: back_dir,
            status: DoorStatus::Connected(from_room_id),
        });

        // 2. Find available unblocked directions for further doors
        let mut potential_dirs: Vec<Direction> = Direction::all()
            .into_iter()
            .filter(|&d| d != back_dir)
            .filter(|&d| {
                let d_delta = d.delta();
                let neighbor_coords = (new_coords.0 + d_delta.0, new_coords.1 + d_delta.1);
                !occupied_coords.contains(&neighbor_coords)
            })
            .collect();

        // Add further unopened doors up to available slots
        for _ in 0..further_doors_needed {
            if !potential_dirs.is_empty() {
                let d = potential_dirs.remove(0);
                doors.push(Door {
                    direction: d,
                    status: DoorStatus::Unopened,
                });
            }
        }

        RoomNode {
            id: new_id,
            is_entrance: false,
            shape,
            coordinates: new_coords,
            doors,
            status: RoomStatus::Unexplored,
            content,
        }
    }
}
