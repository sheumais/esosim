use super::grant_rule::{GrantCondition, GrantRule};

pub enum SetBehavior {
    Static(&'static [GrantRule]),
    Dynamic(fn() -> ()),
}

pub struct SetDescriptor {
    pub id: u16,
    pub behavior: SetBehavior,
}

pub static VELOTHI_FORCE_GRANT: GrantRule = GrantRule {
    effect_id: 0, // FORCE_MINOR_ID, see data/effects
    condition: GrantCondition::EquippedSetCount {
        set_id: 694,
        threshold: 1,
    },
};

pub static VELOTHI_HIDDEN_GRANT: GrantRule = GrantRule {
    effect_id: 193447,
    condition: GrantCondition::EquippedSetCount {
        set_id: 694,
        threshold: 1,
    },
};