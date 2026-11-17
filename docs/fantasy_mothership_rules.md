# TOMB-SHIP: A Mothership 1e Dark Fantasy Horror Ruleset

> *"Abandon light, all ye who crawl below. The void above has sunk beneath the stone."*

---

## 1. Core Philosophy & System Overview

**TOMB-SHIP** is a survival horror tabletop roleplaying game engine translating the tense, panic-driven mechanics of **Mothership 1st Edition** into a brutal, claustrophobic dark fantasy dungeon crawl. 

There are no gleaming paladins or heroic power fantasies here. Player characters are desperate trench-veterans, disgraced scholars, alchemical wretches, and cutpurse grave-robbers venturing into pitch-black subterranean vaults, forgotten ossuaries, and blasphemous ruins in search of ancient coin and cursed relics.

### The Five Invariants of Survival Horror
1. **Percentile Roll-Under ($d100 \le \text{Stat}$)**: All resolution is instant and intuitive. If your Combat is `38%`, rolling 38 or less succeeds.
2. **Double-Digit Criticals**: Any roll where both dice match (**11, 22, 33, 44, 55, 66, 77, 88, 99, 00**) is an instant dramatic event. If $\le \text{Stat}$, it is a **Critical Success**. If $> \text{Stat}$, it is a **Catastrophic Failure**. Rolling `00` is always an utter disaster.
3. **Health + Wounds (Anti-HP Bloat)**: Characters do not possess ever-growing pools of hit points. Characters have modest **Health** (10–18 HP) and 2–3 **Wounds**. When Health reaches 0, you do not die immediately—you take 1 **Wound**, roll on a mutilating **Wound Table**, and reset your Health. When all Wounds are consumed, you die.
4. **Armor Points (AP)**: Armor does not make you harder to hit; it directly reduces damage dealt to your Health.
5. **The Stress & Panic Spiral**: Fear is tangible. Surviving traps, suffering critical hits, running out of torches, and casting occult rites accumulate **Stress**. When terror strikes, a **Panic Check** tests whether you maintain discipline or succumb to screaming tremors, frenzy, or heart failure.

---

## 2. Character Creation

Generating a character takes five minutes. Grab a pair of percentile dice ($d100$), a $d20$, and a set of polyhedrals ($d4, d6, d8, d10$).

```
[1. Roll Stats & Saves] ──> [2. Choose Profession] ──> [3. Pick Skills] ──> [4. Roll Health & Gear]
```

### Step 1: Core Stats & Saves
Roll **$2d10 + 25$** for each of the four core Stats (range 27–45):

| Stat | Description | When to Test |
| :--- | :--- | :--- |
| **Strength (STR)** | Physical power, brawn, endurance, carrying capacity | Prying sarcophagi, forcing portcullises, wrestling |
| **Agility (AGI)** | Speed, reflexes, dexterity, balance, stealth | Dodging falling portcullises, balancing over chasms, silent walking |
| **Occult (OCC)** | Lore, forbidden languages, mental fortitude, relic use | Deciphering dead runes, banishing spirits, identifying relics |
| **Combat (COM)** | Weapon precision, melee brawling, ranged archery | Striking foes with blade, bow, flail, or cudgel |

Roll **$2d10 + 10$** for each of the three Saves (range 12–30):

| Save | Description | Trigger |
| :--- | :--- | :--- |
| **Flesh Save** | Bodily resilience and immune vigor | Resisting necro-rot, spider venom, pit gas, blood loss |
| **Courage Save** | Raw nerve, bravery, and composure | Staring down abominations, witnessing gruesome deaths, ambushes |
| **Mind Save** | Mental sanity, willpower, reality anchor | Resisting psychic gaze, psychic possession, mind-bending void geometry |

---

## 3. The Four Dark Professions

Choose or roll $1d4$ for your character's calling.

```
1. Sellsword (The Veteran of Ruined Wars)
2. Homunculus (The Alchemical Construct)
3. Scholar (The Relic-Seeker & Heretic)
4. Grave-Robber (The Catacomb Delver)
```

### 1. The Sellsword (Marine Equivalent)
*A hardened mercenary whose company was slaughtered in the mud. You know only steel, mud, and survival.*

- **Stat Modifiers**: $+10\text{ Combat}$, $+10\text{ Strength}$, $+5\text{ Flesh Save}$.
- **Base Resilience**: **Health**: $2d10 + 12$ | **Wounds**: 3 | **Min Stress**: 2
- **Class Trait — Battle-Scarred Discipline**: Whenever a companion within earshot makes a Panic Check, the Sellsword may bellow orders; the companion rolls with **Advantage**. Additionally, once per delve, when reduced to 0 Health, the Sellsword may immediately make a counter-attack before rolling on the Wound Table.
- **Starting Skills**: Gutting Blades (+10%), Athletics (+10%), Wilderness Survival (+10%).
- **Starting Equipment**:
  - Arming Sword ($1d10$ damage) or Halberd ($2d10$ damage, Two-Handed, Reach).
  - Chainmail (AP 5, Heavy).
  - Round Shield (+1 AP, Splinterable), Dagger ($1d6$).
  - Whetsone, 3 Iron Rations, Waterskin of sour wine, 2 Tallow Torches.
  - Belt pouch with $2d10$ Silver Pieces.

---

### 2. The Homunculus (Android Equivalent)
*An artificial soul molded from clay, quicksilver, and forbidden vitae in an alchemist's crucible. You feel no physical exhaustion, but your hollow gaze terrifies natural creatures.*

