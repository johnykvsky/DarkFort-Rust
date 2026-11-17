# Dark Fort — Complete Rules Reference & Equipment Guide

This reference document compiles the complete, definitive rules of **Dark Fort** (2018) by Pelle Nilsson, David Hoskins, and Johan Nohr (Stockholm Kartell), including equipment tables, scroll mechanics, monster profiles, and character progression systems.

---

## 1. Character: Kargunt (The Catacomb Rogue)

- **Initial Hit Points (HP)**: `15` (Max HP: 15; can increase permanently to 20 via Level Up #3).
- **Initial Silver (s)**: `15 + 1d6`.
- **Inventory Capacity**: Unlimited item slots.
- **Starting Equipment**:
  - **1 Weapon** (Roll 1d4 on Table A):
    1. **Warhammer** (`1d6` damage)
    2. **Dagger** (`1d4` damage, **+1 attack bonus**)
    3. **Sword** (`1d6` damage, **+1 attack bonus**)
    4. **Flail** (`1d6 + 1` damage)
  - **1 Item / Relic** (Roll 1d4 on Table B):
    1. **Armor** (`-1d4` damage absorbed when hit)
    2. **Healing Potion** (heals `1d6` HP)
    3. **Scroll: Summon Weak Daemon** (`1d4` fights, `+1d4` damage)
    4. **Cloak of Invisibility** (`1d4` uses)

> *Rule Note on Attack Bonuses*: Weapons with an attack bonus (Dagger, Sword) add `+1` to the **attack roll** to determine whether you hit the monster's Defense Points. They do **not** add bonus damage to the damage roll itself.

---

## 2. Equipment & Gear Catalog

> **Data-Driven Equipment**: All weapons, armor, potions, scrolls, and utility gear are loaded at startup from `data/items/*.json`. New items can be added or modified directly in those JSON files without modifying the Rust codebase.

### 2.1 Complete Weapons Table

| Weapon | Damage Formula | Attack Roll Bonus | Peddler Value (Buy/Sell) | Description |
| :--- | :---: | :---: | :---: | :--- |
| **Dagger** | `1d4` | **+1** | 6 silver | Light, concealable blade; strikes quickly |
| **Warhammer** | `1d6` | +0 | 9 silver | Heavy bludgeon; crumbles bone and armor |
| **Sword** | `1d6` | **+1** | 12 silver | Fine balanced steel; reliable and deadly |
| **Flail** | `1d6 + 1` | +0 | 15 silver | Spiked ball and chain; crushing devastation |
| **Mighty Zweihänder** | `1d6 + 2` | +0 | 25 silver | Massive two-handed greatsword (Level Up #5 reward) |
| **Fists (Unarmed)** | `(1d4 - 1)` *(min 0)* | +0 | 0 silver | Desperate bare-knuckle brawl |

### 2.2 Armor

- **Effect**: Whenever Kargunt takes damage from a monster or trap spikes, roll **1d4** to absorb that much damage (e.g. `Damage Taken = Max(0, Incoming Damage - 1d4 roll)`).
- **Pit Trap Interaction**: Heavy armor impairs agility, but still cushions the fall. When falling into pit trap spikes, armor absorbs 1d4 damage.
- **Peddler Value**: 10 silver.

### 2.3 Healing Potions

- **Effect**: Drinking a potion immediately restores **1d6 Hit Points**.
- **Max HP Bound**: Healing is capped at Kargunt's current maximum HP (15 HP initially, 20 HP after Level Up #3). It cannot overheal.
- **Peddler Value**: 4 silver.

### 2.4 Other Utility Relics

- **Hemp Rope**:
  - Adds a **+1 bonus to all pit trap escape rolls** (so damage is avoided on an effective roll of 3+ instead of 4+).
  - Peddler Value: 5 silver.
- **Cloak of Invisibility**:
  - Starts with **1d4 charges**.
  - Using it allows Kargunt to **slip past any active monster encounter unseen**, immediately clearing the room and claiming all monster progression points without rolling combat or suffering counterattacks.
  - Can be sold to the peddler even with 1 charge left.
  - Peddler Value: 15 silver.

---

## 3. Arcane Scrolls & Magic

Roll **1d4** when discovering a random scroll:

| # | Scroll Name | Charges / Duration | In-Game Effect | Peddler Value |
|---|---|:---:|---|:---:|
| **1** | **Summon Weak Daemon** | `1d4` fights | A lesser hell-imp assists Kargunt in battle, dealing **+1d4 bonus damage** each combat round. Disappears once fights expire. | 7 silver |
| **2** | **Palms Open the Southern Gate** | `1d4` uses | Kargunt unleashes sulfurous hellfire from his palms, dealing **1d6 + 1 unblockable damage** directly to the target monster. Bypasses monster defense rolls. | 7 silver |
| **3** | **Aegis of Sorrow** | `1d4` uses | A woven shroud of spectral despair absorbs **1d4 damage** per monster strike. Stacks with leather armor for double absorption. | 7 silver |
| **4** | **False Omen** | 1 use | Single-use fate distortion. When entering a room, you may **choose the room table result** (1–6) instead of rolling randomly, OR **reroll any single die**. | 7 silver |

---

## 4. The Peddler Economy

> **Data-Driven Shop**: Shop wares, hotkeys, and item price tables are loaded at startup from `data/shop.json`. Custom wares, new items, and price tweaks can be adjusted directly in `data/shop.json` without recompilation.

When discovering the **Peddler from beyond the void** (Room Content #6), Kargunt may buy or sell gear:

| Item | Buy Price | Sell Price |
| :--- | :---: | :---: |
| Healing Potion | 4s | 4s |
| Hemp Rope | 5s | 5s |
| Dagger (`1d4`, +1 att) | 6s | 6s |
| Random Scroll | 7s | 7s |
| Warhammer (`1d6`) | 9s | 9s |
| Leather Armor (`-1d4`) | 10s | 10s |
| Sword (`1d6`, +1 att) | 12s | 12s |
| Flail (`1d6 + 1`) | 15s | 15s |
| Cloak of Invisibility | 15s | 15s |
| Mighty Zweihänder (`1d6 + 2`) | 25s | 25s |

---

## 5. Level Up System & Progression

### 5.1 Level-Up Triggers
Kargunt can trigger a Level Up through three distinct paths:

1. **Delving & Slaughter (Standard)**:
   - Slay monsters and overcome obstacles to accumulate **Points**.
   - Upon exploring at least **12 rooms** AND accumulating at least **15 points**, Kargunt qualifies for a level up.
   - Points reset to 0 upon leveling up.
2. **Charity (Altruism)**:
   - Accumulate at least **40 silver** and distribute it to ruined Wästland homes (spends 40 silver).
3. **Basilisk Boon (Eldritch Favor)**:
   - Slaying a **Ruin Basilisk** triggers a 2-in-6 chance of an **instant level up** on the spot, regardless of points or room count.

### 5.2 The 6 Scratch-Off Rewards (Roll 1d6)
Whenever a level up occurs, roll a **1d6**. Each reward can only be claimed once; reroll duplicate results:

| Roll | Reward Title | Mechanical Effect |
| :---: | :--- | :--- |
| **#1** | **Knighted** | Kargunt gains an honorary aristocratic title: *"Sir Kargunt"* or *"Lady Kargunt"*. |
| **#2** | **Weapon Mastery** | Permanently gain **+1 on all attack rolls** against monsters. Stacks with weapon bonuses (e.g. Sword with Weapon Mastery rolls `1d6 + 2`). |
| **#3** | **Toughness** | Maximum Hit Points permanently increases from **15 to 20 HP**. Current HP increases by +5. |
| **#4** | **Herbmaster's Favor** | A sympathetic herbalist gifts Kargunt **5 Healing Potions** directly into inventory. |
| **#5** | **The Greatsword** | Kargunt discovers the **Mighty Zweihänder** (`1d6 + 2` damage, worth 25 silver). |
| **#6** | **Slayer's Aegis** | Choose **one Weak monster** and **one Tough monster** from the bestiary. All damage dealt to Kargunt by those two monster types is permanently **halved**. |

### 5.3 Grand Victory & The 7th Misery
- Kargunt starts at **Level 0**.
- Each level up scratches off one of the 6 destiny rewards.
- When all 6 results have been achieved, Kargunt reaches **Level 6** (**Grand Victory**).
- Kargunt retires to a stronghold or quiet cottage with his plundered silver until the apocalyptic **7th Misery** occurs and the world ends in ash and darkness!

---

## 6. Dungeon Delving & Generation Tables

### 6.1 Entrance Encounter (Roll 1d4)
1. You find a **random Item** (Roll on Item Table, 1d6).
2. A **Weak monster** stands guard. Attack!
3. A **dying mystic** gives you a **random Scroll** (Roll on Scroll Table, 1d4).
4. The entrance is eerily quiet and desolate (empty room).

### 6.2 Room Content (Roll 1d6)
| Roll | Encounter | Resolution |
| :---: | :--- | :--- |
| **1** | **Nothing** | Empty chamber. Marked explored. |
| **2** | **Pit Trap** | Roll 1d6: On `1–3`, fall into spikes for `1d6` damage (armor absorbs 1d4). If carrying a **rope**, add `+1` to roll. Surviving marks room explored. |
| **3** | **Riddling Soothsayer** | Roll 1d6. **Odd**: Riddle solved; choose either **+10 silver** OR **+3 points**. **Even**: Arcane shockwave deals `1d4` unblockable damage. Surviving marks room explored. |
| **4** | **Weak Monster** | Roll on Weak Monster table. Slaying monster marks room explored and awards points/loot. |
| **5** | **Tough Monster** | Roll on Tough Monster table. Slaying monster marks room explored and awards points/loot. |
| **6** | **Peddler** | Trade steel, potions, and scrolls. Leaving marks room explored. |

### 6.3 Room Shape (Roll 2d6) & Doors (Roll 1d4)
- **Shape (2d6)**: `2`: Irregular cave, `3`: Oval, `4`: Cross-shaped, `5`: Corridor, `6–8`: Square, `9`: Round, `10`: Rectangular, `11`: Triangular, `12`: Skull-shaped.
- **Doors (1d4)**: `1`: No doors (dead end), `2`: 1 door, `3–4`: 2 doors.
- **Backtracking**: Returning to cleared rooms incurs a **2-in-6 chance** of a Wandering Monster ambush.

---

## 7. Bestiary

> **Data-Driven Enemies**: All monster statistics, damage formulas, hit targets, awards, loot tables, and post-fight hazards are loaded at startup from `data/enemies/weak.json` and `data/enemies/tough.json`. New citadel horrors can be added directly to those JSON files.

### 7.1 Weak Monsters (Roll 1d4)

| Roll | Monster | Defense Points | HP | Damage | Loot / Special |
| :---: | :--- | :---: | :---: | :---: | :--- |
| **1** | **Blood-drenched Skeleton** | 3 | 6 | `1d4` | 2-in-6 chance: Loot a **Dagger**. |
| **2** | **Catacomb Cultist** | 3 | 6 | `1d4` | 2-in-6 chance: Loot a **Random Scroll**. |
| **3** | **Goblin** | 3 | 5 | `1d4` | 2-in-6 chance: Loot a **Hemp Rope**. |
| **4** | **Undead Hound** | 4 | 6 | `1d4` | Rabid bites; no loot. |

### 7.2 Tough Monsters (Roll 1d4)

| Roll | Monster | Defense Points | HP | Damage | Special Abilities, Loot & Hazards |
| :---: | :--- | :---: | :---: | :---: | :--- |
| **1** | **Necro-Sorcerer** | 4 | 8 | `1d4` / `1d6` | Alternates basic attacks with **Death Rays** (`1d6` dmg).<br>**Loot**: `3d6` silver.<br>**Post-Fight Hazard**: 1-in-6 chance transformed into a maggot (instant death). |
| **2** | **Small Stone Troll** | 5 | 9 | `1d6 + 1` | High defense (needs 5+ to hit); awards a massive **7 points** when slain. |
| **3** | **Medusa** | 4 | 10 | `1d6` | **Loot**: `1d4 × 1d6` silver in ancient coffer.<br>**Post-Fight Hazard**: 1-in-6 chance petrified to salt (instant death). |
| **4** | **Ruin Basilisk** | 4 | 11 | `1d6` | Primordial horror.<br>**Post-Fight Boon**: 2-in-6 chance of an **Immediate Level Up**! |
