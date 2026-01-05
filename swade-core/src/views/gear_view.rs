use serde::{Deserialize, Serialize};

use crate::models::{Gear, Modifier};
use crate::views::{
    AmmunitionStatsView, ArmorStatsView, GearCategoryView, PackContentsView, PoisonStatsView,
    RequirementTree, ShieldStatsView, WeaponStatsView,
};

/// View model for a gear item with all related data
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct GearView {
    pub id: i64,
    pub name: String,
    pub category: GearCategoryView,
    pub era: String,
    pub cost: i64,
    pub weight: f64,
    pub source: String,
    pub notes: Option<String>,
    pub description: Option<String>,

    /// Modifiers this gear provides (Parry, Toughness, skill bonuses)
    pub modifiers: Vec<Modifier>,

    /// Requirements for this gear (minimum strength, etc.)
    /// Interpreted as graduated penalties rather than hard requirements
    pub requirements: RequirementTree,

    /// Type-specific stats (only one will be populated based on category)
    pub weapon_stats: Option<WeaponStatsView>,
    pub armor_stats: Option<ArmorStatsView>,
    pub shield_stats: Option<ShieldStatsView>,
    pub ammunition_stats: Option<AmmunitionStatsView>,
    pub poison_stats: Option<PoisonStatsView>,

    /// Pack contents (only populated for items in the Packs category)
    pub pack_contents: Option<Vec<PackContentsView>>,
}

impl GearView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gear: Gear,
        category: GearCategoryView,
        modifiers: Vec<Modifier>,
        requirements: RequirementTree,
        weapon_stats: Option<WeaponStatsView>,
        armor_stats: Option<ArmorStatsView>,
        shield_stats: Option<ShieldStatsView>,
        ammunition_stats: Option<AmmunitionStatsView>,
        poison_stats: Option<PoisonStatsView>,
        pack_contents: Option<Vec<PackContentsView>>,
    ) -> Self {
        Self {
            id: gear.id,
            name: gear.name,
            category,
            era: gear.era,
            cost: gear.cost,
            weight: gear.weight,
            source: gear.source,
            notes: gear.notes,
            description: gear.description,
            modifiers,
            requirements,
            weapon_stats,
            armor_stats,
            shield_stats,
            ammunition_stats,
            poison_stats,
            pack_contents,
        }
    }

    /// Check if this gear is a weapon (melee or ranged)
    pub fn is_weapon(&self) -> bool {
        self.weapon_stats.is_some()
    }

    /// Check if this gear is armor
    pub fn is_armor(&self) -> bool {
        self.armor_stats.is_some()
    }

    /// Check if this gear is a shield
    pub fn is_shield(&self) -> bool {
        self.shield_stats.is_some()
    }

    /// Check if this gear is ammunition
    pub fn is_ammunition(&self) -> bool {
        self.ammunition_stats.is_some()
    }

    /// Check if this gear is common gear (no type-specific stats)
    pub fn is_common_gear(&self) -> bool {
        self.weapon_stats.is_none()
            && self.armor_stats.is_none()
            && self.shield_stats.is_none()
            && self.ammunition_stats.is_none()
            && self.poison_stats.is_none()
    }

    /// Check if this gear is a poison
    pub fn is_poison(&self) -> bool {
        self.poison_stats.is_some()
    }

    /// Check if this gear is a pack (contains other items)
    pub fn is_pack(&self) -> bool {
        self.pack_contents.is_some()
    }
}
