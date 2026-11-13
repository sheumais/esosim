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
//! 8. passives (roadmap 2.4), switched on by `passive_effect_rules`. Every passive works one way: it is on only when
//!    `Build::passives` lists its ability id, so nothing is on because of gear, race or class alone. That holds for
//!    racial (`EffectSource::Race`), class (`Class`), armour-weight, weapon, world and guild (`Passive`) passives; a
//!    race or class passive must also be the build's own. Only the top rank of each is written, which is what a
//!    character at the top of the line has. "Per piece worn" and "per ability slotted" are stacks
//!    (`Stacking::Additive` with an `ArmourPieces` / `SkillLineSlotted` condition); a weapon passive is gated by
//!    `WeaponEquipped`, so a bar swap changes which of them count.
//! Steps 3 to 8 are effects: each is switched on in `Player::active_effects` and aggregated by
//! `recompute`, which decides their build-time conditions with the player as the context (1.2).
//! Not gathered yet, because there is no data for them: the world and guild passives other than Mage Adept (they need a
//! compile with the `world_passives` roots, roadmap 2.4), bar abilities that give stats (2.4), debuffs on the enemy
//! (2.6, 2.9).
//!
//! The player is on the front bar; `Player::active_bar` switches it and `recompute` again.
//!
//! From the player to the numbers (roadmap 1.7): [`Build::sheet`] is the build's sheet, which decides
//! every build-time condition and leaves out the effects that need combat or a target;
//! [`Build::conditional_effects`] lists those, with their condition as text; [`Build::sheet_against`]
//! is the second sheet, the same build against an [`Enemy`], where the target-aware effects count.

use esosim_types::TargetKind;

use crate::data::bundle::Bundle;
use crate::engine::grant_rule::{GrantRule, all_grant_rules, passive_effect_rules, set_effect_rules};
use crate::engine::registry::EffectRegistry;
use crate::entity::build::Build;
use crate::entity::condition_context::TargetContext;
use crate::entity::conditional::ConditionalEffect;
use crate::entity::enemy::Enemy;
use crate::entity::player::Player;
use crate::stats::sheet::{StatContext, StatSheet, derive_sheet};

/// A build turned into a player, with what is needed to aggregate it again under another context.
struct Prepared {
    player: Player,
    registry: EffectRegistry,
    /// Set bonuses written as effects (class 3, roadmap 2.2) and the passives written as effects (roadmap 2.4); they
    /// depend on the bundle in use, and the passives on the build's race and class.
    set_effects: Vec<GrantRule>,
}

impl Prepared {
    fn rules(&self) -> Vec<&GrantRule> {
        all_grant_rules().into_iter().chain(&self.set_effects).collect()
    }
}

impl Build {
    /// Everything the engine derives from this build: the aggregated channels (`Player::channels`)
    /// and where each came from (`Player::explain`).
    pub fn to_player(&self, bundle: &Bundle) -> Player {
        self.prepare(bundle).player
    }

    /// The build's sheet: the numbers the in-game character sheet shows on the player alone. Effects
    /// that need combat or a target are not in it (see [`conditional_effects`](Self::conditional_effects)).
    pub fn sheet(&self, bundle: &Bundle) -> StatSheet {
        derive_sheet(&self.prepare(bundle).player.channels, &self.stat_context(None))
    }

    /// The effects the build's sheet leaves out because only combat or a target can decide their
    /// condition, each with its condition as text (`ConditionalEffect::when`). With `target`, each
    /// also says whether that target settles it (`vs_target`).
    pub fn conditional_effects(&self, bundle: &Bundle, target: Option<(&Enemy, &[TargetKind])>) -> Vec<ConditionalEffect> {
        let prepared = self.prepare_for(bundle, target.map(|(enemy, _)| enemy));
        let ctx = target.map(|(enemy, kinds)| TargetContext::new(&prepared.player, enemy, kinds));
        prepared.player.conditional_effects(&prepared.registry, &prepared.rules(), ctx.as_ref().map(|c| c as &dyn esosim_types::ConditionContext))
    }

