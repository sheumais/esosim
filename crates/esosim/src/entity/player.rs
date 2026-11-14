use std::collections::HashMap;

use eso_skill_data::enums::{mechanic::Mechanic::{Stamina, Magicka}, skill_line::SkillLine};
use esosim_types::{AggKind::{self, Multiplicative}, Bucket, Channel, ConditionContext, DamageDoneContext, Location, MundusLoadout, PlayerClass, channel::{HealingDoneContext, StatusEffectContext}};

use crate::{data::{enums::{cca::CoreCombatAbility::{Block, DodgeRoll, Sprint}, damage::ResistableDamageType::{All as ResistableDamageTypeAll, *}, gear::{EnchantType, GearSlot, GearTrait, ItemType}}, tables::{enchant::get_armour_enchant_multiplier, power::OFFHAND_MULTIPLIER, sets::SETS}}, engine::{active_effects::ActiveEffects, aggregator::ChannelSet, effect::{AggKind::Additive, Channel::*, DamageDoneContext::Bash as BashDamage, ResourceKind::{Health, Magicka as Mag, Stamina as Stam}}, grant_rule::GrantRule, registry::EffectRegistry}, entity::{gear::{GearPiece, Loadout, PrismaticValueWrapper}, player::ActiveBar::*}, stats::sheet::Attributes};

use crate::engine::effect::EffectSource;

pub enum ActiveBar {
    Front,
    Backup,
}
impl ActiveBar {
    pub fn to_bool(&self) -> bool {
        match &self {
            Front => false,
            Backup => true,
        }
    }
}

