use crate::item_type::{GearSlot, ItemQuality, ItemType, is_armour};

pub fn armour_from_armour_piece(item_type: &ItemType, item_slot: &GearSlot, quality: &ItemQuality) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    use GearSlot as G;

    if !is_armour(item_type) {return None}

    Some(match item_type {
        T::Light => match item_slot {
            G::Chest => match quality {
                Q::Normal => 1220,
                Q::Fine => 1268,
                Q::Superior => 1316,
                Q::Epic => 1348,
                Q::Legendary => 1396,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 1067,
                Q::Fine => 1109,
                Q::Superior => 1151,
                Q::Epic => 1179,
                Q::Legendary => 1221,
            },
            G::Hands => match quality {
                Q::Normal => 610,
                Q::Fine => 634,
                Q::Superior => 658,
                Q::Epic => 674,
                Q::Legendary => 698,
            },
            G::Waist => match quality {
                Q::Normal => 457,
                Q::Fine => 475,
                Q::Superior => 493,
                Q::Epic => 505,
                Q::Legendary => 523,
            },
            _ => return None,
        },
        T::Medium => match item_slot {
            G::Chest => match quality {
                Q::Normal => 1820,
                Q::Fine => 1892,
                Q::Superior => 1964,
                Q::Epic => 2012,
                Q::Legendary => 2084,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 1592,
                Q::Fine => 1655,
                Q::Superior => 1718,
                Q::Epic => 1760,
                Q::Legendary => 1823,
            },
            G::Hands => match quality {
                Q::Normal => 910,
                Q::Fine => 946,
                Q::Superior => 982,
                Q::Epic => 1006,
                Q::Legendary => 1042,
            },
            G::Waist => match quality {
                Q::Normal => 682,
                Q::Fine => 709,
                Q::Superior => 736,
                Q::Epic => 754,
                Q::Legendary => 781,
            },
            _ => return None,
        },
        T::Heavy => match item_slot {
            G::Chest => match quality {
                Q::Normal => 2420,
                Q::Fine => 2516,
                Q::Superior => 2612,
                Q::Epic => 2676,
                Q::Legendary => 2772,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 2117,
                Q::Fine => 2201,
                Q::Superior => 2285,
                Q::Epic => 2341,
                Q::Legendary => 2425,
            },
            G::Hands => match quality {
                Q::Normal => 1210,
                Q::Fine => 1258,
                Q::Superior => 1306,
                Q::Epic => 1338,
                Q::Legendary => 1386,
            },
            G::Waist => match quality {
                Q::Normal => 907,
                Q::Fine => 943,
                Q::Superior => 979,
                Q::Epic => 1003,
                Q::Legendary => 1039,
            },
            _ => return None,
        },
        T::Shield => match item_slot {
            G::OffHand | G::OffHandBackup => match quality {
                Q::Normal => 1500,
                Q::Fine => 1560,
                Q::Superior => 1620,
                Q::Epic => 1660,
                Q::Legendary => 1720,
            },
            _ => return None,
        },
        _ => return None,
    })
}