use super::grant_rule::GrantRule;

pub enum SetBehavior {
    Static(&'static [GrantRule]),
    Dynamic(fn() -> ()),
}

pub struct SetDescriptor {
    pub id: u16,
    pub behavior: SetBehavior,
}
