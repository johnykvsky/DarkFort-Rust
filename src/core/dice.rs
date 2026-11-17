use rand::Rng;
use serde::{Deserialize, Serialize};

/// Supported dice expression types in Dark Fort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiceFormula {
    #[serde(alias = "1d2", alias = "d2")]
    D2,         // MÖRK BORG Tier 1 armor
    #[serde(alias = "1d3", alias = "d3")]
    D3,
    #[serde(alias = "1d4", alias = "d4")]
    D4,
    #[serde(alias = "1d4+1", alias = "d4+1")]
    D4Plus1,
    #[serde(alias = "1d6", alias = "d6")]
    D6,
    #[serde(alias = "1d8", alias = "d8")]
    D8,
    #[serde(alias = "1d8+1", alias = "d8+1")]
    D8Plus1,
    #[serde(alias = "1d10", alias = "d10")]
    D10,
    #[serde(alias = "1d10+2", alias = "d10+2")]
    D10Plus2,
    #[serde(alias = "2d6")]
    TwoD6,
    #[serde(alias = "2d8")]
    TwoD8,
    #[serde(alias = "1d4-1", alias = "d4-1")]
    D4Minus1,   // Unarmed attack (min 0)
    #[serde(alias = "1d6+1", alias = "d6+1")]
    D6Plus1,    // Flail / Troll damage
    #[serde(alias = "1d6+2", alias = "d6+2")]
    D6Plus2,    // Mighty Zweihänder damage
    #[serde(alias = "3d6")]
    ThreeD6,    // Necro-Sorcerer loot
    #[serde(alias = "1d4*1d6", alias = "d4*d6")]
    D4TimesD6,  // Medusa loot
    #[serde(alias = "1d20", alias = "d20")]
    D20,        // d20 Action roll
    Constant(i32),
}

/// Abstract dice roller trait allowing deterministic mocks in tests.
pub trait DiceRoller {
    fn roll_d4(&mut self) -> i32;
    fn roll_d6(&mut self) -> i32;

    fn roll_d2(&mut self) -> i32 {
        (self.roll_d6() % 2) + 1
    }

    fn roll_d8(&mut self) -> i32 {
        (self.roll_d6() % 8) + 1
    }

    fn roll_d10(&mut self) -> i32 {
        (self.roll_d6() % 10) + 1
    }

    fn roll_d20(&mut self) -> i32 {
        (self.roll_d6() % 20) + 1
    }

    fn roll_formula(&mut self, formula: DiceFormula) -> i32 {
        match formula {
            DiceFormula::D4 => self.roll_d4(),
            DiceFormula::D6 => self.roll_d6(),
            DiceFormula::D8 => self.roll_d8(),
            DiceFormula::D8Plus1 => self.roll_d8() + 1,
            DiceFormula::D10 => self.roll_d10(),
            DiceFormula::D10Plus2 => self.roll_d10() + 2,
            DiceFormula::D2 => self.roll_d2(),
            DiceFormula::D3 => (self.roll_d6() % 3) + 1,
            DiceFormula::D4Plus1 => self.roll_d4() + 1,
            DiceFormula::D20 => self.roll_d20(),
            DiceFormula::TwoD6 => self.roll_d6() + self.roll_d6(),
            DiceFormula::TwoD8 => self.roll_d8() + self.roll_d8(),
            DiceFormula::D4Minus1 => (self.roll_d4() - 1).max(0),
            DiceFormula::D6Plus1 => self.roll_d6() + 1,
            DiceFormula::D6Plus2 => self.roll_d6() + 2,
            DiceFormula::ThreeD6 => self.roll_d6() + self.roll_d6() + self.roll_d6(),
            DiceFormula::D4TimesD6 => self.roll_d4() * self.roll_d6(),
            DiceFormula::Constant(val) => val,
        }
    }

    /// Roll 1d6 and check against a 1-in-N or M-in-N chance (e.g. 2-in-6).
    fn check_x_in_6(&mut self, target_x: i32) -> bool {
        let roll = self.roll_d6();
        roll <= target_x
    }