- **Stat Modifiers**: $+15\text{ Occult}$, $+5\text{ Agility}$, $-10\text{ to all companions' Courage Saves if unmasked}$.
- **Base Resilience**: **Health**: $2d10 + 10$ | **Wounds**: 3 | **Min Stress**: 1
- **Class Trait — Synthetic Anatomy**:
  - **Immune** to mundane diseases, poisons, drowning, and suffocating pit gases.
  - **Cold Logic**: Panic Checks are rolled on a **$1d10$** instead of a $d20$, but failure results in mechanical catatonia, circuit-spasms, or cold logic paralysis.
  - **Repairs**: Cannot be healed by mundane bandages or food; requires alchemical quicksilver, forge tools, and an Occult/Smithing check to repair Health and Wounds.
- **Starting Skills**: Alchemical Lore (+10%), Repair & Smithing (+10%), Anatomy (+10%).
- **Starting Equipment**:
  - Heavy Crowbar ($1d8$ damage, Bludgeoning, +10% to force doors).
  - Quicksilver Infused Gambeson (AP 2).
  - Alchemist's Field Kit (Vials, tweezers, burner, 2 flasks of quicksilver).
  - Hooded Lantern, 2 Flasks of Lamp Oil, Slate Tablet & Chalk.
  - Leather pouch with strange lead slugs and $1d10$ Silver Pieces.

---

### 3. The Scholar (Scientist Equivalent)
*An expelled academic or defrocked cleric obsessed with forbidden tomes and cyclopean architecture.*

- **Stat Modifiers**: $+15\text{ Occult}$, $+10\text{ Mind Save}$, $+5\text{ Courage Save}$.
- **Base Resilience**: **Health**: $2d10 + 8$ | **Wounds**: 2 | **Min Stress**: 2
- **Class Trait — Forbidden Intuition**: Whenever the Scholar encounters ancient glyphs, strange contraptions, or eldritch relics, they may test Occult to deduce its purpose, function, and danger without triggering it. Furthermore, the Scholar begins play knowing **1 Occult Ritual**.
- **Starting Skills**: Decipher Glyphs (+10%), Ancient Tongues (+10%), Chirurgery (+10%), Occult Relics (+10%).
- **Starting Equipment**:
  - Bone Dagger ($1d6$ damage, Concealed) or Walking Quarterstaff ($1d8$ damage).
  - Waxed Leather Surcoat (AP 1).
  - Vellum Journal, Charcoal Sticks, Magnifying Loupe, Holy Symbol or Astrolabe.
  - 1 Flask of Holy Oil, 3 Wax Candles (silent, low light), Tinderbox.
  - Velvet satchel containing $3d10$ Silver Pieces and 1 Forbidden Parchment Scroll.

---

### 4. The Grave-Robber (Teamster Equivalent)
*A rat-catcher, sewer-jack, or tomb-plunderer who works the deep wet dark for bread and silver.*

- **Stat Modifiers**: $+10\text{ Agility}$, $+5\text{ Strength}$, $+5\text{ Flesh Save}$, $+5\text{ Mind Save}$.
- **Base Resilience**: **Health**: $2d10 + 10$ | **Wounds**: 2 | **Min Stress**: 2
- **Class Trait — Gut Instinct**: Once per delve session, when the Grave-Robber fails any Save or Stat check, they may immediately re-roll it by trusting their scavenger senses. Additionally, when searching a chamber, they discover hidden features in half the normal time.
- **Starting Skills**: Trapcraft & Locks (+10%), Caving & Tunneling (+10%), Stealth (+10%).
- **Starting Equipment**:
  - Heavy Pickaxe ($1d10$ damage, Piercing, destroys locks) or Shortbow ($1d8$ damage, 20 Arrows).
  - Boiled Leather Jerkin (AP 3).
  - Set of Thieves' Picks, 50-foot Hemp Rope, Grappling Hook, 10 Iron Spikes.
  - 3 Tallow Torches, Scent-masking grease, Sack of dry flour (reveals invisible steps/air drafts).
  - Belt pouch with $1d20$ Silver Pieces.

---

## 4. Skills & Progression

Skills provide a flat bonus added directly to the relevant Stat when performing specialized tasks:
- **Trained**: $+10\%$ to relevant Stat.
- **Expert**: $+15\%$ to relevant Stat (requires prerequisite Trained skill).
- **Master**: $+20\%$ to relevant Stat (requires prerequisite Expert skill).

```
                      ┌──> Traps & Locks ───> Siege Engineering
                      │
Athletics ────────────┼──> Caving & Spelunking ──> Deep Cartography
                      │
                      └──> Gutting Blades ───> Flensing Mastery

                      ┌──> Chirurgery ──────> Necromantic Anatomy
                      │
Alchemical Lore ──────┼──> Sarcophagus Chemistry ──> Void Metallurgy
                      │
                      └──> Decipher Glyphs ──> Forbidden Sorcery
```

### Skill Catalog

| Skill Name | Tier | Governed By | Typical Uses |
| :--- | :---: | :---: | :--- |
| **Athletics** | Trained | STR / AGI | Climbing ropes, leaping chasms, sprinting, swimming icy water |
| **Caving & Spelunking** | Trained | STR / AGI | Squeezing through fissures, anchoring pitons, navigating stone |
| **Gutting Blades** | Trained | COM | Precision strikes with daggers, shortswords, and scythes |
| **Traps & Locks** | Trained | AGI | Detecting tripwires, picking padlocks, disarming deadfalls |
| **Decipher Glyphs** | Trained | OCC | Reading dead languages, tomb inscriptions, warding runes |
| **First Aid & Chirurgery** | Trained | OCC | Bandaging gaping wounds, setting broken bones, extracting arrowheads |
| **Stealth & Prowl** | Trained | AGI | Moving silently across bone-strewn tiles, slipping past sentries |
| **Wilderness Survival** | Trained | STR / OCC | Tracking, foraging safe fungus, building barricades |
| **Anatomy & Flensing** | Expert | OCC / COM | Harvesting monster venom/glands, targeting anatomical weakpoints |
| **Sarcophagus Chemistry** | Expert | OCC | Neutralizing tomb acids, brewing blinding powders, identifying poison |
| **Occult Relics** | Expert | OCC | Activating cursed talismans without paying blood tithe |
| **Forbidden Sorcery** | Master | OCC | Channeling outer void miracles; ritual incantations |
| **Void Geometry** | Master | OCC | Navigating non-Euclidean dungeons; detecting spatial rifts |

---

## 5. Equipment, Armory & Inventory Management

### Inventory Slots
To preserve survival tension without tedious weight bookkeeping, TOMB-SHIP uses an **Inventory Slot System**:
- A character has **$\text{Strength} / 10 + 2$** Inventory Slots (typically 5 to 7 slots).
- Worn clothing and small belt trinkets (chalk, silver coin purse, single dagger) do not consume slots.
- Standard items (shield, rope, 3 torches, crowbar) take **1 slot**.
- Heavy/Bulky items (two-handed polearms, chainmail, heavy chest) take **2 slots**.
- **Encumbrance**: Carrying more than your maximum slots imposes **Disadvantage** on all Agility and Combat tests, and increases Stress by $+1$ each exploration turn.

### Weapons Table

| Weapon | Damage | Slots | Cost | Special Qualities |
| :--- | :---: | :---: | :---: | :--- |
| **Dagger / Dirk** | $1d6$ | 0 (Belt) | 3 sp | Concealable, throwable (Close). On Crit: bypasses non-magical AP. |
| **Arming Sword** | $1d10$ | 1 | 15 sp | Versatile. Critical hits sever fingers or inflict deep bleeding. |
| **Spiked Flail** | $1d10$ | 1 | 12 sp | Ignores shield AP. On Crit: wrap around weapon and disarm foe. |
| **Heavy Pickaxe** | $1d10$ | 1 | 8 sp | Piercing (ignores 2 points of Armor). Useful for tunneling. |
| **Warhammer / Mace**| $1d10$ | 1 | 10 sp | Bludgeoning. Deals double damage against skeletal undead. |
| **Halberd / Glaive** | $2d10$ | 2 | 25 sp | Reach, Two-Handed. Attacker strikes first in narrow hallways. |
| **Greatsword / Flammenschwert** | $2d10$ | 2 | 30 sp | Brutal, Two-Handed. On Crit: cleaves into an adjacent foe. |
| **Shortbow** | $1d8$ | 1 | 20 sp | Ranged (Medium), Requires Quiver (1 slot, holds 20 arrows). |
| **Heavy Crossbow** | $2d8$ | 2 | 35 sp | Piercing (ignores 3 AP). Requires 1 full turn to reload. |

### Armor & Protection

| Armor Type | Armor Points (AP) | Slots | Cost | Drawbacks & Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Waxed Jerkin** | AP 1 | 0 | 5 sp | Light, silent, water-repellent. |
| **Boiled Leather** | AP 3 | 1 | 15 sp | Flexible, standard dungeon delve protection. |
| **Chainmail Hauberk** | AP 5 | 2 | 40 sp | Heavy. Clinking metal imposes **Disadvantage** on Stealth. |
| **Black Fluted Plate**| AP 7 | 3 | 120 sp | Bulky. Disadvantage on Stealth and Swimming. Incurable drowning. |
| **Round Shield** | +1 AP | 1 | 10 sp | **Splinterable**: When taking a lethal blow or Wound, you may choose to permanently shatter the shield to negate all damage from that single attack! |
| **Bascinet / Sallet** | +1 AP (Head) | 1 | 12 sp | Protects against falling rocks and decapitation crits. |

---

## 6. The Rules of Blood & Steel (Combat System)

Combat in TOMB-SHIP is fast, terrifying, and decisive. Rounds represent roughly **10 seconds** of chaotic, desperate violence.

```
[1. Surprise & Initiative] ──> [2. Player Actions] ──> [3. Enemy Reactions] ──> [4. Damage & Wounds]
```

### 1. Surprise & Initiative
- If either side is unawares, an ambush occurs: the ambushing party acts with **Advantage** on Round 1.
- Otherwise, test **Agility ($d100$)**:
  - Success: Player acts before the monsters.
  - Failure: Monsters act first.

### 2. Actions in Combat
On your turn, you may perform **one Movement** and **one Action**:
- **Melee Attack**: Roll $d100 \le \text{Combat}$.
- **Ranged Attack**: Roll $d100 \le \text{Combat}$ (impaired if an enemy is in melee contact).
- **Parry / Defensive Stance**: Roll $d100 \le \text{Agility}$ or $\text{Combat}$. On success, add $+3\text{ AP}$ against melee attacks until your next turn.
- **Cast Occult Ritual**: Begin chanting or triggering an artifact (requires concentration, forces $+1$ Stress).
- **Flee / Disengage**: Test Agility ($d100$). Failure allows the opponent a free parting strike.

### 3. Resolving Attacks & Damage
1. **Roll $d100 \le \text{Combat}$**:
   - **Regular Success**: Roll weapon damage. Target subtracts its **Armor Points (AP)**. Remainder damages **Health**.
   - **Critical Success (Matching Digits $\le \text{Combat}$)**: Maximum damage rolled! In addition, roll on the **Critical Trauma Table** or apply weapon special effect.
   - **Regular Failure**: Miss or glancing blow.
   - **Critical Failure (Matching Digits $> \text{Combat}$ or `00`)**: Disaster! Weapon snaps, slips from bloody fingers, or strikes an ally. Gain **$+1$ Stress**.
2. **Damage Overflow & Wounds**:
   - If an attack reduces a target to **0 Health**, any remaining damage is discarded, the character suffers **1 Wound**, rolls on the **Wound Table**, and their Health pool is immediately reset to its maximum!
   - If a character suffers their final Wound (e.g., reaching 0 Wounds remaining), they drop dead or are torn apart.

---

## 7. Mutilating Wound Tables

When an attack or trap consumes a Wound, roll $1d10$ on the table matching the damage type:

### Table A: Bludgeoning & Crushing Trauma ($1d10$)
| Roll | Wound Result | Mechanical Effect |
| :---: | :--- | :--- |
| **1–2** | **Knocked Senseless** | Stunned for 1 round. Gain $+1$ Stress. |
| **3–4** | **Cracked Ribs** | Coughing blood. Disadvantage on all Strength checks until treated. |
| **5–6** | **Shattered Arm** | Limb hangs limp. Cannot wield two-handed weapons or shields. |
| **7–8** | **Crushed Knee** | Movement halved. Cannot sprint or jump. |
| **9** | **Severe Concussion** | Mind and Courage saves reduced by $-15\%$. Hallucinations in the dark. |
| **10** | **Skull Caved In** | Instant death or comatose with brain trauma. Comrades make Courage Save. |

### Table B: Slashing & Piercing Trauma ($1d10$)
| Roll | Wound Result | Mechanical Effect |
| :---: | :--- | :--- |
| **1–2** | **Flesh Gouge** | Deep laceration. $-5\text{ HP}$ from next maximum reset. |
| **3–4** | **Severed Tendon** | Agility reduced by $-10\%$. Disadvantage on dodge tests. |
| **5–6** | **Gushing Artery** | Bleeding out! Takes $1d6$ damage at the end of each round until bandaged. |
| **7–8** | **Impaled Lung** | Gasping for air. Cannot run or yell. Flesh Save each turn or pass out. |
| **9** | **Gouged Eye / Facial Ruin** | Permanent loss of eye. Disadvantage on ranged attacks. $+2$ Stress. |
| **10** | **Decapitation / Cleaved in Twain**| Immediate gruesome demise. All companions must make an immediate Panic Check. |

---

## 8. The Stress & Panic Engine

In the depths, madness and fear are as lethal as steel.

```
Fear Event / Critical Fumble ──> [+Stress] ──> Trigger Event ──> [Panic Check: 1d20 <= Stress?]
                                                                          │
                                                      ┌───────────────────┴───────────────────┐
                                                  [SUCCESS: Composed]                    [FAIL: PANIC TABLE]
```

### Accumulating Stress
A character's Stress starts at their **Minimum Stress** (typically 1 or 2). Stress increases whenever:
- You roll a **Critical Failure** on any check ($+1$ Stress).
- You take a **Wound** ($+1$ Stress).
- A companion drops to 0 Health or is slaughtered ($+1$ or $+2$ Stress).
- You are trapped in **Total Darkness** without a light source for a full turn ($+1$ Stress/turn).
- You cast an **Occult Ritual** or touch a cursed artifact ($+1$ to $+3$ Stress).
- You encounter an unnatural abomination and fail a **Courage Save** ($+1$ Stress).

### When to Make a Panic Check
Whenever a character experiences a traumatic shock (e.g. failing a Courage Save vs an abomination, suffering a catastrophic critical fumble, or witnessing a friend's head get crushed), the GM calls for a **Panic Check**:
1. Roll **$1d20$**.
2. If the roll is **greater than your Current Stress**: You maintain discipline. Grit your teeth and soldier on.
3. If the roll is **$\le$ your Current Stress**: You break! Roll **$1d20$** on the **Panic Table** below.

### The Panic Table ($1d20$)

| Roll | Panic Reaction | Description & Mechanical Penalty |
| :---: | :--- | :--- |
| **1** | **Laser Focus** | The adrenaline clears your mind. Gain $+10\%$ to all tests for the next 3 rounds. Remove 1 Stress. |
| **2** | **Grim Defiance** | You refuse to die in the dirt. Add $+5\text{ AP}$ to your armor for the next attack against you. |
| **3–4** | **Tremors** | Hands shake violently. Disadvantage on all Combat and Agility checks for $1d4$ rounds. |
| **5–6** | **Fumble & Drop** | In pure terror, you drop whatever is in your hands (weapon, torch, or lantern). |
| **7–8** | **Suffocating Paranoia**| You suspect your comrades are possessed or plotting against you. Refuse to be healed or touched for 10 minutes. |
| **9–10** | **Involuntary Scream** | You let out a bloodcurdling shriek. Alerts all monsters in nearby rooms. Immediate roll on the Crawl Hazard Die! |
| **11–12** | **Flee in Blind Terror**| You bolt back the way you came for 2 rounds, ignoring traps, dropping torches. |
| **13–14** | **Frozen in Place** | Total paralysis. You can take no actions for $1d4$ rounds; enemies attack you with Advantage. |
| **15–16** | **Berserk Rampage** | You lash out blindly at the nearest living creature (friend or foe) with two-handed ferocity for 2 rounds. |
| **17–18** | **Catatonic Collapse** | You drop to your knees in silent shock. Cannot act until treated with smelling salts or an Occult/Mind check. |
| **19** | **Psychotic Rupture** | Gain a permanent phobia or affliction (e.g., Nyctophobia, Haemophobia). Minimum Stress permanently increases by +2. |
| **20+** | **Fatal Heart Attack / Rupture** | Your chest seizes. Make an immediate **Flesh Save**. If failed, your heart stops and you die on the spot. If successful, drop to 0 Health and gain 1 Wound. |

---

## 9. Dungeon Discovery & Crawling Procedures

Dungeon exploration in TOMB-SHIP is conducted in **The 10-Minute Crawl Turn**. The dungeon is not a static map—it is an active, predatory environment that decays resources, breeds horrors, and feeds on hesitation.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   THE 10-MINUTE CRAWL TURN ENGINE                      │
│                                                                        │
│  [1. Declare Intent] ──> [2. Advance Time] ──> [3. Roll Hazard Die]   │
│           │                                             │              │
│           ▼                                             ▼              │
│  [4. Resolve Action] ──────────────────────────> [5. Stress & Dread]   │
└────────────────────────────────────────────────────────────────────────┘
```

### 1. Step-by-Step Turn Sequence
At the start of every 10-minute crawl turn, the Warden (GM) runs this five-step procedure:

1. **Declare Intent**: Delvers decide their primary focus for the next 10 minutes:
   - *Careful Movement*: Travel through one corridor or chamber, testing flagstones and probing ceilings with a 10-foot pole.
   - *Active Room Search*: Prying sarcophagi, tapping hollow walls for secret vaults, examining altar runes.
   - *Breaching / Forcing*: Picking an iron padlock, forcing a rusted portcullis, or breaking through loose masonry.
   - *Short Breather*: Barring a doorway, bandaging wounds, catching breath, and checking gear.
   - *Ritual Casting*: Chanting an Occult rite in the dark.
2. **Advance Time & Light**:
   - Tick **1 mark (10 minutes)** on the Dungeon Crawl Tracker.
   - Every **6 turns (60 minutes)**, an active **Tallow Torch** gutters and dies.
   - Every **18 turns (3 hours)**, a **Hooded Lantern** consumes 1 flask of lamp oil.
3. **Roll Crawl Hazard Die ($1d6$)**:
   - Roll openly on the table below to see what stirs in the depths.
4. **Resolve Action & Environment**:
   - Warden narrates the outcome of the party's declared action, describes the discovery, or initiates combat/traps.
5. **Stress & Light Check**:
   - If characters are in **Total Darkness**, every delver gains **$+1$ Stress**.
   - If encumbered or starving, delvers gain **$+1$ Stress**.

---

### 2. The Crawl Hazard Die ($1d6$)

At the end of every 10-minute turn, roll **$1d6$**:

| Die Roll | Hazard Event | Tabletop Description & Mechanical Impact |
| :---: | :--- | :--- |
| **1** | **Wandering Abomination** | Roll immediately on the **Wandering Encounter Sub-Table**. An ambush or patrol breaches the chamber door! |
| **2** | **Sign & Spoor** | A telltale warning: claw gouges on stone, a fresh puddle of steaming bile, dragging chains, or a cold draft carrying tomb stench. |
| **3** | **Resource Depletion** | Torches flare and gutter (lose 1 turn of burn time). A waterskin leaks, tallow spoils, or a backpack strap snaps. |
| **4** | **Environmental Hazard** | Sudden ceiling cave-in (Agility check or $1d10$ damage), toxic spore pocket (Flesh Save or $+1$ Stress and coughing fits), or floor stones buckle. |
| **5** | **Subterranean Dread** | The crushing weight of miles of stone hums in their teeth. The darkness closes in. **All characters gain $+1$ Stress**. |
| **6** | **Ominous Silence** | The tomb holds its breath. Echoes fade. No hazard this turn. |

#### Wandering Encounter Sub-Table ($1d6$)
When a **1** is rolled on the Hazard Die, roll $1d6$:
1. **$1d4$ Fissure Goblins**: Dropping from ceiling vents or wall cracks attempting a *Snatch & Sever* ambush.
2. **$1d3$ Bleached Crypt Skeletons**: Clattering down the hallway, brandishing rusty cleavers.
3. **1 Rotting Ghoul**: Tracking the party by the warm smell of sweat and open wounds.
4. **1 Trench Orc Scout**: Prowling the perimeter, hunting for fresh meat and tallow.
5. **1 Tomb Wraith**: Drifting through the solid masonry, extinguishing small candles.
6. **Distant Tremors of the Abyssal Troll**: Heavy thuds shake the ceiling dust. The Troll will arrive in the chamber in **$1d4$ turns** unless the party flees or masks their scent.

---

### 3. Light, Vision & The Terror of the Dark

Light is the only barrier separating delvers from the suffocating void.

| Light Source | Burn Duration | Illumination Radius | Qualities & Hazards |
| :--- | :---: | :---: | :--- |
| **Tallow Torch** | 6 turns (1 hour) | 30 ft. bright / 60 ft. dim | Improvised weapon ($1d6 + \text{fire}$). Smells of burnt fat; attracts ghouls and troglodytes. |
| **Hooded Lantern** | 18 turns (3 hours / flask) | 40 ft. focused beam | Shielded from drafts. Shutterable to 5 ft. dim light for stealthy advance. |
| **Wax Candle** | 12 turns (2 hours) | 10 ft. dim | Odorless, smokeless. Easily snuffed by breezes or passing wraiths. |

> [!WARNING]
> **Total Darkness (Pitch Black)**:
> When the last flame dies:
> - **$+1$ Stress per 10-minute turn** to all living characters.
> - All player attacks, defenses, and physical saves suffer **Disadvantage**.
> - Subterranean predators (Goblins, Orcs, Trolls, Undead) gain **Advantage** on all strikes.
> - Wandering horrors automatically detect the party by sound and scent.

---

### 4. Room Discovery: Passive vs. Active Exploration

To keep exploration brisk and avoid "pixel-hunting":

- **Passive Room Entry (Free / No Time Cost)**:
  - When delvers step into a chamber, the Warden immediately describes **obvious visible features**: room dimensions, visible exits, statues, open sarcophagi, odors, prominent altars, and active monsters.
  - No dice roll is required to notice obvious details.
- **Active Searching (Costs 1 Full 10-Minute Turn)**:
  - Required to probe stone floor tiles for pressure plates, tap walls for secret doors, pry sealed lids, or decipher eroded inscriptions.
  - *Risk vs. Reward*: Every search triggers a Crawl Hazard Die roll. The Grave-Robber's *Gut Instinct* trait allows them to conduct active searches in **half the normal time** (2 searches per 10-minute turn).

---

### 5. Resting & Fortifying in the Crypts

Delvers cannot survive prolonged excursions without resting, but resting in a living tomb is dangerous.

- **Short Breather (1 Turn / 10 Minutes)**:
  - Delvers spike the door shut with iron pitons, lean against the wall, and drink sour wine.
  - A character may make a **Flesh Save** to bind gaping wounds, restoring **$1d6$ Health**.
  - All delvers relieve **1 Stress**.
  - *Costs 1 turn; roll the Crawl Hazard Die normally.*
- **Catacomb Encampment (6 Turns / 1 Hour)**:
  - Necessary to recover from concussions and deep trauma.
  - Requires setting a watch. The Warden rolls the **Crawl Hazard Die 6 times** (once per 10-minute segment).
  - If uninterrupted: All characters restore **Health to maximum**, reduce Stress by **$1d6$**, and Homunculi may conduct field repairs.
  - If a **1** is rolled during the encampment, delvers are ambushed while unarmored or sleeping!

---

### 6. Printable Tabletop Crawl Tracker

Copy or sketch this 6-turn tracker on your character sheet or Warden notepad:

```
DELVE TURN TRACKER (1 Hour / 1 Torch Block)
Turn 1: [ ] Move / Recon       --> Hazard Die: [ ]
Turn 2: [ ] Search Crypt Vault --> Hazard Die: [ ]
Turn 3: [ ] Pick Iron Door     --> Hazard Die: [ ]
Turn 4: [ ] Short Breather     --> Hazard Die: [ ]
Turn 5: [ ] Advance Corridors  --> Hazard Die: [ ]
Turn 6: [ ] *TORCH EXPIRES!*   --> Hazard Die: [ ]
=====================================================
[ ] Light Fresh Torch  |  Current Party Stress: _____
```

---

## 10. Bestiary: Horrors of the Deep Vaults

Monsters in TOMB-SHIP do not have bloated stats. They are lethal, terrifying, and visceral survival challenges.

### Part A: The Undead Legions

#### 1. Bleached Crypt Skeleton
*The calcified, dry bones of long-buried tomb wardens, held together by ancient malice and dark resonance.*

- **Combat**: 35% | **Health**: 10 | **Wounds**: 1 | **Armor Points**: AP 2 (Dense bone)
- **Attack**: Rusty Scythe or Cleaver ($1d10$ damage)
- **Special Qualities**:
  - **Immune** to Mind Saves, fear, poison, and bleeding.
  - **Brittle Anatomy**: Piercing weapons (arrows, daggers) deal half damage; Bludgeoning weapons (maces, flails) deal double damage.
  - **Bone Clatter**: Entering an unlit room with dormant skeletons forces an immediate **Courage Save**; failure causes $+1$ Stress as the dry bones rattle awake.

---

#### 2. Rotting Ghoul (The Corpse-Eater)
*Twisted, hairless humanoids with elongated jaws and diseased talons, lurking where ancient plague pits lie.*

- **Combat**: 45% | **Health**: 14 | **Wounds**: 2 | **Armor Points**: AP 1 (Leathery rotting hide)
- **Attacks**: Filth Claws ($1d8$ damage) + Gaping Bite ($1d10$ damage)
- **Special Qualities**:
  - **Paralytic Slime**: Any target suffering damage from claws must make an immediate **Flesh Save**. Failure causes total muscular paralysis for $1d4$ rounds.
  - **Foul Stench**: Characters within 10 feet must pass a Flesh Save or suffer Disadvantage on Combat checks due to dry heaving.
  - **Ravenous Frenzy**: If a ghoul slays a companion, it spends its next turn voraciously gorging on the corpse rather than pursuing fleeing survivors.

---

#### 3. Barrow Wight (The Sarcophagus Lord)
*A withered ancient noble preserved in pitch and frost, crowned in tarnished silver with burning blue corpse-fire in its eyes.*

- **Combat**: 55% | **Health**: 20 | **Wounds**: 3 | **Armor Points**: AP 4 (Ancient bronze plate + withered flesh)
- **Attack**: Frost-Bitten Broadsword ($2d10$ damage) or Freezing Grasp
- **Special Qualities**:
  - **Life Drain**: On a Critical Hit with its sword or touch, target loses $1d10$ maximum Health until consecrated and rested, and gains $+2$ Stress.
  - **Aura of the Tomb**: Radiates unnatural cold. Torches within 15 feet flicker out twice as fast.
  - **Terror of the Crypt**: Simply viewing the Barrow Wight rise from its sarcophagus forces an immediate **Panic Check** on all characters.

---

#### 4. The Flesh Amalgam (Stitched Abomination)
*A grotesque colossus stitched together from dozens of subterranean corpses, reanimated with quicksilver and necrotic humors.*

- **Combat**: 50% | **Health**: 30 | **Wounds**: 4 | **Armor Points**: AP 3 (Thick layers of dead fat and muscle)
- **Attack**: Sweeping Meat Flail ($2d10$ damage, hits two adjacent targets) or Crushing Slam ($3d10$ damage)
- **Special Qualities**:
  - **Unstoppable Mass**: Cannot be knocked down, stunned, or pushed back.
  - **Severed Thrashing**: Whenever the Amalgam loses a Wound, a severed limb drops to the floor and begins attacking independently as a Crawling Claw (Health 4, Combat 30%, $1d4$ damage).
  - **Visceral Horror**: Any character within melee range when the Amalgam takes a Wound gets splattered with bile and must pass a Flesh Save or gain $+1$ Stress.

---

#### 5. Tomb Wraith (The Void Specter)
*A shadowy silhouette of pure malice, weeping cold darkness and drifting silently through solid stone.*

- **Combat**: 40% | **Health**: 12 | **Wounds**: 2 | **Armor Points**: AP 0 (Incorporeal)
- **Attack**: Chilling Touch of the Void ($1d10$ psychic damage, ignores all physical AP)
- **Special Qualities**:
  - **Incorporeal**: Completely immune to non-magical and non-silvered weapons. Silver weapons deal normal damage; holy relics deal double damage.
  - **Soul Chill**: Attacks directly target the victim's **Mind Save**; failure causes the target's flesh to frost over, dealing direct damage to Health.
  - **Phantasmagoria**: Extinguishes mundane candles and torches within 20 feet automatically.

---

### Part B: Subterranean Monstrosities & Reavers

#### 6. Fissure Goblin (The Grasping Deep-Slinker)
*Emaciated, hairless subterranean vermin with milky bulbous eyes and elongated clawed digits. They nest in deep cracks and crawl along ceilings, waiting to sever supply lines.*

- **Combat**: 30% | **Health**: 8 | **Wounds**: 1 | **Armor Points**: AP 0 (Rags and dried bat guano)
- **Attacks**: Rusted Bone Shank ($1d6$ damage + Venom) or Jagged Flint Sling ($1d6$ damage, Medium)
- **Special Qualities**:
  - **Ceiling Ambush**: When dropping from dark ceiling cracks onto unsuspecting delvers, attacks with **Advantage** and causes **$+1$ Stress** to the victim.
  - **Snatch & Sever**: On a Critical Hit, instead of bonus damage, the goblin cuts a backpack strap, snatches a lit torch or prized item, and scurries into a 6-inch rock fissure!
  - **Numbing Cave Venom**: Taking damage from the shank requires an immediate **Flesh Save**. Failure inflicts **Disadvantage** on all Agility checks and cuts movement speed in half for 10 minutes.
  - **Craven Swarm**: If more than half the goblin pack is slain or if a goblin is killed with a Critical Hit, remaining goblins must pass a Courage Save (30%) or scatter blindly into the rock.

---

#### 7. Trench Orc (The Iron-Scarred Reaver)
*Towering, slate-skinned marauders bred in forgotten subterranean trench wars. Their ears and noses are docked, their torsos ritually branded with crude occult warding runes, and their faces concealed behind iron butchering visors.*

- **Combat**: 50% | **Health**: 16 | **Wounds**: 2 | **Armor Points**: AP 4 (Crude iron splints riveted to heavy hide)
- **Attacks**: Serrated Flaying Cleaver ($2d8$ damage) or Heavy Barbed Spetum ($1d10$ damage, Reach)
- **Special Qualities**:
  - **Bloodcurdling War-Roar**: When charging into combat, all opposing delvers must make an immediate **Courage Save**; failure causes **$+1$ Stress** and denies any reaction for that round.
  - **Savage Butchery**: When an Orc attack drops a delver to 0 Health, the resulting roll on the Wound Table is made with **Disadvantage** (roll twice, apply the deadlier result).
  - **Iron Conditioning**: Orcs gain **$+20\%$** to Flesh Saves to resist stunning, bleedout, and inhaled gas.
  - **Cannibal Frenzy**: Slaying an opponent restores $1d10$ Health or 1 lost Wound to the Orc as it ravenously tastes warm flesh.

---

#### 8. Abyssal Bile Troll (The Cavern Devourer)
*A 9-foot-tall, hunchbacked horror of rubbery, gangrenous gray hide. It possesses no eyes—hunting entirely by scent, air vibrations, and the sound of thumping heartbeats. Its throat bubbles with acidic bile that liquefies rock, armor, and bone.*

- **Combat**: 55% | **Health**: 25 | **Wounds**: 4 | **Armor Points**: AP 2 (Rubbery, shock-absorbing blubber)
- **Attacks**: Crushing Bone-Torn Claws ($2d10$ damage) + Acidic Bile Vomit (Special)
- **Special Qualities**:
  - **Voracious Regeneration**: At the start of its turn, the Troll recovers **$1d10$ Health**. It cannot be truly slain while it has remaining Wounds unless damaged by **Fire** or **Pure Acid**. If reduced to 0 Wounds without fire, it revives at full Health within $1d6$ crawl turns!
  - **Corrosive Bile Spew (Recharge: 3 rounds)**: Vomits a 20-foot cone of black digestive fluid. All targets in the area take $1d10$ damage (bypassing all Armor Points), and any worn armor's **AP is permanently reduced by 2**.
  - **Limb-Ripping Violence**: On a Critical Hit with claws, the Troll severs or crushes a limb: roll immediately on the Slashing/Piercing Wound Table with no Health reset allowed for that blow.
  - **Apex Dread**: The sudden appearance or roar of an Abyssal Bile Troll forces an immediate **Panic Check ($1d20 \le \text{Stress}$)** on every explorer who witnesses it.

---

## 11. Forbidden Occult Rites & Void Calamities

Sorcery in TOMB-SHIP is never safe. It is the forbidden art of bargaining with ancient subterranean void entities.

```
[Cast Ritual] ──> [+1 Stress] ──> [Test Occult d100]
                                          │
                  ┌───────────────────────┼───────────────────────┐
             [SUCCESS]               [FAILURE]             [MATCHING DIGITS: CALAMITY]
       (Rite takes effect)    (Spell fizzles, +1 Stress)   (Roll 1d10 on Calamity Table)
```

### The Rules of Magic
1. **Stress Toll**: Casting any ritual immediately adds **$+1$ Stress** to the caster before rolling.
2. **The Occult Check**: Roll $d100 \le \text{Occult}$:
   - **Success**: The spell takes effect as written.
   - **Failure**: The incantation fizzles, waste of action, gain an additional $+1$ Stress.
   - **Critical Failure (Matching Digits $> \text{Occult}$ or `00`)**: An **Arcane Calamity** occurs! Roll $1d10$ on the table below.

### The Arcane Calamity Table ($1d10$)

| Roll | Calamity | Horrific Effect |
| :---: | :--- | :--- |
| **1** | **Eldritch Backlash** | Caster is knocked unconscious for $1d6$ rounds. Bleeds from eyes and nose. |
| **2** | **Void Blackout** | Every flame, torch, and lantern in the chamber is instantly snuffed out into pitch darkness. |
| **3** | **Corpse Resonance** | All corpses within 60 feet twitch and rise as minor skeletons or zombies on the next round. |
| **4** | **Mutagenic Rot** | Caster’s skin bubbles with black pustules. Flesh Save or lose 1 Wound to permanent necrotic rotting. |
| **5** | **Psychic Shockwave** | A piercing scream echoes through the minds of everyone in the room. All allies make immediate Panic Checks. |
| **6** | **Spatial Displacement**| Caster is teleported $2d10$ feet in a random direction—possibly into a solid wall or pit trap! |
| **7** | **The Void Leech** | An invisible entity latches onto the caster's soul. Caster cannot cast rituals or regain Health until exorcised. |
| **8** | **Blood Geyser** | Caster’s veins rupture. Takes $2d10$ direct damage (bypassing armor) and gains $+3$ Stress. |
| **9** | **Demonic Possession** | Caster’s mind is hijacked by a void parasite for $1d4$ rounds; acts as an enemy targeting allies. |
| **10** | **Tear in the Veil** | A gaping rift to the outer void opens in the center of the room. Spawns a Tomb Wraith. |

---

## 12. Quick-Reference Deluxe Cheat Sheet

### Core Engine
- **Percentile Roll-Under**: $d100 \le \text{Stat / Save}$.
- **Advantage / Disadvantage**: Roll 2 tens dice ($d10$), take lower (Advantage) or higher (Disadvantage).
- **Matching Double Digits**:
  - $\le \text{Stat}$: **Critical Success** (Max damage, special boon, -1 Stress).
  - $> \text{Stat}$: **Critical Failure** (Catastrophe, broken weapon, +1 Stress).
  - `00`: Utter disaster.

### Health, Damage & Wounds
- **Health**: Temporary meat buffer ($10\text{--}18$ HP).
- **Armor Points (AP)**: Subtracted from incoming damage.
- **Wounds (2–3)**: Triggered when Health hits 0. Roll on Wound Table ($1d10$), reset Health pool to max. 
- At 0 Wounds remaining: **True Death**.

### The Panic Loop
- **Gain Stress**: Critical fumbles, wounds, pitch dark, terrifying abominations, failed saves.
- **Panic Check**: Roll $1d20 \le \text{Current Stress}$.
- **If Failed**: Roll $1d20$ on Panic Table.

### Dungeon Crawl Turn (10 Minutes)
- **1 Action per Turn**: Move 1 room, search, pick lock, or rest.
- **Torches**: Last 6 turns (1 hour). **Lanterns**: Last 18 turns (3 hours).
- **Hazard Die ($1d6$ every turn)**:
  - `1`: Wandering Monster
  - `2`: Sign / Spoor
  - `3`: Torch / Resource Drain
  - `4`: Trap / Environmental Hazard
  - `5`: Dread / +1 Stress to all
  - `6`: Safe / Silence

---

*Written for the Dark Fort / MÖRK BORG lineage. May your tallow burn long, and the void forgive your trespassing.*
