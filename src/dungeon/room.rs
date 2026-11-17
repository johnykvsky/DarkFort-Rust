use crate::core::dice::{DiceFormula, DiceRoller};
use crate::core::equipment::Item;
use crate::core::monsters::Monster;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoomId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomShape {
    IrregularCave,  // 2
    Oval,           // 3
    CrossShaped,    // 4
    Corridor,       // 5
    Square,         // 6-8
    Round,          // 9
    Rectangular,    // 10
    Triangular,     // 11
    SkullShaped,    // 12
}

impl RoomShape {
    pub fn name(&self) -> &'static str {
        match self {
            RoomShape::IrregularCave => "Irregular Cave",
            RoomShape::Oval => "Oval Chamber",
            RoomShape::CrossShaped => "Cross-shaped Crypt",
            RoomShape::Corridor => "Narrow Corridor",
            RoomShape::Square => "Square Room",
            RoomShape::Round => "Round Vault",
            RoomShape::Rectangular => "Rectangular Hall",
            RoomShape::Triangular => "Triangular Alcove",
            RoomShape::SkullShaped => "Skull-shaped Catacomb",
        }
    }

    /// Roll 2d6 on Room Shape Table.
    pub fn roll<R: DiceRoller>(roller: &mut R) -> Self {
        match roller.roll_formula(DiceFormula::TwoD6) {
            2 => RoomShape::IrregularCave,
            3 => RoomShape::Oval,
            4 => RoomShape::CrossShaped,
            5 => RoomShape::Corridor,
            6..=8 => RoomShape::Square,
            9 => RoomShape::Round,
            10 => RoomShape::Rectangular,
            11 => RoomShape::Triangular,
            _ => RoomShape::SkullShaped,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorStatus {
    Unopened,
    Connected(RoomId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Door {
    pub direction: Direction,
    pub status: DoorStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomStatus {
    Unexplored,
    Explored,
    FledFrom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PitTrapResult {
    Safe,
    Damaged(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoothsayerChoice {
    TenSilver,
    ThreePoints,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomContent {
    Empty,
    ItemStash(Item),
    MysticScroll(Item),
    WeakMonsterEncounter(Monster),
    ToughMonsterEncounter(Monster),
    PitTrap,
    RiddlingSoothsayer,
    Peddler,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomNode {
    pub id: RoomId,
    pub is_entrance: bool,
    pub shape: RoomShape,
    pub coordinates: (i32, i32),
    pub doors: Vec<Door>,
    pub status: RoomStatus,
    pub content: RoomContent,
}

impl RoomNode {
    pub fn has_unopened_doors(&self) -> bool {
        self.doors.iter().any(|d| matches!(d.status, DoorStatus::Unopened))
    }

    pub fn unopened_directions(&self) -> Vec<Direction> {
        self.doors
            .iter()
            .filter_map(|d| match d.status {
                DoorStatus::Unopened => Some(d.direction),
                _ => None,
            })
            .collect()
    }

    pub fn connect_door(&mut self, dir: Direction, target: RoomId) -> bool {
        for door in &mut self.doors {
            if door.direction == dir {
                door.status = DoorStatus::Connected(target);
                return true;
            }
        }
        false
    }

    pub fn get_connected_room(&self, dir: Direction) -> Option<RoomId> {
        for door in &self.doors {
            if door.direction == dir {
                if let DoorStatus::Connected(id) = door.status {
                    return Some(id);
                }
            }
        }
        None
    }
}
