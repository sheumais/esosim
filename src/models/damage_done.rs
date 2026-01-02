#[derive(Default)]
pub struct DamageDone {
    dot: f32,
    direct: f32,
    single_target: f32,
    aoe: f32,
    magic: f32,
    physical: f32,
    shock: f32,
    flame: f32,
    frost: f32,
    poison: f32,
    disease: f32,
    bow: f32,
    pet: f32,
    global: f32,
    monster: f32,
}

impl DamageDone {
    pub fn new() -> Self {
        Self::default()
    }
}
