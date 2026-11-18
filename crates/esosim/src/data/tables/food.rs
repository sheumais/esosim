//! Food and drink: item id -> the buff (ability id) it gives, for `Build::food` (an item id).
//!
//! Many foods give the same buff (Garlic-and-Pepper Venison Steak, Millet and Beef Stuffed Peppers and Lilmoth Garlic
//! Hagfish are all 61259), so the buff cannot name the food and the item id is what a build stores. The table is
//! `food_items.csv`, one `item id,ability id` line per item, from Wizard's Wardrobe's `BUFFFOOD`. It stops at the
//! items that table has (Sparkling Mudcrab Apple Cider, 171322): a newer food needs a line added.
//! The buff's *value* is not here; the server works it out from the item.

use std::collections::HashMap;
use std::sync::LazyLock;

static FOOD: LazyLock<HashMap<u32, u32>> = LazyLock::new(|| food_from(include_str!("../food_items.csv")));

/// The buff the food item gives, or `None` for an item the table does not have.
pub fn food_ability_of(item_id: u32) -> Option<u32> {
    FOOD.get(&item_id).copied()
}

fn food_from(csv: &str) -> HashMap<u32, u32> {
    csv.lines()
        .filter_map(|line| {
            let (item, ability) = line.trim().split_once(',')?;
            Some((item.parse().ok()?, ability.parse().ok()?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_map_to_their_buff_and_several_items_share_one() {
        assert_eq!(food_ability_of(133_556), Some(100_498)); // Clockwork Citrus Filet
        assert_eq!(food_ability_of(71_059), Some(72_824)); // Orzorga's Smoked Bear Haunch
        assert_eq!(food_ability_of(68_233), food_ability_of(68_235)); // two foods, one buff (61259)
        assert_eq!(food_ability_of(1), None);
        assert_eq!(FOOD.len(), 83);
    }
}
