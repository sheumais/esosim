//! Champion points in the engine (roadmap 2.2b + 2.2c).
//!
//! A star is an effect like any other: `cp_effects.jsonl` holds one `EffectDef` per modelled star,
//! keyed by the star's ability id, `PerStack` with the star's stages as its cap. So a CP loadout is
//! just "set each star's ability to the stages that count", which is what [`ActiveEffects::apply_champion_points`]
//! does; the stage rules themselves are `esosim_types::CpLoadout::stages_in_effect`.
//!
//! The calculator never refuses a loadout (what-if builds are the point), so the rules of the CP map
//! (4 slots per tree, a star needs a connected star with a stage, no points beyond a star's last
//! stage) are reported by [`Bundle::champion_point_issues`], not enforced, like `Player::set_issues`.

use super::active_effects::ActiveEffects;
use crate::data::bundle::Bundle;
use esosim_types::{CpIssue, CpLoadout, EffectSource};

impl ActiveEffects {
    /// Set every modelled star to the stages `loadout` has in effect (0 switches it off, so calling
    /// this again with another loadout replaces the earlier one). Stars with no effect in the bundle
    /// are skipped, and so is an ability id that another source owns.
    pub fn apply_champion_points(&mut self, bundle: &Bundle, loadout: &CpLoadout) {
        for star in bundle.stars.values() {
            let Some(fx) = bundle.effect(star.ability_id) else { continue };
            if fx.source != (EffectSource::ChampionPoint { cp_id: star.id }) {
                continue;
            }
            self.set(star.ability_id, loadout.stages_in_effect(star));
        }
    }
}

