use crate::core::dice::DiceRoller;
use crate::core::monsters::Monster;
use crate::dungeon::generator::DungeonGenerator;
use crate::dungeon::room::{Direction, DoorStatus, RoomId, RoomNode, RoomStatus};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationEvent {
    EnteredNewRoom(RoomId),
    BacktrackedToRoom(RoomId),
    WanderingMonsterAmbush(Monster),
    DeadEndNoDoorsLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonGraph {
    pub rooms: HashMap<RoomId, RoomNode>,
    pub current_room_id: RoomId,
    pub entrance_id: RoomId,
    pub next_room_id: u32,
    pub explored_room_count: u32,
}

impl DungeonGraph {
    pub fn new<R: DiceRoller>(roller: &mut R) -> Self {
        let entrance = DungeonGenerator::generate_entrance(roller);
        let entrance_id = entrance.id;

        let mut rooms = HashMap::new();
        rooms.insert(entrance_id, entrance);

        Self {
            rooms,
            current_room_id: entrance_id,
            entrance_id,
            next_room_id: 1,
            explored_room_count: 0,
        }
    }

    pub fn current_room(&self) -> &RoomNode {
        &self.rooms[&self.current_room_id]
    }

    pub fn current_room_mut(&mut self) -> &mut RoomNode {
        self.rooms.get_mut(&self.current_room_id).expect("Current room must exist")
    }

    pub fn get_room(&self, id: RoomId) -> Option<&RoomNode> {
        self.rooms.get(&id)
    }

    pub fn get_room_mut(&mut self, id: RoomId) -> Option<&mut RoomNode> {
        self.rooms.get_mut(&id)
    }

    /// Mark the current room as explored and increment explored count.
    pub fn mark_current_explored(&mut self) {
        let room = self.current_room_mut();
        if room.status != RoomStatus::Explored {
            room.status = RoomStatus::Explored;
            self.explored_room_count += 1;
        }
    }

    /// Mark the current room as fled from.
    pub fn mark_current_fled(&mut self) {
        let room = self.current_room_mut();
        room.status = RoomStatus::FledFrom;
    }

    /// Set of occupied coordinates on the 2D grid.
    pub fn occupied_coordinates(&self) -> HashSet<(i32, i32)> {
        self.rooms.values().map(|r| r.coordinates).collect()
    }

    /// Move through a door in `dir`.
    /// Handles both exploring brand new rooms and backtracking to previously explored rooms.
    pub fn traverse_door<R: DiceRoller>(
        &mut self,
        dir: Direction,
        force_content_roll: Option<i32>,
        roller: &mut R,
    ) -> Result<NavigationEvent, String> {
        let current_coords = self.current_room().coordinates;
        let current_id = self.current_room_id;

        let door_status = {
            let door = self.current_room().doors.iter().find(|d| d.direction == dir);
            match door {
                Some(d) => d.status.clone(),
                None => return Err(format!("No door facing {:?}", dir)),
            }
        };

        match door_status {
            DoorStatus::Connected(target_id) => {
                // Backtracking into existing room
                self.current_room_id = target_id;

                // When entering an explored room or a room you fled from:
                // 1-in-4 chance you face a WEAK monster (roll 1 on 1d4).
                let ambush = roller.roll_d4() == 1;
                if ambush {
                    let weak_monster = Monster::roll_weak(roller);
                    Ok(NavigationEvent::WanderingMonsterAmbush(weak_monster))
                } else {
                    Ok(NavigationEvent::BacktrackedToRoom(target_id))
                }
            }
            DoorStatus::Unopened => {
                // Generate a brand new room
                let new_id = RoomId(self.next_room_id);
                self.next_room_id += 1;

                let occupied = self.occupied_coordinates();
                let new_room = DungeonGenerator::generate_next_room(
                    new_id,
                    current_id,
                    current_coords,
                    dir,
                    &occupied,
                    force_content_roll,
                    roller,
                );

                // Connect current room door to new room
                self.current_room_mut().connect_door(dir, new_id);

                // Insert new room and move player into it
                self.rooms.insert(new_id, new_room);
                self.current_room_id = new_id;

                Ok(NavigationEvent::EnteredNewRoom(new_id))
            }
        }
    }

    /// Check if any room anywhere in the explored catacombs still has unopened doors.
    /// If none exist and player hits a dead end, the delve is over.
    pub fn has_any_unopened_doors(&self) -> bool {
        self.rooms.values().any(|r| r.has_unopened_doors())
    }

    /// Get bounding box of all discovered rooms for centering map views: ((min_x, min_y), (max_x, max_y)).
    pub fn bounding_box(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for r in self.rooms.values() {
            min_x = min_x.min(r.coordinates.0);
            max_x = max_x.max(r.coordinates.0);
            min_y = min_y.min(r.coordinates.1);
            max_y = max_y.max(r.coordinates.1);
        }

        ((min_x, min_y), (max_x, max_y))
    }
}
