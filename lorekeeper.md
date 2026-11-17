# LOREKEEPER.md — The Dark Fort Compendium & Style Bible

> *"The world dies. The dark fort stands, a monolithic tomb of petrified sins, awaiting one more carcass."*  
> — Fragment from the Basilisks' Catechism

---

## 1. Setting & Atmosphere

### The World
Dark Fort is set in the bruised, apocalyptic dying world of **MÖRK BORG**. The sun is blackened, the soil weeps bile, and prophecies of doom unfold with agonizing certainty. The Dark Fort itself is an ancient, shifting subterranean labyrinth of damp rot, fractured flagstones, and splintered iron doors. 

### Core Aesthetic Pillars
1. **Bleak, Fatalistic Grimdark**: Death is not an accident; it is the default outcome. Hope is an illusion, but stubborn survival is legendary.
2. **Visceral Medieval Grunge**: Rusty steel, bone marrow, sulfur fumes, clotting blood, mud-caked boots, tallow candles, and splintered shields.
3. **Laconic & Atmospheric Prose**: Messages in the Chronicle must be short, cutting, evocative, and darkly humorous without becoming comedic.

---

## 2. Dramatis Personae

### Kargunt — The Protagonist
- **Identity**: A desperate catacomb rogue, banished from the dying cities, delving into the Dark Fort for silver or a shallow grave.
- **Progression**:
  - Begins at **Level 0** with a few silver pieces, a random starting weapon, and 15 HP.
  - Every 15 progression points accumulated through slaughter and survival triggers a **Level Up Scratch-Off**.
  - Reaching **Level 6** fulfills the ultimate dark prophecy: Kargunt survives, achieves legendary notoriety, and retires in grim glory before the 7th Misery consumes the earth.

---

## 3. Bestiary of the Catacombs

> *Note on Content Modding*: In-game narrative descriptions and stats for monsters live in [`data/enemies/weak.json`](data/enemies/weak.json) and [`data/enemies/tough.json`](data/enemies/tough.json). Equipment and scroll flavor texts reside in [`data/items/`](data/items/).

### Weak Monsters
- **Blood-Drenched Skeleton**: Ancient bone re-animated by fresh slaughter, slick with crimson and burning with undead fury. (Loot: Daggers)
- **Catacomb Cultist**: Flagellant ascetics chanting praises to forgotten gods in the dark corridors. (Loot: Scrolls)
- **Goblin**: Spiteful, venomous wretches lurking in the damp gloom. They bite, shriek, and scavenge dead adventurers. (Loot: Hemp Rope)
- **Undead Hound**: Emaciated, skinless beasts haunting the outer chambers, driven by rabies and hunger.

### Tough Monsters & Eldritch Terrors
- **Necro-Sorcerer**:
  - *Lore*: A robed arch-heretic sustaining himself on the decay of the fort.
  - *Special Mechanics*: Alternates standard attacks with a lethal **Death Ray**. Upon death, releases a **Maggot Hazard** curse that threatens to end the delve.
- **Small Stone Troll**:
  - *Lore*: A hulking entity of calcified riverstone and crushed gravel, nearly impervious to mundane steel.
  - *Special Mechanics*: Boasts exceptional defense (5+ points to hit), but bestows a massive **7 progression points** upon death.
- **Medusa**:
  - *Lore*: A crowned terror of hissing vipers and sorrowful stone gazes.
  - *Special Mechanics*: Guards hoards of silver (`1d4 × 1d6`), but threatens instantaneous petrification to salt upon defeat.
- **Ruin Basilisk**:
  - *Lore*: A primordial, scaled horror whose gaze turns flesh to ash.
  - *Special Mechanics*: Slashing its throat releases an ancient black ichor that floods Kargunt's veins, granting a 2-in-6 chance of an **Immediate Level Up**.

---

## 4. Arcane Scrolls & Esoterica

- **Palms Open the Southern Gate**:
  - *Incantation*: Kargunt invokes the sulfurous breach of the southern hells.
  - *Manifestation*: White-hot hellfire streams from scarred palms, dealing unblockable damage directly to the foe.
- **Summon Weak Daemon**:
  - *Incantation*: A pact etched in ash summoning a weeping, vicious lesser imp.
  - *Manifestation*: The daemon assists in combat for 1d4 encounters before dissolving into greasy black smoke.
- **Aegis of Sorrow**:
  - *Incantation*: A woven shroud of spectral despair that bends incoming steel away from Kargunt's flesh.
  - *Manifestation*: Absorbs 1d4 damage per strike for up to 1d4 uses.
- **False Omen**:
  - *Incantation*: A fleeting gaze through a cracked prophetic mirror.
  - *Manifestation*: Allows Kargunt to bend the dungeon's procedural destiny, dictating the outcome of a room.

---

## 5. Other Denizens

- **The Riddling Soothsayer**: A blind, hooded oracle hovering in a silent chamber. Answers given by rolling the d6 determine whether Kargunt is blessed with arcane gifts or cursed with torment.
- **The Peddler**: A hollow-eyed merchant traversing dimensional fissures, willing to trade potions, scrolls, and armor for cold silver coins.

---

## 6. Narrative Tone & Writing Guide for the Chronicle

When generating or editing in-game text, messages, or chronicle logs, follow these principles:

| Context | Good Example | Avoid |
| :--- | :--- | :--- |
| **Room Entry** | *"The chamber rears open, smelling of old slaughter."* | *"You entered a new room."* |
| **Combat Hit** | *"Steel bites deep into marrow. 4 damage dealt."* | *"You hit the enemy for 4 damage."* |
| **Combat Miss** | *"Your blade clatters off petrified bone."* | *"Attack failed."* |
| **Fleeing** | *"You panic and scramble through the iron door, bleeding."* | *"You ran away successfully."* |
| **Level Up** | *"Blood awakens your cursed potential. A new power stirs."* | *"Level up achieved!"* |
| **Death** | *"Darkness swallows your eyes. The fort claims another skull."* | *"You died. Game over."* |

---

## 7. Permanent Rules Reference Links

- Data Files: [`data/`](data/) (`items/`, `enemies/`, `shop.json`)
- Technical Architecture: [`docs/architecture.md`](docs/architecture.md)
- OSR Mechanics Comparison: [`docs/osr_mechanics_comparison.md`](docs/osr_mechanics_comparison.md)
- Rules & Tables: [`docs/rules_reference.md`](docs/rules_reference.md)
- Agent Instructions: [`AGENTS.md`](AGENTS.md)
