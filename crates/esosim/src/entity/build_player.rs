//! `Build::to_player` (roadmap 1.3): one call from what a person chose to what the engine derived.
//!
//! The sources are gathered in this order, and each one ends up in `Player::sources`, so
//! `Player::explain(channel)` can say where a number came from:
//!
//! 1. gear: the pieces, their traits, their enchants (`Player::gear_parts`)
//! 2. sets: the stat rows of the worn sets (`Player::set_bonus_parts`)
//! 3. grants: buffs the gear gives (Minor Slayer at 3 pieces ...) and the set bonuses that are
//!    effects of their own (`EffectSource::Set`, written in `overrides.toml`), decided in `recompute`
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
use crate::engine::grant_rule::{all_grant_rules, set_effect_rules};
use crate::engine::registry::EffectRegistry;
use crate::entity::build::Build;
use crate::entity::player::Player;

impl Build {
    /// Everything the engine derives from this build: the aggregated channels (`Player::channels`)
    /// and where each came from (`Player::explain`).
    pub fn to_player(&self, bundle: &Bundle) -> Player {
        let mut player = Player::new();
        player.class = Some(self.class);
        player.food = self.food;
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

        // set bonuses written as effects (class 3, roadmap 2.2) are switched on by the set they name
        let set_effects = set_effect_rules(bundle.effects.values());
        player.recompute_with(&EffectRegistry::from_bundle(bundle), all_grant_rules().into_iter().chain(&set_effects));
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
            buff_code: None,
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

    /// A build wearing `pieces` pieces of `set` (head, chest, legs, hands, feet in that order) and
    /// nothing else that gives stats, so the only set effects are the ones under test.
    fn wearing(set: u16, pieces: usize, main_hand: Option<ItemType>) -> Player {
        wearing_build(set, pieces, main_hand).to_player(Bundle::embedded())
    }

    fn wearing_build(set: u16, pieces: usize, main_hand: Option<ItemType>) -> Build {
        let mut b = build();
        b.mundus = MundusLoadout::default();
        b.champion = CpLoadout::default();
        let piece = GearPiece::new(ItemType::Heavy, 66, None, Legendary, Some(set), None);
        for slot in [GearSlot::Head, GearSlot::Chest, GearSlot::Legs, GearSlot::Hands, GearSlot::Feet].into_iter().take(pieces) {
            b.gear.set_gear_piece(slot, piece.clone());
        }
        if let Some(weapon) = main_hand {
            b.gear.set_gear_piece(GearSlot::MainHand, GearPiece::new(weapon, 66, None, Legendary, None, None));
        }
        b
    }

    fn from_set(p: &Player, channel: Channel, set: u16, pieces_required: u8) -> f64 {
        p.explain(channel)
            .into_iter()
            .filter(|(s, _)| *s == EffectSource::Set { set_id: set, pieces_required })
            .map(|(_, v)| v)
            .sum()
    }

    /// Roadmap 2.2, class 3: Order's Wrath's 5-piece ability is an override (`EffectSource::Set`), and
    /// the set's piece count is all that switches it on. Critical damage and healing are both +8 points.
    #[test]
    fn a_class_3_set_bonus_applies_at_its_piece_count_and_says_where_it_came_from() {
        use crate::data::tables::set_ids::ORDERS_WRATH_SET_ID as SET;
        let four = wearing(SET, 4, None);
        assert_eq!(from_set(&four, Channel::CriticalDamage, SET, 5), 0.0);
        let five = wearing(SET, 5, None);
        assert_eq!(from_set(&five, Channel::CriticalDamage, SET, 5), 8.0);
        assert_eq!(from_set(&five, Channel::CriticalHealing, SET, 5), 8.0);
        assert_eq!(five.channels.get(Channel::CriticalDamage), 8);
    }

    /// A class 3 effect keeps its own condition on top of the piece count: Robes of Destruction Mastery
    /// gives its Max Magicka only with a Destruction Staff in hand; Permafrost needs a damage shield, which
    /// a build cannot answer, so it is left out.
    #[test]
    fn a_class_3_set_bonus_keeps_its_own_condition() {
        use crate::data::tables::set_ids::{PERMAFROST_SET_ID, ROBES_OF_DESTRUCTION_MASTERY_SET_ID as ROBES};
        let magicka = Channel::Resource(crate::engine::effect::ResourceKind::Magicka, crate::engine::effect::AggKind::Additive);
        assert_eq!(from_set(&wearing(ROBES, 5, None), magicka, ROBES, 5), 0.0);
        assert_eq!(from_set(&wearing(ROBES, 5, Some(ItemType::FireStaff)), magicka, ROBES, 5), 3303.0);
        assert_eq!(from_set(&wearing(ROBES, 4, Some(ItemType::FireStaff)), magicka, ROBES, 5), 0.0);

        let recovery = Channel::Recovery(crate::engine::effect::ResourceKind::Health, crate::engine::effect::AggKind::Additive);
        assert_eq!(from_set(&wearing(PERMAFROST_SET_ID, 5, None), recovery, PERMAFROST_SET_ID, 5), 0.0);
    }

    /// A bonus on a channel the sheet has no field for yet (healing received) is still in the channels
    /// and in `explain`, so it is not lost. (Perfected pieces reaching the regular set's effect is
    /// covered in `grant_rule.rs`.)
    #[test]
    fn a_class_3_healing_bonus_is_in_the_channels() {
        use crate::data::tables::set_ids::SANCTUARY_SET_ID as SET;
        let p = wearing(SET, 5, None);
        // the 5-piece ability is its own source; Sanctuary's 3-piece stat row also gives healing received
        assert_eq!(from_set(&p, Channel::HealingTaken, SET, 5), 10.0);
        assert!(p.channels.get(Channel::HealingTaken) > 10);
    }

    /// Roadmap 2.2 answers 1 and 5: percentages are fractions in the multiplicative bucket.
    #[test]
    fn percent_cost_reduction_and_percent_recovery_are_multiplicative() {
        use crate::data::tables::set_ids::{ARMOR_OF_THE_SEDUCER_SET_ID, WILLOWS_PATH_SET_ID};
        use crate::engine::effect::ResourceKind::{Health, Magicka, Stamina};
        use eso_skill_data::enums::mechanic::Mechanic;
        let seducer = wearing(ARMOR_OF_THE_SEDUCER_SET_ID, 5, None);
        assert_eq!(seducer.channels.get_multiplicative_bps(Channel::CostReductionPct(Mechanic::Magicka)), 1000);
        assert_eq!(seducer.channels.get(Channel::CostReduction(Mechanic::Magicka)), 0, "the flat channel is untouched");
        assert_eq!(wearing(ARMOR_OF_THE_SEDUCER_SET_ID, 4, None).channels.get_multiplicative_bps(Channel::CostReductionPct(Mechanic::Magicka)), 0);
        let willow = wearing(WILLOWS_PATH_SET_ID, 5, None);
        for r in [Health, Magicka, Stamina] {
            let ch = Channel::Recovery(r, crate::engine::effect::AggKind::Multiplicative);
            assert_eq!(willow.channels.get_multiplicative_bps(ch), 1800, "{r:?}");
        }
    }

    /// Roadmap 2.2 answer 3: `Condition::FoodActive` follows `Build::food`.
    #[test]
    fn a_food_condition_set_bonus_needs_a_food_in_the_build() {
        use crate::data::tables::set_ids::BACK_ALLEY_GOURMAND_SET_ID as SET;
        let without = wearing(SET, 5, None);
        assert_eq!(from_set(&without, Channel::CriticalDamage, SET, 5), 0.0);
        let mut b = wearing_build(SET, 5, None);
        b.food = Some(61259); // any food: the condition only asks that the build names one
        let with = b.to_player(Bundle::embedded());
        assert_eq!(from_set(&with, Channel::CriticalDamage, SET, 5), 13.0);
        assert_eq!(from_set(&with, Channel::CriticalHealing, SET, 5), 13.0);
    }

    /// A fight in progress, for the effects whose condition the calculator cannot decide.
    struct Fight {
        health: u8,
        combat: bool,
    }
    impl esosim_types::ConditionContext for Fight {
        fn self_health_pct(&self) -> Option<u8> { Some(self.health) }
        fn in_combat(&self) -> Option<bool> { Some(self.combat) }
        fn self_resource_pct(&self, r: esosim_types::ResourceKind) -> Option<u8> {
            Some(if r == esosim_types::ResourceKind::Health { self.health } else { 30 })
        }
    }

    fn in_fight(ids: &[u32], health: u8, combat: bool) -> crate::engine::aggregator::ChannelSet {
        let registry = EffectRegistry::from_bundle(Bundle::embedded());
        let mut active = crate::engine::active_effects::ActiveEffects::new();
        for id in ids {
            active.set(*id, 1);
        }
        active.aggregate_with(&registry, &Fight { health, combat })
    }

    /// Roadmap 2.2 answer 6: three tiers that add up to 1x, 2x and 4x of the base.
    #[test]
    fn true_sworn_fury_doubles_under_75_and_quadruples_under_50_in_combat() {
        let ids = [150703, 150704, 150705, 150706, 150707, 150708];
        let at = |health, combat| {
            let c = in_fight(&ids, health, combat);
            (c.get(Channel::CriticalChance), c.get(Channel::CriticalDamage))
        };
        assert_eq!(at(100, true), (824, 5));
        assert_eq!(at(75, true), (824, 5), "75 % is not under 75 %");
        assert_eq!(at(60, true), (1647, 10));
        assert_eq!(at(40, true), (3294, 19));
        assert_eq!(at(40, false), (824, 5), "the doubling is for combat only");
        let pen = |health| in_fight(&[134271, 134273, 134274], health, true).get(Channel::Penetration(crate::engine::effect::ResistableDamageType::All));
        assert_eq!((pen(90), pen(70), pen(30)), (1442, 2884, 5768));
    }

    /// Roadmap 2.2 answer 4: each number of a two-number tooltip is the value of its own record.
    #[test]
    fn two_number_tooltips_use_one_record_per_number() {
        use crate::engine::effect::{AggKind::Additive, ResistableDamageType::All as AllTypes, ResourceKind::Health};
        let armour = Channel::Armour(AllTypes);
        let low = in_fight(&[32801, 137154], 50, true);
        assert_eq!((low.get(Channel::Recovery(Health, Additive)), low.get(armour)), (931, 7442));
        let high = in_fight(&[32801, 137154], 80, true);
        assert_eq!((high.get(Channel::Recovery(Health, Additive)), high.get(armour)), (0, 0));
        // Ancient Dragonguard: above 50 % damage (record 129532), 50 % or less resistance (record 129508)
        let power = Channel::Power(crate::engine::effect::DamageDoneContext::All, Additive);
        let d = |health| { let c = in_fight(&[129532, 129508], health, false); (c.get(power), c.get(armour)) };
        assert_eq!((d(51), d(50)), ((349, 0), (0, 4024)));
    }

    #[test]
    fn per_ability_damage_has_its_own_context_and_stays_off_the_sheet_channel() {
        use crate::engine::effect::{AggKind::Additive, DamageDoneContext, ResistableDamageType::Flame};
        use crate::data::tables::set_ids::SILKS_OF_THE_SUN_SET_ID as SET;
        let p = wearing(SET, 5, None);
        assert_eq!(p.channels.get(Channel::Power(DamageDoneContext::Type(Flame), Additive)), 466);
        assert_eq!(from_set(&p, Channel::Power(DamageDoneContext::All, Additive), SET, 5), 0.0);
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