impl Bundle {
    /// What the game would not let `loadout` do (see `CpLoadout::issues`). Empty for a legal loadout.
    pub fn champion_point_issues(&self, loadout: &CpLoadout) -> Vec<CpIssue> {
        loadout.issues(&self.stars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::aggregator::ChannelSet;
    use crate::engine::effect::{
        AggKind::{Additive, Multiplicative},
        Channel, ConditionContext, DamageDoneContext, DamageTakenContext,
        ResistableDamageType as Dmg,
    };
    use crate::engine::registry::EffectRegistry;
    use esosim_types::{CoreCombatAbility, Stacking, channel::StatusEffectContext};

    fn loadout(points: &[(u32, u32)], slotted: &[u32]) -> CpLoadout {
        CpLoadout { points: points.iter().copied().collect(), slotted: slotted.to_vec() }
    }

    fn sheet(l: &CpLoadout) -> ChannelSet {
        let mut active = ActiveEffects::new();
        active.apply_champion_points(Bundle::embedded(), l);
        active.aggregate(&EffectRegistry::embedded())
    }

    #[test]
    fn every_modelled_star_is_a_star_of_the_table_with_its_own_stage_cap() {
        let b = Bundle::embedded();
        let mut modelled = 0;
        for fx in b.effects.values() {
            let EffectSource::ChampionPoint { cp_id } = fx.source else { continue };
            modelled += 1;
            let star = b.stars.get(&cp_id).unwrap_or_else(|| panic!("{} names star {cp_id}, which the table lacks", fx.name));
            assert_eq!((star.ability_id, &star.name), (fx.id, &fx.name));
            assert_eq!(fx.stacking, Stacking::PerStack { max: star.max_stages }, "{}", fx.name);
        }
        assert_eq!(modelled, 44);
    }

    #[test]
    fn every_star_is_in_a_tree_and_slottable_stars_are_in_trees_of_four_slots() {
        let b = Bundle::embedded();
        assert!(b.stars.values().all(|s| !s.tree.is_empty()), "cp.jsonl was written without cp_trees.toml");
        let trees: std::collections::BTreeSet<&str> = b.stars.values().map(|s| s.tree.as_str()).collect();
        assert_eq!(trees.len(), 3, "{trees:?}");
    }

    #[test]
    fn base_and_passive_stars_apply_by_the_stages_bought() {
        // Fortified: 34.61 armour a stage, 1 point a stage, 50 stages
        assert_eq!(sheet(&loadout(&[(34, 10)], &[])).get(Channel::Armour(Dmg::All)), 346);
        let armour = |points| sheet(&loadout(&[(34, points)], &[])).get(Channel::Armour(Dmg::All));
        assert_eq!(armour(49), 1696); // 1695.89
        assert!(armour(50) > armour(49));
        assert_eq!(armour(900), armour(50), "points past the last stage do nothing");
        // Piercing: 350 a stage, 10 points a stage, 2 stages; 9 points is no stage yet
        let pen = Channel::Penetration(Dmg::All);
        assert_eq!([9, 10, 20, 500].map(|p| sheet(&loadout(&[(10, p)], &[])).get(pen)), [0, 350, 700, 700]);
        assert_eq!(sheet(&loadout(&[(11, 20)], &[])).get(Channel::CriticalChance), 320);
    }

    #[test]
    fn a_slottable_star_applies_only_while_slotted() {
        // Wrathful Strikes: 41 a stage, 10 points a stage, 5 stages: the 205 of the damage tests
        let power = Channel::Power(DamageDoneContext::All, Additive);
        assert_eq!(sheet(&loadout(&[(8, 50)], &[])).get(power), 0);
        assert_eq!(sheet(&loadout(&[(8, 50)], &[8])).get(power), 205);
        assert_eq!(sheet(&loadout(&[(8, 50)], &[9])).get(power), 0, "another star's slot is no help");
        // Untamed Aggression is always on and feeds the same channel
        assert_eq!(sheet(&loadout(&[(4, 7), (8, 50)], &[8])).get(power), 226); // 205 + 7 * 3
    }

    #[test]
    fn applying_a_loadout_replaces_the_one_before() {
        let mut active = ActiveEffects::new();
        let b = Bundle::embedded();
        active.apply_champion_points(b, &loadout(&[(34, 10), (8, 50)], &[8]));
        assert_eq!((active.stacks_of(142035), active.stacks_of(141941)), (10, 5));
        active.apply_champion_points(b, &loadout(&[(34, 3)], &[]));
        assert_eq!((active.stacks_of(142035), active.stacks_of(141941)), (3, 0));
        active.apply_champion_points(b, &CpLoadout::default());
        assert_eq!(active.ids().count(), 0);
    }

    #[test]
    fn percentages_go_into_the_multiplicative_bucket_as_basis_points() {
        // Biting Aura: 3 % a stage, 25 points a stage, 2 stages, slottable
        let aoe = Channel::DamageDone(DamageDoneContext::AreaOfEffect, Multiplicative);
        let c = sheet(&loadout(&[(23, 50)], &[23]));
        assert_eq!((c.get_multiplicative_bps(aoe), c.get(aoe)), (600, 0));
        assert_eq!(sheet(&loadout(&[(23, 25)], &[23])).get_multiplicative_bps(aoe), 300);
        // Weapons Expert feeds light and heavy attacks, 4 % a stage
        let c = sheet(&loadout(&[(259, 20)], &[259]));
        for ctx in [DamageDoneContext::LightAttack, DamageDoneContext::HeavyAttack] {
            assert_eq!(c.get_multiplicative_bps(Channel::DamageDone(ctx, Multiplicative)), 800);
        }
        // Sustaining Shadows: 1 % of the sneak cost a stage, 50 stages
        let sneak = Channel::CostReductionCCA(CoreCombatAbility::Sneak, Multiplicative);
        assert_eq!(sheet(&loadout(&[(65, 50)], &[])).get_multiplicative_bps(sneak), 5000);
        // flat cost reductions stay additive: Tumbling is 120 Stamina a stage
        let roll = Channel::CostReductionCCA(CoreCombatAbility::DodgeRoll, Additive);
        assert_eq!(sheet(&loadout(&[(37, 30)], &[])).get(roll), 240);
    }

    #[test]
    fn reductions_of_damage_taken_are_negative_and_cover_the_listed_types() {
        let taken = |c: &ChannelSet, ctx| c.get_multiplicative_bps(Channel::DamageTaken(ctx, Multiplicative));
        // Hardy: 1 % a stage against Physical, Poison, Disease and Bleed only
        let c = sheet(&loadout(&[(16, 20)], &[]));
        for t in [Dmg::Physical, Dmg::Poison, Dmg::Disease, Dmg::Bleed] {
            assert_eq!(taken(&c, DamageTakenContext::Type(t)), -200, "{t:?}");
        }
        for t in [Dmg::Magic, Dmg::Flame, Dmg::Martial, Dmg::All] {
            assert_eq!(taken(&c, DamageTakenContext::Type(t)), 0, "{t:?}");
        }
        // Elemental Aegis: Magic, Flame, Frost and Shock
        let c = sheet(&loadout(&[(15, 10)], &[]));
        for t in [Dmg::Magic, Dmg::Flame, Dmg::Frost, Dmg::Shock] {
            assert_eq!(taken(&c, DamageTakenContext::Type(t)), -100, "{t:?}");
        }
        assert_eq!(taken(&c, DamageTakenContext::Type(Dmg::Physical)), 0);
        // Preparation (non-player attacks are monsters) and Unassailable
        assert_eq!(taken(&sheet(&loadout(&[(14, 20)], &[])), DamageTakenContext::Monsters), -1000);
        assert_eq!(taken(&sheet(&loadout(&[(133, 50)], &[133])), DamageTakenContext::AreaOfEffect), -600);
    }

    #[test]
    fn war_mage_and_mighty_are_power_for_their_own_damage_types_only() {
        let power = |t| Channel::Power(DamageDoneContext::Type(t), Additive);
        let c = sheet(&loadout(&[(21, 30)], &[]));
        for t in [Dmg::Magic, Dmg::Flame, Dmg::Frost, Dmg::Shock] {
            assert_eq!(c.get(power(t)), 100, "{t:?}");
        }
        assert_eq!(c.get(power(Dmg::Physical)), 0);
        let c = sheet(&loadout(&[(22, 30)], &[]));
        for t in [Dmg::Physical, Dmg::Poison, Dmg::Disease, Dmg::Bleed] {
            assert_eq!(c.get(power(t)), 100, "{t:?}");
        }
        assert_eq!(c.get(power(Dmg::Flame)), 0);
        assert_eq!(c.get(Channel::Power(DamageDoneContext::All, Additive)), 0);
    }

    #[test]
    fn status_effect_chance_is_magical_or_martial() {
        let chance = |c| Channel::StatusEffectChance(c);
        let c = sheet(&loadout(&[(17, 40), (18, 20)], &[]));
        assert_eq!(c.get(chance(StatusEffectContext::Magical)), 60);
        assert_eq!(c.get(chance(StatusEffectContext::Martial)), 30);
        assert_eq!(c.get(chance(StatusEffectContext::All)), 0);
    }

    #[test]
    fn bash_damage_is_flat_and_movement_speed_is_added() {
        let c = sheet(&loadout(&[(50, 20)], &[]));
        assert_eq!(c.get(Channel::DamageDone(DamageDoneContext::Bash, Additive)), 120);
        assert_eq!(c.get_multiplicative_bps(Channel::DamageDone(DamageDoneContext::Bash, Multiplicative)), 0);
        // Celerity is slottable: 2 % a stage
        assert_eq!(sheet(&loadout(&[(270, 50)], &[])).get(Channel::MovementSpeed), 0);
        assert_eq!(sheet(&loadout(&[(270, 50)], &[270])).get(Channel::MovementSpeed), 10);
        assert_eq!(sheet(&loadout(&[(92, 50)], &[92])).get(Channel::MountSpeed), 10);
    }

    #[test]
    fn steeds_blessing_needs_to_be_out_of_combat() {
        struct Out(bool);
        impl ConditionContext for Out {
            fn in_combat(&self) -> Option<bool> { Some(self.0) }
        }
        let mut active = ActiveEffects::new();
        active.apply_champion_points(Bundle::embedded(), &loadout(&[(66, 50)], &[]));
        let registry = EffectRegistry::embedded();
        assert_eq!(active.aggregate(&registry).get(Channel::MovementSpeed), 0, "undecided is left out");
        assert_eq!(active.aggregate_with(&registry, &Out(true)).get(Channel::MovementSpeed), 0);
        assert_eq!(active.aggregate_with(&registry, &Out(false)).get(Channel::MovementSpeed), 20);
    }

    #[test]
    fn a_fifth_slot_in_one_tree_is_reported_not_refused() {
        let b = Bundle::embedded();
        // Rejuvenator, Wrathful Strikes, Biting Aura, Deadly Aim and Thaumaturge are all in one tree
        let l = loadout(&[(8, 50), (9, 50), (23, 50), (25, 50), (27, 50)], &[8, 9, 23, 25, 27]);
        let tree = b.stars[&8].tree.clone();
        assert!([9u32, 23, 25, 27].iter().all(|id| b.stars[id].tree == tree));
        let slot_issues = |l: &CpLoadout| {
            b.champion_point_issues(l).into_iter().filter(|i| matches!(i, CpIssue::TooManySlotted { .. })).collect::<Vec<_>>()
        };
        assert_eq!(slot_issues(&l), vec![CpIssue::TooManySlotted { tree, slotted: 5, max: 4 }]);
        // and they all still apply
        assert_eq!(sheet(&l).get(Channel::Power(DamageDoneContext::All, Additive)), 205);
        // four is legal
        assert_eq!(slot_issues(&loadout(&[(8, 50), (9, 50), (23, 50), (25, 50)], &[8, 9, 23, 25])), vec![]);
    }

    #[test]
    fn prerequisites_are_checked_where_the_data_names_a_way_in() {
        let mut b = Bundle::default();
        let star = |id: u32, links: &str, entry: &str| {
            format!(
                r#"{{"id":{id},"name":"S{id}","ability_id":{},"perk":"passive","max_stages":2,"cost_per_stage":10,"value_per_stage":1.0,"links":{links},"tree":"T"{entry},"pos_x":1.0,"pos_y":1.0}}"#,
                100 + id
            )
        };
        b.add_stars_jsonl(&format!("{}\n{}\n", star(1, "[2]", r#","entry":true"#), star(2, "[1]", ""))).unwrap();
        assert_eq!(b.champion_point_issues(&loadout(&[(2, 10)], &[])), vec![CpIssue::MissingPrerequisite { star_id: 2 }]);
        assert_eq!(b.champion_point_issues(&loadout(&[(1, 10), (2, 10)], &[])), vec![]);
        // the shipped table, which names no way in yet, is not held to it
        let shipped = Bundle::embedded();
        if !shipped.stars.values().any(|s| s.entry) {
            assert_eq!(shipped.champion_point_issues(&loadout(&[(27, 50)], &[27])), vec![]);
        }
    }
}
