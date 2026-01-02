use crate::data::item_type::ItemQuality;

pub fn get_weapon_powered_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.025,
        ItemQuality::Fine => 1.03,
        ItemQuality::Superior => 1.035,
        ItemQuality::Epic => 1.04,
        ItemQuality::Legendary => 1.045,
    }
}

pub fn get_weapon_charged_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.975,
        ItemQuality::Fine => 2.025,
        ItemQuality::Superior => 2.075,
        ItemQuality::Epic => 2.125,
        ItemQuality::Legendary => 2.175,
    }
}

pub fn get_weapon_precise_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.016,
        ItemQuality::Fine => 1.021,
        ItemQuality::Superior => 1.026,
        ItemQuality::Epic => 1.031,
        ItemQuality::Legendary => 1.036,
    }
}

pub fn get_weapon_infused_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.10,
        ItemQuality::Fine => 1.15,
        ItemQuality::Superior => 1.20,
        ItemQuality::Epic => 1.25,
        ItemQuality::Legendary => 1.30,
    }
}

pub fn get_weapon_defending_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1428.0,
        ItemQuality::Fine => 1485.0,
        ItemQuality::Superior => 1542.0,
        ItemQuality::Epic => 1580.0,
        ItemQuality::Legendary => 1638.0,
    }
}

pub fn get_weapon_training_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.025,
        ItemQuality::Fine => 1.03,
        ItemQuality::Superior => 1.035,
        ItemQuality::Epic => 1.04,
        ItemQuality::Legendary => 1.045,
    }
}

pub fn get_weapon_sharpened_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1428.0,
        ItemQuality::Fine => 1485.0,
        ItemQuality::Superior => 1542.0,
        ItemQuality::Epic => 1580.0,
        ItemQuality::Legendary => 1638.0,
    }
}

pub fn get_weapon_decisive_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.191,
        ItemQuality::Fine => 1.212,
        ItemQuality::Superior => 1.232,
        ItemQuality::Epic => 1.254,
        ItemQuality::Legendary => 1.275,
    }
}

pub fn get_weapon_nirnhoned_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.11,
        ItemQuality::Fine => 1.12,
        ItemQuality::Superior => 1.13,
        ItemQuality::Epic => 1.14,
        ItemQuality::Legendary => 1.15,
    }
}

pub fn get_armor_sturdy_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.02,
        ItemQuality::Fine => 1.025,
        ItemQuality::Superior => 1.03,
        ItemQuality::Epic => 1.035,
        ItemQuality::Legendary => 1.04,
    }
}

pub fn get_armor_impenetrable_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 116.0,
        ItemQuality::Fine => 118.0,
        ItemQuality::Superior => 121.0,
        ItemQuality::Epic => 124.0,
        ItemQuality::Legendary => 127.0,
    }
}

pub fn get_armor_reinforced_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.12,
        ItemQuality::Fine => 1.13,
        ItemQuality::Superior => 1.14,
        ItemQuality::Epic => 1.15,
        ItemQuality::Legendary => 1.16,
    }
}

pub fn get_armor_well_fitted_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.012,
        ItemQuality::Fine => 1.024,
        ItemQuality::Superior => 1.036,
        ItemQuality::Epic => 1.048,
        ItemQuality::Legendary => 1.06,
    }
}

pub fn get_armor_training_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.07,
        ItemQuality::Fine => 1.08,
        ItemQuality::Superior => 1.09,
        ItemQuality::Epic => 1.10,
        ItemQuality::Legendary => 1.11,
    }
}

pub fn get_armor_infused_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.09,
        ItemQuality::Fine => 1.13,
        ItemQuality::Superior => 1.17,
        ItemQuality::Epic => 1.21,
        ItemQuality::Legendary => 1.25,
    }
}

pub fn get_armor_invigorating_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.08,
        ItemQuality::Fine => 1.10,
        ItemQuality::Superior => 1.12,
        ItemQuality::Epic => 1.14,
        ItemQuality::Legendary => 1.16,
    }
}

pub fn get_armor_divines_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.051,
        ItemQuality::Fine => 1.061,
        ItemQuality::Superior => 1.071,
        ItemQuality::Epic => 1.081,
        ItemQuality::Legendary => 1.091,
    }
}

pub fn get_armor_nirnhoned_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 220.0,
        ItemQuality::Fine => 228.0,
        ItemQuality::Superior => 236.0,
        ItemQuality::Epic => 244.0,
        ItemQuality::Legendary => 253.0,
    }
}

pub fn get_jewelry_healthy_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 844.0,
        ItemQuality::Fine => 877.0,
        ItemQuality::Superior => 910.0,
        ItemQuality::Epic => 932.0,
        ItemQuality::Legendary => 965.0,
    }
}

pub fn get_jewelry_arcane_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 767.0,
        ItemQuality::Fine => 797.0,
        ItemQuality::Superior => 827.0,
        ItemQuality::Epic => 847.0,
        ItemQuality::Legendary => 877.0,
    }
}

pub fn get_jewelry_robust_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 767.0,
        ItemQuality::Fine => 797.0,
        ItemQuality::Superior => 827.0,
        ItemQuality::Epic => 847.0,
        ItemQuality::Legendary => 877.0,
    }
}

pub fn get_jewelry_bloodthirsty_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 70.0,
        ItemQuality::Fine => 140.0,
        ItemQuality::Superior => 210.0,
        ItemQuality::Epic => 280.0,
        ItemQuality::Legendary => 350.0,
    }
}

pub fn get_jewelry_harmony_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 770.0,
        ItemQuality::Fine => 797.0,
        ItemQuality::Superior => 825.0,
        ItemQuality::Epic => 852.0,
        ItemQuality::Legendary => 880.0,
    }
}

pub fn get_jewelry_infused_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.24,
        ItemQuality::Fine => 1.33,
        ItemQuality::Superior => 1.42,
        ItemQuality::Epic => 1.51,
        ItemQuality::Legendary => 1.60,
    }
}

// Health, Magicka, Stamina
pub fn get_jewelry_triune_value(q: &ItemQuality) -> (f32, f32, f32) {
    match q {
        ItemQuality::Normal => (418.0, 380.0, 380.0),
        ItemQuality::Fine => (429.0, 390.0, 390.0),
        ItemQuality::Superior => (451.0, 410.0, 410.0),
        ItemQuality::Epic => (467.0, 425.0, 425.0),
        ItemQuality::Legendary => (478.0, 435.0, 435.0),
    }
}

pub fn get_jewelry_protective_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1624.0,
        ItemQuality::Fine => 1664.0,
        ItemQuality::Superior => 1744.0,
        ItemQuality::Epic => 1804.0,
        ItemQuality::Legendary => 1844.0,
    }
}

pub fn get_jewelry_swift_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 1.03,
        ItemQuality::Fine => 1.04,
        ItemQuality::Superior => 1.05,
        ItemQuality::Epic => 1.06,
        ItemQuality::Legendary => 1.07,
    }
}