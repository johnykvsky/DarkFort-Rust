use dark_fort::core::character::Character;
use dark_fort::core::combat::{CombatEngine, RoundOutcome};
use dark_fort::core::dice::{DiceFormula, MockRoller};
use dark_fort::core::equipment::{Item, WeaponType};
use dark_fort::core::monsters::{Monster, MonsterCategory, MonsterLoot, PostFightHazard};
use dark_fort::core::registry::{EnemyRegistry, ItemRegistry, ShopConfig};
use std::fs;

#[test]
fn test_default_registry_loads_correctly() {
    let registry = ItemRegistry::load_or_default();

    // Verify all baseline equipment categories exist
    assert_eq!(registry.weapons.len(), 6);
    assert_eq!(registry.armours.len(), 1);
    assert_eq!(registry.potions.len(), 1);
    assert_eq!(registry.scrolls.len(), 4);
    assert_eq!(registry.gear.len(), 2);

    // Verify weapon index lookup
    let dagger = registry.get_weapon("dagger").expect("dagger should exist");
    assert_eq!(dagger.name, "Dagger");
    assert_eq!(dagger.attack_bonus, 1);
    assert_eq!(dagger.damage_formula, DiceFormula::D4);
}

#[test]
fn test_custom_items_loaded_from_json_files() {
    let temp_dir = std::env::temp_dir().join("dark_fort_custom_items_test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let custom_weapons_json = r#"[
        {
            "id": "sun_blade",
            "name": "Sun Blade",
            "damage_formula": "2d6",
            "attack_bonus": 2,
            "price": 40,
            "is_starting": true,
            "description": "A radiant sword forged from fallen star-metal."
        }
    ]"#;

    let custom_armours_json = r#"[
        {
            "id": "obsidian_plate",
            "name": "Obsidian Plate",
            "absorption_formula": "1d6",
            "price": 30,
            "is_starting": true,
            "description": "Heavy interlocking plates carved from volcanic glass."
        }
    ]"#;

    let custom_potions_json = r#"[
        {
            "id": "greater_healing_draught",
            "name": "Greater Healing Draught",
            "heal_formula": "2d6",
            "price": 12,
            "is_starting": true,
            "description": "A glowing tincture that mends cracked bones."
        }
    ]"#;

    let custom_scrolls_json = r#"[
        {
            "id": "scroll_false_omen",
            "name": "Scroll: False Omen",
            "scroll_kind": "FalseOmen",
            "charges_formula": null,
            "price": 7,
            "is_starting": true,
            "description": "Bend fate to avert catastrophe."
        }
    ]"#;

    let custom_gear_json = r#"[
        {
            "id": "elven_silk_rope",
            "name": "Elven Silk Rope",
            "gear_kind": "Rope",
            "charges_formula": null,
            "price": 8,
            "is_starting": true,
            "description": "Featherweight yet impossible to snap."
        }
    ]"#;

    fs::write(temp_dir.join("weapons.json"), custom_weapons_json).unwrap();
    fs::write(temp_dir.join("armours.json"), custom_armours_json).unwrap();
    fs::write(temp_dir.join("potions.json"), custom_potions_json).unwrap();
    fs::write(temp_dir.join("scrolls.json"), custom_scrolls_json).unwrap();
    fs::write(temp_dir.join("gear.json"), custom_gear_json).unwrap();

    let registry = ItemRegistry::load_from_dir(&temp_dir).expect("Should load custom registry");

    assert_eq!(registry.weapons.len(), 1);
    assert_eq!(registry.weapons[0].name, "Sun Blade");
    assert_eq!(registry.armours.len(), 1);
    assert_eq!(registry.armours[0].name, "Obsidian Plate");
    assert_eq!(registry.potions.len(), 1);
    assert_eq!(registry.potions[0].name, "Greater Healing Draught");
    assert_eq!(registry.gear.len(), 1);
    assert_eq!(registry.gear[0].name, "Elven Silk Rope");

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_custom_weapon_and_combat_calculations() {
    let custom_weapon = WeaponType::Custom {
        id: "blood_scythe".to_string(),
        name: "Blood Scythe".to_string(),
        damage_formula: DiceFormula::D6Plus2,
        attack_bonus: 2,
        price: 35,
    };

    assert_eq!(custom_weapon.name(), "Blood Scythe");
    assert_eq!(custom_weapon.attack_bonus(), 2);
    assert_eq!(custom_weapon.damage_formula(), DiceFormula::D6Plus2);

    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.weapon = custom_weapon;

    assert_eq!(kargunt.total_attack_bonus(), 2);

    let mut goblin = Monster {
        name: "Goblin".to_string(),
        category: MonsterCategory::Weak,
        points_to_hit: 5,
        points_awarded: 1,
        hp: 4,
        max_hp: 4,
        damage_formula: DiceFormula::D4Minus1,
        attack_count: 0,
        loot: MonsterLoot::None,
        post_hazard: PostFightHazard::None,
    };

    // Attack roll: d6 roll 3 + 2 (weapon attack bonus) = 5 >= 5 (Hit!)
    // Weapon damage: d6 roll 4 + 2 = 6 damage -> kills Goblin (4 HP)!
    let mut combat_mock = MockRoller::new(vec![], vec![3, 4]);
    let outcome = CombatEngine::resolve_attack_round(&mut kargunt, &mut goblin, &mut combat_mock);

    match outcome {
        RoundOutcome::PlayerHit { weapon_damage, monster_hp_left, .. } => {
            assert_eq!(weapon_damage, 6);
            assert_eq!(monster_hp_left, -2);
            assert!(goblin.hp <= 0);
        }
        other => panic!("Expected PlayerHit, got {:?}", other),
    }
}

