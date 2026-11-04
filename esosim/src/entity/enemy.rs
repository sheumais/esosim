use crate::engine::active_effects::ActiveEffects;
use crate::stats::armour::Armour;

pub struct Enemy {
    pub id: u32,
    pub active_effects: ActiveEffects,
    pub armour: Armour,
    pub max_health: f64,
    pub current_health: f64,
}

impl Enemy {
    pub fn health_percent(&self) -> f32 {
        if self.max_health <= 0.0 {
            return 0.0;
        }
        (self.current_health / self.max_health * 100.0) as f32
    }
}