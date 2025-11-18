use crate::{monster::Monster, player::Player};

pub trait Effect {
    fn apply(&self, caster: &Player, targets: &mut [Monster]);
}