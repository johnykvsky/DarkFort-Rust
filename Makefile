# Dark Fort (Rust) — Terminal ASCII Roguelike
# Makefile for building, running, testing, and simulating

CARGO ?= $(shell which cargo 2>/dev/null || echo $(HOME)/.cargo/bin/cargo)

.PHONY: all run run-debug build test lint sim clean help \
	engine-durf engine-mothership engine-morkborg engine-adnd engine-classic \
	scifi run-scifi theme-scifi theme-white theme-amber theme-green theme-morkborg \
	sim-durf sim-mothership sim-morkborg sim-adnd sim-classic new

all: run

## Run the default configured engine (DURF)
run:
	@$(CARGO) run --release

## Run with Sci-Fi Derelict Ship theme & Mothership 1e engine (Aliens, Pulse Rifles, Alien Queen)
scifi:
	@$(CARGO) run --release -- --engine mothership --scifi

run-scifi: scifi

## Run with a fresh game (ignores existing save)
new:
	@$(CARGO) run --release -- --new

# ------------------------------------------------------------------------------
# Tabletop RPG Engine Launch Targets
# ------------------------------------------------------------------------------

## Launch DURF engine (Ablative armor, Hit Dice mortality check)
engine-durf:
	@$(CARGO) run --release -- --engine durf

## Launch Mothership 1e engine (d100 roll-under, Wounds, Stress & Panic)
engine-mothership:
	@$(CARGO) run --release -- --engine mothership

## Launch Full MÖRK BORG engine (d20 DR 12, rolled armor, active defense)
engine-morkborg:
	@$(CARGO) run --release -- --engine mork-borg

## Launch AD&D 2nd Edition engine (THAC0, descending AC, 3-18 stats, 5 save tables)
engine-adnd:
	@$(CARGO) run --release -- --engine adnd

## Launch Classic Dark Fort (2018 micro-game rules)
engine-classic:
	@$(CARGO) run --release -- --engine classic

# ------------------------------------------------------------------------------
# Visual Theme Launch Targets
# ------------------------------------------------------------------------------

## Run with Sci-Fi Void CRT theme (Cyan/Amber HUD)
theme-scifi:
	@$(CARGO) run --release -- --theme scifi

## Run with classic White terminal theme (default)
theme-white:
	@$(CARGO) run --release -- --theme white

## Run with Retro Amber CRT theme
theme-amber:
	@$(CARGO) run --release -- --theme amber

## Run with Phosphor Green CRT theme
theme-green:
	@$(CARGO) run --release -- --theme green

## Run with MÖRK BORG theme (Yellow/Red)
theme-morkborg:
	@$(CARGO) run --release -- --theme mork-borg

# ------------------------------------------------------------------------------
# Development, Testing, and Simulation
# ------------------------------------------------------------------------------

## Run debug build
run-debug:
	@$(CARGO) run

## Build optimized release binary (target/release/dark-fort)
build:
	@$(CARGO) build --release

## Run test suite
test:
	@$(CARGO) test

## Check code quality and clippy lints
lint:
	@$(CARGO) clippy --all-targets

## Run 500 automated Monte-Carlo delves (default engine)
sim:
	@$(CARGO) run --release -- --headless-sim 500

sim-durf:
	@$(CARGO) run --release -- --engine durf --headless-sim 500

sim-mothership:
	@$(CARGO) run --release -- --engine mothership --headless-sim 500

sim-morkborg:
	@$(CARGO) run --release -- --engine mork-borg --headless-sim 500

sim-adnd:
	@$(CARGO) run --release -- --engine adnd --headless-sim 500

sim-classic:
	@$(CARGO) run --release -- --engine classic --headless-sim 500

## Clean build artifacts and local saves
clean:
	@$(CARGO) clean
	@rm -f savegame.json target/test_*.json
	@echo "Cleaned build artifacts and save files."

## Show available commands
help:
	@echo "Dark Fort — Terminal ASCII Roguelike"
	@echo ""
	@echo "Available commands:"
	@echo "  make                   - Run the game (default engine)"
	@echo "  make scifi             - Run Mothership 1e in Sci-Fi Derelict Ship mode"
	@echo ""
	@echo "Engines:"
	@echo "  make engine-durf       - Run DURF (Ablative armor, HD mortality check)"
	@echo "  make engine-mothership - Run Mothership 1e (d100 roll-under, Wounds, Stress)"
	@echo "  make engine-morkborg   - Run Full MÖRK BORG (DR 12, rolled armor, active defense)"
	@echo "  make engine-adnd       - Run AD&D 2nd Edition (THAC0, descending AC, 3-18 stats)"
	@echo "  make engine-classic    - Run Classic Dark Fort (2018 micro-game)"
	@echo ""
	@echo "Themes:"
	@echo "  make theme-scifi       - Play with Cyan/Amber Sci-Fi Void HUD"
	@echo "  make theme-white       - Play with White terminal palette (default)"
	@echo "  make theme-amber       - Play with retro Amber CRT palette"
	@echo "  make theme-green       - Play with classic Green phosphor CRT palette"
	@echo "  make theme-morkborg    - Play with Yellow/Red MÖRK BORG palette"
	@echo ""
	@echo "Development & Testing:"
	@echo "  make build             - Compile optimized release binary"
	@echo "  make test              - Run all 66 unit and integration tests"
	@echo "  make lint              - Run cargo clippy code checks"
	@echo "  make sim               - Run 500 automated Monte-Carlo balance simulations"
	@echo "  make sim-adnd          - Run 500 AD&D 2e Monte-Carlo simulations"
	@echo "  make sim-mothership    - Run 500 Mothership 1e Monte-Carlo simulations"
	@echo "  make clean             - Remove build artifacts and save files"
	@echo "  make help              - Show this help message"
