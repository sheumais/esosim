//! Conditional output (roadmap 1.7): what the build's sheet leaves out because the build alone cannot
//! decide it, and what a target settles.
//!
//! The first sheet answers every build-time condition (set pieces, weapons, bar, class, food ...) and
//! leaves out an effect whose condition needs combat or a target ("while in combat", "against a
//! target with Minor Vulnerability"). [`Player::conditional_effects`] lists exactly those effects,
//! each with its condition as text, so an editor can show "+X critical damage, when: ...". The second
//! sheet (`Build::sheet_against`) adds a target: the effects it settles switch on, and nothing else
//! changes.

use esosim_types::{ChannelValue, Condition, ConditionContext, EffectSource};

use crate::engine::grant_rule::GrantRule;
use crate::engine::registry::EffectRegistry;
use crate::entity::player::Player;

/// An effect that is on in the build but whose condition the build cannot decide.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalEffect {
    pub id: u32,
    pub name: String,
    pub source: EffectSource,
    pub condition: Condition,
    /// What it gives, per stack as the effect's data says (`EffectDef::channels`).
    pub channels: Vec<ChannelValue>,
    pub stacks: u32,
    /// Whether the target this was listed against settles the condition: `Some(true)` it applies and
    /// is in the second sheet, `Some(false)` it does not, `None` the target does not decide it either
    /// (or no target was given).
    pub vs_target: Option<bool>,
}

impl ConditionalEffect {
    /// The condition in words, as `Condition`'s `Display` writes it.
    pub fn when(&self) -> String {
        self.condition.to_string()
    }
}

impl Player {
    /// The effects that are on but left out of `channels` because their condition is `None` for the
    /// player alone, in effect id order. Effects the build decides are not listed: one that holds is
    /// in the sheet, one that fails (a Destruction Staff bonus with no staff) is simply not on.
    ///
    /// With `target`, `vs_target` says what that context makes of each condition.
    pub fn conditional_effects(
        &self,
        registry: &EffectRegistry,
        grant_rules: &[&GrantRule],
        target: Option<&dyn ConditionContext>,
    ) -> Vec<ConditionalEffect> {
        let active = self.effective_effects(registry, grant_rules);
        let mut ids: Vec<u32> = active.ids().collect();
        ids.sort_unstable();
        ids.into_iter()
            .filter_map(|id| {
                let fx = registry.get(id)?;
                if fx.condition == Condition::Always || fx.condition.evaluate(self).is_some() {
                    return None;
                }
                Some(ConditionalEffect {
                    id,
                    name: fx.name.clone(),
                    source: fx.source.clone(),
                    condition: fx.condition.clone(),
                    channels: fx.channels.clone(),
                    stacks: fx.effective_stacks(active.stacks_of(id)),
                    vs_target: target.and_then(|ctx| fx.condition.evaluate(ctx)),
                })
            })
            .collect()
    }
}
