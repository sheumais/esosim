use crate::data::enums::gear::ItemQuality;

pub fn get_weapon_powered_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.025,
        ItemQuality::Fine => 0.03,
        ItemQuality::Superior => 0.035,
        ItemQuality::Epic => 0.04,
        ItemQuality::Legendary => 0.045,
    }
}

pub fn get_weapon_charged_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.925,
        ItemQuality::Fine => 0.975,
        ItemQuality::Superior => 1.025,
        ItemQuality::Epic => 1.075,
        ItemQuality::Legendary => 1.125,
    }
}

pub fn get_weapon_precise_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 372.0,
        ItemQuality::Fine => 482.0,
        ItemQuality::Superior => 591.0,
        ItemQuality::Epic => 701.0,
        ItemQuality::Legendary => 810.0,
    }
}

pub fn get_weapon_infused_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.32,
        ItemQuality::Fine => 0.34,
        ItemQuality::Superior => 0.36,
        ItemQuality::Epic => 0.38,
        ItemQuality::Legendary => 0.40,
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
        ItemQuality::Normal => 0.025,
        ItemQuality::Fine => 0.03,
        ItemQuality::Superior => 0.035,
        ItemQuality::Epic => 0.04,
        ItemQuality::Legendary => 0.045,
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
        ItemQuality::Normal => 0.204,
        ItemQuality::Fine => 0.238,
        ItemQuality::Superior => 0.272,
        ItemQuality::Epic => 0.306,
        ItemQuality::Legendary => 0.34,
    }
}

pub fn get_weapon_nirnhoned_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.144,
        ItemQuality::Fine => 0.158,
        ItemQuality::Superior => 0.172,
        ItemQuality::Epic => 0.186,
        ItemQuality::Legendary => 0.2,
    }
}

pub fn get_armor_sturdy_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.02,
        ItemQuality::Fine => 0.025,
        ItemQuality::Superior => 0.03,
        ItemQuality::Epic => 0.035,
        ItemQuality::Legendary => 0.04,
    }
}

pub fn get_armor_impenetrable_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 154.0,
        ItemQuality::Fine => 157.0,
        ItemQuality::Superior => 160.0,
        ItemQuality::Epic => 162.0,
        ItemQuality::Legendary => 165.0,
    }
}

pub fn get_armor_reinforced_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.12,
        ItemQuality::Fine => 0.13,
        ItemQuality::Superior => 0.14,
        ItemQuality::Epic => 0.15,
        ItemQuality::Legendary => 0.16,
    }
}

pub fn get_armor_well_fitted_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.012,
        ItemQuality::Fine => 0.024,
        ItemQuality::Superior => 0.036,
        ItemQuality::Epic => 0.048,
        ItemQuality::Legendary => 0.06,
    }
}

pub fn get_armor_training_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.07,
        ItemQuality::Fine => 0.08,
        ItemQuality::Superior => 0.09,
        ItemQuality::Epic => 0.10,
        ItemQuality::Legendary => 0.11,
    }
}

pub fn get_armor_infused_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.09,
        ItemQuality::Fine => 0.13,
        ItemQuality::Superior => 0.17,
        ItemQuality::Epic => 0.21,
        ItemQuality::Legendary => 0.25,
    }
}

pub fn get_armor_invigorating_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 8.0,
        ItemQuality::Fine => 10.0,
        ItemQuality::Superior => 12.0,
        ItemQuality::Epic => 14.0,
        ItemQuality::Legendary => 16.0,
    }
}

pub fn get_armor_divines_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.051,
        ItemQuality::Fine => 0.061,
        ItemQuality::Superior => 0.071,
        ItemQuality::Epic => 0.081,
        ItemQuality::Legendary => 0.091,
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

/// Bloodthirsty only works on enemies below this health (percent).
pub const BLOODTHIRSTY_THRESHOLD_PCT: u8 = 90;

/// How much of a Bloodthirsty piece's value applies against an enemy at `target_health_pct`:
/// nothing at 90 % health or more, rising in a straight line per 1 % of missing health to all of it
/// at 0 % (the game's "up to 350 Weapon and Spell Damage against enemies under 90 % Health").
pub fn bloodthirsty_fraction(target_health_pct: u8) -> f32 {
    let missing = BLOODTHIRSTY_THRESHOLD_PCT.saturating_sub(target_health_pct);
    missing as f32 / BLOODTHIRSTY_THRESHOLD_PCT as f32
}

/// The most Weapon and Spell Damage a Bloodthirsty piece gives (at 0 % enemy health).
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
        ItemQuality::Normal => 0.24,
        ItemQuality::Fine => 0.33,
        ItemQuality::Superior => 0.42,
        ItemQuality::Epic => 0.51,
        ItemQuality::Legendary => 0.60,
    }
}

/// Health, Magicka, Stamina
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
        ItemQuality::Normal => 1053.0,
        ItemQuality::Fine => 1091.0,
        ItemQuality::Superior => 1128.0,
        ItemQuality::Epic => 1153.0,
        ItemQuality::Legendary => 1190.0,
    }
}

pub fn get_jewelry_swift_value(q: &ItemQuality) -> f32 {
    match q {
        ItemQuality::Normal => 0.03,
        ItemQuality::Fine => 0.04,
        ItemQuality::Superior => 0.05,
        ItemQuality::Epic => 0.06,
        ItemQuality::Legendary => 0.07,
    }
}