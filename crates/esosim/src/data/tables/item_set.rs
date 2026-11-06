//! Item id -> set id (roadmap 2.1e), for import and search ("I have this item").
//!
//! The data is `set_items.csv` (written by the compiler from the set table's item lists):
//! one line per set, `set id,item id,item id,...`. An item belongs to at most one set.

use std::collections::HashMap;
use std::sync::LazyLock;

pub static ITEM_SETS: LazyLock<HashMap<u32, u16>> = LazyLock::new(|| {
    item_sets_from(esosim_data::SET_ITEMS_CSV).unwrap_or_else(|e| panic!("esosim-data set_items.csv: {e}"))
});

/// The set `item_id` belongs to, or `None` for an item that is in no set.
pub fn set_of_item(item_id: u32) -> Option<u16> {
    ITEM_SETS.get(&item_id).copied()
}

pub fn item_sets_from(csv: &str) -> Result<HashMap<u32, u16>, String> {
    let mut out = HashMap::new();
    for (n, line) in csv.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let mut cells = line.trim().split(',');
        let set = cells.next().unwrap_or("");
        let set: u16 = set.parse().map_err(|_| format!("line {}: bad set id {set:?}", n + 1))?;
        for cell in cells {
            let item: u32 = cell.parse().map_err(|_| format!("line {}: bad item id {cell:?}", n + 1))?;
            if let Some(prev) = out.insert(item, set) {
                return Err(format!("line {}: item {item} is in sets {prev} and {set}", n + 1));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lines_and_rejects_an_item_in_two_sets() {
        let m = item_sets_from("19,5,6\n20,7\n").unwrap();
        assert_eq!((m[&5], m[&6], m[&7]), (19, 19, 20));
        assert!(item_sets_from("19,5\n20,5\n").is_err());
        assert!(item_sets_from("x,5\n").is_err());
    }

    /// Pinned from the 717-set table: 119727 items, none shared. Set 20 (id 0x14) starts 101533.
    #[test]
    fn the_shipped_table_maps_items_to_sets() {
        assert_eq!(ITEM_SETS.len(), 119_727);
        assert_eq!(set_of_item(101_533), Some(20));
        assert_eq!(set_of_item(1), None);
    }
}