#[test]
fn test_custom_armor_absorption_and_potion_quaffing() {
    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);

    // Equip custom armor with 1d6 absorption
    kargunt.has_armor = true;
    kargunt.armor_formula = Some(DiceFormula::D6);

    // Incoming 7 damage. Armor rolls d6 of 5 -> net damage 2
    let mut roll_mock = MockRoller::new(vec![], vec![5]);
    let (net_dmg, absorbed, aegis) = kargunt.take_damage(7, false, &mut roll_mock);
    assert_eq!(absorbed, 5);
    assert_eq!(aegis, 0);
    assert_eq!(net_dmg, 2);
    assert_eq!(kargunt.hp, 13);

    // Add and drink custom potion with 2d6 heal
    kargunt.inventory.push(Item::CustomPotion {
        id: "elixir_of_life".to_string(),
        name: "Elixir of Life".to_string(),
        heal_formula: DiceFormula::TwoD6,
        price: 20,
    });

    // Drink potion: rolls 2d6 (e.g. 1 and 1 = 2) -> recovers 2 HP up to max 15
    let mut pot_mock = MockRoller::new(vec![], vec![1, 1]);
    let healed = kargunt.drink_potion(&mut pot_mock);
    assert_eq!(healed, Some(2));
    assert_eq!(kargunt.hp, 15);
}

#[test]
fn test_custom_item_serde_roundtrip() {
    let custom_weapon = WeaponType::Custom {
        id: "meteor_hammer".to_string(),
        name: "Meteor Hammer".to_string(),
        damage_formula: DiceFormula::D6Plus1,
        attack_bonus: 0,
        price: 18,
    };

    let custom_armor = Item::CustomArmor {
        id: "dwarven_chainmail".to_string(),
        name: "Dwarven Chainmail".to_string(),
        absorption_formula: DiceFormula::D4,
        price: 15,
    };

    let custom_gear = Item::CustomGear {
        id: "shadow_cloak".to_string(),
        name: "Shadow Cloak".to_string(),
        gear_kind: dark_fort::core::registry::GearKind::CloakOfInvisibility,
        uses_remaining: Some(3),
        price: 25,
    };

    let mut mock = MockRoller::new(vec![1, 1], vec![1]);
    let mut kargunt = Character::new_kargunt(&mut mock);
    kargunt.weapon = custom_weapon;
    kargunt.inventory.push(custom_armor);
    kargunt.inventory.push(custom_gear);

    let serialized = serde_json::to_string(&kargunt).expect("Serialization must succeed");
    let deserialized: Character = serde_json::from_str(&serialized).expect("Deserialization must succeed");

    assert_eq!(kargunt, deserialized);
    assert_eq!(deserialized.weapon.name(), "Meteor Hammer");
    assert!(deserialized.has_cloak());
}

