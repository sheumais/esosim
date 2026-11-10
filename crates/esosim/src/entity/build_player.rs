//! `Build::to_player` (roadmap 1.3): one call from what a person chose to what the engine derived.
//!
//! The sources are gathered in this order, and each one ends up in `Player::sources`, so
//! `Player::explain(channel)` can say where a number came from:
//!
//! 1. gear: the pieces, their traits, their enchants (`Player::gear_parts`)
//! 2. sets: the stat rows of the worn sets (`Player::set_bonus_parts`)
//! 3. grants: buffs the gear gives (Minor Slayer at 3 pieces ...), decided in `recompute`
//! 4. champion points
//! 5. mundus stones
//! 6. consumables (`Build::food`, `Build::potion`)
//! 7. buffs the group provides (`Build::external_buffs`)
//!
//! Steps 3 to 7 are effects: each is switched on in `Player::active_effects` and aggregated by
//! `recompute`, which decides their build-time conditions with the player as the context (1.2).
//! Not gathered yet, because there is no data for them: race and class passives and skill-line
//! passives (2.4), bar abilities that give stats (2.4), debuffs on the enemy (2.6, 2.9).
//!
//! The player is on the front bar; `Player::active_bar` switches it and `recompute` again.

use crate::data::bundle::Bundle;
use crate::engine::grant_rule::all_grant_rules;
use crate::engine::registry::EffectRegistry;
use crate::entity::build::Build;
use crate::entity::player::Player;

