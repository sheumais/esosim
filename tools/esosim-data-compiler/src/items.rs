//! `item_types.csv` from the game's item table (roadmap 2.8), through the `eso-item-data` crate.
//!
//! The engine needs to know, for an item id, what kind of gear it is: a weapon type, an armour weight,
//! or a ring or necklace (`data/tables/item_type.rs`, `calculate_item_type`). The item table says it in
//! every entry's `item_type_id`; this turns that into the file the engine includes, one line per kind:
//!
//! ```text
//! SWORD,12345,12346,...
//! RING,2001,2002,...
//! ```
//!
//! Kinds are written in the order of [`LABELS`] and ids ascending, so a patch changes only the ids that
//! changed. Items that are not weapons, armour, shields, rings or necklaces are left out.

use std::collections::BTreeMap;
use std::io::Cursor;

use eso_common::ByteReader;
use eso_item_data::enums::item_type::ItemType as Game;
use eso_item_data::{Entry, read_entry, read_header};

/// The labels `calculate_item_type` understands, in output order.
pub const LABELS: [&str; 18] = [
    "AXE", "DAGGER", "MACE", "SWORD", "TWO_HANDED_AXE", "TWO_HANDED_MACE", "TWO_HANDED_SWORD", "FROST_STAFF",
    "FIRE_STAFF", "LIGHTNING_STAFF", "HEALING_STAFF", "SHIELD", "BOW", "LIGHT", "MEDIUM", "HEAVY", "RING",
    "NECKLACE",
];

/// The engine's label for a game item type, `None` for everything the engine has no gear type for.
///
/// Armour is by weight, whichever slot the piece is. `Jerkin` is a light chest (a second style of
/// robe). The weighted neck and ring types are jewellery. The unweighted slot types (`Head`, `Chest`,
/// ...) and `RuneOffHand` are not mapped: nothing says what weight they are.
pub fn label_of(item_type: Game) -> Option<&'static str> {
    use Game::*;
    Some(match item_type {
        OneHandedAxe => "AXE",
        OneHandedMace => "MACE",
        OneHandedSword => "SWORD",
        OneHandedDagger => "DAGGER",
        Shield => "SHIELD",
        TwoHandedSword => "TWO_HANDED_SWORD",
        TwoHandedAxe => "TWO_HANDED_AXE",
        TwoHandedMaul => "TWO_HANDED_MACE",
        Bow => "BOW",
        RestorationStaff => "HEALING_STAFF",
        InfernoStaff => "FIRE_STAFF",
        IceStaff => "FROST_STAFF",
        LightningStaff => "LIGHTNING_STAFF",

        LightHead | LightChest | LightShoulders | LightWaist | LightLegs | LightFeet | LightHands | Jerkin => "LIGHT",
        MediumHead | MediumChest | MediumShoulders | MediumWaist | MediumLegs | MediumFeet | MediumHands => "MEDIUM",
        HeavyHelm | HeavyChestplate | HeavyShoulders | HeavyWaist | HeavyLegs | HeavyFeet | HeavyHands => "HEAVY",

        Amulet | LightNeck | MediumNeck | HeavyNeck => "NECKLACE",
        Ring | LightRing | MediumRing | HeavyRing => "RING",
        _ => return None,
    })
}

/// The `item_types.csv` text for these entries.
pub fn item_types_csv(entries: &[Entry]) -> String {
    let mut by_label: BTreeMap<&'static str, Vec<u32>> = BTreeMap::new();
    for entry in entries {
        if let Some(label) = Game::from_id(&entry.item_type_id).and_then(label_of) {
            by_label.entry(label).or_default().push(entry.item_id);
        }
    }
    let mut out = String::new();
    for label in LABELS {
        let Some(ids) = by_label.get_mut(label) else { continue };
        ids.sort_unstable();
        ids.dedup();
        out.push_str(label);
        for id in ids.iter() {
            out.push(',');
            out.push_str(&id.to_string());
        }
        out.push('\n');
    }
    out
}

/// Decode a raw item table (the file the `eso-item-data` crate reads: its header, then one entry after
/// another). The header's entry count is checked against what was read, because a mismatch means
/// entries were skipped or misread and the types cannot be trusted.
pub fn parse(table: &[u8]) -> Result<Vec<Entry>, String> {
    let mut reader = ByteReader::new(Cursor::new(table));
    let header = read_header(&mut reader).map_err(|e| format!("item table header: {e}"))?;
    let mut entries = Vec::with_capacity(header.total_entries as usize);
    for i in 0..header.total_entries {
        // 0: do not check the entry's own index against its position
        let entry = read_entry(&mut reader, 0).map_err(|e| format!("item table entry {i}: {e}"))?;
        entries.push(entry);
    }
    Ok(entries)
}

/// `item_types.csv` straight from a raw item table.
pub fn item_types_from_table(table: &[u8]) -> Result<String, String> {
    Ok(item_types_csv(&parse(table)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item_id: u32, game: Game) -> Entry {
        Entry { item_id, item_type_id: game.to_id(), ..Entry::default() }
    }

    #[test]
    fn every_label_is_one_the_engine_knows() {
        // the engine's `calculate_item_type` has an arm for each of these and `Unknown` for anything else
        let all = [
            Game::OneHandedAxe, Game::OneHandedMace, Game::OneHandedSword, Game::OneHandedDagger, Game::Shield,
            Game::TwoHandedSword, Game::TwoHandedAxe, Game::TwoHandedMaul, Game::Bow, Game::RestorationStaff,
            Game::InfernoStaff, Game::IceStaff, Game::LightningStaff, Game::LightChest, Game::MediumChest,
            Game::HeavyChestplate, Game::Ring, Game::Amulet,
        ];
        let labels: Vec<&str> = all.iter().map(|t| label_of(*t).unwrap()).collect();
        assert_eq!(labels.len(), LABELS.len());
        for label in labels {
            assert!(LABELS.contains(&label), "{label}");
        }
    }

    #[test]
    fn weapons_armour_and_jewellery_are_written_by_kind_with_ids_ascending() {
        let entries = [
            entry(30, Game::OneHandedSword),
            entry(10, Game::OneHandedSword),
            entry(20, Game::HeavyHands),
            entry(5, Game::Ring),
            entry(6, Game::LightRing),
            entry(7, Game::Amulet),
            entry(8, Game::Jerkin),
            entry(9, Game::Food),
            entry(10, Game::OneHandedSword), // the same id twice is one id
        ];
        assert_eq!(
            item_types_csv(&entries),
            "SWORD,10,30\nLIGHT,8\nHEAVY,20\nRING,5,6\nNECKLACE,7\n"
        );
    }
}
