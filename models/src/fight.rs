use crate::{monster::Monster, player::Player};

pub struct Fight {
    player: Player,
    monster: Monster,
}

impl Fight {
    fn new_trial_dummy() -> Self {
        let player = Player::new();
        let monster = Monster::new_trial_dummy();

        Self {
            player,
            monster,
        }
    }
}