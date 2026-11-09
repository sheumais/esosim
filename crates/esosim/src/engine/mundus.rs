//! Mundus stones in the engine (roadmap 2.3).
//!
//! A stone is an effect like any other: `effects.jsonl` (the compiler's `overrides.toml`) holds one
//! `EffectDef` per stone, keyed by the game's ability id ("Boon: The Thief" = 13975), `Unique`, with
//! source `EffectSource::Mundus`. Picking a stone is switching its ability on
//! ([`ActiveEffects::apply_mundus_stones`]). What makes a stone special is its boost: every effect
//! of a stone is worth `value * (1 + MundusBoost)`, where the boost is the sum of the Divines pieces
//! worn (`ActiveEffects::aggregate_with_boost`, called by `Player::recompute`).
//!
//! The calculator never refuses a loadout (what-if builds are the point), so the rules of the stones
//! (one at a time, two different ones with five pieces of Twice-Born Star) are reported by
//! [`Bundle::mundus_issues`] / [`Player::mundus_issues`], not enforced.

use std::sync::LazyLock;

use esosim_types::{
    BASE_MUNDUS_STONES, EffectDef, EffectSource, MUNDUS_STONES_WITH_EXTRA_SLOT, MundusIssue, MundusLoadout,
    SetBehaviour,
};

use super::active_effects::ActiveEffects;
use crate::data::bundle::Bundle;
use crate::data::tables::set_ids::TWICE_BORN_STAR_SET_ID;
use crate::entity::player::Player;

impl Bundle {
    /// Every mundus stone of the data, by ability id.
    pub fn mundus_stones(&self) -> Vec<&EffectDef> {
        let mut stones: Vec<&EffectDef> =
            self.effects.values().filter(|d| matches!(d.source, EffectSource::Mundus { .. })).collect();
        stones.sort_by_key(|d| d.id);
        stones
    }

    pub fn is_mundus_stone(&self, id: u32) -> bool {
        self.effect(id).is_some_and(|d| matches!(d.source, EffectSource::Mundus { .. }))
    }

    /// What the game would not let `loadout` do with room for `max_stones` (see `MundusLoadout::issues`).
    pub fn mundus_issues(&self, loadout: &MundusLoadout, max_stones: usize) -> Vec<MundusIssue> {
        loadout.issues(|id| self.is_mundus_stone(id), max_stones)
    }
}

impl ActiveEffects {
    /// Switch on exactly the stones of `loadout`; every other stone goes off, so calling this again
    /// with another loadout replaces the earlier one. An id that is not a stone is skipped (it is
    /// reported by [`Bundle::mundus_issues`]). Stones past the room a build has still apply.
    pub fn apply_mundus_stones(&mut self, bundle: &Bundle, loadout: &MundusLoadout) {
        for stone in bundle.mundus_stones() {
            self.set(stone.id, 0);
        }
        for id in loadout.distinct() {
            if bundle.is_mundus_stone(id) {
                self.set(id, 1);
            }
        }
    }

    /// The stones that are on, by ability id.
    pub fn mundus_loadout(&self, bundle: &Bundle) -> MundusLoadout {
        let mut stones: Vec<u32> = self.ids().filter(|id| bundle.is_mundus_stone(*id)).collect();
        stones.sort_unstable();
        MundusLoadout { stones }
    }
}

/// Pieces of Twice-Born Star that allow the second stone. Read from `set_behaviour.jsonl` (the
/// piece count of the set's ability block, which the compiler infers: 5 in the game).
pub static EXTRA_STONE_PIECES: LazyLock<Option<u8>> = LazyLock::new(|| {
    extra_stone_pieces_from(esosim_data::SET_BEHAVIOUR_JSONL)
        .unwrap_or_else(|e| panic!("esosim-data set_behaviour.jsonl: {e}"))
});

/// The piece count of the first ability block of Twice-Born Star that has one.
pub fn extra_stone_pieces_from(jsonl: &str) -> Result<Option<u8>, String> {
    for (i, line) in jsonl.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let set: SetBehaviour = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        if set.id == TWICE_BORN_STAR_SET_ID as u32 {
            return Ok(set.blocks.iter().find_map(|b| b.pieces));
        }
    }
    Ok(None)
}

