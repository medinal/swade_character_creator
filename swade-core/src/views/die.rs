use serde::{Deserialize, Serialize};
use std::fmt;

/// A SWADE die value.
///
/// Represents die sizes from d4 to d12, and beyond d12 with modifiers (d12+1, d12+2, etc.).
/// The progression is: d4 → d6 → d8 → d10 → d12 → d12+1 → d12+2 → ...
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub struct Die {
    size: u8,
    modifier: u8,
}

impl Die {
    const VALID_SIZES: [u8; 5] = [4, 6, 8, 10, 12];

    /// Create a die from a size, returning None if the size is invalid.
    /// Valid sizes are 4, 6, 8, 10, 12.
    pub fn new(size: u8) -> Option<Self> {
        if Self::VALID_SIZES.contains(&size) {
            Some(Self { size, modifier: 0 })
        } else {
            None
        }
    }

    /// Create a die with a modifier (e.g., d12+2).
    /// Only d12 can have a modifier; other sizes return None if modifier > 0.
    pub fn with_modifier(size: u8, modifier: u8) -> Option<Self> {
        if !Self::VALID_SIZES.contains(&size) {
            return None;
        }
        if size < 12 && modifier > 0 {
            return None;
        }
        Some(Self { size, modifier })
    }

    pub fn d4() -> Self {
        Self {
            size: 4,
            modifier: 0,
        }
    }

    pub fn d6() -> Self {
        Self {
            size: 6,
            modifier: 0,
        }
    }

    pub fn d8() -> Self {
        Self {
            size: 8,
            modifier: 0,
        }
    }

    pub fn d10() -> Self {
        Self {
            size: 10,
            modifier: 0,
        }
    }

    pub fn d12() -> Self {
        Self {
            size: 12,
            modifier: 0,
        }
    }

    /// Get the die size (4, 6, 8, 10, or 12).
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Get the modifier (0 for normal dice, 1+ for d12+N).
    pub fn modifier(&self) -> u8 {
        self.modifier
    }

    /// Increment to the next die value.
    /// d4 → d6 → d8 → d10 → d12 → d12+1 → d12+2 → ...
    pub fn increment(&self) -> Self {
        if self.size == 12 {
            Self {
                size: 12,
                modifier: self.modifier + 1,
            }
        } else {
            let idx = Self::VALID_SIZES
                .iter()
                .position(|&s| s == self.size)
                .unwrap();
            Self {
                size: Self::VALID_SIZES[idx + 1],
                modifier: 0,
            }
        }
    }

    /// Calculate the number of steps between two dice values.
    /// Returns how many increments it takes to go from `from` to `self`.
    /// Returns 0 if `from` is greater than or equal to `self`.
    pub fn steps_from(&self, from: Die) -> u8 {
        let self_index = Self::VALID_SIZES
            .iter()
            .position(|&s| s == self.size)
            .unwrap_or(0);
        let from_index = Self::VALID_SIZES
            .iter()
            .position(|&s| s == from.size)
            .unwrap_or(0);

        if from_index > self_index {
            return 0;
        }

        let base_steps = (self_index - from_index) as u8;

        // Add modifier difference if both are d12
        if self.size == 12 && from.size == 12 {
            if self.modifier >= from.modifier {
                base_steps + (self.modifier - from.modifier)
            } else {
                0
            }
        } else if self.size == 12 {
            base_steps + self.modifier
        } else {
            base_steps
        }
    }

