use eso_skill_data::enums::skill_line::SkillLine;

use std::sync::LazyLock;

use esosim_types::SetBehaviour;

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
pub fn all_grant_rules() -> Vec<&'static GrantRule> {
    GRANT_RULES.iter().chain(SET_GRANT_RULES.iter()).collect()
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
