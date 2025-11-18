use crate::{buff::Effect, damage::DamageType, monster::Monster, player::Player};

pub struct Skill {
    components: Vec<SkillComponent>,
}

pub struct SkillComponent {
    pub max_targets: u8,
    pub damage_type: DamageType,
    pub delay: Option<u32>,
    pub effect: Box<dyn Fn(&Self)>,
}

impl SkillComponent {
    pub fn apply(&self, caster: &Player, targets: &mut [Monster]) {
        (self.effect)(self);
    }
}