    /// Roll 1d100 (1..=100) for percentile rulesets (e.g. Mothership 1e).
    fn roll_d100(&mut self) -> u8 {
        ((self.roll_d6() - 1) * 16 + self.roll_d6()) as u8 % 100 + 1
    }
}

/// Standard RNG-backed dice roller.
pub struct StandardRoller<R: Rng> {
    rng: R,
}

impl<R: Rng> StandardRoller<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> DiceRoller for StandardRoller<R> {
    fn roll_d4(&mut self) -> i32 {
        self.rng.gen_range(1..=4)
    }

    fn roll_d6(&mut self) -> i32 {
        self.rng.gen_range(1..=6)
    }

    fn roll_d2(&mut self) -> i32 {
        self.rng.gen_range(1..=2)
    }

    fn roll_d8(&mut self) -> i32 {
        self.rng.gen_range(1..=8)
    }

    fn roll_d10(&mut self) -> i32 {
        self.rng.gen_range(1..=10)
    }

    fn roll_d20(&mut self) -> i32 {
        self.rng.gen_range(1..=20)
    }

    fn roll_d100(&mut self) -> u8 {
        self.rng.gen_range(1..=100)
    }
}

/// Deterministic mock roller for unit testing exact roll sequences.
pub struct MockRoller {
    d4_rolls: Vec<i32>,
    d6_rolls: Vec<i32>,
    d100_rolls: Vec<u8>,
    d20_rolls: Vec<i32>,
}

impl MockRoller {
    pub fn new(d4_rolls: Vec<i32>, d6_rolls: Vec<i32>) -> Self {
        Self {
            d4_rolls,
            d6_rolls,
            d100_rolls: Vec::new(),
            d20_rolls: Vec::new(),
        }
    }

    pub fn with_d100(mut self, d100_rolls: Vec<u8>) -> Self {
        self.d100_rolls = d100_rolls;
        self
    }

    pub fn with_d20(mut self, d20_rolls: Vec<i32>) -> Self {
        self.d20_rolls = d20_rolls;
        self
    }
}

impl DiceRoller for MockRoller {
    fn roll_d4(&mut self) -> i32 {
        if self.d4_rolls.is_empty() { 1 } else { self.d4_rolls.remove(0) }
    }

    fn roll_d6(&mut self) -> i32 {
        if self.d6_rolls.is_empty() { 1 } else { self.d6_rolls.remove(0) }
    }

    fn roll_d20(&mut self) -> i32 {
        if self.d20_rolls.is_empty() { 12 } else { self.d20_rolls.remove(0) }
    }

    fn roll_d100(&mut self) -> u8 {
        if self.d100_rolls.is_empty() { 50 } else { self.d100_rolls.remove(0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_formulas() {
        let mut mock = MockRoller::new(vec![3, 1, 4], vec![5, 2, 6, 3, 4, 1]);
        assert_eq!(mock.roll_formula(DiceFormula::D4), 3);
        assert_eq!(mock.roll_formula(DiceFormula::D6), 5);
        assert_eq!(mock.roll_formula(DiceFormula::TwoD6), 2 + 6);
        assert_eq!(mock.roll_formula(DiceFormula::D4Minus1), 0); // (1 - 1).max(0)
        assert_eq!(mock.roll_formula(DiceFormula::D6Plus1), 3 + 1);
        assert_eq!(mock.roll_formula(DiceFormula::D6Plus2), 4 + 2);
    }

    #[test]
    fn test_check_x_in_6() {
        let mut mock = MockRoller::new(vec![], vec![2, 5, 1, 6]);
        assert!(mock.check_x_in_6(2));  // roll 2 <= 2 -> true
        assert!(!mock.check_x_in_6(2)); // roll 5 <= 2 -> false
        assert!(mock.check_x_in_6(1));  // roll 1 <= 1 -> true
        assert!(!mock.check_x_in_6(1)); // roll 6 <= 1 -> false
    }
}
