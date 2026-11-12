use crate::engine::active_effects::ActiveEffects;
use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::{Channel, ResistableDamageType};
use crate::engine::registry::EffectRegistry;
use crate::stats::armour;
use serde::{Deserialize, Serialize};

pub const TRIAL_DUMMY_ROOT_ID: u32 = 120006;
/// Caused by the root: every ability under it is a buff the dummy gives the *player*; everything else
/// the root causes is a debuff on the dummy.
pub const TRIAL_DUMMY_PLAYER_BUFF_ROOT_ID: u32 = 120016;
pub const TRIAL_DUMMY_MAX_HEALTH: f64 = 21_002_944.0;
pub const TRIAL_DUMMY_RESISTANCE: u32 = 18_200;

/// Debuffs on the trial dummy: Major Breach, Minor Breach, Minor Vulnerability, Crusher, Minor Brittle,
/// Major Vulnerability, Traumatic Burns, Roar of Alkosh, Elemental Catalyst, Off Balance.
pub const TRIAL_DUMMY_DEBUFFS: [u32; 10] = [120010, 120019, 120030, 120007, 148977, 148976, 120011, 120018, 181606, 120014];

/// Buffs the trial dummy gives the player (the abilities under 120016): Minor Toughness, 10 % max
/// resources, Major Force, Major Slayer, Major Savagery, Worm's Raiment, Hircine's Veneer, Minor
/// Brutality, Minor Savagery, Minor Berserk, Major Courage, Minor Courage, Major Brutality, Minor
/// Heroism, Illuminate, Exploitation, Minor Sorcery.
pub const TRIAL_DUMMY_PLAYER_BUFFS: [u32; 17] = [
    120020, 120021, 120013, 177886, 265984, 120024, 120026, 120023, 120029, 120008, 120015, 177885, 265932, 265933,
    265985, 120028, 120017,
];

/// The target a build is calculated against; [`EnemyPreset::to_enemy`] turns it into an [`Enemy`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum EnemyPreset {
    /// A trial dummy: fixed health and resistance, its nine debuffs, and 17 buffs it gives the player.
    #[default]
    TrialDummy,
    /// Any target: health, base resistance and a starting list of debuff ids (none by default).
    Custom {
        max_health: f64,
        resistance: u32,
        #[serde(default)]
        debuffs: Vec<u32>,
    },
}

impl EnemyPreset {
    pub fn max_health(&self) -> f64 {
        match self {
            EnemyPreset::TrialDummy => TRIAL_DUMMY_MAX_HEALTH,
            EnemyPreset::Custom { max_health, .. } => *max_health,
        }
    }

    /// Resistance before any debuff (a `Custom` one or the dummy's 18,200).
    pub fn base_resistance(&self) -> u32 {
        match self {
            EnemyPreset::TrialDummy => TRIAL_DUMMY_RESISTANCE,
            EnemyPreset::Custom { resistance, .. } => *resistance,
        }
    }

    /// Ability ids of the debuffs the enemy starts with.
    pub fn debuffs(&self) -> Vec<u32> {
        match self {
            EnemyPreset::TrialDummy => TRIAL_DUMMY_DEBUFFS.to_vec(),
            EnemyPreset::Custom { debuffs, .. } => debuffs.clone(),
        }
    }

    /// Ability ids of the buffs this target gives the player while it is the target. Only the trial
    /// dummy gives any.
    pub fn player_buffs(&self) -> &'static [u32] {
        match self {
            EnemyPreset::TrialDummy => &TRIAL_DUMMY_PLAYER_BUFFS,
            EnemyPreset::Custom { .. } => &[],
        }
    }

    /// The enemy at full health, its debuffs switched on and its channels aggregated.
    pub fn to_enemy(&self, registry: &EffectRegistry) -> Enemy {
        let mut base_channels = ChannelSet::new();
        base_channels.add_additive(Channel::Armour(ResistableDamageType::All), self.base_resistance() as i64);
        let mut active_effects = ActiveEffects::new();
        for id in self.debuffs() {
            active_effects.set(id, 1);
        }
        let max_health = self.max_health();
        let mut enemy = Enemy {
            id: 0,
            active_effects,
            base_channels,
            channels: ChannelSet::new(),
            player_buffs: self.player_buffs().to_vec(),
            max_health,
            current_health: max_health,
        };
        enemy.recompute(registry);
        enemy
    }
}

pub struct Enemy {
    pub id: u32,
    pub active_effects: ActiveEffects,
    /// What the enemy has before any effect: its resistance. `recompute` starts from it.
    pub base_channels: ChannelSet,
    /// Aggregated stats (resistance, and later anything else a target has), like `Player::channels`:
    /// `base_channels` plus `active_effects`. Call [`Enemy::recompute`] after changing `active_effects`.
    pub channels: ChannelSet,
    /// Ability ids of the buffs this target gives the player (the trial dummy's). `Build::sheet_against`
    /// switches them on for the player; the build's own sheet does not have them.
    pub player_buffs: Vec<u32>,
    pub max_health: f64,
    pub current_health: f64,
}

