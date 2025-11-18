use crate::Percent;

#[derive(Default)]
pub struct DamageDone {
    dot: Percent,
    direct: Percent,
    single_target: Percent,
    aoe: Percent,
    magic: Percent,
    physical: Percent,
    shock: Percent,
    flame: Percent,
    frost: Percent,
    poison: Percent,
    disease: Percent,
    bow: Percent,
    pet: Percent,
    global: Percent,
    monster: Percent,
}

impl DamageDone {
    fn new() -> Self {
        Self::default()
    }
}
