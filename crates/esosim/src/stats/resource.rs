//! Max health/magicka/stamina and recovery as pure functions of a `ChannelSet`.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::{AggKind, Channel, ResourceKind};
use super::power::{DEFAULT_LEVEL, apply_multiplicative_bps};

/// Base pool: `level_coeff * level + 1000 + attr_coeff * attribute_points`.
fn base_resource(kind: ResourceKind, level: u8, attribute_points: u8) -> i64 {
    let (level_coeff, attr_coeff) = match kind {
        ResourceKind::Health => (300, 122),
        ResourceKind::Magicka | ResourceKind::Stamina => (220, 111),
    };
    level_coeff * level as i64 + 1000 + attr_coeff * attribute_points as i64
}

/// Max pool at `level`: `(base + additive) * (1 + multiplicative)`.
pub fn max_resource_at_level(c: &ChannelSet, kind: ResourceKind, attribute_points: u8, level: u8) -> u32 {
    let base = base_resource(kind, level, attribute_points) + c.get(Channel::Resource(kind, AggKind::Additive));
    apply_multiplicative_bps(base, c.get_multiplicative_bps(Channel::Resource(kind, AggKind::Multiplicative)))
}

/// Max pool at level 50, the level the old `PlayerMaxResource` assumed.
pub fn max_resource(c: &ChannelSet, kind: ResourceKind, attribute_points: u8) -> u32 {
    max_resource_at_level(c, kind, attribute_points, DEFAULT_LEVEL)
}

/// Recovery per tick: the additive bucket scaled by the multiplicative one. No base recovery is
/// modelled yet (the old code had none either).
pub fn recovery(c: &ChannelSet, kind: ResourceKind) -> u32 {
    apply_multiplicative_bps(
        c.get(Channel::Recovery(kind, AggKind::Additive)),
        c.get_multiplicative_bps(Channel::Recovery(kind, AggKind::Multiplicative)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_default_max_health() {
        assert_eq!(max_resource(&ChannelSet::new(), ResourceKind::Health, 0), 16000u32);
    }

    #[test]
    fn test_calculate_default_max_magicka() {
        assert_eq!(max_resource(&ChannelSet::new(), ResourceKind::Magicka, 0), 12000u32);
    }

    #[test]
    fn test_calculate_default_max_stamina() {
        assert_eq!(max_resource(&ChannelSet::new(), ResourceKind::Stamina, 0), 12000u32);
    }

    #[test]
    fn test_calculate_max_health_tank() {
        let mut c = ChannelSet::new();
        let health = |agg| Channel::Resource(ResourceKind::Health, agg);
        c.add_additive(
            health(AggKind::Additive),
            4462 // random crown tri max food
                + 477 * 2 + 193 * 3 + 560 // hakeijo enchants + hero's vigor CP
                + 1206 * 2 // pearlescent ward passive
                + 1000, // nord passive
        );
        c.add_multiplicative_bps(health(AggKind::Multiplicative), 1200); // undaunted passive + heavy armour passive x5
        assert_eq!(max_resource(&c, ResourceKind::Health, 32), 33456u32); // compared with tested value in game
    }

    #[test]
    fn recovery_is_the_additive_bucket_scaled() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), 1000);
        c.add_multiplicative_bps(Channel::Recovery(ResourceKind::Magicka, AggKind::Multiplicative), 500);
        assert_eq!(recovery(&c, ResourceKind::Magicka), 1050);
        assert_eq!(recovery(&c, ResourceKind::Stamina), 0);
    }
}
