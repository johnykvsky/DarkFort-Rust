# Dark Fort (Terminal ASCII Roguelike)

A faithful ASCII terminal roguelike adaptation in Rust of **Dark Fort** (2018) by Pelle Nilsson, David Hoskins, and Johan Nohr (Stockholm Kartell) — the minimalist micro-game that became *MÖRK BORG*.

---

## Quick Start

### Build & Run via Make
```bash
make              # Run the game (resumes existing save if present)
make new          # Start a brand new delve (ignoring existing save)
make theme-amber  # Play with retro Amber CRT palette
make theme-green  # Play with classic Green phosphor CRT palette
make test         # Run all 23 unit & integration tests
make sim          # Run 500 automated Monte-Carlo balance simulations
make clean        # Clean build artifacts and saved game
```

### Or via Cargo directly
```bash
cargo run --release
cargo run --release -- --help
cargo run --release -- --seed 1337
cargo run --release -- --theme amber    # mork-borg | amber | green
cargo run --release -- --headless-sim 500
```

---

## Terminal UI Overview

The terminal is divided into four responsive quadrants with authentic retro ASCII graphics:

```
+----------------------------------------------------------------------------------------------------+
| DARK FORT [v0.1.0]           Room #03: Oval Chamber               "The air is thick with damp rot" |
+--------------------------------------------------+-------------------------------------------------+
|                                                  | KARGUNT (The Catacomb Rogue, Lvl 0)             |
|                DUNGEON MAP                       | ----------------------------------------------- |
|                                                  | HP:     [██████████░░░░░] 10/15                 |
|                   [ E]                           | Silver: 19s  |  Points: 3/15  |  Explored: 3/12 |
|                    |                             | ----------------------------------------------- |
|              [#02]-[#03]                         | Weapon: Sword (+1 att)                          |
|                     |                            | Armor:  Leather Armor (-1d4 damage absorbed)    |
|                    [@]                           | ----------------------------------------------- |
|                    :?                            | Inventory:                                      |
|                                                  |  [1] Healing Potion (heals 1d6 hp)              |
|                                                  |  [2] Rope (+1 on trap rolls)                    |
|                                                  |  [3] Cloak of Invisibility (2 uses left)        |
+--------------------------------------------------+-------------------------------------------------+
| ENCOUNTER / CHAMBER                              | CHRONICLE & ACTIONS                             |
|                                                  |                                                 |
|          .-''''-.                                | > You push open the iron door into Room #3...   |
|         / _    _ \                               | > A Blood-drenched Skeleton stands guard!       |
|        | (o)  (o) |                              | > HIT! Attack 5 vs 3 points! You deal 4 damage. |
|         \   /\   /                               | > The Blood-drenched Skeleton collapses!        |
|                                                  | > Victory! Slew Blood-drenched Skeleton (+3 pts)|
|   BLOOD-DRENCHED SKELETON                        |                                                 |
|   Weak Monster | Points: 3 | HP: 6/6             | Choose a door direction or action:              |
|                                                  | [W/↑] North  [D/→] East  [S/↓] South  [A/←] West|
+--------------------------------------------------+-------------------------------------------------+
```

---

## Controls

### Exploration & Navigation
- **`W` / `Up` / `k`**: Move North through door
- **`D` / `Right` / `l`**: Move East through door
- **`S` / `Down` / `j`**: Move South through door
- **`A` / `Left` / `h`**: Move West through door
- **`P`**: Drink healing potion (heals `1d6` HP)
- **`T`**: Cycle color theme (*Classic White Terminal*, *Amber CRT*, *Green Phosphorus CRT*, *MÖRK BORG*)
- **`Q` / `Esc`**: Prompt to quit with confirmation (auto-saves progress)

### Combat Encounters
- **`A` / `Enter`**: Attack with equipped weapon (roll `1d6 + bonus` vs monster points)
- **`S`**: Cast offensive spell scroll (*Palms Open the Southern Gate*)
- **`C`**: Use charge of *Cloak of Invisibility* to slip past and gain monster points
- **`P`**: Quaff healing potion
- **`F`**: Flee fight (take `1d4` damage and retreat to previous chamber)
- **`Q` / `Esc`**: Prompt to quit with confirmation (auto-saves progress)

### Special Chambers
- **Pit Trap**: Press `[Space]` to jump (roll `1d6`; `+1` bonus if carrying Rope)
- **Riddling Soothsayer**: Press `[Space]` to answer (Odd = reward, Even = psychic shockwave)
- **Peddler from Beyond**: Press `1` (Potion, 4s), `2` (Random Scroll, 7s), `3` (Armor, 10s), or `X` (Exit)
- **Level Up**: Press `[Space]` to roll on the Level-Up table

---

## Rulebook Reference

Full transcribed rules and implementation details can be found in [`docs/rules_reference.md`](docs/rules_reference.md).