    /// Decrement to the previous die value, returning None if already at d4.
    /// d12+2 → d12+1 → d12 → d10 → d8 → d6 → d4 → None
    pub fn decrement(&self) -> Option<Self> {
        if self.modifier > 0 {
            Some(Self {
                size: 12,
                modifier: self.modifier - 1,
            })
        } else if self.size == 4 {
            None
        } else {
            let idx = Self::VALID_SIZES
                .iter()
                .position(|&s| s == self.size)
                .unwrap();
            Some(Self {
                size: Self::VALID_SIZES[idx - 1],
                modifier: 0,
            })
        }
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifier == 0 {
            write!(f, "d{}", self.size)
        } else {
            write!(f, "d{}+{}", self.size, self.modifier)
        }
    }
}

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Die {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.size.cmp(&other.size) {
            std::cmp::Ordering::Equal => self.modifier.cmp(&other.modifier),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_sizes() {
        assert!(Die::new(4).is_some());
        assert!(Die::new(6).is_some());
        assert!(Die::new(8).is_some());
        assert!(Die::new(10).is_some());
        assert!(Die::new(12).is_some());
    }

    #[test]
    fn new_invalid_sizes() {
        assert!(Die::new(0).is_none());
        assert!(Die::new(2).is_none());
        assert!(Die::new(5).is_none());
        assert!(Die::new(7).is_none());
        assert!(Die::new(20).is_none());
    }

    #[test]
    fn with_modifier_d12() {
        let die = Die::with_modifier(12, 2).unwrap();
        assert_eq!(die.size(), 12);
        assert_eq!(die.modifier(), 2);
    }

    #[test]
    fn with_modifier_non_d12_zero() {
        let die = Die::with_modifier(8, 0).unwrap();
        assert_eq!(die.size(), 8);
        assert_eq!(die.modifier(), 0);
    }

    #[test]
    fn with_modifier_non_d12_nonzero_fails() {
        assert!(Die::with_modifier(8, 1).is_none());
        assert!(Die::with_modifier(4, 2).is_none());
    }

    #[test]
    fn increment_basic_dice() {
        assert_eq!(Die::d4().increment(), Die::d6());
        assert_eq!(Die::d6().increment(), Die::d8());
        assert_eq!(Die::d8().increment(), Die::d10());
        assert_eq!(Die::d10().increment(), Die::d12());
    }

    #[test]
    fn increment_beyond_d12() {
        let d12_plus_1 = Die::d12().increment();
        assert_eq!(d12_plus_1.size(), 12);
        assert_eq!(d12_plus_1.modifier(), 1);

        let d12_plus_2 = d12_plus_1.increment();
        assert_eq!(d12_plus_2.size(), 12);
        assert_eq!(d12_plus_2.modifier(), 2);
    }

    #[test]
    fn decrement_basic_dice() {
        assert_eq!(Die::d12().decrement(), Some(Die::d10()));
        assert_eq!(Die::d10().decrement(), Some(Die::d8()));
        assert_eq!(Die::d8().decrement(), Some(Die::d6()));
        assert_eq!(Die::d6().decrement(), Some(Die::d4()));
        assert_eq!(Die::d4().decrement(), None);
    }

    #[test]
    fn decrement_from_d12_plus() {
        let d12_plus_2 = Die::with_modifier(12, 2).unwrap();
        let d12_plus_1 = d12_plus_2.decrement().unwrap();
        assert_eq!(d12_plus_1.size(), 12);
        assert_eq!(d12_plus_1.modifier(), 1);

        let d12 = d12_plus_1.decrement().unwrap();
        assert_eq!(d12, Die::d12());
    }

    #[test]
    fn display() {
        assert_eq!(Die::d4().to_string(), "d4");
        assert_eq!(Die::d12().to_string(), "d12");
        assert_eq!(Die::with_modifier(12, 1).unwrap().to_string(), "d12+1");
        assert_eq!(Die::with_modifier(12, 3).unwrap().to_string(), "d12+3");
    }

    #[test]
    fn ordering() {
        assert!(Die::d4() < Die::d6());
        assert!(Die::d6() < Die::d8());
        assert!(Die::d12() < Die::with_modifier(12, 1).unwrap());
        assert!(Die::with_modifier(12, 1).unwrap() < Die::with_modifier(12, 2).unwrap());
    }

    #[test]
    fn equality() {
        assert_eq!(Die::d8(), Die::d8());
        assert_eq!(Die::d8(), Die::new(8).unwrap());
        assert_ne!(Die::d8(), Die::d10());
    }
}
