use eso_skill_data::enums::skill_line::SkillLine;

use std::sync::LazyLock;

use esosim_types::{ArmourWeight, Condition, EffectDef, EffectSource, SetBehaviour, Stacking};

use crate::{data::enums::gear::ItemType, entity::player::Player};

#[derive(Debug)]
pub enum GrantCondition {
    EquippedItemTypeCount(ItemType),
    ActiveSkillsFromLine(SkillLine),
    EquippedSetCount { set_id: u16, threshold: u8 },
    Always,
}

#[derive(Debug)]
pub struct GrantRule { 
    pub effect_id: u32,
    pub condition: GrantCondition 
}

impl GrantRule {
    pub fn evaluate(&self, player: &Player) -> Option<u32> {
        let stacks = match self.condition {
            GrantCondition::EquippedItemTypeCount(t) =>
                player.get_number_of_equipped_item_type(&t) as u32,
            GrantCondition::ActiveSkillsFromLine(l) =>
                player.get_number_of_active_skills_from_skill_line(&l) as u32,
            GrantCondition::EquippedSetCount { set_id, threshold } =>
                (player.get_number_of_equipped_set(set_id) >= threshold) as u32,
            GrantCondition::Always => 1,
        };
        (stacks > 0).then_some(stacks)
    }
}

pub static GRANT_RULES: &[GrantRule] = &[
    // GrantRule { effect_id: DEXTERITY_ID, condition: GrantCondition::EquippedItemTypeCount(ItemType::Medium) },
    // GrantRule { effect_id: FROZEN_ARMOUR_ID, condition: GrantCondition::ActiveSkillsFromLine(SkillLine::WintersEmbrace) },
];

/// Rules for every set that gives a Major/Minor buff at all times, generated from
/// `set_behaviour.jsonl` (roadmap 2.1b/2.1c): "Minor Slayer at 3 pieces of Ansuul's Torment", and
/// so on. A perfected set has none of its own: the engine already counts its pieces towards the
/// regular set (`Player::get_active_sets_counts`), so the regular set's rule fires for it.
pub static SET_GRANT_RULES: LazyLock<Vec<GrantRule>> = LazyLock::new(|| {
    set_grant_rules_from(esosim_data::SET_BEHAVIOUR_JSONL)
        .unwrap_or_else(|e| panic!("esosim-data set_behaviour.jsonl: {e}"))
});

/// One rule per `(buff, pieces)` of every set's own `GrantsBuff` blocks.
pub fn set_grant_rules_from(jsonl: &str) -> Result<Vec<GrantRule>, String> {
    let mut rules = Vec::new();
    for (i, line) in jsonl.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let set: SetBehaviour = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        if set.inherits.is_some() {
            continue;
        }
        let set_id = u16::try_from(set.id).map_err(|_| format!("line {}: set id {} does not fit u16", i + 1, set.id))?;
        for (effect_id, threshold) in set.grants() {
            rules.push(GrantRule {
                effect_id,
                condition: GrantCondition::EquippedSetCount { set_id, threshold },
            });
        }
    }
    Ok(rules)
}

/// Every grant rule the calculator uses: the hand-written ones plus the generated set grants.
/// (The rules for hand-written *set effects* depend on the effect data in use: see
/// [`set_effect_rules`].)
pub fn all_grant_rules() -> Vec<&'static GrantRule> {
    GRANT_RULES.iter().chain(SET_GRANT_RULES.iter()).collect()
}

/// One rule per effect whose source is `EffectSource::Set { set_id, pieces_required }`: switch the
/// effect on (its own id, one stack) when at least `pieces_required` pieces of the set are worn
/// (roadmap 2.2). This is how a class 3 set bonus written in `overrides.toml` reaches the sheet: the
/// effect carries the set and the piece count, so nothing else has to say when it applies. Any
/// further condition (a weapon, a health threshold) is the effect's own `condition`, decided when the
/// effects are aggregated.
///
/// Perfected sets need no rule of their own: their pieces already count towards the regular set
/// (`Player::get_active_sets_counts`), so write the effect against the regular set. Rules come out in
/// effect id order.
pub fn set_effect_rules<'a>(effects: impl IntoIterator<Item = &'a EffectDef>) -> Vec<GrantRule> {
    let mut found: Vec<(u32, u16, u8)> = effects
        .into_iter()
        .filter_map(|def| match def.source {
            EffectSource::Set { set_id, pieces_required } => Some((def.id, set_id, pieces_required)),
            _ => None,
        })
        .collect();
    found.sort_unstable();
    found
        .into_iter()
        .map(|(effect_id, set_id, threshold)| GrantRule {
            effect_id,
            condition: GrantCondition::EquippedSetCount { set_id, threshold },
        })
        .collect()
}