impl Build {
    /// Everything the engine derives from this build: the aggregated channels (`Player::channels`)
    /// and where each came from (`Player::explain`).
    pub fn to_player(&self, bundle: &Bundle) -> Player {
        let mut player = Player::new();
        player.class = Some(self.class);
        player.attributes = self.attributes;
        player.gear = self.gear.clone();

        // bars: which skill line each slotted ability is in (what skill-line conditions count)
        player.primary_abilities = self.front_bar.clone();
        player.backup_abilities = self.back_bar.clone();
        for id in self.front_bar.iter().chain(&self.back_bar) {
            if let Some(line) = bundle.skill_line_of(*id) {
                player.ability_skill_lines.insert(*id, line);
            }
        }

        player.active_effects.apply_champion_points(bundle, &self.champion);
        player.set_mundus(bundle, &self.mundus);
        // an id the bundle has no effect for is skipped when aggregating
        for id in self.food.iter().chain(&self.potion).chain(&self.external_buffs) {
            player.active_effects.set(*id, 1);
        }

        player.recompute_with(&EffectRegistry::from_bundle(bundle), all_grant_rules());
        player
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::{GearSlot, GearTrait, ItemQuality::Legendary, ItemType};
    use crate::data::tables::set_ids::ANSUULS_TORMENT_SET_ID;
    use crate::data::skill_data::major_minor::SLAYER_MINOR_ID;
    use crate::engine::effect::{Channel, ChannelValue, Condition, EffectDef, EffectSource, Stacking};
    use crate::entity::gear::GearPiece;
    use crate::stats::sheet::Attributes;
    use esosim_types::{CpLoadout, MundusLoadout, PlayerClass, Race};
    use std::collections::BTreeMap;

    const THIEF: u32 = 13975;

    fn build() -> Build {
        Build {
            race: Race::Dunmer,
            class: PlayerClass::Nightblade,
            level: 50,
            attributes: Attributes { health: 12, magicka: 0, stamina: 52 },
            mundus: MundusLoadout::new([THIEF]),
            // star 11: 20 points are 320 critical chance (see engine/champion.rs)
            champion: CpLoadout { points: BTreeMap::from([(11, 20)]), slotted: vec![] },
            gear: Default::default(),
            front_bar: vec![15540], // Toppling Charge
            back_bar: vec![],
            food: None,
            potion: None,
            external_buffs: vec![],
            enemy: Default::default(),
        }
    }

    fn with_gear(mut b: Build) -> Build {
        let precise = GearPiece::new(ItemType::Sword, 66, Some(GearTrait::WeaponPrecise), Legendary, None, None);
        b.gear.set_gear_piece(GearSlot::MainHand, precise);
        for slot in [GearSlot::Chest, GearSlot::Legs, GearSlot::Hands] {
            b.gear.set_gear_piece(slot, GearPiece::new(ItemType::Heavy, 66, None, Legendary, Some(ANSUULS_TORMENT_SET_ID), None));
        }
        b
    }

    #[test]
    fn a_build_becomes_a_player_with_its_class_attributes_and_skill_lines() {
        let bundle = Bundle::embedded();
        let b = build();
        let p = b.to_player(bundle);
        assert_eq!(p.class, Some(PlayerClass::Nightblade));
        assert_eq!(p.attributes, b.attributes);
        assert_eq!(p.primary_abilities, vec![15540]);
        assert_eq!(p.ability_skill_lines.get(&15540).copied(), bundle.skill_line_of(15540));
        assert!(p.ability_skill_lines.contains_key(&15540));
    }

    #[test]
    fn why_is_my_crit_chance_what_it_is() {
        let p = with_gear(build()).to_player(Bundle::embedded());
        let why = p.explain(Channel::CriticalChance);
        // 810 from a legendary Precise sword, 320 from the star, 1212 from the Thief, and 657 from
        // the stat rows of the three Ansuul's Torment pieces (`SET_CRITICAL_CHANCE_DEFAULT` in sets.rs)
        assert!(why.contains(&(EffectSource::GearTrait, 810.0)), "{why:?}");
        assert!(why.contains(&(EffectSource::ChampionPoint { cp_id: 11 }, 320.0)), "{why:?}");
        assert!(why.contains(&(EffectSource::Mundus { ability_id: THIEF }, 1212.0)), "{why:?}");
        let from_set: f64 = why
            .iter()
            .filter(|(s, _)| matches!(s, EffectSource::Set { set_id, .. } if *set_id == ANSUULS_TORMENT_SET_ID))
            .map(|(_, v)| v)
            .sum();
        assert_eq!(from_set, 657.0, "{why:?}");
        assert_eq!(p.channels.get(Channel::CriticalChance), 810 + 320 + 1212 + 657);
    }

    #[test]
    fn every_channel_is_the_sum_of_its_sources() {
        let p = with_gear(build()).to_player(Bundle::embedded());
        let mut merged = crate::engine::aggregator::ChannelSet::new();
        for (_, part) in &p.sources {
            merged.merge(part);
        }
        for c in [Channel::CriticalChance, Channel::Armour(crate::engine::effect::ResistableDamageType::All)] {
            assert_eq!(merged.get(c), p.channels.get(c));
        }
        // gear armour is its own source, apart from traits and enchants
        let armour = p.explain(Channel::Armour(crate::engine::effect::ResistableDamageType::All));
        assert!(armour.iter().any(|(s, _)| *s == EffectSource::Gear));
    }

    #[test]
    fn a_set_bonus_row_names_its_set_and_piece_count() {
        let p = with_gear(build()).to_player(Bundle::embedded());
        assert!(
            p.sources.iter().any(|(s, _)| matches!(s, EffectSource::Set { set_id, .. } if *set_id == ANSUULS_TORMENT_SET_ID)),
            "Ansuul's Torment gives a stat row at 3 pieces"
        );
    }

    #[test]
    fn three_ansuuls_pieces_fire_the_minor_slayer_grant_rule() {
        let bundle = Bundle::embedded();
        let p = with_gear(build()).to_player(bundle);
        // the rule fires whether or not the bundle models the buff yet (it is inert until it does)
        let rules_fire = crate::engine::grant_rule::all_grant_rules()
            .iter()
            .any(|r| r.effect_id == SLAYER_MINOR_ID && r.evaluate(&p).is_some());
        assert!(rules_fire);
    }

    fn effect(id: u32, source: EffectSource, condition: Condition, crit: f64) -> EffectDef {
        EffectDef {
            id,
            name: format!("test {id}"),
            source,
            channels: vec![ChannelValue { channel: Channel::CriticalChance, value: crit, value_per_stack: 0.0 }],
            condition,
            stacking: Stacking::Unique,
            duration_ms: None,
            cooldown_ms: None,
            note: None,
        }
    }

    #[test]
    fn consumables_and_group_buffs_apply_and_build_time_conditions_are_decided() {
        let jsonl: String = [
            effect(1, EffectSource::Food, Condition::Always, 100.0),
            effect(2, EffectSource::Potion, Condition::Always, 10.0),
            effect(3, EffectSource::Buff { ability_id: 3 }, Condition::Always, 1000.0),
            // only for Nightblades / only for Templars
            effect(4, EffectSource::Buff { ability_id: 4 }, Condition::ClassIs(PlayerClass::Nightblade), 10_000.0),
            effect(5, EffectSource::Buff { ability_id: 5 }, Condition::ClassIs(PlayerClass::Templar), 100_000.0),
            // needs combat state: stays out of the build's sheet
            effect(6, EffectSource::Buff { ability_id: 6 }, Condition::InCombat, 1_000_000.0),
        ]
        .iter()
        .map(|d| serde_json::to_string(d).unwrap() + "\n")
        .collect();
        let bundle = Bundle::from_jsonl(&jsonl).unwrap();
        let mut b = build();
        b.mundus = MundusLoadout::default();
        b.champion = CpLoadout::default();
        b.food = Some(1);
        b.potion = Some(2);
        // 3 is given twice (a Unique buff counts once)
        b.external_buffs = vec![3, 3, 4, 5, 6];
        let p = b.to_player(&bundle);
        assert_eq!(p.channels.get(Channel::CriticalChance), 100 + 10 + 1000 + 10_000);
        let why = p.explain(Channel::CriticalChance);
        assert!(why.contains(&(EffectSource::Food, 100.0)) && why.contains(&(EffectSource::Potion, 10.0)));
        assert_eq!(why.len(), 4, "{why:?}");
    }

    #[test]
    fn an_empty_build_is_an_empty_sheet() {
        let mut b = build();
        b.front_bar.clear();
        b.mundus = MundusLoadout::default();
        b.champion = CpLoadout::default();
        let p = b.to_player(Bundle::embedded());
        assert!(p.explain(Channel::CriticalChance).is_empty());
        assert_eq!(p.channels.get(Channel::CriticalChance), 0);
    }
}
