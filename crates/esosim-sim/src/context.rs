//! 5.4 — `ConditionContext` over live combat state.
//!
//! The engine's `Player` answers every *build-time* condition (set pieces, bar, weapons ...) and
//! `TargetContext` adds what an `Enemy` snapshot knows (its debuffs and health). Everything else a
//! condition can ask for (my buffs and stacks, my health, how long ago something happened, whether I
//! am casting) is only known in a run, so [`CombatContext`] answers it from [`CombatState`].
//!
//! It wraps a `TargetContext` and delegates what that answers, as `TargetContext` delegates to
//! `Player`; the combat-time methods are the simulator's own. Passing it to
//! `Player::derive` / `EffectDef` aggregation is what makes "while the target has Off Balance" and
//! "within 5 seconds of a critical" decide for real. See the doc on each method for where its answer
//! comes from.

use eso_skill_data::enums::skill_line::SkillLine;
use esosim::entity::condition_context::TargetContext;
use esosim::entity::player::Player;
use esosim_types::{
    ArmourWeight, Bar, CastKind, ConditionContext, PlayerClass, ResourceKind, SelfState, TargetKind, TriggerEvent,
    WeaponKind, WeaponType,
};

use crate::sim::CombatState;

pub struct CombatContext<'a> {
    /// Build-time answers (the player) and the target's (the enemy snapshot in `state`).
    pub target: TargetContext<'a>,
    pub state: &'a CombatState,
}

impl<'a> CombatContext<'a> {
    pub fn new(player: &'a Player, state: &'a CombatState, kinds: &'a [TargetKind]) -> Self {
        unimplemented!()
    }
}

impl ConditionContext for CombatContext<'_> {
    // --- build-time: the player's answers, through the target context ---
    fn set_pieces(&self, set_id: u16) -> Option<u8> {
        unimplemented!()
    }
    fn armour_pieces(&self, weight: ArmourWeight) -> Option<u8> {
        unimplemented!()
    }
    fn skill_line_slotted(&self, line: SkillLine) -> Option<u8> {
        unimplemented!()
    }
    fn ability_slotted(&self, ability_id: u32) -> Option<bool> {
        unimplemented!()
    }
    fn on_bar(&self, bar: Bar) -> Option<bool> {
        unimplemented!()
    }
    fn weapon_equipped(&self, kind: WeaponKind) -> Option<bool> {
        unimplemented!()
    }
    fn weapons_of_type(&self, weapon: WeaponType) -> Option<u8> {
        unimplemented!()
    }
    fn class(&self) -> Option<PlayerClass> {
        unimplemented!()
    }
    fn food_active(&self) -> Option<bool> {
        unimplemented!()
    }
    fn armour_weights_worn(&self) -> Option<u8> {
        unimplemented!()
    }
    fn class_abilities_slotted(&self, class: PlayerClass) -> Option<u8> {
        unimplemented!()
    }
    fn ability_slotted_any_bar(&self, ability_id: u32) -> Option<bool> {
        unimplemented!()
    }
    fn in_pvp_area(&self) -> Option<bool> {
        unimplemented!()
    }
    fn near_keep(&self) -> Option<bool> {
        unimplemented!()
    }

    // --- the target: the enemy snapshot kept in step with the run ---
    fn target_debuff_active(&self, id: u32) -> Option<bool> {
        unimplemented!()
    }
    fn target_debuff_stacks(&self, id: u32) -> Option<u32> {
        unimplemented!()
    }
    /// From `EnemyState::health_pct` at the current time, rounded down like `TargetContext`'s.
    fn target_health_pct(&self) -> Option<u8> {
        unimplemented!()
    }
    fn target_is(&self, kind: TargetKind) -> Option<bool> {
        unimplemented!()
    }
    /// From `EnemyState::distance_m`.
    fn target_distance(&self) -> Option<f32> {
        unimplemented!()
    }

    // --- the player in combat: the simulator's own ---
    /// From the player's `LiveEffects`.
    fn self_buff_active(&self, id: u32) -> Option<bool> {
        unimplemented!()
    }
    fn self_buff_stacks(&self, id: u32) -> Option<u32> {
        unimplemented!()
    }
    fn self_health_pct(&self) -> Option<u8> {
        unimplemented!()
    }
    fn self_resource_pct(&self, resource: ResourceKind) -> Option<u8> {
        unimplemented!()
    }
    fn ultimate(&self) -> Option<u16> {
        unimplemented!()
    }
    fn in_combat(&self) -> Option<bool> {
        unimplemented!()
    }
    /// `Channeling` while a cast time or channel is running; `DamageShielded`, `Blocking` ... as the
    /// run models them (none yet: `None`).
    fn self_state(&self, state: SelfState) -> Option<bool> {
        unimplemented!()
    }
    /// From the `EventHistory`: this is what `Condition::RecentlyOccurred` reads.
    fn ms_since(&self, event: TriggerEvent) -> Option<u32> {
        unimplemented!()
    }
    /// Whether the cast in progress is of this kind: needs the ability being cast.
    fn casting(&self, kind: CastKind) -> Option<bool> {
        unimplemented!()
    }
}