#[test]
fn test_default_enemies_and_shop_config_load_correctly() {
    let enemies = EnemyRegistry::load_or_default();
    assert_eq!(enemies.weak_monsters.len(), 4);
    assert_eq!(enemies.tough_monsters.len(), 4);

    let cultist = enemies.get_weak("catacomb_cultist").expect("catacomb_cultist should exist");
    assert_eq!(cultist.name, "Catacomb Cultist");
    assert_eq!(cultist.hp, 6);
    assert_eq!(cultist.points_to_hit, 3);
    assert_eq!(cultist.loot, MonsterLoot::ScrollChance);

    let basilisk = enemies.get_tough("ruin_basilisk").expect("ruin_basilisk should exist");
    assert_eq!(basilisk.name, "Ruin Basilisk");
    assert_eq!(basilisk.hp, 11);
    assert_eq!(basilisk.post_hazard, PostFightHazard::ImmediateLevelUp);

    let shop = ShopConfig::load_or_default();
    assert_eq!(shop.wares.len(), 3);
    assert_eq!(shop.wares[0].name, "Potion");
    assert_eq!(shop.wares[0].price, 4);
    assert_eq!(shop.wares[1].name, "Scroll");
    assert_eq!(shop.wares[1].price, 7);
    assert_eq!(shop.wares[2].name, "Armor");
    assert_eq!(shop.wares[2].price, 10);

    assert_eq!(shop.get_price("dagger"), Some(6));
    assert_eq!(shop.get_price("flail"), Some(15));
    assert_eq!(shop.get_price("mighty_zweihaender"), Some(25));
}

#[test]
fn test_custom_enemies_loaded_from_json() {
    let temp_dir = std::env::temp_dir().join("dark_fort_custom_enemies_test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let custom_weak_json = r#"[
        {
            "id": "shadow_goblin",
            "name": "Shadow Goblin",
            "category": "Weak",
            "points_to_hit": 4,
            "points_awarded": 3,
            "hp": 5,
            "damage_formula": "1d4",
            "loot": "RopeChance",
            "post_hazard": "None",
            "description": "A stealthy goblin cloaked in living darkness."
        }
    ]"#;

    let custom_tough_json = r#"[
        {
            "id": "gargoyle",
            "name": "Ancient Gargoyle",
            "category": "Tough",
            "points_to_hit": 5,
            "points_awarded": 6,
            "hp": 12,
            "damage_formula": "1d6+1",
            "loot": "None",
            "post_hazard": "Petrification",
            "description": "Winged stone abomination."
        }
    ]"#;

    fs::write(temp_dir.join("weak.json"), custom_weak_json).unwrap();
    fs::write(temp_dir.join("tough.json"), custom_tough_json).unwrap();

    let registry = EnemyRegistry::load_from_dir(&temp_dir).expect("Should load custom enemy registry");
    assert_eq!(registry.weak_monsters.len(), 1);
    assert_eq!(registry.weak_monsters[0].name, "Shadow Goblin");
    assert_eq!(registry.tough_monsters.len(), 1);
    assert_eq!(registry.tough_monsters[0].name, "Ancient Gargoyle");

    let mut mock = MockRoller::new(vec![1], vec![]);
    let monster = registry.roll_weak(&mut mock);
    assert_eq!(monster.name, "Shadow Goblin");
    assert_eq!(monster.hp, 5);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_custom_shop_config_and_item_creation() {
    let temp_dir = std::env::temp_dir().join("dark_fort_custom_shop_test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let custom_shop_json = r#"{
        "wares": [
            {
                "key": "1",
                "name": "Cheap Potion",
                "item_type": "potion",
                "item_id": "healing_potion",
                "price": 2
            },
            {
                "key": "4",
                "name": "Rope",
                "item_type": "gear",
                "item_id": "rope",
                "price": 3
            }
        ],
        "prices": {
            "cheap_potion": 2,
            "rope": 3
        }
    }"#;

    let shop_file = temp_dir.join("shop.json");
    fs::write(&shop_file, custom_shop_json).unwrap();

    let shop_config = ShopConfig::load_from_file(&shop_file).expect("Should load custom shop config");
    assert_eq!(shop_config.wares.len(), 2);
    assert_eq!(shop_config.wares[0].name, "Cheap Potion");
    assert_eq!(shop_config.wares[0].price, 2);
    assert_eq!(shop_config.wares[1].name, "Rope");
    assert_eq!(shop_config.wares[1].price, 3);
    assert_eq!(shop_config.get_price("cheap_potion"), Some(2));

    let items_registry = ItemRegistry::load_or_default();
    let mut mock = MockRoller::new(vec![], vec![]);
    let bought_item = shop_config.wares[1].create_item(&items_registry, &mut mock);
    assert_eq!(bought_item, Item::Rope);

    let _ = fs::remove_dir_all(&temp_dir);
}
