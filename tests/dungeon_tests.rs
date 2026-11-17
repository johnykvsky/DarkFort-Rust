use dark_fort::core::dice::MockRoller;
use dark_fort::dungeon::map::{DungeonGraph, NavigationEvent};
use dark_fort::dungeon::room::{Direction, DoorStatus, RoomContent, RoomId, RoomShape};

#[test]
fn test_entrance_generation_empty() {
    // d4 roll 4 -> Entrance has 4 doors and is empty (eerily quiet)
    let mut mock = MockRoller::new(vec![4], vec![]);
    let dungeon = DungeonGraph::new(&mut mock);

    let entrance = dungeon.current_room();
    assert!(entrance.is_entrance);
    assert_eq!(entrance.id, RoomId(0));
    assert_eq!(entrance.doors.len(), 4);
    assert_eq!(entrance.content, RoomContent::Empty);
    assert_eq!(entrance.coordinates, (0, 0));
    assert!(entrance.has_unopened_doors());
}

#[test]
fn test_entrance_generation_with_guard() {
    // d4 roll 2 -> Entrance has 2 doors and a WEAK monster stands guard!
    // Next d4 roll for Monster::roll_weak: roll 1 (Blood-drenched Skeleton)
    let mut mock = MockRoller::new(vec![2, 1], vec![]);
    let dungeon = DungeonGraph::new(&mut mock);

    let entrance = dungeon.current_room();
    assert_eq!(entrance.doors.len(), 2);
    match &entrance.content {
        RoomContent::WeakMonsterEncounter(monster) => {
            assert_eq!(monster.name, "Blood-drenched Skeleton");
        }
        _ => panic!("Expected WeakMonsterEncounter"),
    }
}

#[test]
fn test_procedural_room_generation_and_traversal() {
    // Entrance: roll 2 doors (North, East), empty content (roll 4)
    let mut mock = MockRoller::new(vec![4], vec![]);
    let mut dungeon = DungeonGraph::new(&mut mock);

    let entrance_door_dir = dungeon.current_room().doors[0].direction; // North

    // Next room generation rolls:
    // Shape 2d6: roll 6, 2 -> 8 (Square)
    // Content 1d6: roll 1 (Empty)
    // Doors 1d4: roll 3 (2 further doors)
    let mut step_mock = MockRoller::new(vec![3], vec![6, 2, 1]);
    let event = dungeon.traverse_door(entrance_door_dir, None, &mut step_mock).unwrap();

    assert_eq!(event, NavigationEvent::EnteredNewRoom(RoomId(1)));
    assert_eq!(dungeon.current_room_id, RoomId(1));

    let new_room = dungeon.current_room();
    assert_eq!(new_room.shape, RoomShape::Square);
    assert_eq!(new_room.coordinates, (0, -1)); // North delta
    assert_eq!(new_room.content, RoomContent::Empty);

    // Should have door back to South connected to RoomId(0)
    let back_door = new_room.doors.iter().find(|d| d.direction == Direction::South).unwrap();
    assert_eq!(back_door.status, DoorStatus::Connected(RoomId(0)));

    // Should have 2 further unopened doors
    let unopened_count = new_room.doors.iter().filter(|d| matches!(d.status, DoorStatus::Unopened)).count();
    assert_eq!(unopened_count, 2);
}

#[test]
fn test_backtracking_and_ambush_chance() {
    let mut mock = MockRoller::new(vec![4], vec![]);
    let mut dungeon = DungeonGraph::new(&mut mock);

    let dir = dungeon.current_room().doors[0].direction;

    // Traverse North into new room: shape 2d6 (3+3=6), content 1d6 (1), doors 1d4 (2 = 1 door)
    let mut gen_mock = MockRoller::new(vec![2], vec![3, 3, 1]);
    dungeon.traverse_door(dir, None, &mut gen_mock).unwrap();
    dungeon.mark_current_explored();
    assert_eq!(dungeon.explored_room_count, 1);

    // Backtrack to South:
    // Ambush check: 1d4 roll 1 -> Ambush occurs!
    // Next d4 for monster roll: roll 3 (Goblin)
    let mut ambush_mock = MockRoller::new(vec![1, 3], vec![]);
    let back_event = dungeon.traverse_door(Direction::South, None, &mut ambush_mock).unwrap();

    match back_event {
        NavigationEvent::WanderingMonsterAmbush(m) => {
            assert_eq!(m.name, "Goblin");
        }
        _ => panic!("Expected WanderingMonsterAmbush"),
    }
    assert_eq!(dungeon.current_room_id, RoomId(0));

    // Backtrack North again:
    // Ambush check: 1d4 roll 3 -> No ambush!
    let mut safe_mock = MockRoller::new(vec![3], vec![]);
    let safe_event = dungeon.traverse_door(dir, None, &mut safe_mock).unwrap();
    assert_eq!(safe_event, NavigationEvent::BacktrackedToRoom(RoomId(1)));
}

#[test]
fn test_false_omen_override() {
    let mut mock = MockRoller::new(vec![4], vec![]);
    let mut dungeon = DungeonGraph::new(&mut mock);
    let dir = dungeon.current_room().doors[0].direction;

    // Player uses False Omen scroll to force room table result #6 (Peddler)
    let mut gen_mock = MockRoller::new(vec![1], vec![3, 3]); // door roll 1 (0 doors), shape (Square)
    dungeon.traverse_door(dir, Some(6), &mut gen_mock).unwrap();

    let room = dungeon.current_room();
    assert_eq!(room.content, RoomContent::Peddler);
}

#[test]
fn test_dead_end_detection() {
    // Entrance with 1 door (d4 roll 1)
    let mut mock = MockRoller::new(vec![1], vec![]);
    let mut dungeon = DungeonGraph::new(&mut mock);
    let dir = dungeon.current_room().doors[0].direction;

    assert!(dungeon.has_any_unopened_doors());

    // Next room rolls door roll 1 -> No further doors! (Dead end)
    let mut gen_mock = MockRoller::new(vec![1], vec![3, 3, 1]);
    dungeon.traverse_door(dir, None, &mut gen_mock).unwrap();

    // Now current room has only back door (Connected), entrance has only connected door.
    // Total unopened doors across dungeon = 0!
    assert!(!dungeon.has_any_unopened_doors());
}
