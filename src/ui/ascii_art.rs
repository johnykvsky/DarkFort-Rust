pub const LOGO_BANNER: &str = r#"
 ██████╗  █████╗ ██████╗ ██╗  ██╗    ███████╗██████╗ ██████╗ ████████╗
 ██╔══██╗██╔══██╗██╔══██╗██║ ██╔╝    ██╔════╝██╔═══██╗██╔══██╗╚══██╔══╝
 ██║  ██║███████║██████╔╝█████╔╝     █████╗  ██║   ██║██████╔╝   ██║   
 ██║  ██║██╔══██║██╔══██╗██╔═██╗     ██╔══╝  ██║   ██║██╔══██╗   ██║   
 ██████╔╝██║  ██║██║  ██║██║  ██╗    ██║     ╚██████╔╝██║  ██║   ██║   
 ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝    ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   
             ~ The Catacomb Micro-Roguelike ~
"#;

pub const SKELETON_ART: &str = r#"
      .-''''-.
     / _    _ \
    | (o)  (o) |
     \   /\   /
     /'-....-'\
    / /|    |\ \
   /_/ |    | \_\
       /____\
      |  ||  |
      |__||__|
"#;

pub const CULTIST_ART: &str = r#"
       _/\_
      /    \
     | -  - |
     |  __  |
    /| \__/ |\
   / |======| \
  / /|      |\ \
    /|  /\  |\
   / |  ||  | \
     '--''--'
"#;

pub const GOBLIN_ART: &str = r#"
    (\____/)
    / @  @ \
   ( ==  == )
    )      (
   /        \
  / / |||| \ \
 ( (  ||||  ) )
      ====
      /  \
     (__)(__)
"#;

pub const HOUND_ART: &str = r#"
     /^ ^\
    / 0 0 \
    V\ Y /V
     / - \
    /|    |\
   ( |    | )
    \|____|/
     ||  ||
     ""  ""
"#;

pub const NECRO_SORCERER_ART: &str = r#"
       .---.
      /     \
     | () () |
     |  ===  |
    .-'-----'-.
   /  \  *  /  \
  | *  \   /  * |
  |     \_/     |
   \  Death-Ray/
    '---------'
"#;

pub const STONE_TROLL_ART: &str = r#"
     [######]
    [# -  - #]
   [##  ()  ##]
  [### ==== ###]
  #############
  #  #######  #
  #  #     #  #
 [][#]     [#][]
"#;

pub const MEDUSA_ART: &str = r#"
    S~S~S~S~S
   ( S S S S )
   | (o) (o) |
   |    ^    |
    \  ---  /
    /`~~~~~`\
   / / ~~~ \ \
  (_/       \_)
"#;

pub const BASILISK_ART: &str = r#"
     /)/)
    ( ..\_____
    /          \
   /  / \   / \ \
  /  (   ) (   ) )
 (____\ /___\ /__)
   ==   ==   ==
"#;

pub const PEDDLER_ART: &str = r#"
      ,----.
     / _  _ \
    ( <o><o> )
     \  __  /
    .-'----'-.
   / [TRINKETS]\
  |  [ STEEL ]  |
   \ [ COINS ] /
    '---------'
"#;

pub const SOOTHSAYER_ART: &str = r#"
       /\
      /  \
     / () \
    /______\
   |  o  o  |
   |   /\   |
    \  ~~  /
     '----'
"#;

pub const PIT_TRAP_ART: &str = r#"
   +--------------+
   |  \\  ||  //  |
   |   \\ || //   |
   |    \\||//    |
   |   (CHASM)    |
   |    //||\\    |
   |   // || \\   |
   +--------------+
"#;

pub const DEATH_ART: &str = r#"
         .---.
        /     \
       | () () |
        \  ^  /
         |||||
         '---'
     YOUR ADVENTURE
       ENDS HERE.
"#;

pub const VICTORY_ART: &str = r#"
        \O/
         |
        / \
    SIR KARGUNT RETIRES!
  UNTIL THE 7TH MISERY OCCURS
  AND EVERYTHING YOU KNOW
    BLACKENS AND BURNS.
"#;

pub fn get_art_for_monster(name: &str) -> &'static str {
    match name {
        "Blood-drenched Skeleton" => SKELETON_ART,
        "Catacomb Cultist" => CULTIST_ART,
        "Goblin" => GOBLIN_ART,
        "Undead Hound" => HOUND_ART,
        "Necro-Sorcerer" => NECRO_SORCERER_ART,
        "Small Stone Troll" => STONE_TROLL_ART,
        "Medusa" => MEDUSA_ART,
        "Ruin Basilisk" => BASILISK_ART,
        _ => SKELETON_ART,
    }
}