impl Player {
    /// How many stones the equipped gear has room for: one, or two with the pieces of Twice-Born Star.
    /// (The game also drops the second stone when a bar change leaves fewer pieces; this looks at the
    /// active bar only.)
    pub fn mundus_slots(&self) -> usize {
        match *EXTRA_STONE_PIECES {
            Some(pieces) if self.get_number_of_equipped_set(TWICE_BORN_STAR_SET_ID) >= pieces => {
                MUNDUS_STONES_WITH_EXTRA_SLOT
            }
            _ => BASE_MUNDUS_STONES,
        }
    }

    /// Pick the stones (replacing the earlier pick). Call `recompute` afterwards: it applies the first
    /// `mundus_slots()` of them, so a second stone that the active bar has no room for is switched off
    /// and comes back, as the same stone, when the room does.
    pub fn set_mundus(&mut self, bundle: &Bundle, loadout: &MundusLoadout) {
        self.active_effects.apply_mundus_stones(bundle, loadout);
        self.mundus_pick = loadout.clone();
    }

    /// The stones picked, in pick order, whether or not the gear has room for all of them right now.
    pub fn mundus(&self, _bundle: &Bundle) -> MundusLoadout {
        self.mundus_pick.clone()
    }

    /// What is wrong with the stones picked, given the gear: empty for a legal build.
    pub fn mundus_issues(&self, bundle: &Bundle) -> Vec<MundusIssue> {
        bundle.mundus_issues(&self.mundus(bundle), self.mundus_slots())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::{GearSlot, GearTrait, ItemQuality, ItemType};
    use crate::engine::aggregator::ChannelSet;
    use crate::engine::effect::{AggKind::Additive, Channel, DamageDoneContext, ResistableDamageType as Dmg, ResourceKind};
    use crate::engine::grant_rule::GRANT_RULES;
    use crate::engine::registry::EffectRegistry;
    use crate::entity::gear::GearPiece;

    const WARRIOR: u32 = 13940;
    const MAGE: u32 = 13943;
    const SERPENT: u32 = 13974;
    const THIEF: u32 = 13975;
    const LADY: u32 = 13976;
    const STEED: u32 = 13977;
    const LORD: u32 = 13978;
    const APPRENTICE: u32 = 13979;
    const RITUAL: u32 = 13980;
    const LOVER: u32 = 13981;
    const ATRONACH: u32 = 13982;
    const SHADOW: u32 = 13984;
    const TOWER: u32 = 13985;

    /// Divines is an armour trait: seven pieces and the shield make eight.
    const DIVINES_SLOTS: [GearSlot; 8] = [
        GearSlot::Head,
        GearSlot::Shoulders,
        GearSlot::Chest,
        GearSlot::Hands,
        GearSlot::Waist,
        GearSlot::Legs,
        GearSlot::Feet,
        GearSlot::OffHand,
    ];

    /// A player wearing `divines` pieces of the given quality (Divines armour, no set).
    fn player_in_divines(divines: usize, quality: ItemQuality) -> Player {
        let mut p = Player::new();
        for slot in DIVINES_SLOTS.into_iter().take(divines) {
            let item = if slot == GearSlot::OffHand { ItemType::Shield } else { ItemType::Heavy };
            p.set_gear_piece(slot, GearPiece::new(item, 66, Some(GearTrait::ArmorDivines), quality, None, None));
        }
        p
    }

    fn sheet_of(p: &mut Player, stones: &[u32]) -> ChannelSet {
        let b = Bundle::embedded();
        p.set_mundus(b, &MundusLoadout::new(stones.iter().copied()));
        p.recompute(&EffectRegistry::embedded(), GRANT_RULES);
        p.channels.clone()
    }

    #[test]
    fn the_thirteen_stones_are_in_the_data_with_the_games_names() {
        let b = Bundle::embedded();
        let stones = b.mundus_stones();
        assert_eq!(stones.len(), 13);
        let ids: Vec<u32> = stones.iter().map(|d| d.id).collect();
        assert_eq!(
            ids,
            [WARRIOR, MAGE, SERPENT, THIEF, LADY, STEED, LORD, APPRENTICE, RITUAL, LOVER, ATRONACH, SHADOW, TOWER]
        );
        for d in stones {
            assert_eq!(d.source, EffectSource::Mundus { ability_id: d.id }, "{}", d.name);
            assert_eq!(d.stacking, esosim_types::Stacking::Unique, "{}", d.name);
        }
        // the Apprentice's twin record (269724) is not a second stone
        assert!(!b.is_mundus_stone(269724));
    }

    struct Added {
        with: ChannelSet,
        without: ChannelSet,
    }
    impl Added {
        fn get(&self, c: Channel) -> i64 {
            self.with.get(c) - self.without.get(c)
        }
        fn get_multiplicative_bps(&self, c: Channel) -> i64 {
            self.with.get_multiplicative_bps(c)
        }
    }

    #[test]
    fn a_stone_without_divines_is_its_base_value() {
        let sheet = |stone| sheet_of(&mut Player::new(), &[stone]);
        assert_eq!(sheet(THIEF).get(Channel::CriticalChance), 1212);
        assert_eq!(sheet(LORD).get(Channel::Resource(ResourceKind::Health, Additive)), 2225);
        assert_eq!(sheet(MAGE).get(Channel::Resource(ResourceKind::Magicka, Additive)), 2023);
        assert_eq!(sheet(TOWER).get(Channel::Resource(ResourceKind::Stamina, Additive)), 2023);
        assert_eq!(sheet(LADY).get(Channel::Armour(Dmg::All)), 2744);
        assert_eq!(sheet(LOVER).get(Channel::Penetration(Dmg::All)), 2744);
        assert_eq!(sheet(WARRIOR).get(Channel::Power(DamageDoneContext::All, Additive)), 238);
        assert_eq!(sheet(APPRENTICE).get(Channel::KillExperience), 8);
        assert_eq!(sheet(ATRONACH).get(Channel::Recovery(ResourceKind::Magicka, Additive)), 310);
        assert_eq!(sheet(SERPENT).get(Channel::Recovery(ResourceKind::Stamina, Additive)), 310);
        assert_eq!(sheet(SHADOW).get(Channel::CriticalDamage), 11);
        assert_eq!(sheet(RITUAL).get(Channel::HealingDone), 8);
        let steed = sheet(STEED);
        assert_eq!((steed.get(Channel::MovementSpeed), steed.get(Channel::Recovery(ResourceKind::Health, Additive))), (10, 264));
        // and nothing else: one stone, one thing
        assert_eq!(sheet(THIEF).get(Channel::CriticalDamage), 0);
    }

    /// The numbers read off the game's character sheet with eight Legendary Divines pieces (seven armour
    /// pieces and a shield): +72.7 % (the second piece counts as 9.0 %), values truncated.
    #[test]
    fn eight_legendary_divines_give_the_games_figures() {
        // what the stone adds on top of the same gear without it (the armour pieces carry resistance of their own)
        let full = |stone| {
            let with = sheet_of(&mut player_in_divines(8, ItemQuality::Legendary), &[stone]);
            let without = sheet_of(&mut player_in_divines(8, ItemQuality::Legendary), &[]);
            Added { with, without }
        };
        assert_eq!(full(THIEF).get(Channel::CriticalChance), 2093);
        assert_eq!(full(LORD).get(Channel::Resource(ResourceKind::Health, Additive)), 3842);
        assert_eq!(full(MAGE).get(Channel::Resource(ResourceKind::Magicka, Additive)), 3493);
        assert_eq!(full(TOWER).get(Channel::Resource(ResourceKind::Stamina, Additive)), 3493);
        assert_eq!(full(LADY).get(Channel::Armour(Dmg::All)), 4738);
        assert_eq!(full(LOVER).get(Channel::Penetration(Dmg::All)), 4738);
        assert_eq!(full(WARRIOR).get(Channel::Power(DamageDoneContext::All, Additive)), 411);
        assert_eq!(full(ATRONACH).get(Channel::Recovery(ResourceKind::Magicka, Additive)), 535);
        assert_eq!(full(SERPENT).get(Channel::Recovery(ResourceKind::Stamina, Additive)), 535);
        assert_eq!(full(SHADOW).get(Channel::CriticalDamage), 18);
        assert_eq!(full(RITUAL).get(Channel::HealingDone), 13);
        assert_eq!(full(APPRENTICE).get(Channel::KillExperience), 13);
        let steed = full(STEED);
        assert_eq!((steed.get(Channel::MovementSpeed), steed.get(Channel::Recovery(ResourceKind::Health, Additive))), (17, 455));
        assert_eq!(full(THIEF).get_multiplicative_bps(Channel::MundusBoost), 7270);
    }

    #[test]
    fn the_boost_adds_per_piece_and_follows_the_quality_of_each() {
        let thief = |p: &mut Player| sheet_of(p, &[THIEF]).get(Channel::CriticalChance);
        assert_eq!(thief(&mut player_in_divines(0, ItemQuality::Legendary)), 1212);
        assert_eq!(thief(&mut player_in_divines(1, ItemQuality::Legendary)), 1322); // 9.1 %
        assert_eq!(thief(&mut player_in_divines(7, ItemQuality::Epic)), 1897); // 6 x 8.1 % + 8.0 %
        assert_eq!(thief(&mut player_in_divines(7, ItemQuality::Normal)), 1643); // 6 x 5.1 % + 5.0 %
    }

    #[test]
    fn divines_without_a_stone_only_show_the_boost() {
        let c = sheet_of(&mut player_in_divines(7, ItemQuality::Legendary), &[]);
        assert_eq!(c.get_multiplicative_bps(Channel::MundusBoost), 6360);
        assert_eq!(c.get(Channel::CriticalChance), 0);
    }

    #[test]
    fn a_boost_from_an_effect_adds_to_the_boost_from_gear() {
        use esosim_types::{ChannelValue, Condition, NoContext, Stacking};
        let boost = EffectDef {
            id: 1,
            name: "a ten percent boost".into(),
            source: EffectSource::Buff { ability_id: 1 },
            channels: vec![ChannelValue { channel: Channel::MundusBoost, value: 0.1, value_per_stack: 0.0 }],
            condition: Condition::Always,
            stacking: Stacking::Unique,
            duration_ms: None,
            cooldown_ms: None,
            note: None,
        };
        let mut defs: Vec<EffectDef> = Bundle::embedded().effects.values().cloned().collect();
        defs.push(boost);
        let registry = EffectRegistry::from_defs(defs);
        let mut active = ActiveEffects::new();
        active.set(THIEF, 1);
        let thief = |active: &ActiveEffects, gear_bps| {
            active.aggregate_with_boost(&registry, &NoContext, gear_bps).get(Channel::CriticalChance)
        };
        assert_eq!(thief(&active, 0), 1212);
        active.set(1, 1);
        assert_eq!(thief(&active, 0), 1333); // 1212 * 1.1 = 1333.2
        assert_eq!(thief(&active, 910), 1443); // 1212 * 1.191 = 1443.5: 10 % + one Legendary Divines piece
        assert_eq!(active.aggregate_with_boost(&registry, &NoContext, 910).get_multiplicative_bps(Channel::MundusBoost), 1000);
    }

    #[test]
    fn applying_a_loadout_replaces_the_one_before() {
        let b = Bundle::embedded();
        let mut active = ActiveEffects::new();
        active.apply_mundus_stones(b, &MundusLoadout::new([THIEF]));
        assert_eq!(active.mundus_loadout(b), MundusLoadout::new([THIEF]));
        active.apply_mundus_stones(b, &MundusLoadout::new([SHADOW, 7]));
        assert_eq!(active.mundus_loadout(b), MundusLoadout::new([SHADOW]), "the Thief went off, 7 is no stone");
        active.apply_mundus_stones(b, &MundusLoadout::default());
        assert_eq!(active.ids().count(), 0);
    }

    #[test]
    fn two_stones_need_five_pieces_of_twice_born_star() {
        let b = Bundle::embedded();
        assert_eq!(*EXTRA_STONE_PIECES, Some(5));
        let mut p = Player::new();
        p.set_mundus(b, &MundusLoadout::new([THIEF, SHADOW]));
        assert_eq!(p.mundus_slots(), 1);
        assert_eq!(p.mundus_issues(b), [MundusIssue::TooMany { stones: 2, max: 1 }]);

        for slot in DIVINES_SLOTS.into_iter().take(4) {
            p.set_gear_piece(slot, GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(TWICE_BORN_STAR_SET_ID), None));
        }
        assert_eq!((p.mundus_slots(), p.mundus_issues(b).len()), (1, 1), "four pieces are not enough");
        p.set_gear_piece(
            GearSlot::Feet,
            GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(TWICE_BORN_STAR_SET_ID), None),
        );
        assert_eq!(p.mundus_slots(), 2);
        assert_eq!(p.mundus_issues(b), []);

