//! Conditional output (roadmap 1.7): what the build's sheet leaves out because the build alone cannot
//! decide it, and what a target settles.
//!
//! The first sheet answers every build-time condition (set pieces, weapons, bar, class, food ...) and
//! leaves out an effect whose condition needs combat or a target ("while in combat", "against a
//! target with Minor Vulnerability"). [`Player::conditional_effects`] lists exactly those effects,
//! each with its condition as text, so an editor can show "+X critical damage, when: ...". The second
//! sheet (`Build::sheet_against`) adds a target: the effects it settles switch on, and nothing else
//! changes.

use esosim_types::{Application, ChannelValue, Condition, ConditionContext, EffectSource, Trigger};

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
    /// For an effect whose stacks are the held count. A health gradient has no count until a fight gives it a Health:
    /// this is the number of steps the target settles (`vs_target`'s context), 0 while it does not.
    pub stacks: u32,
    /// What fires the effect, when it is a proc. A proc is always conditional: no sheet counts it.
    pub trigger: Option<Trigger>,
    /// What it puts on the target or the group when it fires (`EffectDef::applies`). Not part of the wearer's stats:
    /// the calculator lists them ("applies Major Breach to the enemy") and a simulator carries them out.
    pub applies: Vec<Application>,
    /// What the effect scales with, when it is a health gradient ("per 10% of the enemy's current Health").
    pub scales_with: Option<String>,
    /// Whether the target this was listed against settles the condition: `Some(true)` it applies and
    /// is in the second sheet, `Some(false)` it does not, `None` the target does not decide it either
    /// (or no target was given).
    pub vs_target: Option<bool>,
}

impl ConditionalEffect {
    /// The condition in words, as `Condition`'s `Display` writes it, with what a health gradient scales with after it
    /// ("always, per 10% of the enemy's current Health").
    pub fn when(&self) -> String {
        let mut text = match &self.trigger {
            Some(t) if self.condition == Condition::Always => format!("when {t}"),
            Some(t) => format!("when {t}, {}", self.condition),
            None => self.condition.to_string(),
        };
        if let Some(scale) = &self.scales_with {
            text = format!("{text}, {scale}");
        }
        text
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
                let gradient = fx.stacking.is_health_gradient();
                // a gradient is undecided until a fight gives it the Health it reads (unless the build already rules
                // it out); any other effect is undecided when its condition is
                let undecided = if fx.trigger.is_some() {
                    // a proc waits for its event, whatever the build says (unless the build already rules it out)
                    fx.condition.evaluate(self) != Some(false)
                } else if gradient {
                    fx.stacking.health_steps(self).is_none() && fx.condition.evaluate(self) != Some(false)
                } else {
                    fx.condition != Condition::Always && fx.condition.evaluate(self).is_none()
                };
                if !undecided {
                    return None;
                }
                // what the target settles: the condition, and for a gradient also that it has a step to count
                let steps = target.and_then(|ctx| fx.stacking.health_steps(ctx));
                let vs_target = target.and_then(|ctx| match (fx.condition.evaluate(ctx), gradient, steps) {
                    (Some(false), _, _) => Some(false),
                    (_, _, _) if fx.trigger.is_some() => None,
                    (_, true, None) => None,
                    (_, true, Some(0)) => Some(false),
                    (decided, _, _) => decided,
                });
                Some(ConditionalEffect {
                    id,
                    name: fx.name.clone(),
                    source: fx.source.clone(),
                    condition: fx.condition.clone(),
                    channels: fx.channels.clone(),
                    stacks: if gradient { steps.unwrap_or(0) } else { fx.effective_stacks(active.stacks_of(id)) },
                    trigger: fx.trigger,
                    applies: fx.applies.clone(),
                    scales_with: fx.stacking.gradient_text(),
                    vs_target,
                })
            })
            .collect()
    }
}
