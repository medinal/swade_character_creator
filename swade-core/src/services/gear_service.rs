use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::{
    AmmunitionStatsRepository, ArmorStatsRepository, GearCategoryRepository, GearRepository,
    PackContentsRepository, PoisonStatsRepository, ShieldStatsRepository, WeaponStatsRepository,
};
use crate::services::{ModifierService, RequirementService};
use crate::views::{
    AmmunitionStatsView, ArmorStatsView, GearCategoryView, GearView, PackContentsView,
    PoisonStatsView, ShieldStatsView, WeaponStatsView,
};

pub struct GearService;

impl GearService {
    /// Get all gear items with full details
    pub fn get_all(conn: &Connection) -> Result<Vec<GearView>> {
        let gear_items = GearRepository::get_all(conn)?;

        let mut views = Vec::new();
        for gear in gear_items {
            let view = Self::build_gear_view(conn, gear)?;
            views.push(view);
        }

        Ok(views)
    }

    /// Get a single gear item by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<GearView>> {
        let gear = GearRepository::get_by_id(conn, id)?;

        match gear {
            Some(gear) => Ok(Some(Self::build_gear_view(conn, gear)?)),
            None => Ok(None),
        }
    }

    /// Get all gear items in a specific category
    pub fn get_by_category_id(conn: &Connection, category_id: i64) -> Result<Vec<GearView>> {
        let gear_items = GearRepository::get_by_category_id(conn, category_id)?;

        let mut views = Vec::new();
        for gear in gear_items {
            let view = Self::build_gear_view(conn, gear)?;
            views.push(view);
        }

        Ok(views)
    }

    /// Get all gear categories
    pub fn get_all_categories(conn: &Connection) -> Result<Vec<GearCategoryView>> {
        let categories = GearCategoryRepository::get_all(conn)?;
        Ok(categories.into_iter().map(GearCategoryView::new).collect())
    }

    /// Build a complete GearView from a Gear model
    fn build_gear_view(conn: &Connection, gear: crate::models::Gear) -> Result<GearView> {
        Self::build_gear_view_internal(conn, gear, true)
    }

    /// Internal helper that can optionally skip loading pack contents to avoid recursion issues
    fn build_gear_view_internal(
        conn: &Connection,
        gear: crate::models::Gear,
        load_pack_contents: bool,
    ) -> Result<GearView> {
        // Load category
        let category = GearCategoryRepository::get_by_id(conn, gear.category_id)?
            .map(GearCategoryView::new)
            .unwrap_or_else(|| GearCategoryView {
                id: gear.category_id,
                name: "Unknown".to_string(),
                description: None,
            });

        // Load modifiers and requirements
        let modifiers = ModifierService::get_for_gear(conn, gear.id)?;
        let requirements = RequirementService::get_for_gear(conn, gear.id)?;

        // Load type-specific stats
        let weapon_stats = WeaponStatsRepository::get_by_gear_id(conn, gear.id)?
            .map(WeaponStatsView::new);
        let armor_stats = ArmorStatsRepository::get_by_gear_id(conn, gear.id)?
            .map(ArmorStatsView::new);
        let shield_stats = ShieldStatsRepository::get_by_gear_id(conn, gear.id)?
            .map(ShieldStatsView::new);
        let ammunition_stats = AmmunitionStatsRepository::get_by_gear_id(conn, gear.id)?
            .map(AmmunitionStatsView::new);
        let poison_stats = PoisonStatsRepository::get_by_gear_id(conn, gear.id)?
            .map(PoisonStatsView::new);

        // Load pack contents if this is a pack (category 13) and we should load them
        let pack_contents = if load_pack_contents {
            Self::load_pack_contents(conn, gear.id)?
        } else {
            None
        };

        Ok(GearView::new(
            gear,
            category,
            modifiers,
            requirements,
            weapon_stats,
            armor_stats,
            shield_stats,
            ammunition_stats,
            poison_stats,
            pack_contents,
        ))
    }

    /// Load pack contents for a gear item (returns None if not a pack or no contents)
    fn load_pack_contents(conn: &Connection, gear_id: i64) -> Result<Option<Vec<PackContentsView>>> {
        let contents = PackContentsRepository::get_by_pack_gear_id(conn, gear_id)?;

        if contents.is_empty() {
            return Ok(None);
        }

        let mut views = Vec::new();
        for content in contents {
            // Load the item gear (don't load its pack contents to avoid recursion)
            if let Some(item_gear) = GearRepository::get_by_id(conn, content.item_gear_id)? {
                let item_view = Self::build_gear_view_internal(conn, item_gear, false)?;
                views.push(PackContentsView::new(
                    item_view,
                    content.quantity,
                    content.notes,
                ));
            }
        }

        if views.is_empty() {
            Ok(None)
        } else {
            Ok(Some(views))
        }
    }
}
