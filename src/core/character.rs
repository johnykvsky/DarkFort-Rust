use crate::core::config::EngineType;
use crate::core::dice::{DiceFormula, DiceRoller};
use crate::core::equipment::{Item, ScrollType, WeaponType};
use crate::core::monsters::{ToughMonsterType, WeakMonsterType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterVitals {
    Classic {
        hp: i32,
        max_hp: i32,
    },
    Mothership {
        health: i32,
        max_health: i32,
        wounds: i32,
        max_wounds: i32,
        stress: i32,
        max_stress: i32,
        combat: i32,
        strength: i32,
        agility: i32,
        occult: i32,
        armor_points: i32,
    },
    MorkBorg {
        hp: i32,
        max_hp: i32,
        omens: i32,
        max_omens: i32,
        strength: i8,
        agility: i8,
        presence: i8,
        toughness: i8,
        armor_tier: u8,
    },
    Durf {
        armor_pool: i32,
        max_armor: i32,
        wounds: i32,
        hit_dice: i32,
        force: i8,
        finesse: i8,
        will: i8,
        shield_soak: i32,
        is_dead: bool,
    },
    Adnd2e {
        thac0: i32,
        ac: i32,
        hp: i32,
        max_hp: i32,
        strength: u8,
        dexterity: u8,
        constitution: u8,
        intelligence: u8,
        wisdom: u8,
        charisma: u8,
        save_death_poison: u8,
        save_petrify: u8,
        save_spell: u8,
    },
}

impl Default for CharacterVitals {
    fn default() -> Self {
        Self::Classic { hp: 15, max_hp: 15 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub title: Option<String>,
    #[serde(default)]
    pub engine_type: EngineType,
    #[serde(default)]
    pub vitals: CharacterVitals,
    pub hp: i32,
    pub max_hp: i32,
    pub silver: i32,
    pub points: i32,
    pub level: u32,
    pub weapon: WeaponType,
    pub has_armor: bool,
    #[serde(default)]
    pub armor_formula: Option<DiceFormula>,
    pub inventory: Vec<Item>,
    pub permanent_attack_bonus: i32,
    pub halved_weak_monster: Option<WeakMonsterType>,
    pub halved_tough_monster: Option<ToughMonsterType>,
    pub active_daemon_fights_left: u32,
    pub active_aegis_uses_left: u32,
    pub rooms_explored: u32,
}

impl Character {
    /// Create Kargunt with engine-specific character profile.
    pub fn new_with_engine<R: DiceRoller>(engine_type: EngineType, roller: &mut R) -> Self {
        Self::new_with_engine_and_theme(engine_type, false, roller)
    }

    /// Create character with engine-specific profile and optional sci-fi derelict ship theme.
    pub fn new_with_engine_and_theme<R: DiceRoller>(
        engine_type: EngineType,
        is_scifi: bool,
        roller: &mut R,
    ) -> Self {
        if is_scifi {
            let scifi_items = crate::core::registry::ItemRegistry::default_scifi_embedded();
            let starting_weapon = scifi_items.roll_starting_weapon(roller);
            let initial_silver = 25 + roller.roll_d6() * 2;
            let mut character = Self {
                name: "Kargunt".to_string(),
                title: Some("Warrant Officer (Colonial Marine)".to_string()),
                engine_type,
                vitals: CharacterVitals::Classic { hp: 15, max_hp: 15 },
                hp: 15,
                max_hp: 15,
                silver: initial_silver,
                points: 0,
                level: 0,
                weapon: starting_weapon,
                has_armor: true,
                armor_formula: Some(DiceFormula::D4),
                inventory: vec![
                    Item::CustomPotion {
                        id: "stimm_pak".to_string(),
                        name: "Combat Stimm-Pak".to_string(),
                        heal_formula: DiceFormula::D6,
                        price: 15,
                    },
                    Item::CustomGear {
                        id: "mag_boots".to_string(),
                        name: "Magnetic Boots".to_string(),
                        gear_kind: crate::core::registry::GearKind::Rope,
                        uses_remaining: None,
                        price: 10,
                    },
                ],
                permanent_attack_bonus: 0,
                halved_weak_monster: None,
                halved_tough_monster: None,
                active_daemon_fights_left: 0,
                active_aegis_uses_left: 0,
                rooms_explored: 0,
            };

            if engine_type == EngineType::Mothership {
                let max_health = 15;
                character.hp = max_health;
                character.max_hp = max_health;
                character.title = Some("Warrant Officer Kargunt (Colonial Marine)".to_string());
                character.vitals = CharacterVitals::Mothership {
                    health: max_health,
                    max_health,
                    wounds: 0,
                    max_wounds: 2,
                    stress: 2,
                    max_stress: 20,
                    combat: 45,
                    strength: 35,
                    agility: 40,
                    occult: 25,
                    armor_points: 3,
                };
            }

            return character;
        }

        let initial_silver = 15 + roller.roll_d6();
        let starting_weapon = WeaponType::roll_starting(roller);
        let starting_item = Item::roll_starting(roller);

        let mut character = Self {
            name: "Kargunt".to_string(),
            title: None,
            engine_type,
            vitals: CharacterVitals::Classic { hp: 15, max_hp: 15 },
            hp: 15,
            max_hp: 15,
            silver: initial_silver,
            points: 0,
            level: 0,
            weapon: starting_weapon,
            has_armor: false,
            armor_formula: None,
            inventory: Vec::new(),
            permanent_attack_bonus: 0,
            halved_weak_monster: None,
            halved_tough_monster: None,
            active_daemon_fights_left: 0,
            active_aegis_uses_left: 0,
            rooms_explored: 0,
        };

        match starting_item {
            Item::Armor => {
                character.has_armor = true;
                character.armor_formula = Some(DiceFormula::D4);
            }
            Item::CustomArmor { absorption_formula, .. } => {
                character.has_armor = true;
                character.armor_formula = Some(absorption_formula);
            }
            Item::Scroll(ScrollType::SummonWeakDaemon { fights_remaining }) => {
                character.inventory.push(Item::Scroll(ScrollType::SummonWeakDaemon { fights_remaining }));
            }
            other => {
                character.inventory.push(other);
            }
        }

        match engine_type {
            EngineType::Classic => {
                character.vitals = CharacterVitals::Classic {
                    hp: character.hp,
                    max_hp: character.max_hp,
                };
            }
            EngineType::Mothership => {
                let max_health = 14;
                let ap = if character.has_armor { 3 } else { 0 };
                character.hp = max_health;
                character.max_hp = max_health;
                character.title = Some("Kargunt the Grave-Robber".to_string());
                character.vitals = CharacterVitals::Mothership {
                    health: max_health,
                    max_health,
                    wounds: 0,
                    max_wounds: 2,
                    stress: 2,
                    max_stress: 20,
                    combat: 40,
                    strength: 35,
                    agility: 45,
                    occult: 25,
                    armor_points: ap,
                };
            }
            EngineType::MorkBorg => {
                let max_hp = 8;
                let tier = if character.has_armor { 1 } else { 0 };
                character.hp = max_hp;
                character.max_hp = max_hp;
                character.title = Some("Kargunt the Gutter Born Scum".to_string());
                character.vitals = CharacterVitals::MorkBorg {
                    hp: max_hp,
                    max_hp,
                    omens: 2,
                    max_omens: 2,
                    strength: -1,
                    agility: 2,
                    presence: 0,
                    toughness: 1,
                    armor_tier: tier,
                };
            }
            EngineType::Durf => {
                let armor = if character.has_armor { 3 } else { 1 };
                character.hp = armor;
                character.max_hp = armor;
                character.title = Some("Kargunt the Catacomb Delver".to_string());
                character.vitals = CharacterVitals::Durf {
                    armor_pool: armor,
                    max_armor: armor,
                    wounds: 0,
                    hit_dice: 1,
                    force: 2,
                    finesse: 1,
                    will: 0,
                    shield_soak: 1,
                    is_dead: false,
                };
            }
            EngineType::Adnd2e => {
                let max_hp = 6;
                let base_ac = if character.has_armor { 8 } else { 10 };
                let dex_mod = 2; // DEX 16 grants -2 AC bonus in AD&D 2e
                let final_ac = base_ac - dex_mod;

                character.hp = max_hp;
                character.max_hp = max_hp;
                character.title = Some("Kargunt the Shadow Thief".to_string());
                character.vitals = CharacterVitals::Adnd2e {
                    thac0: 20,
                    ac: final_ac,
                    hp: max_hp,
                    max_hp,
                    strength: 14,
                    dexterity: 16,
                    constitution: 14,
                    intelligence: 12,
                    wisdom: 10,
                    charisma: 11,
                    save_death_poison: 13,
                    save_petrify: 12,
                    save_spell: 15,
                };
            }
        }

        character
    }

    /// Create Kargunt with starting stats and rolls (default Classic engine).
    pub fn new_kargunt<R: DiceRoller>(roller: &mut R) -> Self {
        Self::new_with_engine(EngineType::Classic, roller)
    }

    /// Total attack bonus = weapon attack bonus (Dagger/Sword +1) + permanent bonus (+1 from Level Up #2).
    pub fn total_attack_bonus(&self) -> i32 {
        self.weapon.attack_bonus() + self.permanent_attack_bonus
    }

    /// Heal by amount, bounded by max_hp.
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
        match &mut self.vitals {
            CharacterVitals::Classic { hp, max_hp } => *hp = self.hp.min(*max_hp),
            CharacterVitals::Mothership { health, max_health, stress, .. } => {
                *health = self.hp.min(*max_health);
                *stress = (*stress - 2).max(2);
            }
            CharacterVitals::MorkBorg { hp, max_hp, .. } => *hp = self.hp.min(*max_hp),
            CharacterVitals::Durf { armor_pool, max_armor, .. } => *armor_pool = (*armor_pool + amount).min(*max_armor),
            CharacterVitals::Adnd2e { hp, max_hp, .. } => *hp = self.hp.min(*max_hp),
        }
    }

    /// Take raw damage, applying armor roll if equipped, and Aegis of Sorrow (-1d4) if active.
    /// Returns the net damage taken, armor absorbed, and aegis absorbed.
    pub fn take_damage<R: DiceRoller>(
        &mut self,
        raw_damage: i32,
        ignore_armor: bool,
        roller: &mut R,
    ) -> (i32, i32, i32) {
        if self.engine_type == EngineType::Mothership {
            let ap = if !ignore_armor && self.has_armor { 3 } else { 0 };
            let net_damage = (raw_damage - ap).max(0);
            self.hp -= net_damage;

            if let CharacterVitals::Mothership {
                health,
                max_health,
                wounds,
                max_wounds,
                stress,
                max_stress,
                ..
            } = &mut self.vitals
            {
                *health = self.hp;
                if self.hp <= 0 {
                    *wounds += 1;
                    *stress = (*stress + 1).min(*max_stress);
                    if *wounds < *max_wounds {
                        self.hp = *max_health;
                        *health = *max_health;
                    } else {
                        self.hp = 0;
                        *health = 0;
                    }
                }
            }

            return (net_damage, ap, 0);
        } else if self.engine_type == EngineType::Durf {
            let shield = if !ignore_armor { 1 } else { 0 };
            let post_shield = (raw_damage - shield).max(1);

            let mut armor_absorbed = 0;
            let mut wounds_taken = 0;

            if let CharacterVitals::Durf {
                armor_pool,
                wounds,
                hit_dice,
                is_dead,
                ..
            } = &mut self.vitals
            {
                if !ignore_armor && *armor_pool > 0 {
                    if *armor_pool >= post_shield {
                        *armor_pool -= post_shield;
                        armor_absorbed = post_shield;
                    } else {
                        armor_absorbed = *armor_pool;
                        let excess = post_shield - *armor_pool;
                        *armor_pool = 0;
                        *wounds += excess;
                        wounds_taken = excess;
                    }
                } else {
                    *wounds += post_shield;
                    wounds_taken = post_shield;
                }

                self.hp = *armor_pool;

                if wounds_taken > 0 {
                    let mut hd_sum = 0;
                    for _ in 0..*hit_dice {
                        hd_sum += roller.roll_d6();
                    }
                    if hd_sum <= *wounds {
                        *is_dead = true;
                        self.hp = 0;
                    }
                }
            }

            return (post_shield, armor_absorbed + shield, 0);
        } else if self.engine_type == EngineType::MorkBorg {
            let mut armor_absorbed = 0;
            if !ignore_armor && self.has_armor {
                armor_absorbed = roller.roll_d2();
            }
            let net_damage = (raw_damage - armor_absorbed).max(0);
            self.hp -= net_damage;
            if let CharacterVitals::MorkBorg { hp, .. } = &mut self.vitals {
                *hp = self.hp;
            }
            return (net_damage, armor_absorbed, 0);
        } else if self.engine_type == EngineType::Adnd2e {
            self.hp -= raw_damage;
            if let CharacterVitals::Adnd2e { hp, .. } = &mut self.vitals {
                *hp = self.hp;
            }
            return (raw_damage, 0, 0);
        }

        let mut armor_absorbed = 0;
        let mut aegis_absorbed = 0;

        if !ignore_armor && self.has_armor {
            let formula = self.armor_formula.unwrap_or(DiceFormula::D4);
            armor_absorbed = roller.roll_formula(formula).max(0);
        }

        if !ignore_armor && self.active_aegis_uses_left > 0 {
            aegis_absorbed = roller.roll_d4();
            self.active_aegis_uses_left -= 1;
        }

        let total_absorbed = armor_absorbed + aegis_absorbed;
        let net_damage = (raw_damage - total_absorbed).max(0);
        self.hp -= net_damage;

        (net_damage, armor_absorbed, aegis_absorbed)
    }

    pub fn is_alive(&self) -> bool {
        match &self.vitals {
            CharacterVitals::Classic { .. } => self.hp > 0,
            CharacterVitals::Mothership { wounds, max_wounds, .. } => *wounds < *max_wounds && self.hp > 0,
            CharacterVitals::MorkBorg { hp, .. } => *hp > 0,
            CharacterVitals::Durf { is_dead, .. } => !*is_dead,
            CharacterVitals::Adnd2e { hp, .. } => *hp > 0,
        }
    }

    /// Check if character has a rope for +1 on pit trap rolls.
    pub fn has_rope(&self) -> bool {
        self.inventory.iter().any(|i| match i {
            Item::Rope => true,
            Item::CustomGear {
                gear_kind: crate::core::registry::GearKind::Rope,
                ..
            } => true,
            _ => false,
        })
    }

    /// Check if character has usable Cloak of Invisibility.
    pub fn has_cloak(&self) -> bool {
        self.inventory.iter().any(|i| match i {
            Item::CloakOfInvisibility { uses_remaining } => *uses_remaining > 0,
            Item::CustomGear {
                gear_kind: crate::core::registry::GearKind::CloakOfInvisibility,
                uses_remaining: Some(uses),
                ..
            } => *uses > 0,
            _ => false,
        })
    }

    /// Use one charge of the Cloak of Invisibility to bypass a fight.
    pub fn use_cloak_charge(&mut self) -> bool {
        for item in &mut self.inventory {
            match item {
                Item::CloakOfInvisibility { uses_remaining } => {
                    if *uses_remaining > 0 {
                        *uses_remaining -= 1;
                        return true;
                    }
                }
                Item::CustomGear {
                    gear_kind: crate::core::registry::GearKind::CloakOfInvisibility,
                    uses_remaining: Some(uses),
                    ..
                } => {
                    if *uses > 0 {
                        *uses -= 1;
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Consume a potion from inventory to heal HP.
    pub fn drink_potion<R: DiceRoller>(&mut self, roller: &mut R) -> Option<i32> {
        let potion_idx = self
            .inventory
            .iter()
            .position(|i| matches!(i, Item::Potion | Item::CustomPotion { .. }));

        if let Some(pos) = potion_idx {
            let item = self.inventory.remove(pos);
            let healed = match item {
                Item::Potion => roller.roll_d6(),
                Item::CustomPotion { heal_formula, .. } => roller.roll_formula(heal_formula).max(1),
                _ => 0,
            };
            self.heal(healed);
            Some(healed)
        } else {
            None
        }
    }
}