impl Enemy {
    /// Rebuild `channels` from `base_channels` and the debuffs in `active_effects`. A debuff counts
    /// only when its condition is `Always` (nothing here decides a condition on the target itself),
    /// and two debuffs with the same `buff_code` count once.
    pub fn recompute(&mut self, registry: &EffectRegistry) {
        let mut channels = self.base_channels.clone();
        channels.merge(&self.active_effects.aggregate(registry));
        self.channels = channels;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::effect::{ChannelValue, Condition, EffectDef, EffectSource, Stacking};

    fn debuff(id: u32, armour: f64, buff_code: Option<u8>) -> EffectDef {
        EffectDef {
            id,
            name: format!("test {id}"),
            source: EffectSource::Debuff { ability_id: id },
            channels: vec![ChannelValue { channel: Channel::Armour(ResistableDamageType::All), value: armour, value_per_stack: 0.0 }],
            condition: Condition::Always,
            stacking: Stacking::Unique,
            buff_code,
            duration_ms: None,
            cooldown_ms: None,
            note: None,
        }
    }

    #[test]
    fn a_custom_enemy_has_its_health_resistance_and_debuffs() {
        let registry = EffectRegistry::from_defs([debuff(1, -1000.0, None), debuff(2, -500.0, None)]);
        let preset = EnemyPreset::Custom { max_health: 1_000_000.0, resistance: 10_000, debuffs: vec![1] };
        let enemy = preset.to_enemy(&registry);
        assert_eq!(enemy.max_health, 1_000_000.0);
        assert_eq!(enemy.health_percent(), 100.0);
        assert_eq!(enemy.resistance(ResistableDamageType::Physical), 9_000);
        assert!(enemy.player_buffs.is_empty());
    }

    #[test]
    fn toggling_a_debuff_and_recomputing_changes_the_resistance() {
        let registry = EffectRegistry::from_defs([debuff(1, -1000.0, None)]);
        let mut enemy = EnemyPreset::Custom { max_health: 1.0, resistance: 5_000, debuffs: vec![] }.to_enemy(&registry);
        assert_eq!(enemy.resistance(ResistableDamageType::All), 5_000);
        enemy.active_effects.set(1, 1);
        enemy.recompute(&registry);
        assert_eq!(enemy.resistance(ResistableDamageType::All), 4_000);
        enemy.active_effects.set(1, 0);
        enemy.recompute(&registry);
        assert_eq!(enemy.resistance(ResistableDamageType::All), 5_000);
    }

    #[test]
    fn two_debuffs_with_one_buff_code_count_once() {
        let registry = EffectRegistry::from_defs([debuff(1, -2974.0, Some(40)), debuff(2, -2974.0, Some(40))]);
        let preset = EnemyPreset::Custom { max_health: 1.0, resistance: 10_000, debuffs: vec![1, 2] };
        assert_eq!(preset.to_enemy(&registry).resistance(ResistableDamageType::All), 7_026);
    }

    #[test]
    fn a_custom_enemy_in_json_needs_no_debuff_list() {
        let preset: EnemyPreset = serde_json::from_str(r#"{"Custom":{"max_health":5.0,"resistance":7}}"#).unwrap();
        assert_eq!(preset, EnemyPreset::Custom { max_health: 5.0, resistance: 7, debuffs: vec![] });
    }

    #[test]
    fn the_trial_dummy_is_fixed() {
        let p = EnemyPreset::TrialDummy;
        assert_eq!((p.max_health(), p.base_resistance()), (21_002_944.0, 18_200));
        assert_eq!((p.debuffs().len(), p.player_buffs().len()), (10, 17));
    }

    /// The shipped data: the dummy's nine debuffs take 18,200 resistance down by Major Breach 5948,
    /// Minor Breach 2974, Crusher 2108 and Roar of Alkosh 6000, and every id of the preset has an effect.
    #[test]
    fn the_trial_dummy_against_the_shipped_effects() {
        let registry = EffectRegistry::embedded();
        let preset = EnemyPreset::TrialDummy;
        for id in preset.debuffs().into_iter().chain(preset.player_buffs().iter().copied()) {
            if id != 120014 {
                assert!(registry.contains(id), "no effect for trial dummy ability {id}");
            }
        }
        for id in preset.debuffs() {
            if id != 120014 {
                assert!(matches!(registry.get(id).unwrap().source, EffectSource::Debuff { .. }), "{id} is not a debuff");
            }
        }
        for id in preset.player_buffs() {
            assert!(matches!(registry.get(*id).unwrap().source, EffectSource::Buff { .. }), "{id} is not a buff");
        }
        let enemy = preset.to_enemy(&registry);
        assert_eq!(enemy.resistance(ResistableDamageType::Physical), 18_200 - 5_948 - 2_974 - 2_108 - 6_000);
        assert_eq!(enemy.channels.get(Channel::CriticalDamageTaken), 10 + 15);
    }
}