    /// The second sheet: this build against `enemy` (of the given `kinds`). The conditions the target
    /// settles (its debuffs and health, what it is) are decided and the effects they gate count; every
    /// other value is the same as in [`sheet`](Self::sheet), because everything else is decided by the
    /// player exactly as before. Toggling a debuff on `enemy.active_effects` changes this sheet only.
    /// The buffs the target gives the player (`enemy.player_buffs`: the trial dummy's) are on in this
    /// sheet and not in [`sheet`](Self::sheet).
    pub fn sheet_against(&self, bundle: &Bundle, enemy: &Enemy, kinds: &[TargetKind]) -> StatSheet {
        let prepared = self.prepare_for(bundle, Some(enemy));
        let ctx = TargetContext::new(&prepared.player, enemy, kinds);
        let (_, channels) = prepared.player.derive(&prepared.registry, &prepared.rules(), &ctx);
        derive_sheet(&channels, &self.stat_context(Some(enemy)))
    }

    fn stat_context<'a>(&self, target: Option<&'a Enemy>) -> StatContext<'a> {
        StatContext { level: self.level, attributes: self.attributes, target }
    }

    /// The target `self.enemy` describes: health, resistance and debuffs, ready for
    /// [`sheet_against`](Self::sheet_against) (roadmap 2.9).
    pub fn enemy(&self, bundle: &Bundle) -> Enemy {
        self.enemy.to_enemy(&EffectRegistry::from_bundle(bundle))
    }

    fn prepare(&self, bundle: &Bundle) -> Prepared {
        self.prepare_for(bundle, None)
    }

    /// `prepare`, with the buffs `target` gives the player switched on as well.
    fn prepare_for(&self, bundle: &Bundle, target: Option<&Enemy>) -> Prepared {
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
        // what the target gives the player (the trial dummy's buffs); the build alone does not have them
        if let Some(target) = target {
            for id in &target.player_buffs {
                player.active_effects.set(*id, 1);
            }
        }

        // set bonuses written as effects (class 3, roadmap 2.2) are switched on by the set they name
        let registry = EffectRegistry::from_bundle(bundle);
        let mut set_effects = set_effect_rules(bundle.effects.values());
        // passives (roadmap 2.4): the effects of the passives the build unlocked (`Build::passives`), a race or class
        // one only if it is this race's or class's; a per-piece or per-slotted-ability one gets its count as stacks
        let (race, class) = (format!("{:?}", self.race), format!("{:?}", self.class));
        set_effects.extend(passive_effect_rules(bundle.effects.values(), &race, &class, &self.passives));
        player.recompute_with(&registry, all_grant_rules().into_iter().chain(&set_effects));
        Prepared { player, registry, set_effects }
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
            passives: vec![],
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

    /// Roadmap 2.6: the Major/Minor buffs the player gets are reviewed data now, so a set's grant rule
    /// reaches the channels: three Ansuul's Torment pieces give Minor Slayer (+5 % damage to monsters),
    /// two Mighty Chudan pieces give Major Resolve (+5948 resistance), and the buff says where it came from.
    #[test]
    fn a_set_that_grants_a_major_or_minor_buff_puts_it_in_the_channels() {
        use crate::data::skill_data::major_minor::{RESOLVE_MAJOR_ID, SLAYER_MINOR_ID};
        use crate::data::tables::set_ids::MIGHTY_CHUDAN_SET_ID;
        use crate::engine::effect::{AggKind::Multiplicative, DamageDoneContext::Monsters, ResistableDamageType::All};
        let slayer = wearing(ANSUULS_TORMENT_SET_ID, 3, None);
        assert_eq!(slayer.channels.get_multiplicative_bps(Channel::DamageDone(Monsters, Multiplicative)), 500);
        let why = slayer.explain(Channel::DamageDone(Monsters, Multiplicative));
        assert_eq!(why, vec![(EffectSource::Buff { ability_id: SLAYER_MINOR_ID }, 0.05)]);
        assert_eq!(wearing(ANSUULS_TORMENT_SET_ID, 2, None).channels.get_multiplicative_bps(Channel::DamageDone(Monsters, Multiplicative)), 0);

        let resolve = |pieces| wearing(MIGHTY_CHUDAN_SET_ID, pieces, None).explain(Channel::Armour(All));
        let buff = EffectSource::Buff { ability_id: RESOLVE_MAJOR_ID };
        assert!(resolve(2).contains(&(buff.clone(), 5948.0)));
        assert!(!resolve(1).iter().any(|(s, _)| *s == buff));
    }

    /// Infallible Mage's 5-piece "grants" Minor Vulnerability, which is a debuff on the *enemy*: the
    /// wearer must not take +5 % damage from their own set (roadmap 2.6).
    #[test]
    fn a_debuff_a_set_grants_never_reaches_the_wearer() {
        use crate::data::tables::set_ids::INFALLIBLE_MAGE_SET_ID;
        use crate::engine::effect::{AggKind::Multiplicative, DamageTakenContext::All};
        let p = wearing(INFALLIBLE_MAGE_SET_ID, 5, None);
        assert_eq!(p.channels.get_multiplicative_bps(Channel::DamageTaken(All, Multiplicative)), 0);
        assert!(!p.explain(Channel::DamageTaken(All, Multiplicative)).iter().any(|(s, _)| matches!(s, EffectSource::Debuff { .. })));
    }

    /// A target with the given debuffs on it and full health.
    fn dummy(debuffs: &[u32]) -> Enemy {
        let mut active_effects = crate::engine::active_effects::ActiveEffects::new();
        for id in debuffs {
            active_effects.set(*id, 1);
        }
        Enemy {
            id: 0,
            active_effects,
            base_channels: crate::engine::aggregator::ChannelSet::new(),
            channels: crate::engine::aggregator::ChannelSet::new(),
            player_buffs: vec![],
            max_health: 1_000.0,
            current_health: 1_000.0,
        }
    }

    /// Roadmap 1.7: the effects the build cannot decide are listed with their condition as text; a
    /// second sheet against a target switches on only the ones the target settles, so toggling a debuff
    /// on the target changes the second sheet and nothing else.
    #[test]
    fn conditional_effects_are_listed_and_a_target_debuff_changes_only_the_second_sheet() {
        const DEBUFF: u32 = 900; // any id: the condition only asks that the target has it
        let buff = |id| EffectSource::Buff { ability_id: id };
        let jsonl: String = [
            effect(10, buff(10), Condition::TargetDebuffActive(DEBUFF), 1000.0),
            // needs combat: the target does not answer it either
            effect(11, buff(11), Condition::InCombat, 100.0),
            // decided by the build, and false: neither listed nor in a sheet
            effect(12, buff(12), Condition::ClassIs(PlayerClass::Templar), 10_000.0),
            // decided by the build, and true: in both sheets
            effect(13, buff(13), Condition::ClassIs(PlayerClass::Nightblade), 500.0),
        ]
        .iter()
        .map(|d| serde_json::to_string(d).unwrap() + "\n")
        .collect();
        let bundle = Bundle::from_jsonl(&jsonl).unwrap();
        let mut b = build();
        b.mundus = MundusLoadout::default();
        b.champion = CpLoadout::default();
        b.external_buffs = vec![10, 11, 12, 13];

        let listed = b.conditional_effects(&bundle, None);
        assert_eq!(listed.iter().map(|e| e.id).collect::<Vec<_>>(), vec![10, 11]);
        assert_eq!(listed[0].when(), "debuff 900 on the target");
        assert_eq!(listed[1].when(), "in combat");
        assert!(listed.iter().all(|e| e.vs_target.is_none() && e.stacks == 1));

        let first = b.sheet(&bundle);
        assert_eq!(first.crit_rating, 500);

        // a target without the debuff settles it as "no", the combat condition stays open
        let bare = dummy(&[]);
        let against_bare = b.sheet_against(&bundle, &bare, &[]);
        assert_eq!(format!("{against_bare:?}"), format!("{first:?}"));
        let vs: Vec<_> = b.conditional_effects(&bundle, Some((&bare, &[]))).iter().map(|e| (e.id, e.vs_target)).collect();
        assert_eq!(vs, vec![(10, Some(false)), (11, None)]);

        // with the debuff on the target the second sheet changes, by that effect alone
        let debuffed = dummy(&[DEBUFF]);
        let second = b.sheet_against(&bundle, &debuffed, &[]);
        assert_eq!(second.crit_rating, 1500, "500 + the 1000 the debuff gates; the combat effect stays out");
        let mut expected = first.clone();
        expected.crit_rating = second.crit_rating;
        expected.crit_chance = second.crit_chance;
        assert_eq!(format!("{second:?}"), format!("{expected:?}"));
        let vs: Vec<_> = b.conditional_effects(&bundle, Some((&debuffed, &[]))).iter().map(|e| (e.id, e.vs_target)).collect();
        assert_eq!(vs, vec![(10, Some(true)), (11, None)]);

        // and the build's own sheet does not know about the target
        assert_eq!(format!("{:?}", b.sheet(&bundle)), format!("{first:?}"));
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
        use crate::engine::effect::AggKind::{Additive, Multiplicative};
        let percent = Channel::CostReduction(Mechanic::Magicka, Multiplicative);
        assert_eq!(seducer.channels.get_multiplicative_bps(percent), 1000);
        assert_eq!(seducer.channels.get(Channel::CostReduction(Mechanic::Magicka, Additive)), 0, "the flat bucket is untouched");
        assert_eq!(wearing(ARMOR_OF_THE_SEDUCER_SET_ID, 4, None).channels.get_multiplicative_bps(percent), 0);
        let willow = wearing(WILLOWS_PATH_SET_ID, 5, None);
        for r in [Health, Magicka, Stamina] {
            // the set's own share: the build's class and armour passives give Recovery too (roadmap 2.4)
            let ch = Channel::Recovery(r, crate::engine::effect::AggKind::Multiplicative);
            let from_set: f64 = willow.explain(ch).into_iter().filter(|(s, _)| matches!(s, EffectSource::Set { .. })).map(|(_, v)| v).sum();
            assert!((from_set - 0.18).abs() < 1e-9, "{r:?}: {from_set}");
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
        assert_eq!((d(50), d(49)), ((349, 0), (0, 4024)));
    }

    #[test]
    fn per_ability_damage_has_its_own_context_and_stays_off_the_sheet_channel() {
        use crate::engine::effect::{AggKind::Additive, DamageDoneContext, ResistableDamageType::Flame};
        use crate::data::tables::set_ids::SILKS_OF_THE_SUN_SET_ID as SET;
        let p = wearing(SET, 5, None);
        assert_eq!(p.channels.get(Channel::Power(DamageDoneContext::Type(Flame), Additive)), 466);
        assert_eq!(from_set(&p, Channel::Power(DamageDoneContext::All, Additive), SET, 5), 0.0);
    }

    /// What one source gives on a channel, summed over its effects.
    fn from_source(p: &Player, channel: Channel, source: EffectSource) -> f64 {
        p.explain(channel).into_iter().filter(|(s, _)| *s == source).map(|(_, v)| v).sum()
    }

    /// The ability id of every passive in the bundle (race, class, armour, weapon, world, guild): a build that has
    /// unlocked all of them. Whose they are is still decided by the race and class of the build.
    fn every_passive(bundle: &Bundle) -> Vec<u32> {
        let mut ids: Vec<u32> = bundle
            .effects
            .values()
            .filter_map(|d| match d.source {
                EffectSource::Race { ability_id, .. } | EffectSource::Class { ability_id, .. } | EffectSource::Passive { ability_id } => Some(ability_id),
                _ => None,
            })
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    /// Roadmap 2.4: with no passive unlocked, no passive is on, whatever the build wears, plays or is.
    #[test]
    fn no_passive_is_on_until_the_build_unlocks_it() {
        use crate::data::tables::set_ids::ANSUULS_TORMENT_SET_ID as SET;
        let bundle = Bundle::embedded();
        let mut b = wearing_build(SET, 5, Some(ItemType::Sword));
        b.front_bar = (0..3).filter_map(|i| bundle.abilities_in_skill_line(&eso_skill_data::enums::skill_line::SkillLine::Shadow).get(i).copied()).collect();
        let passive_source = |s: &EffectSource| matches!(s, EffectSource::Race { .. } | EffectSource::Class { .. } | EffectSource::Passive { .. });
        let none = b.to_player(bundle);
        assert!(!none.sources.iter().any(|(s, _)| passive_source(s)), "nothing is unlocked");
        b.passives = every_passive(bundle);
        let all = b.to_player(bundle);
        assert!(all.sources.iter().any(|(s, _)| passive_source(s)), "unlocking them switches them on");
    }

    /// Roadmap 2.4 slice 2: armour-weight passives count the pieces worn, class passives belong to the build's class,
    /// "per ability slotted" counts the active bar, and a weapon passive needs its weapon in hand. All of them are
    /// unlocked here (`every_passive`); a passive that is not listed is not on.
    #[test]
    fn armour_class_and_weapon_passives_reach_the_sheet_with_their_counts() {
        use crate::data::tables::set_ids::ANSUULS_TORMENT_SET_ID as SET;
        use crate::engine::effect::{AggKind::{Additive, Multiplicative}, ResistableDamageType::All as AllTypes, ResourceKind::Health};
        use eso_skill_data::enums::skill_line::SkillLine;
        let bundle = Bundle::embedded();
        let unlocked = every_passive(bundle);
        // `wearing` and `wearing_build` with every passive unlocked
        let worn_build = |set, pieces, weapon| {
            let mut b = wearing_build(set, pieces, weapon);
            b.passives = unlocked.clone();
            b
        };
        let worn = |set, pieces, weapon| worn_build(set, pieces, weapon).to_player(bundle);
        let near = |a: f64, b: f64| (a - b).abs() < 1e-9;
        let armour = Channel::Armour(AllTypes);
        let passive = |ability_id| EffectSource::Passive { ability_id };

        // armour weight: Resolve (45533) is 520 per Heavy piece, Juggernaut (45546) 2 % Max Health per piece, and the
        // Light and Medium passives give nothing to a build in Heavy
        let five = worn(SET, 5, None);
        assert_eq!(from_source(&five, armour, passive(45533)), 2600.0);
        assert!(near(from_source(&five, Channel::Resource(Health, Multiplicative), passive(45546)), 0.10));
        assert_eq!(from_source(&five, Channel::CriticalChance, passive(45561)), 0.0, "Prodigy is a Light passive");
        assert_eq!(from_source(&worn(SET, 4, None), armour, passive(45533)), 2080.0);
        assert_eq!(from_source(&worn(SET, 0, None), armour, passive(45533)), 0.0);

        // class: a Nightblade has Hemorrhage (+10 Critical Damage) and Refreshing Shadows (+15 % Recovery), a Templar
        // has Piercing Spear (+12) and neither of those
        let nightblade = |ability_id| EffectSource::Class { name: "Nightblade".into(), ability_id };
        let templar = |ability_id| EffectSource::Class { name: "Templar".into(), ability_id };
        let recovery = Channel::Recovery(Health, Multiplicative);
        assert_eq!(from_source(&five, Channel::CriticalDamage, nightblade(45060)), 10.0);
        assert!(near(from_source(&five, recovery, nightblade(45103)), 0.15));
        let mut b = worn_build(SET, 5, None);
        b.class = PlayerClass::Templar;
        let p = b.to_player(bundle);
        assert_eq!(from_source(&p, Channel::CriticalDamage, templar(44046)), 12.0);
        assert_eq!((from_source(&p, Channel::CriticalDamage, nightblade(45060)), from_source(&p, recovery, nightblade(45103))), (0.0, 0.0));

        // per ability slotted: Dark Vigor (45084) is +5 % Max Health for each Shadow ability on the active bar
        let shadow: Vec<u32> = bundle.abilities_in_skill_line(&SkillLine::Shadow).iter().copied().take(3).collect();
        assert_eq!(shadow.len(), 3);
        let max_health = Channel::Resource(Health, Multiplicative);
        let mut b = worn_build(SET, 0, None);
        b.front_bar = shadow[..2].to_vec();
        b.back_bar = shadow[2..].to_vec();
        let mut p = b.to_player(bundle);
        assert!(near(from_source(&p, max_health, nightblade(45084)), 0.10), "two on the front bar");
        p.active_bar = crate::entity::player::ActiveBar::Backup;
        let registry = EffectRegistry::from_bundle(bundle);
        p.recompute_with(&registry, all_grant_rules().into_iter().chain(&passive_effect_rules(bundle.effects.values(), "Dunmer", "Nightblade", &unlocked)));
        assert!(near(from_source(&p, max_health, nightblade(45084)), 0.05), "one on the backup bar");

        // weapon: Sword and Board (45452) is +5 % Weapon and Spell Damage with a sword and shield, not with a sword alone
        let power = Channel::Power(crate::engine::effect::DamageDoneContext::All, Multiplicative);
        let mut b = worn_build(SET, 0, Some(ItemType::Sword));
        assert_eq!(from_source(&b.to_player(bundle), power, passive(45452)), 0.0);
        b.gear.set_gear_piece(GearSlot::OffHand, GearPiece::new(ItemType::Shield, 66, None, Legendary, None, None));
        assert!(near(from_source(&b.to_player(bundle), power, passive(45452)), 0.05));
    }

    /// Roadmap 2.4: a build gets the racial passives of its race and no other race's, each says where it
    /// came from, and Spell Attunement's doubling (a combat state) is listed as conditional, not added.
    #[test]
    fn a_race_gets_its_own_racial_passives_and_no_others() {
        use crate::engine::effect::{AggKind::Additive, ResistableDamageType::Spell, ResourceKind::Stamina};
        use esosim_types::Race;
        let bundle = Bundle::embedded();
        let stamina = Channel::Resource(Stamina, Additive);
        let unlocked = every_passive(bundle);
        let of = |race: Race| {
            let mut b = build();
            b.race = race;
            b.passives = unlocked.clone();
            b.to_player(bundle)
        };
        for (race, expected) in [(Race::Orc, 1000), (Race::Redguard, 2000), (Race::Nord, 1500), (Race::Imperial, 2000), (Race::Dunmer, 1910), (Race::Altmer, 0)] {
            assert_eq!(of(race).channels.get(stamina), expected, "{race:?}");
        }
        let orc = of(Race::Orc);
        assert!(orc.explain(stamina).contains(&(EffectSource::Race { name: "Orc".into(), ability_id: 45309 }, 1000.0)));
        // two numbers in one tooltip are two effects: Argonian Resistance is Max Health and Disease/Poison resistance
        assert_eq!(of(Race::Argonian).channels.get(Channel::Resource(crate::engine::effect::ResourceKind::Health, Additive)), 1000);

        // nothing is on until it is unlocked, and an id of another race's passive does nothing
        let mut b = build();
        b.race = Race::Orc;
        assert_eq!(b.to_player(bundle).channels.get(stamina), 0, "an Orc who has unlocked nothing");
        b.passives = vec![45309];
        assert_eq!(b.to_player(bundle).channels.get(stamina), 1000, "Brawny alone");
        b.race = Race::Dunmer;
        assert_eq!(b.to_player(bundle).channels.get(stamina), 0, "Brawny is not a Dunmer passive");

        let mut b = build();
        b.race = Race::Breton;
        b.passives = unlocked.clone();
        assert_eq!(b.to_player(bundle).channels.get(Channel::Armour(Spell)), 2310, "the base bonus; the doubling is not in the sheet");
        // Spell Attunement is one passive of three effects: the one entry (45262) is all a build lists for it
        b.passives = vec![45262];
        let listed = b.conditional_effects(bundle, None);
        assert!(listed.iter().any(|e| e.id == 121208), "{listed:?}");
    }

    /// Roadmap 2.4 slice 3: a world / guild passive is a passive like any other. Mage Adept (40436) cuts the cost of
    /// Mages Guild abilities by 8 % and counts only when the build lists it; it is the same for any race and class.
    #[test]
    fn a_guild_passive_counts_only_when_the_build_lists_it() {
        use crate::engine::effect::AggKind::Multiplicative;
        use eso_skill_data::enums::skill_line::SkillLine;
        const MAGE_ADEPT: u32 = 40436;
        let bundle = Bundle::embedded();
        let cost = Channel::CostReductionSkillLine(SkillLine::MagesGuild, Multiplicative);
        let source = EffectSource::Passive { ability_id: MAGE_ADEPT };
        let mut b = build();
        assert_eq!(from_source(&b.to_player(bundle), cost, source.clone()), 0.0, "the passive is not unlocked");
        b.passives = vec![MAGE_ADEPT];
        assert!((from_source(&b.to_player(bundle), cost, source) - 0.08).abs() < 1e-9);
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

    /// Roadmap 2.9: the trial dummy gives the player its buffs in the second sheet only, and a group's
    /// copy of the same buff (same `buff_code`) does not add a second one.
    #[test]
    fn the_trial_dummys_buffs_are_on_the_player_against_it_and_not_in_the_build_alone() {
        use crate::engine::effect::{AggKind::Multiplicative, DamageDoneContext::Monsters};
        use crate::entity::enemy::EnemyPreset;
        let bundle = Bundle::embedded();
        let slayer = Channel::DamageDone(Monsters, Multiplicative);
        let mut b = build();
        let dummy = b.enemy(bundle);
        let bps = |p: &Prepared| p.player.channels.get_multiplicative_bps(slayer);
        assert_eq!(bps(&b.prepare(bundle)), 0, "Major Slayer comes from the dummy only");
        assert_eq!(bps(&b.prepare_for(bundle, Some(&dummy))), 1000);
        b.external_buffs = vec![93109]; // a group's Major Slayer shares buff_code 47 with the dummy's 177886
        assert_eq!(bps(&b.prepare_for(bundle, Some(&dummy))), 1000);
        b.external_buffs.clear();
        b.enemy = EnemyPreset::Custom { max_health: 1.0, resistance: 0, debuffs: vec![] };
        let custom = b.enemy(bundle);
        assert_eq!(bps(&b.prepare_for(bundle, Some(&custom))), 0, "a custom target gives no buffs");
    }
}