        // both stones apply, and the same stone twice is one stone and a report
        p.recompute(&EffectRegistry::embedded(), GRANT_RULES);
        assert_eq!((p.channels.get(Channel::CriticalChance), p.channels.get(Channel::CriticalDamage)), (1212, 11));
        p.set_mundus(b, &MundusLoadout::new([THIEF, THIEF]));
        assert_eq!(p.mundus_issues(b), [MundusIssue::Duplicate { id: THIEF }]);
        assert_eq!(b.mundus_issues(&MundusLoadout::new([THIEF, THIEF]), 2), [MundusIssue::Duplicate { id: THIEF }]);
    }

    /// Twice-Born Star on four armour pieces and one weapon: the front bar has five pieces, the back bar
    /// four. The second stone is off on the back bar and the same stone is back on the front bar again.
    #[test]
    fn a_bar_swap_drops_the_second_stone_and_swapping_back_brings_the_same_one_back() {
        use crate::entity::player::ActiveBar;
        let b = Bundle::embedded();
        let tbs = |item| GearPiece::new(item, 66, None, ItemQuality::Legendary, Some(TWICE_BORN_STAR_SET_ID), None);
        let other = |item| GearPiece::new(item, 66, None, ItemQuality::Legendary, None, None);
        let mut p = Player::new();
        for slot in DIVINES_SLOTS.into_iter().take(4) {
            p.set_gear_piece(slot, tbs(ItemType::Heavy));
        }
        p.set_gear_piece(GearSlot::MainHand, tbs(ItemType::Sword));
        p.set_gear_piece(GearSlot::MainHandBackup, other(ItemType::Sword));
        p.set_mundus(b, &MundusLoadout::new([THIEF, SHADOW]));
        let crit = |p: &mut Player| {
            p.recompute(&EffectRegistry::embedded(), GRANT_RULES);
            (p.channels.get(Channel::CriticalChance), p.channels.get(Channel::CriticalDamage))
        };
        assert_eq!(crit(&mut p), (1212, 11), "front bar: five pieces, both stones");
        p.active_bar = ActiveBar::Backup;
        assert_eq!(p.mundus_slots(), 1);
        assert_eq!(crit(&mut p), (1212, 0), "back bar: the second stone (the Shadow) is off, the first stays");
        assert_eq!(p.mundus(b), MundusLoadout::new([THIEF, SHADOW]), "but it is still picked");
        p.active_bar = ActiveBar::Front;
        assert_eq!(crit(&mut p), (1212, 11), "and it is back, without visiting the stone again");
        // the order is the pick order, not the id order: Shadow first means the Shadow is the one that stays
        p.set_mundus(b, &MundusLoadout::new([SHADOW, THIEF]));
        p.active_bar = ActiveBar::Backup;
        assert_eq!(crit(&mut p), (0, 11));
    }

    #[test]
    fn the_piece_count_comes_from_the_behaviour_file() {
        assert_eq!(extra_stone_pieces_from("").unwrap(), None);
        let line = format!(
            r#"{{"id":{TWICE_BORN_STAR_SET_ID},"class":"static_conditional","blocks":[{{"ability_id":1,"class":"static_conditional","pieces":4}}]}}"#
        );
        assert_eq!(extra_stone_pieces_from(&line).unwrap(), Some(4));
        assert!(extra_stone_pieces_from("{nope}").unwrap_err().starts_with("line 1:"));
    }
}
