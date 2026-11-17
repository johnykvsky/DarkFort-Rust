# AGENTS.md — Developer & AI Agent Guidelines for Dark Fort (Rust)

> **Purpose**: This file contains permanent architectural rules, design constraints, and operational guidelines for AI coding agents and human developers working on the Dark Fort codebase.

---

## 1. Project Overview

**Dark Fort (Rust)** is a terminal-native, ASCII roguelike adaptation of the 2018 micro-game *Dark Fort* by Pelle Nilsson, David Hoskins, and Johan Nohr (the game that evolved into *MÖRK BORG*).

- **Language**: Rust (2021 Edition)
- **TUI Framework**: `ratatui` (v0.28) + `crossterm` (v0.28)
- **RNG**: `rand` (v0.8) with ChaCha12 CSPRNG
- **Persistence**: `serde` + `serde_json` (atomic manual save/load)

---

## 2. Core Architectural Invariants

### 2.1 Pure Engine Separation (Crucial)
- **Rule**: `src/core/` and `src/dungeon/` must remain **100% free of UI dependencies** (`ratatui`, `crossterm`).
- All game mechanics (combat, stats, inventory, dungeon map generation, progression) are purely functional and state-driven.
- Headless simulation (`src/simulation.rs`) and automated balance testing depend on this separation to run thousands of delves without a TUI.

### 2.2 Random Number Generation (`DiceRoller`)
- **Rule**: Never call ad-hoc random functions (e.g. `rand::random()`, modulo arithmetic) in game mechanics.
- Always use the [`DiceRoller`](src/core/dice.rs) trait:
  - **Live Gameplay**: [`StandardRoller`](src/core/dice.rs) wrapping `rand::thread_rng()` (ChaCha12 CSPRNG with unbiased rejection sampling).
  - **Simulations**: `StandardRoller` wrapping `rand::rngs::StdRng` (seeded ChaCha12).
  - **Unit Tests**: [`MockRoller`](src/core/dice.rs) seeded with predetermined roll vectors for 100% deterministic testing.

### 2.3 Explicit State Machine (`AppState`)
- All interaction flows through [`AppState`](src/app.rs):
  - `Exploring` $\to$ `Combat(Monster)` $\to$ `PitTrapPending` $\to$ `SoothsayerPrompt` $\to$ `PeddlerShop` $\to$ `LevelUpPending` $\to$ `GameOver` / `Victory`.
- **Modal Overlays**: Modal states like `QuitConfirmation(Box<AppState>)` **must box and preserve** the previous state so that canceling or resuming restores the exact game context.

### 2.4 Persistence Rules (Manual Only)
- **No Background Auto-saves**: Exploring chambers, buying items, or winning battles does **not** auto-save to disk.
- **Intentional Save**: `Q/Esc` $\to$ `[S]` saves to `savegame.json` and resumes.
- **Intentional Load**: `Q/Esc` $\to$ `[L]` checks if `StorageManager::exists(path)` before attempting to read. If not found, notifies the player and safely cancels without modifying state.
- **No Auto-load on Launch**: The game always starts a fresh delve upon launch.
- **Permadeath**: Player death (`GameOver`) deletes `savegame.json` immediately.
- **Thread Safety in Tests**: When testing save/load, use isolated temporary paths (e.g., `target/test_xyz.json`) via `save_game_to` and `load_game_from` to prevent parallel test runner collisions.

### 2.5 Terminal UI & Modal Rendering
- **Background Clearing**: All popups/modal dialogs (`render_quit_dialog`) must render `ratatui::widgets::Clear` over their `Rect` before rendering borders to prevent underlying content from showing through.
- **Theme Awareness**: The quit/pause dialog must use `theme.quit_border` and `theme.quit_fg`:
  - **Amber** in `ClassicWhite` theme for high contrast against white text/borders.
  - **White** in other themes (`AmberCrt`, `GreenCrt`, `MorkBorg`).
- **Responsive Heights**: Never hardcode small fixed heights that truncate text lines. Build dynamic line arrays and keep choice options (`[Y]`, `[S]`, `[L]`, `[N]`) visible across all terminal dimensions.
- **Prompt Synchronization**: Whenever a modal or state changes, synchronize the chronicle prompt and hotkey bar in `App::draw` with the keyboard bindings in `App::handle_input`.

### 2.6 Data-Driven Registries & Embedded Fallbacks
- **Rule**: All equipment, monsters, and shop wares must remain externalized in `data/` (`data/items/*.json`, `data/enemies/*.json`, `data/shop.json`).
- Game mechanics and starting rolls dynamically query `ItemRegistry`, `EnemyRegistry`, and `ShopConfig`.
- **Zero-Panic Fallback Invariant**: Registries must always retain compile-time embedded copies via `include_str!` (`default_embedded()`). If external JSON files cannot be found on disk (e.g. running in isolated test tempdirs or moving binaries), the game falls back gracefully without panicking.
- Never hardcode new weapon, item, or enemy stat definitions directly into Rust enum matches without updating the registry and corresponding JSON files.

---

## 3. Directory Navigation & Key Files

- [`data/`](data/): External JSON definitions (`items/`, `enemies/`, `shop.json`).
- [`src/main.rs`](src/main.rs): Application entry point, CLI arguments (`--seed`, `--theme`, `--headless-sim`), terminal initialization.
- [`src/app.rs`](src/app.rs): State machine, input handler, and game loop coordinator.
- [`src/core/registry.rs`](src/core/registry.rs): Data loaders, dynamic entity definitions (`WeaponDef`, `ArmourDef`, `MonsterDef`, `ShopWare`), and embedded fallbacks.
- [`src/core/combat.rs`](src/core/combat.rs): Pure combat calculations and damage absorption.
- [`src/core/monsters.rs`](src/core/monsters.rs): Monster stats, abilities, and loot tables.
- [`src/dungeon/map.rs`](src/dungeon/map.rs): Dungeon topology, planar coordinate grid, and wandering monster ambushes.
- [`src/ui/renderer.rs`](src/ui/renderer.rs): 4-quadrant layout engine, modals, and screen compositor.
- [`src/ui/theme.rs`](src/ui/theme.rs): Theme definitions and color tokens.
- [`docs/architecture.md`](docs/architecture.md): Living technical reference document.
- [`docs/osr_mechanics_comparison.md`](docs/osr_mechanics_comparison.md): Comparative evaluation of alternative OSR mechanics (MÖRK BORG, Knave 2e, Cairn, Black Hack, Shadowdark, OSE).
- [`lorekeeper.md`](lorekeeper.md): World atmosphere, creature lore, and narrative tone guidelines.

---

## 4. Testing & Quality Verification

Before committing any code changes:
1. Ensure `cargo test` passes all tests cleanly:
   ```bash
   cargo test
   ```
2. Check for compiler warnings:
   ```bash
   cargo check
   ```
3. Run Monte Carlo simulation smoke test:
   ```bash
   cargo run --release -- --headless-sim 50
   ```
