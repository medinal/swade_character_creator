use serde::{Deserialize, Serialize};

/// Derived characteristics calculated from a character's attributes, skills, and modifiers.
///
/// These values are computed by CharacterView::compute_effective_values() and represent
/// the final calculated stats after all modifiers from edges, hindrances, and ancestries
/// are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct DerivedStatsView {
    /// Movement rate in tabletop inches per round. Base is 6.
    pub pace: i32,
    /// Defense against melee attacks. Base is 2 + half Fighting die.
    pub parry: i32,
    /// Damage resistance. Base is 2 + half Vigor die + Size.
    pub toughness: i32,
    /// Size modifier. Normal humans are 0. Affects Toughness.
    pub size: i32,
}

impl DerivedStatsView {
    /// Create derived stats with SWADE base values.
    pub fn new() -> Self {
        Self {
            pace: 6,
            parry: 2,
            toughness: 2,
            size: 0,
        }
    }
}

impl Default for DerivedStatsView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let stats = DerivedStatsView::default();
        assert_eq!(stats.pace, 6);
        assert_eq!(stats.parry, 2);
        assert_eq!(stats.toughness, 2);
        assert_eq!(stats.size, 0);
    }

    #[test]
    fn test_new_equals_default() {
        assert_eq!(DerivedStatsView::new(), DerivedStatsView::default());
    }
}