/// Which count gives an effect its stacks: "for each piece of Heavy Armor worn" is
/// `ArmourPieces`, "for each Sorcerer ability slotted" is `SkillLineSlotted`. Looked for at the top of the
/// condition and inside an `All`; any other condition is decided when the effects are aggregated.
fn counted_by(condition: &Condition) -> Option<GrantCondition> {
    match condition {
        Condition::ArmourPieces { weight, .. } => Some(GrantCondition::EquippedItemTypeCount(match weight {
            ArmourWeight::Light => ItemType::Light,
            ArmourWeight::Medium => ItemType::Medium,
            ArmourWeight::Heavy => ItemType::Heavy,
        })),
        Condition::SkillLineSlotted { skill_line, .. } => Some(GrantCondition::ActiveSkillsFromLine(*skill_line)),
        Condition::All(parts) => parts.iter().find_map(counted_by),
        _ => None,
    }
}

/// One rule per passive written as an effect (roadmap 2.4) that the build has unlocked. Every passive works the same
/// way: nothing is on unless `unlocked` (the build's `passives`) holds the passive's `ability_id`. The effects whose
/// source is `Race { name, .. }` / `Class { name, .. }` also have to be named like the build's race / class, so an id
/// of another race's or class's passive does nothing; `Passive` (armour-weight, weapon, world and guild passives) has
/// no such restriction. An unlocked passive still has its `condition` decided when the effects are aggregated.
///
/// An `Additive` effect whose condition counts something ("per piece of Heavy Armor", "per Shadow ability
/// slotted": `ArmourPieces`, `SkillLineSlotted`) is on with one stack per piece or ability on the active bar, so its
/// `value` is the amount for one; every other effect is on with one stack and keeps its `condition` for the
/// aggregation. Rules come out in effect id order.
pub fn passive_effect_rules<'a>(
    effects: impl IntoIterator<Item = &'a EffectDef>,
    race: &str,
    class: &str,
    unlocked: &[u32],
) -> Vec<GrantRule> {
    let mut found: Vec<(u32, GrantCondition)> = effects
        .into_iter()
        .filter(|def| match &def.source {
            EffectSource::Race { name, ability_id } => name == race && unlocked.contains(ability_id),
            EffectSource::Class { name, ability_id } => name == class && unlocked.contains(ability_id),
            EffectSource::Passive { ability_id } => unlocked.contains(ability_id),
            _ => false,
        })
        .map(|def| {
            let condition = match def.stacking {
                Stacking::Additive => counted_by(&def.condition).unwrap_or(GrantCondition::Always),
                _ => GrantCondition::Always,
            };
            (def.id, condition)
        })
        .collect();
    found.sort_unstable_by_key(|(id, _)| *id);
    found.into_iter().map(|(effect_id, condition)| GrantRule { effect_id, condition }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::{GearSlot, ItemQuality};
    use crate::data::skill_data::major_minor::{AEGIS_MINOR_ID, FORCE_MINOR_ID, SLAYER_MINOR_ID};
    use crate::data::tables::set_ids::*;
    use crate::entity::gear::GearPiece;

    /// `(set id, pieces)` of the generated rules that give `buff`.
    fn rules_for(buff: u32) -> Vec<(u16, u8)> {
        let mut v: Vec<_> = SET_GRANT_RULES
            .iter()
            .filter(|r| r.effect_id == buff)
            .map(|r| match r.condition {
                GrantCondition::EquippedSetCount { set_id, threshold } => (set_id, threshold),
                _ => panic!("a set grant rule with a non-set condition"),
            })
            .collect();
        v.sort_unstable();
        v
    }

    /// The two hand lists this replaced had 22 Minor Slayer and 21 Minor Aegis sets, all at 3
    /// pieces; the generated rules reproduce them (checked set by set when they were removed).
    #[test]
    fn generated_rules_reproduce_the_old_slayer_and_aegis_lists() {
        let slayer = rules_for(SLAYER_MINOR_ID);
        let aegis = rules_for(AEGIS_MINOR_ID);
        assert_eq!((slayer.len(), aegis.len()), (22, 21));
        assert!(slayer.iter().chain(&aegis).all(|(_, pieces)| *pieces == 3));
        for set in [ANSUULS_TORMENT_SET_ID, MOONDANCER_SET_ID, WAR_MACHINE_SET_ID] {
            assert!(slayer.contains(&(set, 3)), "{set}");
        }
        assert!(aegis.contains(&(TWILIGHT_REMEDY_SET_ID, 3)));
    }

    #[test]
    fn velothi_force_is_generated_as_minor_force_at_one_piece() {
        assert!(rules_for(FORCE_MINOR_ID).contains(&(VELOTHI_UR_MAGES_AMULET_SET_ID, 1)));
    }

    /// Mighty Chudan is a monster set: head + shoulders, 2 pieces at most. The grant used to be
    /// inferred at 3 pieces (the first empty slot), which a monster set can never reach.
    #[test]
    fn mighty_chudan_grants_major_resolve_at_two_pieces_and_slimecraw_minor_berserk() {
        use crate::data::skill_data::major_minor::{BERSERK_MINOR_ID, RESOLVE_MAJOR_ID};
        assert_eq!(rules_for(RESOLVE_MAJOR_ID).iter().filter(|(s, _)| *s == MIGHTY_CHUDAN_SET_ID).collect::<Vec<_>>(), [&(MIGHTY_CHUDAN_SET_ID, 2)]);
        assert!(rules_for(BERSERK_MINOR_ID).contains(&(SLIMECRAW_SET_ID, 2)));

        let mut player = Player::new();
        let piece = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(MIGHTY_CHUDAN_SET_ID), None);
        player.set_gear_piece(GearSlot::Head, piece.clone());
        assert!(!evaluates(RESOLVE_MAJOR_ID, &player));
        player.set_gear_piece(GearSlot::Shoulders, piece);
        assert!(evaluates(RESOLVE_MAJOR_ID, &player));
    }

    /// 2.1c dropped these four because a set with no stat rows had no piece count. A mythic is one item.
    #[test]
    fn the_four_mythics_that_grant_a_buff_are_one_piece_rules() {
        for set in [SEA_SERPENTS_COIL_SET_ID, OAKENSOUL_RING_SET_ID, CRYPTCANON_VESTMENTS_SET_ID, SHATTERED_PATHS_SIGNET_SET_ID] {
            let thresholds: Vec<u8> = SET_GRANT_RULES
                .iter()
                .filter_map(|r| match r.condition {
                    GrantCondition::EquippedSetCount { set_id, threshold } if set_id == set => Some(threshold),
                    _ => None,
                })
                .collect();
            assert!(!thresholds.is_empty() && thresholds.iter().all(|t| *t == 1), "set {set}: {thresholds:?}");
        }
        use crate::data::skill_data::major_minor::{COURAGE_MINOR_ID, HEROISM_MINOR_ID};
        assert!(rules_for(COURAGE_MINOR_ID).contains(&(OAKENSOUL_RING_SET_ID, 1)));
        assert!(rules_for(HEROISM_MINOR_ID).contains(&(CRYPTCANON_VESTMENTS_SET_ID, 1)));
        let mut player = Player::new();
        player.set_gear_piece(GearSlot::Ring1, GearPiece::new(ItemType::Medium, 66, None, ItemQuality::Legendary, Some(OAKENSOUL_RING_SET_ID), None));
        assert!(evaluates(COURAGE_MINOR_ID, &player));
    }

    #[test]
    fn perfected_sets_have_no_rules_of_their_own() {
        assert!(SET_GRANT_RULES.iter().all(|r| !matches!(
            r.condition,
            GrantCondition::EquippedSetCount { set_id, .. } if set_id == PERFECTED_ANSUULS_TORMENT_SET_ID
        )));
    }

    fn player_with(set_id: u16, pieces: usize) -> Player {
        let mut player = Player::new();
        let piece = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(set_id), None);
        for slot in [GearSlot::Chest, GearSlot::Legs, GearSlot::Hands, GearSlot::Feet].into_iter().take(pieces) {
            player.set_gear_piece(slot, piece.clone());
        }
        player
    }

    fn evaluates(buff: u32, player: &Player) -> bool {
        all_grant_rules().iter().any(|r| r.effect_id == buff && r.evaluate(player).is_some())
    }

    #[test]
    fn minor_slayer_needs_three_pieces_and_perfected_pieces_count() {
        assert!(!evaluates(SLAYER_MINOR_ID, &player_with(ANSUULS_TORMENT_SET_ID, 2)));
        assert!(evaluates(SLAYER_MINOR_ID, &player_with(ANSUULS_TORMENT_SET_ID, 3)));
        assert!(evaluates(SLAYER_MINOR_ID, &player_with(PERFECTED_ANSUULS_TORMENT_SET_ID, 3)));
    }

    fn set_effect(id: u32, set_id: u16, pieces_required: u8) -> EffectDef {
        EffectDef {
            id,
            name: format!("set effect {id}"),
            source: EffectSource::Set { set_id, pieces_required },
            channels: vec![],
            condition: esosim_types::Condition::Always,
            stacking: esosim_types::Stacking::Unique,
            buff_code: None,
            duration_ms: None,
            cooldown_ms: None,
            note: None,
        }
    }

    /// Only effects with a set source become rules, and the rule fires at exactly the piece count the
    /// effect names; perfected pieces count towards the regular set's effect.
    #[test]
    fn a_set_effect_is_granted_at_its_piece_count() {
        let mut food = set_effect(9, 0, 0);
        food.source = EffectSource::Food;
        let defs = [set_effect(20, ANSUULS_TORMENT_SET_ID, 3), food, set_effect(10, MOONDANCER_SET_ID, 5)];
        let rules = set_effect_rules(&defs);
        assert_eq!(rules.iter().map(|r| r.effect_id).collect::<Vec<_>>(), [10, 20]);

        let at = |set, n| player_with(set, n);
        let fires = |effect: u32, player: &Player| rules.iter().any(|r| r.effect_id == effect && r.evaluate(player).is_some());
        assert!(!fires(20, &at(ANSUULS_TORMENT_SET_ID, 2)));
        assert!(fires(20, &at(ANSUULS_TORMENT_SET_ID, 3)));
        assert!(fires(20, &at(PERFECTED_ANSUULS_TORMENT_SET_ID, 3)));
        assert!(!fires(10, &at(MOONDANCER_SET_ID, 4)));
    }

    fn passive(id: u32, source: EffectSource, stacking: Stacking, condition: Condition) -> EffectDef {
        EffectDef { source, stacking, condition, ..set_effect(id, 0, 0) }
    }

    /// Roadmap 2.4: a passive is on only when the build unlocked it (lists its `ability_id`); a race / class passive
    /// also has to be the build's own. A per-piece or per-slotted-ability effect gets one stack per piece or ability
    /// on the active bar.
    #[test]
    fn a_counted_passive_gets_one_stack_per_piece_or_slotted_ability() {
        let heavy = Condition::ArmourPieces { weight: ArmourWeight::Heavy, at_least: 1 };
        let shadow = Condition::SkillLineSlotted { skill_line: SkillLine::Shadow, at_least: 1 };
        let class = |name: &str, ability_id| EffectSource::Class { name: name.into(), ability_id };
        let race = |name: &str, ability_id| EffectSource::Race { name: name.into(), ability_id };
        let defs = [
            passive(1, EffectSource::Passive { ability_id: 1 }, Stacking::Additive, heavy.clone()),
            passive(2, class("Nightblade", 2), Stacking::Additive, shadow.clone()),
            // a count inside an All is found; any other condition is left to the aggregation
            passive(3, class("Nightblade", 3), Stacking::Additive, Condition::All(vec![Condition::OnBar(esosim_types::Bar::Front), shadow])),
            passive(4, EffectSource::Passive { ability_id: 4 }, Stacking::Unique, heavy),
            passive(5, race("Orc", 5), Stacking::Unique, Condition::Always),
            // another race / class, and an effect that is not a passive at all
            passive(6, race("Breton", 6), Stacking::Unique, Condition::Always),
            passive(7, class("Templar", 7), Stacking::Unique, Condition::Always),
            passive(8, EffectSource::Food, Stacking::Unique, Condition::Always),
            // a world / guild passive: two effects of one ability, on only when the build lists the ability
            passive(9, EffectSource::Passive { ability_id: 90 }, Stacking::Unique, Condition::Always),
            passive(10, EffectSource::Passive { ability_id: 90 }, Stacking::Unique, Condition::Always),
            passive(11, EffectSource::Passive { ability_id: 91 }, Stacking::Unique, Condition::Always),
            // a racial passive of two effects: one ability id, so one entry turns both on
            passive(12, race("Orc", 5), Stacking::Unique, Condition::Always),
        ];
        let ids = |race: &str, class: &str, unlocked: &[u32]| {
            passive_effect_rules(&defs, race, class, unlocked).iter().map(|r| r.effect_id).collect::<Vec<_>>()
        };
        assert!(ids("Orc", "Nightblade", &[]).is_empty(), "nothing is on until it is unlocked");
        assert_eq!(ids("Orc", "Nightblade", &[1, 2, 3, 4, 5]), [1, 2, 3, 4, 5, 12]);
        assert_eq!(ids("Orc", "Nightblade", &[90]), [9, 10]);
        assert_eq!(ids("Orc", "Nightblade", &[91, 90]), [9, 10, 11]);
        assert_eq!(ids("Orc", "Nightblade", &[6, 7, 8]), Vec::<u32>::new(), "another race's or class's passive, and a food");
        assert_eq!(ids("Breton", "Templar", &[1, 4, 5, 6, 7]), [1, 4, 6, 7]);
        let rules = passive_effect_rules(&defs, "Orc", "Nightblade", &[1, 2, 3, 4, 5]);

        let mut player = Player::new();
        let heavy_piece = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, None, None);
        for slot in [GearSlot::Chest, GearSlot::Legs, GearSlot::Hands] {
            player.set_gear_piece(slot, heavy_piece.clone());
        }
        player.set_skills(false, vec![36590, 36594]);
        player.ability_skill_lines.insert(36590, SkillLine::Shadow);
        player.ability_skill_lines.insert(36594, SkillLine::Shadow);
        let stacks = |id: u32| rules.iter().find(|r| r.effect_id == id).and_then(|r| r.evaluate(&player));
        assert_eq!((stacks(1), stacks(2), stacks(3)), (Some(3), Some(2), Some(2)));
        assert_eq!((stacks(4), stacks(5)), (Some(1), Some(1)), "not counted: one stack, the condition decides");
        // nothing worn, nothing slotted: the counted rules do not fire
        let empty = Player::new();
        assert!(rules[0].evaluate(&empty).is_none() && rules[1].evaluate(&empty).is_none());
    }

    #[test]
    fn bad_lines_are_errors_naming_the_line() {
        assert!(set_grant_rules_from("{nope}").unwrap_err().starts_with("line 1:"));
        assert!(set_grant_rules_from("").unwrap().is_empty());
    }

    #[test]
    fn every_set_has_a_class_and_the_one_unresolved_block_is_known() {
        use esosim_types::BehaviourClass;
        let all: Vec<SetBehaviour> =
            esosim_data::SET_BEHAVIOUR_JSONL.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
        assert!(all.len() > 600);
        let unclassified: Vec<u32> =
            all.iter().filter(|b| b.class == BehaviourClass::Unclassified).map(|b| b.id).collect();
        assert_eq!(unclassified.len(), 1, "{unclassified:?}");
    }
}
