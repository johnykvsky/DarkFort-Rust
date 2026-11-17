use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dark_fort::app::App;
use dark_fort::core::config::{EngineType, GameConfig};
use dark_fort::simulation::HeadlessSimulator;
use dark_fort::ui::theme::ThemeMode;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;
use std::io;
use std::panic;

fn print_help() {
    println!("DARK FORT — Terminal ASCII Roguelike");
    println!();
    println!("USAGE:");
    println!("    dark-fort [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --engine <NAME>         Ruleset: 'durf' (default), 'mork-borg', 'mothership', 'adnd', or 'classic'");
    println!("    --scifi                 Enable Sci-Fi Derelict Ship theme (Aliens, Pulse Rifles, Alien Queen)");
    println!("    --seed <NUMBER>         Seed RNG for deterministic procedural delve");
    println!("    --theme <THEME>         Visual theme: 'white' (default), 'amber', 'green', 'mork-borg', or 'scifi'");
    println!("    --new                   Start a fresh delve even if a savegame exists");
    println!("    --headless-sim <COUNT>  Run N automated Monte-Carlo delves without opening UI");
    println!("    -h, --help              Print this help message");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = GameConfig::load_or_default();

    let mut engine_type = config.engine;
    let mut seed: Option<u64> = config.seed;
    let mut is_scifi = config.is_scifi();
    let mut theme_mode = if is_scifi {
        ThemeMode::SciFiVoid
    } else {
        match config.theme.as_deref() {
            Some("amber") => ThemeMode::AmberCrt,
            Some("green") => ThemeMode::GreenCrt,
            Some("mork-borg") | Some("morkborg") => ThemeMode::MorkBorg,
            Some("scifi") | Some("scifi-void") | Some("alien") | Some("sf") => {
                is_scifi = true;
                ThemeMode::SciFiVoid
            }
            _ => ThemeMode::ClassicWhite,
        }
    };
    let mut _force_new = false;
    let mut headless_sim_count: Option<u32> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--engine" => {
                if i + 1 < args.len() {
                    if let Some(et) = EngineType::parse_str(&args[i + 1]) {
                        engine_type = et;
                    }
                    i += 1;
                }
            }
            "--scifi" | "--sf" | "--alien" => {
                is_scifi = true;
                theme_mode = ThemeMode::SciFiVoid;
            }
            "--seed" => {
                if i + 1 < args.len() {
                    seed = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--theme" => {
                if i + 1 < args.len() {
                    theme_mode = match args[i + 1].to_lowercase().as_str() {
                        "amber" => ThemeMode::AmberCrt,
                        "green" => ThemeMode::GreenCrt,
                        "mork-borg" | "morkborg" => ThemeMode::MorkBorg,
                        "scifi" | "scifi-void" | "alien" | "sf" => {
                            is_scifi = true;
                            ThemeMode::SciFiVoid
                        }
                        _ => ThemeMode::ClassicWhite,
                    };
                    i += 1;
                }
            }
            "--headless-sim" => {
                if i + 1 < args.len() {
                    headless_sim_count = args[i + 1].parse().ok();
                    i += 1;
                } else {
                    headless_sim_count = Some(100);
                }
            }
            "--new" => {
                _force_new = true;
            }
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    // Headless simulation mode
    if let Some(count) = headless_sim_count {
        println!("Running {} automated Dark Fort [{}] Monte-Carlo simulations...", count, engine_type.name());
        let base_seed = seed.unwrap_or(1337);
        let stats = HeadlessSimulator::run_simulations_with_engine(count, base_seed, engine_type);
        stats.print_summary();
        return Ok(());
    }

    // Setup clean panic hook to prevent leaving terminal in broken raw state
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Always start a fresh delve on launch; saves can be loaded in-game via quit/pause menu (L)
    let mut app = App::new_with_engine_and_theme(engine_type, seed, is_scifi, theme_mode);

    // Main Game Loop
    while !app.should_quit {
        terminal.draw(|f| app.draw(f))?;
        app.handle_input()?;
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("Farewell, Kargunt. Until the 7th Misery occurs...");
    Ok(())
}
