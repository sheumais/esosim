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
    effect_id: 193447,
    condition: GrantCondition::EquippedSetCount {
        set_id: 694,
        threshold: 1,
    },
};