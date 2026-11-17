# OSR Mechanics Comparison & Evaluation Guide for Dark Fort (Rust)

> **Document Status**: Living Architectural Evaluation & Design Reference  
> **Topic**: Evaluating Alternative Tabletop RPG / OSR Rulesets under the MÖRK BORG Doom-Metal Theme  
> **Target Audience**: Game Designers, Human Developers, and AI Coding Agents  

---

## 1. Executive Summary & Design Objective

**Dark Fort (Rust)** currently implements the official rules of the 2018 solo micro-game *Dark Fort* by Pelle Nilsson & Johan Nohr (using $1d4$ and $1d6$ dice). While this ruleset is fast and historic, this document evaluates transitioning the game mechanics to a full **Old-School Renaissance (OSR)** tabletop RPG system while preserving the **MÖRK BORG** visual art-punk aesthetic, bleak dying-world lore, and terminal-native roguelike identity.

This guide provides an in-depth breakdown of eleven premier OSR and rules-light systems:
1. **Full MÖRK BORG** (The official player-facing d20 doom-metal TTRPG)
2. **Knave (2nd Edition)** (Pure slot-based, item-driven sandbox crawl)
3. **Cairn / Into the Odd** (Zero-whiff combat, direct damage, Hit Protection & STR trauma)
4. **The Black Hack (2nd Edition)** (Streamlined roll-under d20 with the Usage Die)
5. **Whitehack (4th Edition)** (Blackjack roll-under-yet-high with HP-as-Mana blood magic)
6. **Shadowdark RPG** (Dark fantasy survival horror with a strict torchlight timer)
7. **Old-School Essentials (OSE)** (Pristine 1981 B/X D&D simulation with Gold-as-XP)
8. **Maze Rats** (Punchy 2d6 engine with infinite procedural chaos magic)
9. **Mothership (1e Fantasy Conversion)** ⭐ **DESIGN FAVORITE** (d100 roll-under, double-digit crits, Health + Wounds, Stress & Panic)
10. **DURF** (Ablative armor soak, Hit Dice wound fatality checks, and inventory-slot stress)
11. **AD&D (2nd Edition / Infinity Engine)** (Classic THAC0, descending AC, 3–18 ability scores, and the five saving throw matrices of *Baldur's Gate*, *Icewind Dale*, and *Planescape: Torment*)

For each system, this document provides:
- **System Overview & Core Philosophy**
- **Complete Player Character**: *"Kargunt the Catacomb Rogue"*
- **Complete Monster**: *"Citadel Skeleton"*
- **Step-by-Step Combat Showdown Walkthrough**
- **Technical & Rust Implementation Analysis** (how it translates to `ratatui` and state machines)

---

## 2. Master Comparison Matrix

| Feature | **Full MÖRK BORG** | **Knave 2e** | **Cairn / Into the Odd** | **The Black Hack 2e** | **Whitehack 4e** | **Shadowdark RPG** | **Old-School Essentials** | **Maze Rats** | **Mothership 1e (Fantasy)** ⭐ | **DURF** | **AD&D 2e (Infinity Engine)** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Dice System** | $d20 + d4\text{--}d12$ | $d20 + d6$ Hazard | Polyhedrals ($d4\text{--}d12$) | $d20 + U_d$ Dice | $d20$ | $d20 + d4\text{--}d12$ | $d20 + d6$ damage | **2d6** (bell curve) | **d100 (Percentile)** | **d20 + d6 Buffs/Breaks** | **d20 + variable dice** ($d4\text{--}d12$) |
| **Attack Roll** | Player `d20 + Stat >= 12` | Player `d20 + Stat >= AC` | **None! (Attacks always hit)** | Player `d20 <= Stat` | **Blackjack** ($AC < \text{Roll} \le \text{AV}$) | Player `d20 + Mod >= AC` | Player `d20 >= THAC0/AAC` | Player `2d6 + Stat >= 10` | Player `d100 <= Combat` | Player `d20 + Stat > 15` (or Opposed) | **THAC0**: `d20 >= THAC0 - Target AC` |
| **Defense Roll** | **Active Player Defense** (`d20 + AGI >= 12`) | Passive Monster to-hit vs AC | **Passive Armor subtraction** | **Active Player Defense** (`d20 <= DEX`) | Passive AC or Opposed Blackjack | Passive Monster to-hit vs AC | Passive Monster to-hit vs AC | Passive Opposed or DC 10 | Player `d100 <= Agility/Speed` | **Active Opposed Roll** (`d20 + DEX/STR`) | **Passive Descending AC** ($10$ down to $-10$) |
| **Armor Mechanic** | Rolled Absorption ($1d2/1d4/1d6$) | Static AC + **Splinter Shield** | Direct Damage Reduction (-1, -2, -3) | **Armor Value (AV)** spent to soak hits | **AC sets the lower threshold floor** | Static AC (harder to hit) | Static AC (harder to hit) | Static Armor (1–2 points) | Flat **Armor Points (AP)** damage soak | **Ablative Pool** (3/5/7) + Shield soak (-1) | **Descending AC** (Leather 8, Chain 5, Plate 3; Shield -1) |
| **HP Philosophy** | Meat / Luck buffer (~4–8 HP) | Buffer; excess clogs inventory slots | **Hit Protection (HP)** = poise; excess hits STR | Pool (~4–10 HP) | **HP as Health & Magic Fuel** | Buffer + Death bleeding timer | Fragile (Thief has 1–4 HP) | **Ultra-Fragile** (Flat 4 HP) | **Health Pool (10–15 HP) + Wounds** | **NO HP!** Ablative Armor + Accumulated Wounds | **Hit Dice by Class** ($1d6$ Thief, $1d10$ Fighter) |
| **Wounds / Trauma** | Broken Table (mutilation/death) | **Wounds occupy item slots** | **STR Saves vs. Critical Damage** | Depletes CON; Death Saves | Death at 0 HP / Save vs Death | Downed timer ($1d4$ rounds) | Death at 0 HP | Death / Incapacitation | **Wound Table at 0 HP** (Trauma & resets HP) | **Hit Dice Survival Roll**: $\sum \text{HD} \le \text{Wounds} \implies \textbf{Death}$ | Death at 0 HP (Optional: -10 HP Death's Door) |
| **Dungeon Clock** | **7th Misery Calendar** (Global doom) | **Hazard Die ($1d6$)** every 10 turns | Inventory Fatigue buildup | **Usage Die ($U_d$)** degradation | Turn clock / Auction mechanics | **1-Hour Real-Time Torch Timer** | 10-min Turns + 2-in-6 Wandering | Turn-based Hazard Die | **Stress Accumulation & Panic Checks** | **10-min Turns + Stress clogs slots** | **10-min Turns + Speed Factors (1–10)** |
| **Magic System** | Scrolls + Arcane Catastrophes | 100 Classless Grimoires (1 slot each) | Relics & Fatigue slots | Vancian / Roll-under INT | **Freeform Miracles (Cast with HP)** | Roll-to-Cast + Mishap Tables | Vancian Spell Slots | **Procedural Random Spells** | **Occult Rituals grant Stress; Crits trigger Calamities** | Classless scrolls; **Mishap Blunder on Nat 1** | **Vancian Spell Slots + 5-Category Saves** |
| **Rust Complexity** | Moderate (Player-facing state loop) | Moderate (Slot vector management) | **Low / Cleanest** (Deterministic math) | Low (Roll-under & $U_d$ state) | **Very Low / Elegant** (`ac < roll && roll <= av`) | Moderate (Real-time clock loop) | High (Tables & THAC0 arrays) | Low (Simple 2d6 tables) | **Extremely Low** (`roll <= stat`, `roll % 11 == 0`) | **Very Low / Clean** (Integer soak + HD check) | **Moderate** (`thac0 - roll <= ac`, 5 save tables) |

---

## 3. Deep Dive: Full MÖRK BORG (The Official d20 System)

### 3.1 Design Philosophy & Rules Engine
Full *MÖRK BORG* is an art-punk, rules-light d20 tabletop roleplaying game designed by Pelle Nilsson and Johan Nohr. The mechanics are fast, asymmetrical, and entirely **player-facing**: the game master (or video game engine) never rolls dice in combat. Players roll to strike, players roll to dodge, and players roll to cast powers.

- **The Universal Target**: Difficulty Rating **DR 12** is the baseline for all tests.
- **Attributes (-3 to +3)**: *Strength, Agility, Presence, Toughness*.
- **Omens**: A daily pool of divine fate tokens that grant maximum damage, a free reroll, or reduce a test's DR by 4.
- **The Misery Apocalypse Clock**: Daily dawn rolls on a designated die ($1d100$, $1d20$, or $1d6$). Rolling a 1 triggers one of the 36 apocalyptic Psalms of the Basilisks. Upon the 7th Misery, the universe ends in ash.

### 3.2 Player Character Profile
```
Name:       Kargunt the Gutter Born Scum
Class:      Gutter Born Scum
Hit Points: 6 (Toughness + 1d8)
Omens:      2 (Max damage or instant reroll)
Silver:     35s

Attributes:
  Strength:   -1  (Scrawny, malnourished build)
  Agility:    +2  (Rat-like reflexes, deft hands)
  Presence:   +0  (Unremarkable gaze)
  Toughness:  +1  (Accustomed to dungeon beatings)

Equipment:
  Armor:      Light Armor / Filthy Leather (Tier 1: Absorbs 1d2 damage)
  Weapon:     Shortsword (1d6 damage, Finesse uses Agility)
  Trinkets:   Lockpicks, tallow candle, sack, dried knuckle-bone
```

### 3.3 Enemy Profile
```
Monster:    Citadel Skeleton
Category:   Weak Undead
Hit Points: 5
Morale:     7
Armor:      None (Brittle bone)
Attack:     Splintered Scimitar (1d6 damage)
Special:    Hollow Ribs — Piercing weapons (daggers, arrows) deal only 1 damage.
```

### 3.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Player Attack:
   - Kargunt strikes with Shortsword (Finesse uses Agility: d20 + 2 vs DR 12).
   - Roll: 13 + 2 = 15 >= 12 (HIT!).
   - Damage: Roll 1d6 -> 4 damage dealt.
   - Skeleton HP: 5 -> 1 HP.

2. Enemy Attack (Player Defense Roll):
   - The skeleton counterattacks. Kargunt rolls Defense (d20 + Agility vs DR 12).
   - Roll: 8 + 2 = 10 < 12 (DEFENSE FAILED!).
   - Raw Damage: Skeleton deals 1d6 -> rolls 3 damage.
   - Armor Absorption: Kargunt rolls Tier 1 Leather (1d2) -> rolls 1 absorption.
   - Net Damage: 3 - 1 = 2 damage taken.
   - Kargunt HP: 6 -> 4 HP.
```

### 3.5 Analysis for Dark Fort (Rust)
- **Strengths**: 100% canonical lore compatibility. Player-facing defense rolls (`d20 + AGI >= 12`) and rolled armor absorption (`-1d2/-1d4/-1d6`) feel tactile, dramatic, and intuitive in terminal prompts.
- **TUI Implementation**: A dedicated "Omens" counter widget on the HUD alongside the daily Psalm countdown.

---

## 4. Deep Dive: Knave (2nd Edition)

### 4.1 Design Philosophy & Rules Engine
Designed by Ben Milton (*Questing Beast*), *Knave 2e* is built on the tenet: **You Are What You Carry**. There are no character classes; abilities, spells, and defenses are entirely determined by your inventory slots.

- **Inventory Equals Constitution**: Your total carrying capacity equals your Constitution score (typically 11–15 slots).
- **Wounds as Inventory Clog**: Taking damage past 0 HP inflicts physical wounds (*Broken Rib, Gash, Concussion*) directly into your empty inventory slots. When all slots are full of wounds, you die.
- **Item Quality & Splintering**: Weapons, shields, and armor have durability points (Quality 1–3). When an enemy hits you with lethal force, you can choose to **splinter your shield or helmet** to cancel the strike entirely at the cost of item Quality.
- **The Dungeon Hazard Die ($1d6$)**: Rolled every 10 dungeon turns: `1` Encounter, `2` Dungeon Hazard, `3` Torch burnout, `4` Fatigue.

### 4.2 Player Character Profile
```
Name:       Kargunt, Classless Scavenger
Level:      1
Hit Points: 6 (Constitution Bonus + 4)
Armor Class:13 (Base 11 + Gambeson 1 + Buckler 1)

Attributes (Classic 6, added to d20):
  STR: +1 | DEX: +2 | CON: +2 | INT: +0 | WIS: +1 | CHA: -1

Carrying Capacity: 12 Item Slots (10 + CON Bonus 2)
Inventory Slots (8 / 12 Used):
  [1] Gambeson Armor       (+1 AC, Quality 3/3)
  [2] Wooden Buckler       (+1 AC, Quality 3/3)
  [3] Scramaseax (Dagger)  (1d6 damage, Quality 3/3)
  [4] 50ft Hemp Rope
  [5] Torches (3 pack)
  [6] Lockpicks
  [7] Rations (2 days)
  [8] Silver Pouch
  [9]  [EMPTY SLOT]        <-- Buffer against Wounds
  [10] [EMPTY SLOT]        <-- Buffer against Wounds
  [11] [EMPTY SLOT]        <-- Buffer against Wounds
  [12] [EMPTY SLOT]        <-- Buffer against Wounds
```

### 4.3 Enemy Profile
```
Monster:    Ancient Skeleton
Level / HD: 1
Hit Points: 4
Armor Class:12
Attack:     Rusty Blade +1 (1d6 damage)
Special:    Brittle Architecture — Bludgeoning weapons deal double damage;
            Piercing weapons deal minimum 1 damage.
```

### 4.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Player Attack:
   - Kargunt strikes with Scramaseax (d20 + DEX +2 vs Skeleton AC 12).
   - Roll: 11 + 2 = 13 >= 12 (HIT!).
   - Damage: Roll 1d6 -> 5 damage dealt.
   - Skeleton HP: 4 - 5 = -1 (OBLITERATED!).

ALTERNATIVE SCENARIO: What if Skeleton struck Kargunt for 8 damage?
1. Damage Resolution:
   - Kargunt has 6 HP. Taking 8 damage reduces him to 0 HP with 2 excess damage.
   - The 2 excess damage converts into 2 physical Wounds:
     Slot [9]  becomes [Wound: Gashed Shoulder]
     Slot [10] becomes [Wound: Cracked Rib]
   - Kargunt now has only 2 empty slots remaining!

2. Tactical Reaction (Splintering):
   - Alternatively, Kargunt can announce: "I splinter my Wooden Buckler!"
   - Buckler Quality drops from 3/3 to 2/3.
   - The entire 8-damage strike is completely negated!
```

### 4.5 Analysis for Dark Fort (Rust)
- **Strengths**: Peak roguelike tactical depth. The **Wounds occupying inventory slots** mechanic turns HP into a physical spatial resource inside the TUI inventory grid.
- **TUI Implementation**: A 12-slot visual grid in `ratatui` where slot boxes change color to deep crimson when occupied by physical wounds.

---

## 5. Deep Dive: Cairn / Into the Odd

### 5.1 Design Philosophy & Rules Engine
Created by Yochai Gal (*Cairn*) and Chris McDowall (*Into the Odd*), this engine eliminates combat whiffing entirely.

- **No To-Hit Rolls**: If you attack, you deal damage immediately. Armor subtracts directly from the damage roll.
- **Hit Protection (HP)**: HP is a short-term poise and reflex shield (typically 1–6 HP), which resets completely with a quick rest and a sip of water.
- **Direct Strength Trauma**: When HP reaches 0, excess damage subtracts directly from your **Strength attribute**.
- **Critical Damage Saves**: Any time you suffer direct Strength damage, you must immediately pass a **STR Save (`d20 <= STR`)**. Failure results in Critical Damage: you collapse incapacitated, bleeding to death within 1 hour.
- **Fatigue System**: Casting spells, exposure, and exhaustion add **Fatigue** tokens that take up inventory slots.

### 5.2 Player Character Profile
```
Name:       Kargunt the Desperate Delver
Hit Prot.:  4 HP (1d6)
Armor:      Armor 1 (Padded Brigandine, reduces all damage by 1)

Ability Scores (Roll-Under d20):
  Strength:   11  (Physical health pool & survival threshold)
  Dexterity:  14  (Agility, stealth, dodging hazards)
  Willpower:  8   (Mental resilience & spellcasting)

Inventory (8 / 10 Slots Used):
  [1] Padded Brigandine (Armor 1)
  [2] Shortsword (1d6 damage)
  [3] Throwing Daggers (1d6 damage)
  [4] Torches (3 uses)
  [5] 50ft Rope
  [6] Lockpicks
  [7] Rations (3 days)
  [8] Silver Pouch
  [9]  [EMPTY SLOT]
  [10] [EMPTY SLOT]
```

### 5.3 Enemy Profile
```
Monster:    Ancient Skeleton
Hit Prot.:  4 HP
Armor:      1 (Dense calcified bone reduces damage by 1)
Attributes: STR 8, DEX 11, WIL 5
Attack:     Rusted Scimitar (1d6 damage)
Special:    Hollow Ribs — Piercing weapons are Impaired (damage die drops to 1d4).
```

### 5.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Kargunt Attacks:
   - No to-hit roll required!
   - Kargunt rolls Shortsword damage: 1d6 -> rolls 5.
   - Skeleton Armor 1 reduces damage by 1: Net Damage = 5 - 1 = 4.
   - Skeleton HP: 4 - 4 = 0 HP.

2. Skeleton Counterattacks:
   - No to-hit roll required!
   - Skeleton rolls Scimitar damage: 1d6 -> rolls 6!
   - Kargunt Armor 1 reduces damage by 1: Net Damage = 6 - 1 = 5.
   - Kargunt has 4 HP:
     * First 4 damage reduces HP to 0.
     * Remaining 1 excess damage burns directly into Kargunt's STR attribute!
     * Kargunt STR: 11 -> 10.

3. The Critical Damage Test:
   - Kargunt took STR damage, so he must roll a STR Save (d20 <= 10).
   - Roll: 7 <= 10 (SUCCESS!).
   - Kargunt grinds his teeth, maintains footing, and avoids collapsing!
```

### 5.5 Analysis for Dark Fort (Rust)
- **Strengths**: Lightning-fast, zero-whiff turn loops. Eliminates boring "miss... miss... miss" cycles. Pure integer arithmetic (`Net = Max(0, Damage - Armor)`) is exceptionally clean to implement in Rust.
- **TUI Implementation**: HP bar on top, STR stat bar pulsing red when suffering direct attribute damage.

---

## 6. Deep Dive: The Black Hack (2nd Edition)

### 6.1 Design Philosophy & Rules Engine
Created by David Black, *The Black Hack* popularized the **Roll-Under d20** and the world-famous **Usage Die ($U_d$)**.

- **Roll-Under d20**: To succeed on any attack or check, roll equal to or below your attribute (`d20 <= Stat`).
- **The Usage Die ($U_d$)**: Consumables don't use counters. Instead, they have a die rating ($d20 \to d12 \to d10 \to d8 \to d6 \to d4 \to \text{empty}$). Rolling a 1 or 2 on the die steps it down to the next smaller die.
- **Armor as a Spendable Damage Pool (Armor Value)**: Armor provides a fixed pool of points (e.g. 2 AV) that can be spent during a fight to absorb hits directly, which refresh upon resting.

### 6.2 Player Character Profile
```
Name:       Kargunt the Thief
Class:      Thief
Hit Points: 7 (1d6 + 4)
Armor:      Armor Value 2 (Light leather vest; absorbs 2 hits/damage)

Attributes (Roll-Under d20):
  STR: 10 | DEX: 14 (Primary) | CON: 11 | INT: 9 | WIS: 12 | CHA: 8

Weapons & Consumables:
  Weapon:   Short Dagger (1d6 damage for Thief)
  Torch:    Usage Die Ud8
  Rations:  Usage Die Ud6
  Gear:     Lockpicks, grappling hook, 20 silver
```

### 6.3 Enemy Profile
```
Monster:    Skeleton Minion
Hit Dice:   1 HD
Hit Points: 4
Attack:     2 static damage (or 1d4)
Special:    Undead mind (immune to sleep/fear); piercing deals half damage.
```

### 6.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Player Attack:
   - Player rolls DEX test (d20 <= 14).
   - Roll: 9 <= 14 (SUCCESS!).
   - Damage: Roll 1d6 -> 4 damage.
   - Skeleton HP: 4 - 4 = 0 (DESTROYED!).

2. Enemy Attack (Player Defense Roll):
   - Skeleton swings. Player rolls DEX test to dodge (d20 <= 14).
   - Roll: 16 > 14 (FAILED TO EVADE!).
   - Skeleton deals 2 damage.
   - Player Decision:
     * Take 2 damage to HP (7 -> 5 HP), OR:
     * Spend 1 Armor Value from leather vest (Armor drops to 1 AV, HP stays at 7!).

3. End of Turn Usage Die Check:
   - Kargunt checks active torch (Ud8).
   - Roll: 2! (Steps down to Ud6).
```

### 6.5 Analysis for Dark Fort (Rust)
- **Strengths**: The **Usage Die** is arguably the best resource-tension mechanic in modern tabletop design. Seeing your torch die decay from $d10 \to d8 \to d6 \to d4 \to \text{exhausted}$ in the TUI HUD creates authentic survival dread.

---

## 7. Deep Dive: Shadowdark RPG

### 7.1 Design Philosophy & Rules Engine
Designed by Kelsey Dionne (*The Arcane Library*), *Shadowdark* is a modern, high-voltage dungeon crawl engine centered around light versus darkness.

- **The Strict 1-Hour Torch Timer**: A torch burns for exactly 1 hour of real time (or ~10 dungeon turns in turn-based mode). In darkness, players roll with Disadvantage (worst of 2d20) and monsters roll with Advantage.
- **Roll-to-Cast Spellcasting**: Spells have no spell slots. You roll `d20 + INT/WIS vs. 10 + Tier`. Success keeps the spell; failure loses it for the day; a natural 1 triggers a horrific mishap table.
- **Death Countdown**: Reduced to 0 HP causes bleeding out in `1d4 + CON mod` rounds before true death.

### 7.2 Player Character Profile
```
Name:       Kargunt the Thief
Level:      1
Hit Points: 5 (1d4 + CON mod +1)
Armor Class:13 (Leather 11 + DEX mod +2)

Attributes (Classic 6, modifiers -4 to +4):
  STR: 10 (+0) | DEX: 15 (+2) | CON: 12 (+1)
  INT: 9  (-1) | WIS: 11 (+0) | CHA: 8  (-1)

Gear Slots (Max 10 = STR score):
  [1] Leather Armor (worn)
  [2] Shortsword (1d6, Finesse)
  [3] Dagger (1d4, Finesse)
  [4] Torch (Active: 47 mins left)
  [5] Flint & Steel
  [6] Thieves' Tools
  [7] Rations (3)
  [8] 50ft Hemp Rope
  [9–10] Empty slots for loot
```

### 7.3 Enemy Profile
```
Monster:    Shadowdark Skeleton
Level:      1
Hit Points: 4
Armor Class:11
Movement:   Near
Attack:     1 x Shortsword +1 (1d6 damage)
Special:    Vulnerable to bludgeoning (hammers deal double damage).
```

### 7.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Torch Status Check:
   - Kargunt has an active torch (light radius: Near). Normal rolls apply.

2. Kargunt Attacks:
   - Shortsword uses Finesse (DEX +2). Roll d20 + 2 vs Skeleton AC 11.
   - Roll: 13 + 2 = 15 >= 11 (HIT!).
   - Damage: Roll 1d6 -> 4 damage.
   - Skeleton HP: 4 - 4 = 0 (CRUMBLES TO DUST!).

3. Downed Scenario:
   - If struck to 0 HP, Kargunt rolls Death Timer: 1d4 + 1 CON rounds to be healed or stabilized before death.
```

### 7.5 Analysis for Dark Fort (Rust)
- **Strengths**: Highest atmosphere and survival urgency. Watching a visual torch flame timer drain in ASCII ratatui creates unmatched adrenaline.

---

## 8. Deep Dive: Old-School Essentials (OSE / B/X D&D)

### 8.1 Design Philosophy & Rules Engine
Published by Necrotic Gnome, OSE is a faithful reproduction of the 1981 Moldvay/Cook B/X ruleset.

- **Gold Equals XP**: Slaying monsters yields minimal experience; retrieving treasure (1 silver/gold = 1 XP) is the primary path to leveling up. Combat is a failure state to be avoided.
- **D6 Turn Economy & Side Initiative**: 10-minute dungeon turns, wandering monster checks every 2 turns, and whole-team $1d6$ initiative rolls every combat round.
- **Weapon Damage**: Flat $1d6$ (or optional variable damage).

### 8.2 Player Character Profile
```
Name:       Kargunt the 1st-Level Thief
Hit Points: 3 HP (Single 1d4 roll!)
Armor Class:6 [13 Ascending] (Leather AC 7 + DEX bonus 1)
THAC0:      19 (Needs 12+ on d20 to hit AC 7)

Attributes (Classic 3d6):
  STR: 9 (+0) | INT: 10 (+0) | WIS: 8 (-1)
  DEX: 14 (+1) | CON: 11 (+0) | CHA: 7 (-1)

Saving Throws:
  Death/Poison: 13 | Wands: 14 | Paralysis: 13 | Breath: 16 | Spells: 15

Equipment:
  Leather Armor, Short Sword (1d6), Thieves' tools, 6 torches, 50ft rope, 12s.
```

### 8.3 Enemy Profile
```
Monster:    B/X Skeleton
Hit Dice:   1 HD (1d8 -> 4 HP)
Armor Class:7 [12 Ascending]
Attack:     1 x Scimitar (1d6 damage)
Special:    Immune to charm/sleep; takes half damage from slashing/piercing weapons.
```

### 8.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Side Initiative:
   - Kargunt rolls 1d6 -> 5. Skeleton rolls 1d6 -> 2. Kargunt wins initiative!

2. Kargunt Attacks:
   - Needs 12+ on d20 to hit Skeleton AC 12.
   - Roll: 14 >= 12 (HIT!).
   - Damage: Roll 1d6 -> rolls 4.
   - B/X Trait: Shortsword is slashing -> damage halved to 2!
   - Skeleton HP: 4 - 2 = 2 HP.

3. Skeleton Attacks:
   - Needs 12+ on d20 to hit Kargunt AC 13.
   - Roll: 11 < 12 (MISS! The blade sparks off stone).
   - Kargunt narrowly survives (3 HP means any hit could have killed him).
```

### 8.5 Analysis for Dark Fort (Rust)
- **Strengths**: Pure historical 1981 D&D dungeon crawling authenticity. Demands careful evasion, mapping, and stealth.

---

## 9. Deep Dive: Maze Rats (2d6 Chaos Magic)

### 9.1 Design Philosophy & Rules Engine
Created by Ben Milton, *Maze Rats* uses a fast **2d6 bell curve** and features a legendary procedural spell-generation engine.

- **2d6 Target 10 Engine**: Tests succeed on `2d6 + Stat >= 10`.
- **Flat 4 HP**: Every character starts with only 4 HP.
- **Infinite Procedural Spells**: Spells are generated randomly by combining physical effects with elements:
  - `[Physical Effect]` + `[Element / Form]` $\to$ *"Shattering Vomit"*, *"Corrosive Swarm"*, *"Freezing Tendril"*, *"Ethereal Casket"*.

### 9.2 Player Character Profile
```
Name:       Kargunt the Rat
Hit Points: 4 HP
Armor:      Light Armor (Armor 1)

Attributes (2d6 modifiers +0 to +2):
  Strength:   +1
  Dexterity:  +2
  Willpower:  +0

Gear:
  Short Dagger (1d6), Light Armor (Armor 1), Lockpicks, 2 torches.
```

### 9.3 Enemy Profile
```
Monster:    Dungeon Skeleton
Hit Points: 4 HP
Armor:      1 (Dense bone)
Attack:     Rusty Claw (1d6 damage)
```

### 9.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Kargunt Attacks:
   - Roll 2d6 + DEX +2 vs Target 10.
   - Roll: (4 + 5) + 2 = 11 >= 10 (HIT!).
   - Damage: Roll 1d6 -> rolls 5 - Armor 1 = 4 damage.
   - Skeleton HP: 4 - 4 = 0 (SHATTERED!).
```

### 9.5 Analysis for Dark Fort (Rust)
- **Strengths**: The random spell generation tables match *MÖRK BORG*’s grotesque, surreal magic tone 100%. In Rust, you can generate hundreds of wild, weird spell names and effects procedurally on the fly!

---

## 10. Deep Dive: Whitehack (4th Edition)

### 10.1 Design Philosophy & Rules Engine
Created by Christian Mehrstam, *Whitehack* (now in its 4th Edition) is widely regarded as one of the most intellectually elegant and versatile OSR rulesets ever written. Built on an original 1974 OD&D lineage, it replaces complex subsystem tables with a unified **Blackjack roll-under-yet-high** d20 mechanic and freeform, HP-fueled miracles.

- **Blackjack Roll-Under ($AC < \text{Roll} \le \text{Stat/AV}$)**:
  - To hit or succeed in an opposed test, roll a d20.
  - The roll must be **less than or equal to** your Stat or Attack Value (AV) to be a valid effort.
  - The roll must be **strictly greater than** the opponent's Armor Class (AC) to penetrate armor.
  - *Example*: Kargunt has AV 11; the enemy has AC 2. Rolls from 3 through 11 hit. Rolls 1–2 are stopped by armor. Rolls 12–20 are wide misses.
- **Natural Critical on Exact Match**:
  - If your d20 roll **exactly equals** your AV or Stat (e.g. rolling 11 with AV 11), it is a **Critical Success** (maximum damage, free disarm, or special combat advantage).
- **The Three Archetypes**:
  - Instead of hundreds of disparate classes, characters are built on three broad archetypes: **The Deft** (experts, rogues, rangers), **The Strong** (warriors, berserkers), and **The Wise** (witches, occultists, miracle-workers).
  - Archetypes are customized via **Vocations** (e.g., *Catacomb Prowler*, *Plague Doctor*, *Inquisitor*) and **Attunements**.
- **Deft Attunements**:
  - Deft characters can attune to specific signature items or companions. When using attuned equipment (e.g., an assassin's garrote, a stealth shroud, or lockpicks), they can perform superhuman feats without a roll or with double effectiveness.
- **Freeform Miracles (HP-as-Mana)**:
  - There are no rigid spell slots or arbitrary mana pools. Magic is freeform: casters state an intent, and the miracle's cost is deducted **directly from their current Hit Points**! Casting beyond one's means causes immediate fainting or death.

---

### 10.2 Player Character Profile
```
Name:       Kargunt the Catacomb Prowler
Archetype:  The Deft (Level 1)
Vocation:   Catacomb Prowler & Grave Robber
Hit Points: 5 HP
Attack Value (AV): 11 (Base 10 + 1 Deft melee bonus)
Armor Class (AC):  2 (Padded gambeson + nimble dodging)
Move:       30 ft
Save:       5 (Roll d20 <= 5 to resist poison, traps, or death magic)

Attributes (3d6 in order):
  Strength:      9   (Lean, starved physique)
  Dexterity:    14   (Greased fingers, silent footsteps)
  Constitution: 11   (Accustomed to damp cryptrot)
  Intelligence: 10   (Cunning survivor's wit)
  Wisdom:        8   (Impulsive, drawn to cursed relics)
  Charisma:      7   (Sullen, unwashed dungeon rat)

Groups & Affiliations:
  Vocation:     Catacomb Prowler (Positive double roll on stealth, lockpicking, trap disarming)
  Species:      Human (Wasteland scrapper)
  Affiliation:  The Sunken Guild of Sarkash

Deft Attunements (1 slot at Level 1):
  Attuned Gear: Serrated Gutting Dagger (Double critical damage; bypasses 1 point of monster AC on ambush)

Equipment:
  Serrated Dagger (1d6 damage), Padded Gambeson (AC 2), Thieves' Picks,
  2 Tallow Candles, Flint & Steel, Chalk, 30 ft Hemp Rope, 15 Silver.
```

---

### 10.3 Enemy Profile
```
Monster:    Citadel Skeleton (Restless Tomb Guardian)
Hit Dice:   1 HD (5 HP)
Armor Class: 2 (Dense, calcified ribcage and buckler)
Attack Value (AV): 11
Move:       30 ft
Special:    Piercing weapons deal half damage; immune to sleep, mind charms, and rot.
```

---

### 10.4 Combat Showdown Walkthrough

```
ROUND 1:
1. Kargunt Attacks with Serrated Dagger (Target: Skeleton AC 2, Kargunt AV 11):
   - Hit Condition: AC (2) < Roll <= AV (11).
   - Valid hit range: [3, 4, 5, 6, 7, 8, 9, 10, 11].
   - Roll d20: 9.
   - Result: 2 < 9 <= 11 (HIT!). The blade slips between the calcified ribs.
   - Damage: Roll 1d6 -> rolls 4 damage.
   - Piercing Resistance: Skeleton resists piercing -> 4 / 2 = 2 damage.
   - Skeleton HP: 5 - 2 = 3 HP remaining.

2. Skeleton Counter-Attacks (Target: Kargunt AC 2, Skeleton AV 11):
   - Hit Condition: AC (2) < Roll <= AV (11).
   - Valid hit range: [3, 4, 5, 6, 7, 8, 9, 10, 11].
   - Roll d20: 2.
   - Result: Roll 2 is NOT strictly greater than AC 2 (DEFLECTED!).
   - Description: The rusty scythe glances harmlessly off Kargunt's hardened leather shoulder guard.

ROUND 2:
1. Kargunt Strikes for the Kill (Target: Skeleton AC 2, Kargunt AV 11):
   - Roll d20: 11!
   - Result: Roll EXACTLY equals AV 11 -> CRITICAL HIT!
   - Critical Effect: Attuned dagger triggers crushing critical bypass; maximum damage applied!
   - Damage: Max 1d6 = 6 damage (bypasses resistance on crit).
   - Skeleton HP: 3 - 6 = -3 (SKELETON SHATTERED INTO BONE DUST).
```

---

### 10.5 Freeform Blood Miracles: HP-as-Mana in Dark Fort
In *MÖRK BORG*'s apocalyptic, dying-world setting, Whitehack's miracle system translates directly into gruesome **Blood Magic**:
- Instead of tracking spell slots, sorcery demands biological sacrifice.
- **Example Blood Miracles**:
  - *Grasp of the Pale King* (Stun an enemy for 1 round): Costs **2 HP**.
  - *Corpse Detonation* (Explode a fallen skeleton for $2d6$ blast damage): Costs **4 HP**.
  - *Shadow Veil* (Become invisible until attacking): Costs **3 HP**.
- If casting reduces the player to 0 HP, the spell goes off, but the caster collapses into death or coma. This creates intense tactical tension: *"Do I sacrifice 3 of my remaining 5 HP to banish this demon, knowing a single claw strike will kill me?"*

---

### 10.6 Technical & Rust Implementation Analysis

#### The Blackjack Combat Engine
The beauty of Whitehack's mechanic in Rust is that a single numeric comparison handles accuracy, armor mitigation, misses, and criticals without nested branch trees:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum BlackjackOutcome {
    CriticalHit,              // roll == av
    Hit,                      // target_ac < roll < av
    DeflectedByArmor,         // roll <= target_ac
    Miss,                     // roll > av
}

pub fn resolve_blackjack_attack(roll: u8, target_ac: u8, attacker_av: u8) -> BlackjackOutcome {
    if roll == attacker_av {
        BlackjackOutcome::CriticalHit
    } else if roll > target_ac && roll < attacker_av {
        BlackjackOutcome::Hit
    } else if roll <= target_ac {
        BlackjackOutcome::DeflectedByArmor
    } else {
        BlackjackOutcome::Miss
    }
}
```

#### Why Whitehack is Brilliant for Dark Fort:
1. **Single-Roll Completeness**: In traditional D&D, you roll to hit, check AC, and roll damage. In Whitehack, the d20 roll alone immediately tells you if you hit, whether the armor blocked it, or if you scored a critical hit.
2. **Deterministic Opposed Contests**: When two combatants clash simultaneously, both roll d20. Whoever rolls highest while staying $\le \text{AV}$ wins the exchange. No ties, no math fatigue.
3. **Visceral Resource Economy**: Merging magic fuel into the existing HP pool eliminates auxiliary mana bars, spell slot arrays, and rest tracking in the Rust UI while elevating the doom-metal theme of bodily ruin.

---

## 11. Deep Dive: Mothership 1e (Fantasy Horror Conversion) ⭐ DESIGN FAVORITE

### 11.1 Design Philosophy & Rules Engine
Created by Sean McCoy and Tuesday Knight Games, *Mothership (1st Edition)* is celebrated as the undisputed gold standard of modern tabletop survival horror. While originally designed for sci-fi claustrophobia, its rules engine translates seamlessly into *MÖRK BORG*’s nihilistic dark fantasy. In this conversion (often dubbed *Dungeonship* or *Grim Void*), the high-tech veneer is peeled back to reveal pure, unfiltered gothic dread.

- **Percentile Roll-Under ($d100 \le \text{Stat}$)**:
  - There is zero target number arithmetic or table lookup. If Kargunt's Combat is `38%`, rolling `34` hits; rolling `52` misses.
- **Double-Digit Criticals (Matching Digits)**:
  - Any matching double digit (**11, 22, 33, 44, 55, 66, 77, 88, 99, 00**) is an instant dramatic event:
    - $\text{Roll} \le \text{Stat}$: **Critical Success** (Maximum damage, weapon disarm, or severed limb).
    - $\text{Roll} > \text{Stat}$: **Critical Failure / Disaster** (Fumble, broken weapon, self-injury, $+2$ Stress).
  - In a terminal TUI, double-digit rolls provide electrifying visual flair (`[33] CATASTROPHIC FUMBLE!`).
- **Health + Wounds System (Anti-HP-Bloat)**:
  - Characters do not accumulate giant bags of hit points. Kargunt has a modest **Health pool** (e.g. 14 HP) and **2 Wounds**.
  - Dropping to 0 Health **does not cause instant death**; instead, you take **1 Wound**, roll on the brutal, mutilating **Wound Table** (e.g., *Broken Ribs, Severed Artery, Gouged Eye, Concussion*), and your Health pool resets!
  - When all Wounds are consumed, you bleed out into true death.
- **Armor Points (AP)**:
  - Armor does not make you harder to hit; it directly absorbs damage. Boiled Leather (AP 3) deducts 3 points from any incoming damage.
- **The Stress & Panic Loop**:
  - Traversing unlit tombs, taking critical hits, or missing consecutive strikes generates **Stress**.
  - When traumatized, roll **$1d20$ vs. Current Stress**. Rolling $\le \text{Stress}$ triggers a roll on the Panic Table (*Tremors, Frenzy, Dropped Weapon, Scream, Heart Failure*).
- **Occult Blood Magic**:
  - Spells are cast via the **Occult** stat ($d100$). Casting always adds $+1$ Stress. Rolling double digits on an Occult test triggers an **Arcane Calamity** (demonic possession, mutagenic rot, or psychic shockwave).

---

### 11.2 Player Character Profile
```
Name:       Kargunt the Grave-Robber
Class:      Scoundrel / Catacomb Delver
Health:     14 / 14
Wounds:     2 / 2
Stress:     3 (Min: 2, Max: 20)
Armor:      Boiled Leather (AP 3)

Attributes (d100):
  Strength:   35%  (Wrestling ghouls, prying stone sarcophagi)
  Agility:    42%  (Dodging deadfalls, nimble flight from ambushes)
  Occult:     28%  (Deciphering cursed scrolls, identifying relics)
  Combat:     38%  (Base accuracy with daggers, blades, flails)

Saves (d100):
  Mind Save:   30%  (Resisting psychic basilisk gazes, hallucinations)
  Courage:     25%  (Standing firm against abominations and dread)
  Flesh Save:  35%  (Resisting pit venom, necro-sorcerer rot, plagues)

Skills (Trained +10%):
  - Lockpicking & Traps (+10% Agility -> 52% effective)
  - Scavenging & Lore   (+10% Occult  -> 38% effective)
  - Gutting Blades      (+10% Combat  -> 48% effective)

Equipment:
  - Rusty Scythe: 2d10 Damage, inflicts Bleeding Wound on Critical Hit
  - Boiled Leather: Armor Points (AP) 3 (Absorbs 3 damage from any blow)
  - Thieves' Picks, 2 Tallow Torches (3 exploration turns each)
  - Hemp Rope, 18 Silver Coins
```

---

### 11.3 Enemy Profile
```
Monster:    Citadel Skeleton (Bleached Crypt Guardian)
Health:     10 / 10
Wounds:     1 / 1
Combat:     35%
Armor:      AP 2 (Dense, calcified ribcage)
Attack:     Rusty Cleaver (1d10 damage)
Special:    Piercing weapons deal half damage; immune to Mind and Flesh saves.
            Entering its unlit chamber forces a Courage Save (25%) or gain +1 Stress.
```

---

### 11.4 Combat Showdown Walkthrough

```
ROUND 1:
1. Encounter Dread:
   - Kargunt opens the door into the crypt. A calcified skeleton rises!
   - Courage Save: Roll d100 -> rolls 78 > 25 (FAIL). Kargunt gains +1 Stress (Stress: 4).

2. Kargunt Attacks with Rusty Scythe (Combat 38% + Skill 10% = 48% Target):
   - Roll d100: 22!
   - Result: 22 <= 48 AND matching double digits -> CRITICAL SUCCESS!
   - Damage: Max 2d10 = 20 damage.
   - Skeleton Armor: AP 2 -> 20 - 2 = 18 damage taken.
   - Skeleton Health: 10 - 18 = 0 HP. The overflow damage consumes its only Wound!
   - Outcome: The scythe cleanly severs the spine, scattering bones across the paving stones.
   - Stress Relief: Slaying a foe on a Critical grants composure -> -1 Stress (Stress: 3).

ALTERNATIVE ROUND (Skeleton Strikes Back):
1. Skeleton Attacks Kargunt (Combat 35%):
   - Roll d100: 31 <= 35 (HIT!).
   - Damage: Roll 1d10 -> 8 damage.
   - Kargunt's Leather Armor: AP 3 -> 8 - 3 = 5 damage penetrates armor.
   - Kargunt Health: 14 - 5 = 9 / 14 HP. (No wound taken yet; armor soaked the lethal edge).
```

---

### 11.5 Technical & Rust Implementation Analysis

#### Elegant Match & Digit Parsing in Rust
The percentile engine and double-digit critical detection require zero floating-point math and zero complex lookup tables:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum D100Outcome {
    CriticalSuccess, // roll <= target && is_double
    RegularSuccess,  // roll <= target && !is_double
    RegularFailure,  // roll > target && !is_double
    CriticalFailure, // roll > target && is_double
}

pub fn resolve_d100_check(roll: u8, target: u8) -> D100Outcome {
    let is_double = (roll % 11 == 0) || roll == 100;
    if roll <= target {
        if is_double {
            D100Outcome::CriticalSuccess
        } else {
            D100Outcome::RegularSuccess
        }
    } else {
        if is_double {
            D100Outcome::CriticalFailure
        } else {
            D100Outcome::RegularFailure
        }
    }
}
```

#### Why Mothership 1e is Our #1 Design Favorite:
1. **Zero-Math Speed**: In terminal games, speed is king. Players intuitively understand percentile chances (`48%` means roughly half the time). A single d100 roll instantly determines success, failure, and criticals without comparing against opponent armor.
2. **True Roguelike Scars**: The Health + Wounds system solves the biggest flaw of standard D&D roguelikes (where you are either 100% fine or dead). Taking a Wound permanently alters the character (e.g. broken arm, bleeding artery, shattered teeth), creating emergent survival stories.
3. **Thematic Perfection**: Combining *MÖRK BORG*’s yellow-and-black doom-metal art style with Mothership’s claustrophobic psychological horror creates the definitive terminal survival experience.

---

## 12. Deep Dive: DURF (Ablative Armor & Survival HD)

### 12.1 Design Philosophy & Rules Engine
Created by Emiel Boven (*Penflower Ink*), **DURF** is a rules-light, hyper-lethal dungeon-fantasy tabletop RPG that crossbreeds the inventory ethos of *Knave*, the zero-HP poise buffer of *Into the Odd*, and the boon/bane modifiers of *Shadow of the Demon Lord*.

- **The Universal Target (Over 15)**: Any risky action or reaction resolves by rolling **`d20 + Attribute > 15`**. In opposed conflicts, the higher total wins.
- **Three Core Attributes**: **STR** (*Force*), **DEX** (*Finesse*), and **WIL** (*Willpower*), typically ranging from $+0$ to $+3$.
- **Buffs & Breaks**: Situational advantages add $+1d6$ (Buffs) while disadvantages subtract $-1d6$ (Breaks), canceling out $1:1$.
- **NO Traditional Hit Points (Ablative Armor)**: Armor absorbs incoming damage point-for-point (*Light = 3, Medium = 5, Heavy = 7*). A shield reduces each incoming strike by 1 (minimum 1). Once Armor hits 0, damage penetrates directly as **Wounds**.
- **The Hit Dice Mortality Check**: Characters start with **1 Hit Die** ($1d6$). Whenever a creature suffers new Wounds, they immediately roll their Hit Dice. If **$\sum \text{HD} \le \text{Accumulated Wounds}$**, the character dies immediately.
- **Stress-Clogged Inventory**: Carrying capacity is **$10 + \text{STR}$ slots**. Horrors and exhaustion inflict **Stress**, which takes up inventory slots, forcing hard choices between carrying loot and retaining sanity.

### 12.2 Player Character Profile
```
Name:       Kargunt the Dungeon Scavenger
Hit Dice:   1 HD (1d6 survival check against wounds)
Armor:      3 / 3 (Leather Jerkin, point-for-point ablative buffer)
Shield:     Wooden Targe (-1 incoming damage soak per hit)
Wounds:     0 (Fatal if rolled HD <= Wounds)

Attributes (Added to d20 vs Target 15+):
  STR (Force):   +2  (Melee attacks, raw brawn, slot capacity)
  DEX (Finesse): +1  (Ranged combat, initiative, agile evasion)
  WIL (Resolve): +0  (Mental endurance, spellcasting, horror checks)

Carrying Capacity: 12 Item Slots (10 + STR 2)
Inventory (7 / 12 Slots Used):
  [1]  Shortsword (deals 3 damage)
  [2]  Leather Jerkin (3 Armor pool)
  [3]  Wooden Targe (-1 damage soak)
  [4]  Tallow Torches (3 uses)
  [5]  50ft Hemp Rope
  [6]  Iron Spikes & Hammer
  [7]  Supply Pouch (rations & lockpicks)
  [8]  [EMPTY SLOT]
  [9]  [EMPTY SLOT]
  [10] [EMPTY SLOT]
  [11] [EMPTY SLOT]
  [12] [EMPTY SLOT]  <-- Stress buffers (Stress fills slots)
```

### 12.3 Enemy Profile
```
Monster:    Citadel Skeleton
Skill:      +1 (Flat modifier added to attacks and contests)
Hit Dice:   1 HD (1d6)
Armor:      2 (Brittle ribcage absorbs 2 damage before taking Wounds)
Morale:     7
Attack:     Splintered Scimitar (deals 3 damage)
Special:    Bony Form — Piercing attacks (daggers, arrows) deal only 1 damage.
```

### 12.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Initiative:
   - Roll 1d6 each side: Kargunt rolls 5, Skeleton rolls 2 -> Kargunt acts first.

2. Kargunt Attacks:
   - Kargunt strikes with Shortsword (d20 + STR +2).
   - Roll: 14 + 2 = 16.
   - Skeleton defends with Skill (d20 + Skill +1).
   - Roll: 9 + 1 = 10.
   - Result: HIT! (16 > 10). Shortsword deals 3 damage.
   - Damage Resolution on Skeleton:
     * Skeleton has 2 Armor: The first 2 damage depletes Armor to 0.
     * Remaining 1 damage penetrates as 1 Wound.
     * Skeleton Wounds: 1.
   - The Hit Dice Survival Check:
     * Skeleton rolls its 1 HD (1d6) against total accumulated Wounds (1).
     * Roll: 4.
     * Since 4 > 1, the skeleton absorbs the structural shock and fights on!

3. Skeleton Counterattacks:
   - Skeleton swings its Splintered Scimitar (d20 + Skill +1).
   - Roll: 15 + 1 = 16.
   - Kargunt dodges / parries (d20 + DEX +1).
   - Roll: 11 + 1 = 12.
   - Result: HIT! (16 > 12). Scimitar deals 3 damage.
   - Damage Resolution on Kargunt:
     * Shield Soak: Wooden Targe absorbs 1 damage -> 3 - 1 = 2 damage penetrates.
     * Armor Soak: Leather Jerkin absorbs the 2 damage.
     * Kargunt Armor: 3 -> 1 remaining.
     * Wounds: 0 taken! No Hit Dice survival check required.

ROUND 2:
1. Kargunt Attacks:
   - Kargunt presses the assault (Roll: 17 + 2 = 19 vs Skeleton: 8 + 1 = 9).
   - Result: HIT! Shortsword deals 3 damage.
   - Damage Resolution on Skeleton:
     * Skeleton has 0 Armor remaining, so all 3 damage penetrates as Wounds!
     * Total Accumulated Wounds on Skeleton: 1 + 3 = 4 Wounds.
   - The Hit Dice Survival Check:
     * Skeleton rolls its 1 HD (1d6) vs 4 accumulated Wounds.
     * Roll: 3.
     * VERDICT: Rolled 3 <= 4 Wounds. The skeleton's spine shatters into dust! (INSTANT KILL)
```

### 12.5 Analysis for Dark Fort (Rust)
- **Strengths**: Eliminates HP bloat completely. The mathematical loop (`Damage - Shield -> Ablative Armor -> Wounds -> Mortality Check`) produces incredible white-knuckle tension on every strike without complex tables.
- **TUI Implementation**: Extremely concise in `ratatui`. A visual Armor bar (`ARMOR: [■■■··] 3/5`), Wounds counter (`WOUNDS: 2`), and dynamic death hazard chance (`SURVIVAL RISK: 33% (1d6 <= 2)`) provide instant tactical clarity.

---

## 13. Deep Dive: AD&D 2nd Edition (Infinity Engine / THAC0)

### 13.1 Design Philosophy & Rules Engine
Published in 1989 by TSR under lead designer David "Zeb" Cook, **Advanced Dungeons & Dragons 2nd Edition** represents the definitive bridge between tabletop grognard dungeon crawling and the golden age of computer roleplaying games (CRPGs). Immortalized by BioWare and Black Isle Studios through the **Infinity Engine** (*Baldur's Gate 1 & 2*, *Icewind Dale 1 & 2*, and *Planescape: Torment*), this system features some of the most celebrated and debated arithmetic in gaming history: **THAC0** and **Descending Armor Class**.

- **THAC0 ("To Hit Armor Class 0")**:
  - Every combatant has a THAC0 value determined by class and level (e.g., Level 1 Fighter = 20, Level 1 Thief = 20, Level 1 Mage = 20; 1 HD Monster = 19).
  - To determine the Armor Class hit:
    $$\text{Effective AC Hit} = \text{THAC0} - (\text{d20 Roll} + \text{Hit Modifiers})$$
  - The attack lands successfully if the Effective AC Hit is **less than or equal to** the target's Armor Class:
    $$\text{Effective AC Hit} \le \text{Target AC} \iff \text{d20 Roll} \ge \text{THAC0} - \text{Target AC}$$
- **Descending Armor Class (AC 10 down to -10)**:
  - In stark contrast to modern ascending AC (where higher is better), AD&D 2e treats **lower AC as superior protection**:
    - Unarmored civilian: **AC 10**
    - Leather Armor: **AC 8**
    - Studded Leather: **AC 7**
    - Chain Mail: **AC 5**
    - Field / Full Plate: **AC 2 / AC 1**
    - Tower Shield: **-1 AC bonus** (e.g., AC 5 Chain Mail becomes AC 4)
    - Legendary armor and rings of protection can lower AC down into the negative numbers (all the way to the hard floor of **AC -10**).
- **3–18 Ability Scores with Non-Linear Thresholds**:
  - Ability scores (*Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma*) use non-linear breakpoint tables rather than a linear $(Stat - 10) / 2$:
    - **Strength**: 16 (+0 Hit / +1 Dmg), 17 (+1 Hit / +1 Dmg), 18 (+1 Hit / +2 Dmg), plus exceptional 18/01–18/00 percentile strength for warriors.
    - **Dexterity Defensive Adjustment**: DEX 15 grants -1 AC, DEX 16 grants -2 AC, DEX 17 grants -3 AC, and DEX 18 grants -4 AC.
    - **Constitution**: CON 15 (+1 HP/HD), CON 16 (+2 HP/HD), with warriors scaling up to +4 at 18 CON.
- **The Five Classical Saving Throw Matrices**:
  Instead of modern three-save (Reflex/Fort/Will) or 6-attribute systems, AD&D 2e resolves hazards across five classic category tables rolled on $d20 \ge \text{Target}$:
  1. *Paralyzation, Poison, or Death Magic (PPDM)*
  2. *Rod, Staff, or Wand (RSW)*
  3. *Petrification or Polymorph (PP)*
  4. *Breath Weapon (BW)*
  5. *Spells (Spell)*
- **Weapon Speed Factors & Tactical Initiative**:
  - Initiative is rolled on a $1d10$ per round where **lowest roll acts first**.
  - Weapon speed factors (Dagger = 2, Shortsword = 3, Two-handed Sword = 10) or spell casting times are added to the initiative roll, making light weapons strike before heavy cleaves.

---

### 13.2 Player Character Profile
```
Name:       Kargunt the Shadow Thief
Class:      Thief (Level 1)
Race:       Human
Alignment:  Chaotic Neutral
Hit Points: 6 HP (1d6 maximum at 1st level)
THAC0:      20 (Level 1 Thief baseline)
Armor Class:6 (Leather Armor AC 8 with DEX 16 defensive adjustment of -2)
Movement:   12

Ability Scores (Classic 3-18):
  STR: 14 (+0 to hit, +0 damage, weight allowance 55 lbs)
  DEX: 16 (+1 missile attack bonus, -2 AC defensive adjustment)
  CON: 14 (+0 HP bonus, 92% resurrection survival)
  INT: 12 (3 additional known languages)
  WIS: 10 (Standard mental saves, 0% spell failure)
  CHA: 11 (Base maximum 4 henchmen, 0 modifier)

Saving Throws (Level 1 Thief):
  Paralyzation / Poison / Death Magic: 13
  Rod / Staff / Wand:                  14
  Petrification / Polymorph:           12
  Breath Weapon:                       16
  Spells:                              15

Thief Skills:
  Pick Pockets: 35% | Open Locks: 30% | Find/Remove Traps: 25%
  Move Silently: 20% | Hide in Shadows: 15% | Detect Noise: 15%
  Backstab Multiplier: x2 damage from behind with surprise

Equipment:
  Suit of Leather Armor (Base AC 8)
  Shortsword (1d6 S/M, Speed Factor 3)
  Thieves' Picks & Pricks
  Tallow Torches (3)
  Belt pouch with 35 Gold Pieces (gp)
```

---

### 13.3 Enemy Profile
```
Monster:    Citadel Skeleton
Hit Dice:   1 HD (1d8 -> 5 HP)
THAC0:      19 (Standard 1 HD monster THAC0)
Armor Class:7 (Ancient brittle bone)
Movement:   12
Attacks:    1 attack per round
Damage:     Splintered Scimitar (1d6 damage, Speed Factor 5)
Special Defenses:
  - Half damage from slashing and piercing weapons (daggers, scimitars, arrows)
  - Full normal damage from bludgeoning weapons (maces, clubs, warhammers)
  - Immune to Sleep, Charm, Hold, Poison, Cold, and Death magic
  - Fearless (never makes morale checks, fights until destroyed)
```

---

### 13.4 Combat Showdown Walkthrough
```
ROUND 1:
1. Initiative:
   - Both sides roll 1d10 + Weapon Speed Factor (lowest total strikes first).
   - Kargunt: Roll 4 + Shortsword Speed 3 = 7.
   - Skeleton: Roll 6 + Scimitar Speed 5 = 11.
   - Kargunt acts first!

2. Kargunt Attacks with Shortsword:
   - Target AC: Citadel Skeleton has AC 7.
   - Target Roll Needed: THAC0 (20) - Target AC (7) = 13 on d20.
   - Kargunt rolls d20: 15.
   - Effective AC Hit Calculation:
     * Effective AC Hit = 20 - 15 = AC 5 hit!
     * Since Effective AC 5 <= Skeleton AC 7, the strike connects cleanly.
   - Damage Resolution:
     * Base Damage: Roll 1d6 -> 4 damage.
     * Slashing Resistance: Skeleton takes half damage -> 4 / 2 = 2 damage.
     * Skeleton HP: 5 - 2 = 3 HP remaining.

3. Citadel Skeleton Counter-Attacks:
   - Target AC: Kargunt has AC 6 (Leather AC 8 minus 2 from DEX 16).
   - Target Roll Needed: THAC0 (19) - Target AC (6) = 13 on d20.
   - Skeleton rolls d20: 14.
   - Effective AC Hit Calculation:
     * Effective AC Hit = 19 - 14 = AC 5 hit!
     * Since Effective AC 5 <= Player AC 6, the rusted scimitar slashes home!
   - Damage: Scimitar deals 1d6 -> rolls 3 damage.
   - Kargunt HP: 6 - 3 = 3 HP remaining (Bloodied and gasping!).

ROUND 2:
1. Initiative:
   - Kargunt: Roll 2 + Speed 3 = 5.
   - Skeleton: Roll 8 + Speed 5 = 13.
   - Kargunt seizes the opening with quick reflexes!

2. Kargunt Attacks:
   - Target Roll Needed: THAC0 20 - AC 7 = 13.
   - Kargunt rolls d20: 17.
   - Effective AC Hit: 20 - 17 = AC 3 hit! (Well beneath AC 7).
   - Damage: Roll 1d6 -> 6 damage (maximum roll!).
   - Slashing Resistance: 6 / 2 = 3 damage.
   - Skeleton HP: 3 - 3 = 0 HP!
   - VERDICT: The skeleton's ribs shatter under the strike, collapsing into calcified dust!
```

---

### 13.5 Analysis for Dark Fort (Rust)
- **Mathematical Elegance of THAC0 in Rust**:
  Instead of bulky lookup matrices, the Infinity Engine THAC0 algorithm compiles into a fast, branch-free Rust calculation in `src/core/combat.rs`:
  ```rust
  let needed_roll = attacker_thac0 - defender_ac;
  let effective_ac_hit = attacker_thac0 - d20_roll;
  let is_hit = d20_roll >= needed_roll; // Equivalent to: effective_ac_hit <= defender_ac
  ```
- **TUI Dashboard Representation (`ratatui`)**:
  - The status panel prominently renders the classic Infinity Engine stats:
    `THAC0: 20 | AC: 6 [Leather+DEX] | HP: 6/6`
  - Combat chronicle entries communicate the exact hit-calculation:
    `"Kargunt rolls 15 vs THAC0 20 (hits AC 5 <= AC 7) -> 2 dmg"`
  - The 5 saving throws (`PPDM 13 | RSW 14 | PP 12 | BW 16 | Spell 15`) dynamically resolve room traps, gas vents, and necromantic scrolls.
- **Engine Switchability**:
  With Dark Fort's multi-engine architecture, players can launch AD&D 2e mode at any time via `--engine adnd` or by setting `"engine": "adnd"` in `config.json`, bringing the golden age of CRPGs directly into the terminal roguelike.

---

## 14. Strategic Recommendations

| Priority / Preference | Recommended System | Why |
| :--- | :--- | :--- |
| **⭐ TOP FAVORITE / COMMUNITY WINNER** | **Mothership 1e (Fantasy Conversion)** | **The Ultimate Horror Roguelike Engine.** d100 roll-under with double-digit crits (`11, 22, 33...`), **Health + Wounds** (scars instead of HP bloat), **Stress & Panic** psychological tension, and pure doom-metal horror synergy. |
| **Max Flavor & Lore Purity** | **Full MÖRK BORG** | Canonical player-facing d20 system, Omens, rolled armor absorption, and the Misery Apocalypse clock. |
| **Tactical Roguelike Depth** | **Knave 2e** | The **Wounds clogging inventory slots** mechanic and **Splintering Shields** create the best survival choices. |
| **Pure Lethality & Zero-HP Tension** | **DURF** | **The Ablative Armor + Hit Dice Survival Check.** Eliminates traditional HP bloat; every wound past armor forces a mortality roll ($\sum \text{HD} \le \text{Wounds} \implies \text{Death}$), with Stress dynamically clogging inventory. |
| **CRPG Golden Age & THAC0 Nostalgia** | **AD&D 2nd Edition (Infinity Engine)** | **The Legendary Engine of Baldur's Gate & Icewind Dale.** Classic THAC0 attack resolution, descending AC ($10$ down to $-10$), 3–18 ability scores with non-linear bonus thresholds, and the 5-category saving throw matrix. |
| **Unified d20 Math & Blood Magic** | **Whitehack (4e)** | The **Blackjack roll-under** engine ($AC < \text{Roll} \le \text{AV}$) resolves hit, AC, and crits in a single comparison, while **HP-as-Mana** blood magic perfectly mirrors doom-metal sacrifice. |
| **Speed & Zero-Whiff Combat** | **Cairn / Into the Odd** | Attacks always deal damage; HP acts as poised buffer; excess damage burns into STR attribute. |
| **Dungeon Atmosphere & Tension**| **Shadowdark** | The strict **1-Hour Real-Time Torch Timer** turns darkness into pure survival terror. |
| **Minimalist Resource Tracking**| **The Black Hack 2e** | The **Usage Die ($U_d$)** step-down system ($d10 \to d8 \to d6 \to d4$) provides effortless resource tension. |
| **Procedural Chaos Magic** | **Maze Rats** | The infinite 2d6 word-combination spell matrix allows procedural generation of bizarre doom-metal horrors. |
| **Historical Dungeon Purity** | **Old-School Essentials** | Pristine 1981 B/X D&D simulation with strict 10-minute dungeon turns, reaction rolls, and gold-as-XP. |

---

## 15. Engine Integration & Multi-System Architecture

Dark Fort (Rust) features a pluggable multi-engine runtime architecture. Players can select their desired ruleset in `config.json` or pass the `--engine` command-line flag at launch:

```bash
cargo run -- --engine adnd         # AD&D 2nd Edition (THAC0, descending AC, 3-18 stats)
cargo run -- --engine durf         # DURF (Ablative armor, HD wound check, default)
cargo run -- --engine mothership   # Mothership 1e (d100 roll-under, Wounds, Stress)
cargo run -- --engine mork-borg    # Full MÖRK BORG (DR 12, rolled armor, active defense)
cargo run -- --engine classic      # Original 2018 Dark Fort micro-game
```

### Supported Runtime Engines:
1. **DURF Engine (`EngineType::Durf`)** [Default]:
   - Ablative armor pools soak damage point-for-point.
   - Wounds trigger Hit Dice mortality rolls ($\sum \text{HD} \le \text{Wounds} \implies \text{Death}$).
   - Stress points dynamically occupy inventory carrying slots.
2. **AD&D 2nd Edition Engine (`EngineType::Adnd2e`)**:
   - Descending AC (10 down to -10) with Leather AC 8, Chain AC 5, Plate AC 3.
   - THAC0 attack checks (`d20 >= THAC0 - Target AC`).
   - 3–18 ability scores with non-linear bonus thresholds (DEX AC adjustments, STR hit bonuses).
   - The 5 classical saving throw matrices (PPDM, RSW, PP, BW, Spells).
3. **Mothership 1e Fantasy Engine (`EngineType::Mothership`)**:
   - Percentile ($d100$) roll-under tests against Combat and Stats.
   - Double-digit crits (`11, 22, 33...`) on matching dice.
   - Multi-wound survival buffer (Wounds reset HP; bleeding/trauma upon wound gain).
   - Stress tracker with Panic Checks ($1d20 \le \text{Stress}$).
4. **Full MÖRK BORG Engine (`EngineType::MorkBorg`)**:
   - Player-facing DR 12 tests.
   - Active player defense rolls against monster strikes.
   - Rolled armor damage absorption ($1d2/1d4/1d6$).
   - Omens pool for rerolls and maximum damage.
5. **Classic Dark Fort (`EngineType::Classic`)**:
   - Original 2018 micro-game rules by Pelle Nilsson & Johan Nohr.




