use crate::engine::active_effects::ActiveEffects;
use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::ResistableDamageType;
use crate::stats::armour;
use serde::{Deserialize, Serialize};

/// The target a build is calculated against (roadmap 2.9 turns a preset into an `Enemy`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum EnemyPreset {
    #[default]
    TrialDummy,
    Custom { max_health: f64, resistance: u32 },
}

pub struct Enemy {
    pub id: u32,
    pub active_effects: ActiveEffects,
    /// Aggregated stats (resistance, and later anything else a target has), like `Player::channels`.
    pub channels: ChannelSet,
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

    pub fn resistance(&self, kind: ResistableDamageType) -> u32 {
        armour::resistance(&self.channels, kind)
    }
}
