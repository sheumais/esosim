use crate::data::item_type::ItemQuality;

enum EnchantLevel {
    One,
    Five,
    Ten,
    Fifteen,
    Twenty,
    TwentyFive,
    Thirty,
    ThirtyFive,
    Fourty,
    CPTen,
    CPThirty,
    CPFifty,
    CPSeventy,
    CPOneHundred,
    CPOneFifty,
    CPOneSixty,
}

fn match_effective_level_to_enchant_level(effective_level: &u8) -> Option<&EnchantLevel> {
    use EnchantLevel as E;
    match effective_level {
        1..=4 => Some(&E::One),
        5..=9 => Some(&E::Five),
        10..=14 => Some(&E::Ten),
        15..=19 => Some(&E::Fifteen),
        20..=24 => Some(&E::Twenty),
        25..=29 => Some(&E::TwentyFive),
        30..=34 => Some(&E::Thirty),
        35..=39 => Some(&E::ThirtyFive),
        40..=50 => Some(&E::Fourty),
        51..=52 => Some(&E::CPTen),
        53..=54 => Some(&E::CPThirty),
        55..=56 => Some(&E::CPFifty),
        57..=59 => Some(&E::CPSeventy),
        60..=64 => Some(&E::CPOneHundred),
        65 => Some(&E::CPOneFifty),
        66 => Some(&E::CPOneSixty),
        _ => None
    }
}

fn shared_armour_values(enchant_level: &EnchantLevel) -> [f32; 5] {
    use EnchantLevel::*;
    match enchant_level {
        One   => [70.0, 75.0, 80.0, 84.0, 91.0],
        Five => [82.0, 86.0, 93.0, 98.0, 107.0],
        Ten    => [95.0, 100.0, 109.0, 114.0, 124.0],
        Fifteen   => [111.0, 116.0, 126.0, 133.0, 144.0],
        Twenty  => [128.0, 135.0, 146.0, 154.0, 166.0],
        TwentyFive  => [148.0, 156.0, 168.0, 178.0, 192.0],
        Thirty => [170.0, 180.0, 194.0, 204.0, 221.0],
        ThirtyFive => [196.0, 206.0, 223.0, 235.0, 255.0],
        Fourty   => [225.0, 237.0, 257.0, 270.0, 293.0],
        CPTen  => [259.0, 272.0, 295.0, 311.0, 337.0],
        CPThirty   => [297.0, 313.0, 339.0, 356.0, 386.0],
        CPFifty => [341.0, 358.0, 388.0, 409.0, 443.0],
        CPSeventy   => [390.0, 411.0, 445.0, 468.0, 507.0],
        CPOneHundred   => [447.0, 470.0, 509.0, 536.0, 581.0],
        CPOneFifty => [585.0, 615.0, 666.0, 702.0, 761.0],
        CPOneSixty  => [668.0, 704.0, 763.0, 802.0, 868.0],
    }
}