pub struct Player {
    pub active_effects: ActiveEffects,
    pub channels: ChannelSet,
    pub gear: Loadout,
    pub active_bar: ActiveBar,
    pub primary_abilities: Vec<u32>,
    pub backup_abilities: Vec<u32>,
    pub werewolf_abilities: Vec<u32>,
    /// Which skill line each slotted ability belongs to (`ResolvedAbility::skill_line`). Abilities not
    /// listed here belong to no line the engine knows, so they count towards none. Filled from the
    /// `Bundle` once it carries `skill_line_of` (roadmap 1.4 / 1.3).
    pub ability_skill_lines: HashMap<u32, SkillLine>,
    pub attributes: Attributes,
    /// The stones picked, in pick order (the first is the one that survives without Twice-Born Star).
    /// Kept here, not only in `active_effects`, so a bar swap that loses the second stone's slot and
    /// then gets it back re-applies the same stone (`recompute` gates by `mundus_slots`).
    pub mundus_pick: MundusLoadout,
    /// The character's class. `None` = not set, which leaves `Condition::ClassIs` undecided
    /// (`ConditionContext for Player`, roadmap 1.2); `Build::to_player` sets it (1.3).
    pub class: Option<PlayerClass>,
    /// The food or drink the build names (`Build::food`), which `Condition::FoodActive` asks about.
    /// `None` = no food. The food's own effect is switched on in `active_effects`; this only records
    /// that there is one.
    pub food: Option<u32>,
    /// Where the character is (`Build::location`), which `Condition::InPvpArea` and `Condition::NearKeep` ask about.
    /// `None` = not said, which leaves both undecided.
    pub location: Option<Location>,
    /// Where `channels` came from: one entry per source (a gear category, a set bonus row, an
    /// effect), written by `recompute`. `channels` is the merge of all of them; `explain` reads it.
    pub sources: Vec<(EffectSource, ChannelSet)>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            active_effects: ActiveEffects::new(),
            channels: ChannelSet::new(),
            gear: Loadout::default(),
            active_bar: Front,
            primary_abilities: Vec::new(),
            backup_abilities: Vec::new(),
            werewolf_abilities: Vec::new(),
            ability_skill_lines: HashMap::new(),
            attributes: Attributes::default(),
            mundus_pick: esosim_types::MundusLoadout::default(),
            class: None,
            food: None,
            location: None,
            sources: Vec::new(),
        }
    }

    pub fn get_active_gear(&self) -> Vec<&GearPiece> {
        self.gear.get_active_gear(self.get_active_bar_as_bool())
    }

    pub fn get_active_gear_with_slots(&self) -> impl Iterator<Item = (GearSlot, &GearPiece)> {
        self.gear.iter_active_gear(self.get_active_bar_as_bool())
    }

    pub fn get_active_bar(&self) -> &ActiveBar {
        &self.active_bar
    }

    pub fn get_active_bar_as_bool(&self) -> bool {
        self.get_active_bar().to_bool()
    }

    pub fn add_buff(&mut self, id: u32, stacks: u8) {
        self.active_effects.set(id, stacks as u32);
    }
    pub fn remove_buff(&mut self, id: u32) {
        self.active_effects.set(id, 0);
    }
    pub fn has_buff(&self, id: u32) -> bool {
        self.active_effects.has(id)
    }

    pub fn is_specific_item_equipped(&self, item_id: &u32) -> bool {
        for item in self.get_active_gear() {
            if &item.item_id == item_id {return true}
        }
        false
    }

    pub fn get_number_of_equipped_item_type(&self, item_type: &ItemType) -> u8 {
        self.gear.get_number_of_item_type(item_type, self.get_active_bar_as_bool())
    }

    pub fn get_number_of_equipped_trait(&self, item_trait: &GearTrait) -> u8 {
        self.gear.get_number_of_trait(item_trait, self.get_active_bar_as_bool())
    }

    pub fn set_gear_piece(&mut self, slot: GearSlot, gear: GearPiece) {
        self.gear.set_gear_piece(slot, gear);
    }

    pub fn set_skills(&mut self, backbar: bool, skills: Vec<u32>) {
        match backbar {
            false => self.primary_abilities = skills,
            true => self.backup_abilities = skills,
        }
    }

    pub fn get_gear_piece(&self, slot: &GearSlot) -> Option<&GearPiece> {
        self.gear.get_gear_piece(slot)
    }

    pub fn recompute(&mut self, registry: &EffectRegistry, grant_rules: &[GrantRule]) {
        self.recompute_with(registry, grant_rules);
    }

    /// Derive `channels` and `sources` from the gear, the picks and `active_effects`, in the order
    /// gear, sets, then effects (grants, champion points, mundus, consumables, group buffs: all of
    /// them are effects that were switched on). Effects gated on a build-time condition are decided
    /// with the player as the context (roadmap 1.2).
    pub fn recompute_with<'a>(&mut self, registry: &EffectRegistry, grant_rules: impl IntoIterator<Item = &'a GrantRule>) {
        let rules: Vec<&GrantRule> = grant_rules.into_iter().collect();
        let (sources, channels) = self.derive(registry, &rules, &*self);
        self.channels = channels;
        self.sources = sources;
    }

    /// Every effect that is on: the ones switched on in `active_effects`, minus the mundus stones the
    /// active bar has no slot for, plus whatever the grant rules switch on. What `derive` aggregates and
    /// what `conditional_effects` lists.
    pub(crate) fn effective_effects(&self, registry: &EffectRegistry, grant_rules: &[&GrantRule]) -> ActiveEffects {
        let mut active = self.active_effects.clone();
        // Only as many stones as the gear on the active bar has room for, in pick order. The picks
        // themselves are never forgotten, so swapping to a bar with fewer than 5 Twice-Born Star pieces
        // switches the second stone off and swapping back switches the same stone on again.
        let slots = self.mundus_slots();
        let stones = self.mundus_pick.distinct().into_iter().filter(|id| {
            registry.get(*id).is_some_and(|d| matches!(d.source, crate::engine::effect::EffectSource::Mundus { .. }))
        });
        for id in stones.skip(slots) {
            active.set(id, 0);
        }
        for rule in grant_rules {
            // A set that "grants" Minor Breach, Maim or Vulnerability puts it on the enemy, not on the
            // player: a debuff never reaches the player's own channels (roadmap 2.6).
            if registry.get(rule.effect_id).is_some_and(|d| matches!(d.source, EffectSource::Debuff { .. })) {
                continue;
            }
            if let Some(stacks) = rule.evaluate(self) {
                active.set(rule.effect_id, stacks);
            }
        }
        active
    }

    /// The aggregation behind `recompute_with`, with the effects' conditions decided by `ctx` and
    /// nothing written: `ctx` is the player itself for the build's sheet, or a
    /// [`TargetContext`](crate::entity::condition_context::TargetContext) for the second sheet (roadmap
    /// 1.7). Gear and set rows do not read a condition, so they are the same either way. Returns the
    /// per-source parts (`Player::sources`) and their merge (`Player::channels`).
    pub fn derive(
        &self,
        registry: &EffectRegistry,
        grant_rules: &[&GrantRule],
        ctx: &dyn ConditionContext,
    ) -> (Vec<(EffectSource, ChannelSet)>, ChannelSet) {
        let active = self.effective_effects(registry, grant_rules);
        // Gear and sets first: their `MundusBoost` (Divines) scales the mundus stones, which are
        // worth `value * (1 + boost)` (roadmap 2.3), so the boost has to be known before the effects
        // are added. `aggregate_parts` leaves the boost out of its own `MundusBoost`, so it is
        // counted once.
        let mut sources: Vec<(EffectSource, ChannelSet)> = self.gear_parts();
        sources.extend(self.set_bonus_parts());
        if let Some(part) = self.bloodthirsty_part(ctx) {
            sources.push((EffectSource::GearTrait, part));
        }
        let mut so_far = ChannelSet::new();
        for (_, part) in &sources {
            so_far.merge(part);
        }
        let mundus_boost_bps = so_far.get_multiplicative_bps(MundusBoost);
        for (fx, part) in active.aggregate_parts(registry, ctx, mundus_boost_bps) {
            sources.push((fx.source.clone(), part));
        }
        let mut channels = ChannelSet::new();
        for (_, part) in &sources {
            channels.merge(part);
        }
        (sources, channels)
    }

    /// Where the value of `channel` comes from: one entry per source that contributes, in the order
    /// the sources were gathered, in the channel's own unit (a fraction for a multiplicative
    /// channel: 0.05 is 5 %). Needs `recompute` to have run (`Build::to_player` does).
    pub fn explain(&self, channel: Channel) -> Vec<(EffectSource, f64)> {
        self.sources
            .iter()
            .filter_map(|(source, part)| {
                let value = match channel.bucket() {
                    Bucket::Additive => part.get(channel) as f64,
                    Bucket::Multiplicative => part.get_multiplicative_bps(channel) as f64 / 10_000.0,
                };
                (value != 0.0).then(|| (source.clone(), value))
            })
            .collect()
    }

    /// What Bloodthirsty jewellery gives against the target `ctx` knows: Weapon and Spell Damage that
    /// grows with the enemy's missing health (see `bloodthirsty_fraction`). `None` when `ctx` has no
    /// target health (the build's own sheet, where the player is the context) or no worn piece has the
    /// trait, so it is only ever part of an enemy interaction. Each piece is rounded on its own.
    pub fn bloodthirsty_part(&self, ctx: &dyn ConditionContext) -> Option<ChannelSet> {
        let health = ctx.target_health_pct()?;
        let mut part = ChannelSet::new();
        let mut any = false;
        for gear in self.get_active_gear() {
            let power = gear.bloodthirsty_power(health);
            if power > 0.0 {
                part.add_additive(Power(DamageDoneContext::All, Additive), power);
                any = true;
            }
        }
        any.then_some(part)
    }

    /// Everything the gear gives, in one set (see [`gear_parts`](Self::gear_parts) for where it comes from).
    pub fn apply_gear(&self, channels: &mut ChannelSet) {
        for (_, part) in self.gear_parts() {
            channels.merge(&part);
        }
    }

    /// What the gear gives, by kind: the pieces themselves (armour, weapon power), their traits and
    /// their enchants.
    pub fn gear_parts(&self) -> Vec<(EffectSource, ChannelSet)> {
        let (mut base, mut traits, mut enchants) = (ChannelSet::new(), ChannelSet::new(), ChannelSet::new());
        self.apply_gear_into(&mut base, &mut traits, &mut enchants);
        vec![(EffectSource::Gear, base), (EffectSource::GearTrait, traits), (EffectSource::Enchant, enchants)]
    }

    fn apply_gear_into(&self, base: &mut ChannelSet, traits: &mut ChannelSet, enchants: &mut ChannelSet) { // todo add tests
        let mut divines_seen = 0u32;
        for (slot, gear) in self.get_active_gear_with_slots() {
            let power = gear.get_weapon_power();
            if power > 0f32 {
                let m = 1.0 + match () {
                    _ if !slot.is_offhand() => 0.0,
                    _ => self.channels.get(Channel::OffhandWeaponPowerBonus) as f32 / 100.0 + OFFHAND_MULTIPLIER,
                };
                let p = (power * m).round();
                base.add_additive(Power(DamageDoneContext::All, Additive), p);
            } 
            base.add_additive(Armour(ResistableDamageTypeAll), gear.get_armour_value(&slot));
            if let Some(gear_trait) = gear.get_item_trait() {
                let wrapper = gear.get_trait_value();
                match wrapper {
                    PrismaticValueWrapper::F32(v) => {
                        match gear_trait {
                            GearTrait::JewelryBloodthirsty => {}, // needs an enemy: `Player::bloodthirsty_part`
                            GearTrait::JewelryHarmony => {
                                traits.add_additive(SynergyRestore(Health, Additive), v);
                                traits.add_additive(SynergyRestore(Mag, Additive), v);
                                traits.add_additive(SynergyRestore(Stam, Additive), v);
                            },
                            GearTrait::JewelryProtective => {}, // handle in armour code
                            GearTrait::JewelrySwift => {traits.add_additive(MovementSpeed, v);},
                            GearTrait::JewelryTriune => {}, // handle below
                            GearTrait::JewelryInfused => {}, // handle in enchantment code
                            GearTrait::JewelryArcane => {traits.add_additive(Resource(Mag, Additive), v)},
                            GearTrait::JewelryRobust => {traits.add_additive(Resource(Stam, Additive), v)},
                            GearTrait::JewelryHealthy => {traits.add_additive(Resource(Health, Additive), v)},

                            GearTrait::ArmorSturdy => {traits.add_multiplicative_bps(CostReductionCCA(Block, Multiplicative), (v * 10_000.0).round());},
                            GearTrait::ArmorImpenetrable => {traits.add_additive(CriticalResistance, v);},
                            GearTrait::ArmorReinforced => {}, // handle in armour code
                            GearTrait::ArmorWellFitted => {
                                let bps = (v * 10_000.0).round();
                                traits.add_multiplicative_bps(CostReductionCCA(DodgeRoll, Multiplicative), bps);
                                traits.add_multiplicative_bps(CostReductionCCA(Sprint, Multiplicative), bps);
                            },
                            GearTrait::ArmorDivines => {
                                // The character sheet counts the second Divines piece at a whole percent
                                // (9.1 % shows as 9.0 %), the others exactly: eight Legendary pieces are
                                // +72.7 %, not 72.8 % (checked against the game's sheet, roadmap 2.3).
                                let exact = (v * 10_000.0).round();
                                let bps = if divines_seen == 1 { (v * 100.0).round() * 100.0 } else { exact };
                                divines_seen += 1;
                                traits.add_multiplicative_bps(MundusBoost, bps);
                            },
                            GearTrait::ArmorNirnhoned => {}, // handle in armour code
                            GearTrait::ArmorInfused => {}, // handle in enchantment code
                            GearTrait::ArmorTraining => {traits.add_multiplicative_bps(KillExperience, (v * 10_000.0).round());},
                            GearTrait::ArmorInvigorating => {
                                {traits.add_additive(Recovery(Mag, Additive), v)};
                                {traits.add_additive(Recovery(Stam, Additive), v)};
                                {traits.add_additive(Recovery(Health, Additive), v)};
                            },

                            GearTrait::WeaponInfused => {}, // handled in enchantment code
                            GearTrait::WeaponNirnhoned => {}, // handled in weapon code
                            GearTrait::WeaponCharged => {traits.add_multiplicative_bps(StatusEffectChance(StatusEffectContext::All), (v * 10_000.0).round());},
                            GearTrait::WeaponDecisive => {traits.add_multiplicative_bps(DecisiveUltimateChance, (v * 10_000.0).round());},
                            GearTrait::WeaponDefending => {}, // handle in armour code
                            GearTrait::WeaponPowered => {traits.add_multiplicative_bps(HealingDone(HealingDoneContext::All), (v * 10_000.0).round());},
                            GearTrait::WeaponPrecise => {traits.add_additive(CriticalChance, v)},
                            GearTrait::WeaponSharpened => {traits.add_additive(Penetration(ResistableDamageTypeAll), v)},
                            GearTrait::WeaponTraining => {traits.add_multiplicative_bps(KillExperience, (v * 10_000.0).round());},
                            _ => {},
                        }
                    }
                    PrismaticValueWrapper::Prismatic(v) => {
                        match gear_trait {
                            GearTrait::JewelryTriune => {
                                traits.add_additive(Resource(Health, Additive), v.0);
                                traits.add_additive(Resource(Mag, Additive), v.1);
                                traits.add_additive(Resource(Stam, Additive), v.2);
                            },
                            _ => {},
                        }
                    }
                }
            }
            // Weapon enchants (Fiery, Absorb, ...) have no stat value: `get_enchant_value` is `None` for them.
            if let (Some(e), Some(wrapper)) = (gear.get_enchant(), gear.get_enchant_value()) {
                let infused_multiplier = gear.infused_multiplier();
                let slot_multiplier = get_armour_enchant_multiplier(&slot);
                match wrapper {
                    PrismaticValueWrapper::F32(v_) => {
                        let v = v_ * infused_multiplier * slot_multiplier;
                        match e.glyph {
                            EnchantType::DiseaseResistance => {enchants.add_additive(Armour(Disease), v)},
                            EnchantType::FlameResistance => {enchants.add_additive(Armour(Flame), v)},
                            EnchantType::FrostResistance => {enchants.add_additive(Armour(Frost), v)},
                            EnchantType::Health => {enchants.add_additive(Resource(Health, Additive), v)},
                            EnchantType::HealthRegen => {enchants.add_additive(Recovery(Health, Additive), v)},
                            EnchantType::IncreaseBashDamage => {enchants.add_additive(DamageDone(BashDamage, Additive), v);},
                            EnchantType::IncreasePhysicalDamage => {enchants.add_additive(Power(DamageDoneContext::All, AggKind::Additive), v); enchants.add_additive(Recovery(Stam, Additive), 10. * infused_multiplier * slot_multiplier)},
                            // the glyph tables are in seconds; the channels are in milliseconds
                            EnchantType::IncreasePotionEffectiveness => {enchants.add_additive(PotionEffectiveness, v * 1000.0);},
                            EnchantType::IncreaseSpellDamage => {enchants.add_additive(Power(DamageDoneContext::All, AggKind::Additive), v); enchants.add_additive(Recovery(Mag, Additive), 10. * infused_multiplier * slot_multiplier)},
                            EnchantType::Magicka => {enchants.add_additive(Resource(Mag, Additive), v)},
                            EnchantType::MagickaRegen => {enchants.add_additive(Recovery(Mag, Additive), v)},
                            EnchantType::PhysicalResistance => {enchants.add_additive(Armour(Martial), v)},
                            EnchantType::PoisonResistance => {enchants.add_additive(Armour(Poison), v)},
                            EnchantType::PrismaticDefense => {}, // handle below
                            EnchantType::PrismaticRecovery => {}, // handle below
                            EnchantType::ReduceBlockCost => {enchants.add_additive(CostReductionCCA(Block, Additive), v);},
                            EnchantType::ReduceFeatCost => {enchants.add_additive(CostReduction(Stamina, Additive), v);},
                            EnchantType::ReducePotionCooldown => {enchants.add_additive(PotionCooldown, v * 1000.0);},
                            EnchantType::ReduceSpellCost => {enchants.add_additive(CostReduction(Magicka, Additive), v);},
                            EnchantType::ShockResistance => {enchants.add_additive(Armour(Shock), v)},
                            EnchantType::SpellResistance => {enchants.add_additive(Armour(Spell), v)},
                            EnchantType::Stamina => {enchants.add_additive(Resource(Stam, Additive), v)},
                            EnchantType::StaminaRegen => {enchants.add_additive(Recovery(Stam, Additive), v)},
                            _ => {},
                        }
                    },
                    PrismaticValueWrapper::Prismatic(v_) => {
                        let v = (v_.0 * infused_multiplier * slot_multiplier, v_.1 * infused_multiplier * slot_multiplier, v_.2 * infused_multiplier * slot_multiplier);
                        match e.glyph {
                            EnchantType::PrismaticDefense => {
                                enchants.add_additive(Resource(Health, Additive), v.0);
                                enchants.add_additive(Resource(Mag, Additive), v.1);
                                enchants.add_additive(Resource(Stam, Additive), v.2);
                            },
                            EnchantType::PrismaticRecovery => {
                                enchants.add_additive(Recovery(Health, Additive), v.0);
                                enchants.add_additive(Recovery(Mag, Additive), v.1);
                                enchants.add_additive(Recovery(Stam, Additive), v.2);
                            },
                            _ => {},
                        }
                    }
                }
            }
        }
    }

    pub fn apply_set_bonuses(&self, channels: &mut ChannelSet) {
        for (_, part) in self.set_bonus_parts() {
            channels.merge(&part);
        }
    }

    /// The stat bonuses of the worn sets, one entry per bonus row that applies (`pieces_required` is
    /// the row's piece count), in set id order.
    pub fn set_bonus_parts(&self) -> Vec<(EffectSource, ChannelSet)> {
        let mut counts: Vec<(u16, u8)> = self.get_active_sets_counts().into_iter().collect();
        counts.sort_unstable();
        let mut parts = Vec::new();
        for (set_id, count) in counts {
            let Some(set) = SETS.get(set_id) else { continue };
            for (i, row) in set.bonuses.iter().take(count as usize).enumerate() {
                let mut part = ChannelSet::new();
                let mut any = false;
                for bonus in row.iter() {
                    // `None`: a stat the compiler has no channel for yet (see sets.jsonl)
                    if let Some(channel) = bonus.channel {
                        part.add_additive(channel, bonus.value);
                        any = true;
                    }
                }
                if any {
                    parts.push((EffectSource::Set { set_id, pieces_required: (i + 1) as u8 }, part));
                }
            }
        }
        parts
    }

    pub fn get_active_sets_counts(&self) -> HashMap<u16, u8> {
        let mut sets = HashMap::new();
        for piece in self.get_active_gear() {
            let contribution = 1 + piece.is_two_handed_weapon() as u8;
            if let Some(set_id) = piece.set_id {
                *sets.entry(set_id).or_insert(0) += contribution;
                if let Some(unperfected) = SETS.unperfected_of(set_id) {
                    *sets.entry(unperfected).or_insert(0) += contribution;
                }
            }
        }
        sets
    }

    pub fn get_number_of_equipped_set(&self, set_id: u16) -> u8 {
        self.get_active_sets_counts().get(&set_id).copied().unwrap_or(0)
    }

    /// How many abilities on the active bar belong to `skill_line` (`GrantCondition::ActiveSkillsFromLine`).
    /// Only the active bar counts: a bar swap changes the answer.
    pub fn get_number_of_active_skills_from_skill_line(&self, skill_line: &SkillLine) -> u8 {
        let bar = match self.active_bar {
            Front => &self.primary_abilities,
            Backup => &self.backup_abilities,
        };
        bar.iter()
            .filter(|id| self.ability_skill_lines.get(*id) == Some(skill_line))
            .count() as u8
    }

    /// How many abilities on the active bar belong to one of the character's own class lines
    /// (`GrantCondition::ActiveClassSkills`, `Condition::ClassAbilitiesSlotted`): one number over all three lines.
    /// Only the active bar counts, and an ultimate on the bar counts like any other ability. `None` while the class
    /// is not set.
    pub fn get_number_of_active_class_skills(&self) -> Option<u8> {
        let lines = self.class?.skill_lines();
        let bar = match self.active_bar {
            Front => &self.primary_abilities,
            Backup => &self.backup_abilities,
        };
        Some(bar.iter().filter(|id| self.ability_skill_lines.get(*id).is_some_and(|l| lines.contains(l))).count() as u8)
    }

    /// How many of the three armour weights have at least one piece worn on the active bar's gear, 0 to 3
    /// (`GrantCondition::DistinctArmourWeights`, `Condition::ArmourWeightsWorn`).
    pub fn get_number_of_distinct_armour_weights(&self) -> u8 {
        [ItemType::Heavy, ItemType::Medium, ItemType::Light]
            .iter()
            .filter(|weight| self.get_number_of_equipped_item_type(weight) > 0)
            .count() as u8
    }

    /// Whether the ability is slotted on the front or the back bar, whichever is active.
    pub fn is_ability_slotted_on_either_bar(&self, ability_id: u32) -> bool {
        self.primary_abilities.contains(&ability_id) || self.backup_abilities.contains(&ability_id)
    }

    /// No range check here: the 64-point limit is `Build::validate`'s job (roadmap 4.2). The old
    /// `debug_assert!(m + h + s <= 64)` itself overflowed `u8` on large input.
    pub fn set_attributes(&mut self, attributes: Attributes) {
        self.attributes = attributes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::ItemQuality::Legendary;
    use crate::data::tables::armour::armour_from_armour_piece;
    use crate::entity::gear::GearEnchant;
    use eso_skill_data::enums::mechanic::Mechanic;
    use esosim_types::Channel;

    fn armour() -> Channel {
        Channel::Armour(ResistableDamageTypeAll)
    }

    /// The channels `apply_gear` gives a player wearing only `piece` in `slot`.
    fn gear_channels(slot: GearSlot, piece: GearPiece) -> ChannelSet {
        let mut player = Player::new();
        player.set_gear_piece(slot, piece);
        let mut channels = ChannelSet::new();
        player.apply_gear(&mut channels);
        channels
    }

    fn ring_with_enchant(glyph: EnchantType) -> GearPiece {
        let enchant = GearEnchant { glyph, effective_level: 66, quality: Legendary };
        GearPiece::new(ItemType::Ring, 66, None, Legendary, None, Some(enchant))
    }

    // Armour from a trait is folded into `GearPiece::get_armour_value`; the trait match in
    // `apply_gear` must leave these traits alone, or the value is added twice.

    #[test]
    fn defending_weapon_armour_is_counted_once() {
        let one_hand = GearPiece::new(ItemType::Sword, 66, Some(GearTrait::WeaponDefending), Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::MainHand, one_hand).get(armour()), 1638);
        let two_hand = GearPiece::new(ItemType::TwoHandedSword, 66, Some(GearTrait::WeaponDefending), Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::MainHand, two_hand).get(armour()), 3276);
    }

    #[test]
    fn protective_jewellery_armour_is_counted_once() {
        let ring = GearPiece::new(ItemType::Ring, 66, Some(GearTrait::JewelryProtective), Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::Ring1, ring).get(armour()), 1190);
    }

    #[test]
    fn nirnhoned_armour_is_counted_once() {
        let base = armour_from_armour_piece(&ItemType::Heavy, &GearSlot::Chest, &Legendary).unwrap() as u32;
        let chest = GearPiece::new(ItemType::Heavy, 66, Some(GearTrait::ArmorNirnhoned), Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::Chest, chest).get(armour()), (base + 253) as i64);
    }

    #[test]
    fn feat_cost_spell_cost_and_potion_enchants_are_mapped() {
        // 203 is the CP160 Legendary row of the shared jewellery "reduce" table
        let feat = gear_channels(GearSlot::Ring1, ring_with_enchant(EnchantType::ReduceFeatCost));
        assert_eq!(feat.get(Channel::CostReduction(Mechanic::Stamina, Additive)), 203);
        let spell = gear_channels(GearSlot::Ring1, ring_with_enchant(EnchantType::ReduceSpellCost));
        assert_eq!(spell.get(Channel::CostReduction(Mechanic::Magicka, Additive)), 203);
    }

    /// Roadmap 2.8: both potion channels are in milliseconds. A Legendary CP160 Potion Boost glyph is
    /// "increase the duration of Potion effects by 3.6 seconds" and a Potion Speed glyph takes 5 seconds
    /// off the cooldown; Infused jewellery (+60 %) scales the glyph.
    #[test]
    fn potion_glyphs_are_in_milliseconds() {
        let boost = gear_channels(GearSlot::Ring1, ring_with_enchant(EnchantType::IncreasePotionEffectiveness));
        assert_eq!(boost.get(Channel::PotionEffectiveness), 3600);
        let speed = gear_channels(GearSlot::Ring1, ring_with_enchant(EnchantType::ReducePotionCooldown));
        assert_eq!(speed.get(Channel::PotionCooldown), 5000);

        let enchant = GearEnchant { glyph: EnchantType::IncreasePotionEffectiveness, effective_level: 66, quality: Legendary };
        let infused = GearPiece::new(ItemType::Ring, 66, Some(GearTrait::JewelryInfused), Legendary, None, Some(enchant));
        assert_eq!(gear_channels(GearSlot::Ring1, infused).get(Channel::PotionEffectiveness), 5760);
    }

    /// Weapon power from gear is on the channel the sheet reads, and Nirnhoned raises it by its fraction
    /// (+20 % at Legendary: 1335 -> 1602), where it used to leave 20 % of it.
    #[test]
    fn a_nirnhoned_weapon_has_more_power_not_less() {
        let power = Channel::Power(DamageDoneContext::All, Additive);
        let plain = GearPiece::new(ItemType::Sword, 66, None, Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::MainHand, plain).get(power), 1335);
        let nirnhoned = GearPiece::new(ItemType::Sword, 66, Some(GearTrait::WeaponNirnhoned), Legendary, None, None);
        assert_eq!(gear_channels(GearSlot::MainHand, nirnhoned).get(power), 1602);
    }

    #[test]
    fn infused_is_a_fraction_of_the_piece_and_only_for_infused_traits() {
        let piece = |t| GearPiece::new(ItemType::Heavy, 66, t, Legendary, None, None);
        assert_eq!(piece(Some(GearTrait::ArmorInfused)).infused_bonus(), 0.25);
        assert_eq!(piece(Some(GearTrait::ArmorInfused)).infused_multiplier(), 1.25);
        assert_eq!(piece(Some(GearTrait::ArmorSturdy)).infused_multiplier(), 1.0);
        assert_eq!(piece(None).infused_multiplier(), 1.0);
    }

    /// The target's health and nothing else.
    struct TargetAt(u8);
    impl ConditionContext for TargetAt {
        fn target_health_pct(&self) -> Option<u8> { Some(self.0) }
    }

    /// Roadmap 2.8: Bloodthirsty is Power against a target: none at 90 % health or more, all of it at 0 %,
    /// straight in between, and absent from any context that has no target (the build's own sheet).
    #[test]
    fn bloodthirsty_is_power_against_an_enemy_by_its_health() {
        let power = Channel::Power(DamageDoneContext::All, Additive);
        let ring = || GearPiece::new(ItemType::Ring, 66, Some(GearTrait::JewelryBloodthirsty), Legendary, None, None);
        let mut player = Player::new();
        player.set_gear_piece(GearSlot::Ring1, ring());
        player.set_gear_piece(GearSlot::Ring2, ring());

        let at = |health| player.bloodthirsty_part(&TargetAt(health)).map_or(0, |c| c.get(power));
        assert_eq!((at(100), at(90), at(89)), (0, 0, 2 * 4)); // 350 / 90 = 3.9 per 1 %, rounded per piece
        assert_eq!((at(45), at(0)), (350, 700));
        assert!(player.bloodthirsty_part(&player).is_none(), "the player alone has no enemy health");
        assert!(Player::new().bloodthirsty_part(&TargetAt(0)).is_none(), "no Bloodthirsty piece worn");

        let registry = EffectRegistry::from_defs(Vec::new());
        let (_, alone) = player.derive(&registry, &[], &player);
        let (sources, against) = player.derive(&registry, &[], &TargetAt(0));
        assert_eq!((alone.get(power), against.get(power)), (0, 700));
        assert!(sources.iter().any(|(s, part)| *s == EffectSource::GearTrait && part.get(power) == 700));
    }

    #[test]
    fn a_weapon_enchant_has_no_stat_value_and_does_not_panic() {
        let enchant = GearEnchant { glyph: EnchantType::FieryWeapon, effective_level: 66, quality: Legendary };
        let sword = GearPiece::new(ItemType::Sword, 66, None, Legendary, None, Some(enchant));
        gear_channels(GearSlot::MainHand, sword);
    }

    #[test]
    fn skills_count_towards_a_line_only_on_the_active_bar() {
        let mut lines = (0u32..4096).filter_map(|id| SkillLine::from_id(&id));
        let (a, b) = (lines.next().unwrap(), lines.next().unwrap());
        let mut player = Player::new();
        player.ability_skill_lines = HashMap::from([(1, a), (2, a), (3, b), (4, a)]);
        player.primary_abilities = vec![1, 2, 3, 99]; // 99 has no known line
        player.backup_abilities = vec![4];
        assert_eq!(player.get_number_of_active_skills_from_skill_line(&a), 2);
        assert_eq!(player.get_number_of_active_skills_from_skill_line(&b), 1);
        player.active_bar = Backup;
        assert_eq!(player.get_number_of_active_skills_from_skill_line(&a), 1);
        assert_eq!(player.get_number_of_active_skills_from_skill_line(&b), 0);
    }

    #[test]
    fn class_skills_are_counted_over_all_three_lines_of_the_class_on_the_active_bar() {
        let line = |name: &str| (0u32..4096).filter_map(|i| SkillLine::from_id(&i)).find(|l| l.as_str() == name).unwrap();
        let mut player = Player::new();
        assert_eq!(player.get_number_of_active_class_skills(), None, "no class, no answer");
        player.class = Some(PlayerClass::Sorcerer);
        player.ability_skill_lines = HashMap::from([
            (1, line("Storm Calling")),
            (2, line("Dark Magic")),
            (3, line("Daedric Summoning")),
            (4, line("Assassination")), // another class's line
            (5, line("Mages Guild")),   // a guild line
            (6, line("Storm Calling")),
        ]);
        player.primary_abilities = vec![1, 2, 3, 4, 5, 99]; // 99 has no known line
        player.backup_abilities = vec![6];
        assert_eq!(player.get_number_of_active_class_skills(), Some(3));
        player.active_bar = Backup;
        assert_eq!(player.get_number_of_active_class_skills(), Some(1));
        player.class = Some(PlayerClass::Arcanist);
        assert_eq!(player.get_number_of_active_class_skills(), Some(0));
    }

    #[test]
    fn armour_weights_are_counted_by_type_not_by_piece() {
        let piece = |t| GearPiece::new(t, 66, None, Legendary, None, None);
        let mut player = Player::new();
        assert_eq!(player.get_number_of_distinct_armour_weights(), 0);
        for slot in [GearSlot::Chest, GearSlot::Legs, GearSlot::Hands] {
            player.set_gear_piece(slot, piece(ItemType::Heavy));
        }
        assert_eq!(player.get_number_of_distinct_armour_weights(), 1, "three heavy pieces are one weight");
        player.set_gear_piece(GearSlot::Head, piece(ItemType::Medium));
        assert_eq!(player.get_number_of_distinct_armour_weights(), 2);
        player.set_gear_piece(GearSlot::Feet, piece(ItemType::Light));
        assert_eq!(player.get_number_of_distinct_armour_weights(), 3);
    }

    #[test]
    fn an_ability_can_be_found_on_either_bar() {
        let mut player = Player::new();
        player.primary_abilities = vec![1];
        player.backup_abilities = vec![2];
        assert!(player.is_ability_slotted_on_either_bar(1) && player.is_ability_slotted_on_either_bar(2));
        assert!(!player.is_ability_slotted_on_either_bar(3));
    }

    #[test]
    fn any_attribute_input_is_accepted() {
        Player::new().set_attributes(Attributes { health: 255, magicka: 255, stamina: 255 });
    }
}