pub fn get_enchant_armour_stamina_value(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values: [f32; 5] = shared_armour_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_armour_magicka_value(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values: [f32; 5] = shared_armour_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_armour_health_value(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use EnchantLevel::*;
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = match enchant_level {
        One   => [77.0, 82.0, 88.0, 92.0, 100.0],
        Five => [90.0, 94.0, 102.0, 107.0, 117.0],
        Ten    => [104.0, 110.0, 119.0, 125.0, 136.0],
        Fifteen   => [122.0, 127.0, 138.0, 146.0, 158.0],
        Twenty  => [140.0, 148.0, 160.0, 160.0, 182.0],
        TwentyFive  => [162.0, 171.0, 184.0, 195.0, 211.0],
        Thirty => [187.0, 198.0, 213.0, 224.0, 243.0],
        ThirtyFive => [215.0, 226.0, 245.0, 258.0, 280.0],
        Fourty   => [247.0, 260.0, 282.0, 297.0, 322.0],
        CPTen  => [284.0, 299.0, 324.0, 342.0, 370.0],
        CPThirty   => [326.0, 344.0, 372.0, 391.0, 424.0],
        CPFifty => [375.0, 393.0, 426.0, 449.0, 487.0],
        CPSeventy   => [429.0, 452.0, 489.0, 514.0, 557.0],
        CPOneHundred   => [491.0, 517.0, 559.0, 589.0, 639.0],
        CPOneFifty => [643.0, 676.0, 732.0, 772.0, 837.0],
        CPOneSixty  => [734.0, 774.0, 839.0, 882.0, 954.0],
    };

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

/// Health, Magicka, Stamina
pub fn get_enchant_armour_prismatic_values(effective_level: &u8, quality: &ItemQuality) -> (f32, f32, f32) {
    let health = get_enchant_armour_health_value(effective_level, quality);
    let magicka = get_enchant_armour_magicka_value(effective_level, quality);
    let stamina = get_enchant_armour_stamina_value(effective_level, quality);

    return ((health / 2.0).floor(), (magicka / 2.0).floor(), (stamina / 2.0).floor())
}

fn shared_jewellery_reduce_values(enchant_level: &EnchantLevel) -> [f32; 5] {
    use EnchantLevel::*;
    match enchant_level {
        One          => [17.0, 18.0, 19.0, 21.0, 22.0],
        Five         => [29.0, 31.0, 33.0, 36.0, 38.0],
        Ten          => [41.0, 44.0, 47.0, 50.0, 54.0],
        Fifteen      => [53.0, 57.0, 61.0, 65.0, 69.0],
        Twenty       => [65.0, 70.0, 75.0, 80.0, 85.0],
        TwentyFive   => [77.0, 83.0, 89.0, 95.0, 101.0],
        Thirty       => [89.0, 96.0, 103.0, 110.0, 117.0],
        ThirtyFive   => [101.0, 109.0, 117.0, 124.0, 132.0],
        Fourty       => [113.0, 122.0, 130.0, 139.0, 148.0],
        CPTen        => [125.0, 135.0, 144.0, 154.0, 164.0],
        CPThirty     => [136.0, 147.0, 157.0, 168.0, 179.0],
        CPFifty      => [140.0, 151.0, 162.0, 173.0, 184.0],
        CPSeventy    => [143.0, 154.0, 165.0, 176.0, 187.0],
        CPOneHundred => [147.0, 158.0, 169.0, 181.0, 192.0],
        CPOneFifty   => [153.0, 165.0, 177.0, 189.0, 201.0],
        CPOneSixty   => [154.0, 167.0, 179.0, 191.0, 203.0],
    }
}

pub fn get_enchant_jewellery_reduce_feat_cost(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_reduce_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_reduce_spell_cost(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_reduce_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_reduce_block_cost(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;
    use EnchantLevel::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = match enchant_level {
        One          => [11.0, 11.0, 12.0, 13.0, 14.0],
        Five         => [19.0, 20.0, 21.0, 23.0, 25.0],
        Ten          => [27.0, 29.0, 31.0, 33.0, 35.0],
        Fifteen      => [34.0, 37.0, 40.0, 42.0, 45.0],
        Twenty       => [42.0, 46.0, 49.0, 52.0, 56.0],
        TwentyFive   => [50.0, 54.0, 58.0, 62.0, 66.0],
        Thirty       => [58.0, 63.0, 67.0, 72.0, 77.0],
        ThirtyFive   => [66.0, 71.0, 77.0, 81.0, 87.0],
        Fourty       => [74.0, 80.0, 85.0, 91.0, 97.0],
        CPTen        => [82.0, 89.0, 95.0, 101.0, 108.0],
        CPThirty     => [89.0, 97.0, 103.0, 110.0, 118.0],
        CPFifty      => [92.0, 99.0, 106.0, 114.0, 121.0],
        CPSeventy    => [94.0, 101.0, 108.0, 116.0, 123.0],
        CPOneHundred => [99.0, 106.0, 114.0, 122.0, 130.0],
        CPOneFifty   => [100.0, 108.0, 116.0, 124.0, 132.0],
        CPOneSixty   => [101.0, 110.0, 118.0, 126.0, 133.0],
    };

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_reduce_all_cost(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;
    use EnchantLevel::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = match enchant_level {
        One          => [11.0, 11.0, 12.0, 13.0, 14.0],
        Five         => [19.0, 20.0, 21.0, 23.0, 25.0],
        Ten          => [27.0, 29.0, 31.0, 33.0, 35.0],
        Fifteen      => [34.0, 37.0, 40.0, 42.0, 45.0],
        Twenty       => [42.0, 46.0, 49.0, 52.0, 56.0],
        TwentyFive   => [50.0, 54.0, 58.0, 62.0, 66.0],
        Thirty       => [58.0, 63.0, 67.0, 72.0, 77.0],
        ThirtyFive   => [66.0, 71.0, 77.0, 81.0, 87.0],
        Fourty       => [74.0, 80.0, 85.0, 91.0, 97.0],
        CPTen        => [82.0, 89.0, 95.0, 101.0, 108.0],
        CPThirty     => [89.0, 97.0, 103.0, 110.0, 118.0],
        CPFifty      => [92.0, 99.0, 106.0, 114.0, 121.0],
        CPSeventy    => [94.0, 101.0, 108.0, 116.0, 123.0],
        CPOneHundred => [99.0, 106.0, 114.0, 122.0, 130.0],
        CPOneFifty   => [100.0, 108.0, 116.0, 124.0, 132.0],
        CPOneSixty   => [101.0, 110.0, 118.0, 126.0, 133.0],
    };

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

fn shared_jewellery_resistance_values(enchant_level: &EnchantLevel) -> [f32; 5] {
    use EnchantLevel::*;    
    match enchant_level {
        One          => [320.0, 342.0, 366.0, 384.0, 416.0],
        Five         => [368.0, 386.0, 418.0, 442.0, 478.0],
        Ten          => [420.0, 444.0, 480.0, 504.0, 546.0],
        Fifteen      => [482.0, 506.0, 548.0, 578.0, 627.0],
        Twenty       => [550.0, 580.0, 629.0, 660.0, 715.0],
        TwentyFive   => [631.0, 662.0, 717.0, 757.0, 820.0],
        Thirty       => [719.0, 759.0, 822.0, 863.0, 935.0],
        ThirtyFive   => [824.0, 865.0, 937.0, 989.0, 1071.0],
        Fourty       => [939.0, 991.0, 1073.0, 1127.0, 1221.0],
        CPTen        => [1075.0, 1129.0, 1223.0, 1290.0, 1398.0],
        CPThirty     => [1225.0, 1292.0, 1400.0, 1470.0, 1593.0],
        CPFifty      => [1402.0, 1472.0, 1595.0, 1682.0, 1823.0],
        CPSeventy    => [1597.0, 1684.0, 1825.0, 1916.0, 2076.0],
        CPOneHundred => [1827.0, 1918.0, 2078.0, 2192.0, 2375.0],
        CPOneFifty   => [2379.0, 2498.0, 2706.0, 2855.0, 3093.0],
        CPOneSixty   => [2708.0, 2857.0, 3095.0, 3250.0, 3520.0],
    }
}

pub fn get_enchant_jewellery_increase_frost_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_disease_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_poison_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_shock_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_fire_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

fn shared_jewellery_resistance_values_2(enchant_level: &EnchantLevel) -> [f32; 5] {
    use EnchantLevel::*;    
    match enchant_level {
        One          => [75.0, 80.0, 86.0, 90.0, 98.0],
        Five         => [88.0, 92.0, 100.0, 106.0, 114.0],
        Ten          => [102.0, 108.0, 116.0, 122.0, 133.0],
        Fifteen      => [118.0, 124.0, 125.0, 142.0, 153.0],
        Twenty       => [137.0, 144.0, 155.0, 164.0, 178.0],
        TwentyFive   => [157.0, 166.0, 180.0, 188.0, 204.0],
        Thirty       => [182.0, 190.0, 206.0, 218.0, 237.0],
        ThirtyFive   => [208.0, 220.0, 239.0, 250.0, 270.0],
        Fourty       => [241.0, 252.0, 272.0, 289.0, 313.0],
        CPTen        => [274.0, 291.0, 315.0, 329.0, 356.0],
        CPThirty     => [317.0, 331.0, 358.0, 380.0, 412.0],
        CPFifty      => [360.0, 382.0, 414.0, 532.0, 468.0],
        CPSeventy    => [416.0, 434.0, 470.0, 499.0, 541.0],
        CPOneHundred => [472.0, 501.0, 543.0, 566.0, 614.0],
        CPOneFifty   => [618.0, 656.0, 711.0, 742.0, 803.0],
        CPOneSixty   => [713.0, 744.0, 805.0, 856.0, 927.0],
    }
}

pub fn get_enchant_jewellery_increase_physical_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values_2(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_spell_resistance(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_resistance_values_2(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

fn shared_jewellery_power_values(enchant_level: &EnchantLevel) -> [f32; 5] {
    use EnchantLevel::*;
    match enchant_level {
        One          => [14.0, 15.0, 16.0, 17.0, 18.0],
        Five         => [16.0, 17.0, 19.0, 20.0, 21.0],
        Ten          => [19.0, 20.0, 22.0, 23.0, 25.0],
        Fifteen      => [22.0, 23.0, 25.0, 27.0, 29.0],
        Twenty       => [26.0, 27.0, 29.0, 31.0, 33.0],
        TwentyFive   => [30.0, 31.0, 34.0, 36.0, 38.0],
        Thirty       => [34.0, 36.0, 39.0, 41.0, 44.0],
        ThirtyFive   => [39.0, 41.0, 45.0, 47.0, 51.0],
        Fourty       => [45.0, 47.0, 51.0, 54.0, 59.0],
        CPTen        => [52.0, 54.0, 59.0, 62.0, 67.0],
        CPThirty     => [59.0, 63.0, 68.0, 71.0, 77.0],
        CPFifty      => [68.0, 72.0, 78.0, 82.0, 89.0],
        CPSeventy    => [78.0, 82.0, 89.0, 94.0, 101.0],
        CPOneHundred => [89.0, 94.0, 102.0, 107.0, 116.0],
        CPOneFifty   => [117.0, 123.0, 133.0, 140.0, 152.0],
        CPOneSixty   => [134.0, 141.0, 153.0, 160.0, 174.0],
    }
}

pub fn get_enchant_jewellery_increase_weapon_damage(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_power_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

pub fn get_enchant_jewellery_increase_spell_damage(effective_level: &u8, quality: &ItemQuality) -> f32 {
    use ItemQuality::*;

    let enchant_level = match match_effective_level_to_enchant_level(effective_level) {
        Some(l) => l,
        None => return 0.0,
    };

    let values = shared_jewellery_power_values(enchant_level);

    match quality {
        Normal    => values[0],
        Fine      => values[1],
        Superior  => values[2],
        Epic      => values[3],
        Legendary => values[4],
    }
}

// stam, mag, health recovery
// potion duration/cooldown

// weapon enchants are more like.. skills I guess?
// treat them as implementable skills and auto-cast them for the player in certain situations onto enemy[0]?

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::item_type::ItemQuality::*;

    #[test]
    fn test_match_effective_level_to_enchant_level_basic_ranges() {
        use EnchantLevel::*;

        assert!(matches!(
            match_effective_level_to_enchant_level(&1),
            Some(One)
        ));
        assert!(matches!(
            match_effective_level_to_enchant_level(&7),
            Some(Five)
        ));
        assert!(matches!(
            match_effective_level_to_enchant_level(&15),
            Some(Fifteen)
        ));
        assert!(matches!(
            match_effective_level_to_enchant_level(&40),
            Some(Fourty)
        ));
        assert!(matches!(
            match_effective_level_to_enchant_level(&54),
            Some(CPThirty)
        ));
        assert!(matches!(
            match_effective_level_to_enchant_level(&66),
            Some(CPOneSixty)
        ));
    }

    #[test]
    fn test_match_effective_level_to_enchant_level_out_of_range() {
        assert!(match_effective_level_to_enchant_level(&0).is_none());
        assert!(match_effective_level_to_enchant_level(&100).is_none());
    }

    #[test]
    fn test_magicka_and_stamina_values_are_identical() {
        let level = 30;
        let quality = Epic;

        let magicka = get_enchant_armour_magicka_value(&level, &quality);
        let stamina = get_enchant_armour_stamina_value(&level, &quality);

        assert_eq!(magicka, stamina);
    }

    #[test]
    fn test_invalid_level_returns_zero_for_magicka_and_stamina() {
        let level = 0;

        assert_eq!(
            get_enchant_armour_magicka_value(&level, &Normal),
            0.0
        );
        assert_eq!(
            get_enchant_armour_stamina_value(&level, &Normal),
            0.0
        );
    }

    #[test]
    fn test_prismatic_values_are_halved_and_floored() {
        let level = 10;
        let quality = Legendary;

        // Health: 136.0 -> 68.0
        // Magicka/Stamina: 124.0 -> 62.0
        let (health, magicka, stamina) =
            get_enchant_armour_prismatic_values(&level, &quality);

        assert_eq!(health, 68.0);
        assert_eq!(magicka, 62.0);
        assert_eq!(stamina, 62.0);
    }
}