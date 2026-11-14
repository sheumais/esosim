//! `ConditionContext` for the engine (roadmap 1.2).
//!
//! A [`Player`] answers every *build-time* [`Condition`](esosim_types::Condition) from what the
//! build itself decides, on the **active bar**: set pieces, armour weights, slotted skill lines
//! and abilities, the bar, the weapon setup and the class. Everything combat-time (health, buffs,
//! posture, a target) is left at the trait default, `None`, so such an effect shows as
//! "conditional" and the calculator leaves it out. Where the character is (`Player::location`, from
//! `Build::location`) is answered only once the build says; until then it is `None` too.
//!
//! [`TargetContext`] is the target-aware second pass: the same player plus an [`Enemy`], which
//! additionally answers the target's health and debuffs and, when the caller says what the target
//! is, `TargetIs`. The simulator (Stage 5) will implement the trait over live state instead.

use esosim_types::{ArmourWeight, Bar, ConditionContext, Location, PlayerClass, TargetKind, WeaponKind, WeaponType};
use eso_skill_data::enums::skill_line::SkillLine;

use crate::data::enums::gear::{GearSlot, ItemType};
use crate::entity::enemy::Enemy;
use crate::entity::gear::GearPiece;
use crate::entity::player::{ActiveBar, Player};

impl Player {
    /// The abilities slotted on the active bar.
    pub fn active_bar_abilities(&self) -> &[u32] {
        match self.active_bar {
            ActiveBar::Front => &self.primary_abilities,
            ActiveBar::Backup => &self.backup_abilities,
        }
    }

    /// The pieces in the main and off hand of the active bar.
    fn active_hands(&self) -> (Option<&GearPiece>, Option<&GearPiece>) {
        let (main, off) = match self.active_bar {
            ActiveBar::Front => (GearSlot::MainHand, GearSlot::OffHand),
            ActiveBar::Backup => (GearSlot::MainHandBackup, GearSlot::OffHandBackup),
        };
        (self.gear.get_gear_piece(&main), self.gear.get_gear_piece(&off))
    }

    /// How many weapons of `weapon` type are in the main and off hand of the active bar (`Condition::WeaponTypeEquipped`,
    /// `GrantCondition::WeaponsOfType`). A one-handed and a two-handed sword are both swords, a two-handed weapon is one
    /// weapon however much it counts towards set pieces, and a shield, a poison or an empty hand is none.
    pub fn get_number_of_weapons_of_type(&self, weapon: WeaponType) -> u8 {
        let (main, off) = self.active_hands();
        [main, off].into_iter().flatten().filter(|piece| weapon_type_of(piece.item_type) == Some(weapon)).count() as u8
    }

    /// The weapon setup on the active bar, named as the weapon skill lines name it. `None` when no
    /// setup is complete: an empty main hand, or a one-handed weapon with nothing (or a poison) in
    /// the off hand, which neither One Hand and Shield nor Dual Wield passives count.
    pub fn weapon_setup(&self) -> Option<WeaponKind> {
        let (main, off) = self.active_hands();
        let one_handed = |t: ItemType| matches!(t, ItemType::Axe | ItemType::Dagger | ItemType::Mace | ItemType::Sword);
        match main?.item_type {
            t if one_handed(t) => match off.map(|p| p.item_type) {
                Some(ItemType::Shield) => Some(WeaponKind::OneHandAndShield),
                Some(o) if one_handed(o) => Some(WeaponKind::DualWield),
                _ => None,
            },
            ItemType::TwoHandedAxe | ItemType::TwoHandedMace | ItemType::TwoHandedSword => Some(WeaponKind::TwoHanded),
            ItemType::Bow => Some(WeaponKind::Bow),
            ItemType::FireStaff | ItemType::FrostStaff | ItemType::LightningStaff => Some(WeaponKind::DestructionStaff),
            ItemType::HealingStaff => Some(WeaponKind::RestorationStaff),
            _ => None,
        }
    }
}

/// The weapon type of an item, `None` for everything that is not a sword, axe, mace, dagger or Destruction Staff.
fn weapon_type_of(item: ItemType) -> Option<WeaponType> {
    match item {
        ItemType::Sword | ItemType::TwoHandedSword => Some(WeaponType::Sword),
        ItemType::Axe | ItemType::TwoHandedAxe => Some(WeaponType::Axe),
        ItemType::Mace | ItemType::TwoHandedMace => Some(WeaponType::Mace),
        ItemType::Dagger => Some(WeaponType::Dagger),
        ItemType::FireStaff => Some(WeaponType::FireStaff),
        ItemType::FrostStaff => Some(WeaponType::FrostStaff),
        ItemType::LightningStaff => Some(WeaponType::LightningStaff),
        _ => None,
    }
}

impl ConditionContext for Player {
    fn set_pieces(&self, set_id: u16) -> Option<u8> {
        Some(self.get_number_of_equipped_set(set_id))
    }

    fn armour_pieces(&self, weight: ArmourWeight) -> Option<u8> {
        let item_type = match weight {
            ArmourWeight::Light => ItemType::Light,
            ArmourWeight::Medium => ItemType::Medium,
            ArmourWeight::Heavy => ItemType::Heavy,
        };
        Some(self.get_number_of_equipped_item_type(&item_type))
    }

    /// Unknown (`None`), not zero, while the abilities on the bar have no skill line recorded
    /// (`Player::ability_skill_lines` is filled from the bundle, roadmap 1.3).
    fn skill_line_slotted(&self, line: SkillLine) -> Option<u8> {
        if self.ability_skill_lines.is_empty() && !self.active_bar_abilities().is_empty() {
            return None;
        }
        Some(self.get_number_of_active_skills_from_skill_line(&line))
    }

    fn ability_slotted(&self, ability_id: u32) -> Option<bool> {
        Some(self.active_bar_abilities().contains(&ability_id))
    }

    fn on_bar(&self, bar: Bar) -> Option<bool> {
        Some(matches!((&self.active_bar, bar), (ActiveBar::Front, Bar::Front) | (ActiveBar::Backup, Bar::Backup)))
    }

    fn weapon_equipped(&self, kind: WeaponKind) -> Option<bool> {
        Some(self.weapon_setup() == Some(kind))
    }

    /// The hands of the active bar are known, so this is never unknown.
    fn weapons_of_type(&self, weapon: WeaponType) -> Option<u8> {
        Some(self.get_number_of_weapons_of_type(weapon))
    }

    /// `None` until `Player::class` is set.
    fn class(&self) -> Option<PlayerClass> {
        self.class
    }

    /// The build either names a food or it does not, so this is never unknown.
    fn food_active(&self) -> Option<bool> {
        Some(self.food.is_some())
    }

    /// How many of Heavy, Medium and Light have a piece worn: never unknown.
    fn armour_weights_worn(&self) -> Option<u8> {
        Some(self.get_number_of_distinct_armour_weights())
    }

    /// Unknown (`None`) while the class is not set, and, as for `skill_line_slotted`, while the abilities on the bar
    /// have no skill line recorded.
    fn class_abilities_slotted(&self) -> Option<u8> {
        if self.ability_skill_lines.is_empty() && !self.active_bar_abilities().is_empty() {
            return None;
        }
        self.get_number_of_active_class_skills()
    }

    /// Both bars are known to the player, so this is never unknown.
    fn ability_slotted_any_bar(&self, ability_id: u32) -> Option<bool> {
        Some(self.is_ability_slotted_on_either_bar(ability_id))
    }

    /// `None` until the build says where the character is (`Player::location`).
    fn in_pvp_area(&self) -> Option<bool> {
        self.location.map(Location::in_pvp_area)
    }

    /// `None` until the build says where the character is (`Player::location`).
    fn near_keep(&self) -> Option<bool> {
        self.location.map(Location::near_keep)
    }
}

/// The second pass of the calculator: the player's build-time answers plus a target.
///
/// `kinds` is what the caller says the target is (`Monster`, `Boss`, ...). While it is empty
/// `TargetIs` stays undecided rather than guessing. The target's afflictions are not answered
/// either: there is no map from a status effect to the ability ids that cause it yet.
pub struct TargetContext<'a> {
    pub player: &'a Player,
    pub enemy: &'a Enemy,
    pub kinds: &'a [TargetKind],
}

impl<'a> TargetContext<'a> {
    pub fn new(player: &'a Player, enemy: &'a Enemy, kinds: &'a [TargetKind]) -> Self {
        Self { player, enemy, kinds }
    }
}

impl ConditionContext for TargetContext<'_> {
    fn set_pieces(&self, set_id: u16) -> Option<u8> { self.player.set_pieces(set_id) }
    fn armour_pieces(&self, weight: ArmourWeight) -> Option<u8> { self.player.armour_pieces(weight) }
    fn skill_line_slotted(&self, line: SkillLine) -> Option<u8> { self.player.skill_line_slotted(line) }
    fn ability_slotted(&self, ability_id: u32) -> Option<bool> { self.player.ability_slotted(ability_id) }
    fn on_bar(&self, bar: Bar) -> Option<bool> { self.player.on_bar(bar) }
    fn weapon_equipped(&self, kind: WeaponKind) -> Option<bool> { self.player.weapon_equipped(kind) }
    fn weapons_of_type(&self, weapon: WeaponType) -> Option<u8> { self.player.weapons_of_type(weapon) }
    fn class(&self) -> Option<PlayerClass> { self.player.class() }
    fn food_active(&self) -> Option<bool> { self.player.food_active() }
    fn armour_weights_worn(&self) -> Option<u8> { self.player.armour_weights_worn() }
    fn class_abilities_slotted(&self) -> Option<u8> { self.player.class_abilities_slotted() }
    fn ability_slotted_any_bar(&self, ability_id: u32) -> Option<bool> { self.player.ability_slotted_any_bar(ability_id) }
    fn in_pvp_area(&self) -> Option<bool> { self.player.in_pvp_area() }
    fn near_keep(&self) -> Option<bool> { self.player.near_keep() }

    fn target_debuff_active(&self, id: u32) -> Option<bool> {
        Some(self.enemy.active_effects.has(id))
    }

    fn target_debuff_stacks(&self, id: u32) -> Option<u32> {
        Some(self.enemy.active_effects.stacks_of(id))
    }

    /// Rounded down, so 24.9 % is below 25 %. Unknown for an enemy with no health.
    fn target_health_pct(&self) -> Option<u8> {
        (self.enemy.max_health > 0.0).then(|| self.enemy.health_percent().floor().clamp(0.0, 100.0) as u8)
    }

    fn target_is(&self, kind: TargetKind) -> Option<bool> {
        (!self.kinds.is_empty()).then(|| self.kinds.contains(&kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::ItemQuality::Legendary;
    use crate::data::tables::set_ids::ANSUULS_TORMENT_SET_ID;
    use crate::engine::active_effects::ActiveEffects;
    use crate::engine::aggregator::ChannelSet;
    use esosim_types::{Affliction, Condition, Form, ResourceKind, SelfState, TriggerEvent};

    fn piece(item: ItemType, set_id: Option<u16>) -> GearPiece {
        GearPiece::new(item, 66, None, Legendary, set_id, None)
    }

    /// Heavy chest, legs and hands in Ansuul's Torment, a medium head, dual wielding on the front
    /// bar and a bow on the back bar; two abilities per bar; a Necromancer.
    fn player() -> Player {
        let mut p = Player::new();
        for slot in [GearSlot::Chest, GearSlot::Legs, GearSlot::Hands] {
            p.set_gear_piece(slot, piece(ItemType::Heavy, Some(ANSUULS_TORMENT_SET_ID)));
        }
        p.set_gear_piece(GearSlot::Head, piece(ItemType::Medium, None));
        p.set_gear_piece(GearSlot::MainHand, piece(ItemType::Sword, None));
        p.set_gear_piece(GearSlot::OffHand, piece(ItemType::Dagger, None));
        p.set_gear_piece(GearSlot::MainHandBackup, piece(ItemType::Bow, None));
        p.primary_abilities = vec![10, 11];
        p.backup_abilities = vec![20, 21];
        p.ability_skill_lines = [(10, SkillLine::Support), (11, SkillLine::Support), (20, SkillLine::BoneTyrant)].into();
        p.class = Some(PlayerClass::Necromancer);
        p
    }

    fn holds(c: Condition, p: &Player) -> Option<bool> {
        c.evaluate(p)
    }

    #[test]
    fn a_player_answers_every_build_time_condition() {
        let p = player();
        let ansuul = |n| Condition::SetPieces { set_id: ANSUULS_TORMENT_SET_ID, at_least: n };
        assert_eq!(holds(ansuul(3), &p), Some(true));
        assert_eq!(holds(ansuul(4), &p), Some(false));
        assert_eq!(holds(Condition::SetPieces { set_id: 1, at_least: 1 }, &p), Some(false));

        let armour = |weight, n| Condition::ArmourPieces { weight, at_least: n };
        assert_eq!(holds(armour(ArmourWeight::Heavy, 3), &p), Some(true));
        assert_eq!(holds(armour(ArmourWeight::Heavy, 4), &p), Some(false));
        assert_eq!(holds(armour(ArmourWeight::Medium, 1), &p), Some(true));
        assert_eq!(holds(armour(ArmourWeight::Light, 1), &p), Some(false));

        let line = |skill_line, n| Condition::SkillLineSlotted { skill_line, at_least: n };
        assert_eq!(holds(line(SkillLine::Support, 2), &p), Some(true));
        assert_eq!(holds(line(SkillLine::Support, 3), &p), Some(false));
        assert_eq!(holds(line(SkillLine::BoneTyrant, 1), &p), Some(false)); // only on the back bar

        assert_eq!(holds(Condition::AbilitySlotted { ability_id: 10 }, &p), Some(true));
        assert_eq!(holds(Condition::AbilitySlotted { ability_id: 20 }, &p), Some(false));
        assert_eq!(holds(Condition::OnBar(Bar::Front), &p), Some(true));
        assert_eq!(holds(Condition::OnBar(Bar::Backup), &p), Some(false));
        assert_eq!(holds(Condition::WeaponEquipped(WeaponKind::DualWield), &p), Some(true));
        assert_eq!(holds(Condition::WeaponEquipped(WeaponKind::Bow), &p), Some(false));
        assert_eq!(holds(Condition::ClassIs(PlayerClass::Necromancer), &p), Some(true));
        assert_eq!(holds(Condition::ClassIs(PlayerClass::Templar), &p), Some(false));
    }

    #[test]
    fn food_is_decided_by_whether_the_build_names_one() {
        let mut p = player();
        assert_eq!(holds(Condition::FoodActive, &p), Some(false));
        assert!(Condition::FoodActive.is_build_time());
        p.food = Some(61259);
        assert_eq!(holds(Condition::FoodActive, &p), Some(true));
        assert_eq!(holds(Condition::not(Condition::FoodActive), &p), Some(false));
        // the target pass keeps the player's answer
        let enemy = dummy(0.0, 0.0, None);
        assert_eq!(Condition::FoodActive.evaluate(&TargetContext::new(&p, &enemy, &[])), Some(true));
    }

    #[test]
    fn a_bar_swap_changes_every_bar_dependent_answer() {
        let mut p = player();
        p.active_bar = ActiveBar::Backup;
        assert_eq!(holds(Condition::OnBar(Bar::Backup), &p), Some(true));
        assert_eq!(holds(Condition::AbilitySlotted { ability_id: 10 }, &p), Some(false));
        assert_eq!(holds(Condition::AbilitySlotted { ability_id: 20 }, &p), Some(true));
        assert_eq!(holds(Condition::SkillLineSlotted { skill_line: SkillLine::BoneTyrant, at_least: 1 }, &p), Some(true));
        assert_eq!(holds(Condition::WeaponEquipped(WeaponKind::Bow), &p), Some(true));
        assert_eq!(holds(Condition::WeaponEquipped(WeaponKind::DualWield), &p), Some(false));
        // armour is shared between the bars, weapons are not
        assert_eq!(holds(Condition::ArmourPieces { weight: ArmourWeight::Heavy, at_least: 3 }, &p), Some(true));
    }

    #[test]
    fn every_weapon_setup_is_recognised() {
        let setup = |main, off: Option<ItemType>| {
            let mut p = Player::new();
            p.set_gear_piece(GearSlot::MainHand, piece(main, None));
            if let Some(off) = off {
                p.set_gear_piece(GearSlot::OffHand, piece(off, None));
            }
            p.weapon_setup()
        };
        assert_eq!(setup(ItemType::Mace, Some(ItemType::Shield)), Some(WeaponKind::OneHandAndShield));
        assert_eq!(setup(ItemType::Axe, Some(ItemType::Sword)), Some(WeaponKind::DualWield));
        assert_eq!(setup(ItemType::TwoHandedMace, None), Some(WeaponKind::TwoHanded));
        assert_eq!(setup(ItemType::Bow, None), Some(WeaponKind::Bow));
        assert_eq!(setup(ItemType::FrostStaff, None), Some(WeaponKind::DestructionStaff));
        assert_eq!(setup(ItemType::LightningStaff, None), Some(WeaponKind::DestructionStaff));
        assert_eq!(setup(ItemType::HealingStaff, None), Some(WeaponKind::RestorationStaff));
        // a lone one-hander (or one with only a poison) completes no setup
        assert_eq!(setup(ItemType::Dagger, None), None);
        assert_eq!(setup(ItemType::Dagger, Some(ItemType::Unknown)), None);
        assert_eq!(Player::new().weapon_setup(), None);
    }

    #[test]
    fn what_the_build_does_not_say_is_unknown_not_false() {
        let mut p = Player::new();
        assert_eq!(holds(Condition::ClassIs(PlayerClass::Necromancer), &p), None);
        // abilities on the bar but no skill lines loaded yet
        p.primary_abilities = vec![10];
        assert_eq!(holds(Condition::SkillLineSlotted { skill_line: SkillLine::Support, at_least: 1 }, &p), None);
        // an empty bar needs no lines to be sure of "none"
        assert_eq!(holds(Condition::SkillLineSlotted { skill_line: SkillLine::Support, at_least: 1 }, &Player::new()), Some(false));
    }

    #[test]
    fn every_combat_time_condition_stays_undecided_on_a_player() {
        let p = player();
        let combat = [
            Condition::SelfBuffActive(1),
            Condition::TargetDebuffActive(1),
            Condition::TargetHealthBelowPct(25),
            Condition::SelfHealthBelowPct(50),
            Condition::InCombat,
            Condition::out_of_combat(),
            Condition::SelfBuffStacks { ability_id: 1, at_least: 1 },
            Condition::TargetDebuffStacks { ability_id: 1, at_least: 1 },
            Condition::SelfResourceBelowPct { resource: ResourceKind::Health, pct: 50 },
            Condition::SelfResourceAbovePct { resource: ResourceKind::Stamina, pct: 50 },
            Condition::TargetHealthAbovePct(90),
            Condition::UltimateAtLeast(50),
            Condition::SelfState(SelfState::Blocking),
            Condition::SelfAfflicted(Affliction::Stunned),
            Condition::TargetAfflicted(Affliction::Snared),
            Condition::InForm(Form::Werewolf),
            Condition::TargetIs(TargetKind::Monster),
            Condition::SkillLineAbilityActive { skill_line: SkillLine::Support, at_least: 1 },
            Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 2000 },
        ];
        for c in combat {
            assert!(!c.is_build_time(), "{c}");
            assert_eq!(holds(c.clone(), &p), None, "{c}");
        }
        // a build-time part decides a mixed condition only when it already rules it out
        let mixed = |first| Condition::All(vec![first, Condition::InCombat]);
        assert_eq!(holds(mixed(Condition::OnBar(Bar::Front)), &p), None);
        assert_eq!(holds(mixed(Condition::OnBar(Bar::Backup)), &p), Some(false));
    }

    #[test]
    fn the_location_is_unknown_until_the_build_says_and_a_keep_is_a_pvp_area() {
        let mut p = player();
        for c in [Condition::InPvpArea, Condition::NearKeep] {
            assert!(c.is_build_time());
            assert_eq!(holds(c, &p), None);
        }
        let asks = |p: &Player| (holds(Condition::InPvpArea, p), holds(Condition::NearKeep, p));
        p.location = Some(Location::Pve);
        assert_eq!(asks(&p), (Some(false), Some(false)));
        p.location = Some(Location::PvpArea);
        assert_eq!(asks(&p), (Some(true), Some(false)));
        p.location = Some(Location::NearKeep);
        assert_eq!(asks(&p), (Some(true), Some(true)));
        // the target pass keeps the player's answer
        let enemy = dummy(0.0, 0.0, None);
        let ctx = TargetContext::new(&p, &enemy, &[]);
        assert_eq!(Condition::NearKeep.evaluate(&ctx), Some(true));
        assert_eq!(Condition::InPvpArea.evaluate(&ctx), Some(true));
    }

    #[test]
    fn armour_weights_are_counted_by_type() {
        let p = player(); // three heavy pieces and a medium head
        let weights = |n| Condition::ArmourWeightsWorn { at_least: n };
        assert_eq!(holds(weights(2), &p), Some(true));
        assert_eq!(holds(weights(3), &p), Some(false));
        assert!(weights(1).is_build_time());
        let enemy = dummy(0.0, 0.0, None);
        assert_eq!(weights(2).evaluate(&TargetContext::new(&p, &enemy, &[])), Some(true));
    }

    #[test]
    fn class_abilities_are_counted_over_the_whole_class_on_the_active_bar() {
        let line = |name: &str| (0u32..4096).filter_map(|i| SkillLine::from_id(&i)).find(|l| l.as_str() == name).unwrap();
        let mut p = player(); // a Necromancer
        p.ability_skill_lines = [(10, line("Bone Tyrant")), (11, line("Grave Lord")), (20, line("Living Death")), (21, SkillLine::Support)].into();
        p.backup_abilities = vec![20, 21];
        let slotted = |n| Condition::ClassAbilitiesSlotted { at_least: n };
        assert_eq!(holds(slotted(2), &p), Some(true));
        assert_eq!(holds(slotted(3), &p), Some(false));
        p.active_bar = ActiveBar::Backup;
        assert_eq!(holds(slotted(1), &p), Some(true));
        assert_eq!(holds(slotted(2), &p), Some(false), "the guild-line ability on the back bar does not count");
        // unknown, as for a skill line, while the bar has abilities and no line is known; unknown with no class
        let mut loading = Player::new();
        loading.class = Some(PlayerClass::Necromancer);
        loading.primary_abilities = vec![10];
        assert_eq!(holds(slotted(1), &loading), None);
        assert_eq!(holds(slotted(1), &Player::new()), None);
        loading.primary_abilities.clear();
        assert_eq!(holds(slotted(1), &loading), Some(false));
    }

    #[test]
    fn weapons_are_counted_by_type_on_the_active_bar() {
        let mut p = player(); // a sword and a dagger on the front bar, a bow on the back bar
        let has = |weapon, at_least| Condition::WeaponTypeEquipped { weapon, at_least };
        assert_eq!(holds(has(WeaponType::Sword, 1), &p), Some(true));
        assert_eq!(holds(has(WeaponType::Sword, 2), &p), Some(false));
        assert_eq!(holds(has(WeaponType::Dagger, 1), &p), Some(true));
        assert_eq!(holds(has(WeaponType::Axe, 1), &p), Some(false));
        // each hand counts
        p.set_gear_piece(GearSlot::OffHand, piece(ItemType::Sword, None));
        assert_eq!(p.get_number_of_weapons_of_type(WeaponType::Sword), 2);
        assert_eq!(p.get_number_of_weapons_of_type(WeaponType::Dagger), 0);
        // a two-handed sword is one sword, and the other bar is not asked
        p.set_gear_piece(GearSlot::MainHand, piece(ItemType::TwoHandedSword, None));
        p.set_gear_piece(GearSlot::OffHand, piece(ItemType::Shield, None));
        assert_eq!(p.get_number_of_weapons_of_type(WeaponType::Sword), 1);
        p.active_bar = ActiveBar::Backup; // a bow is none of the types
        assert_eq!(p.get_number_of_weapons_of_type(WeaponType::Sword), 0);
        // the target pass hands the answer through
        let enemy = dummy(0.0, 0.0, None);
        p.active_bar = ActiveBar::Front;
        assert_eq!(has(WeaponType::Sword, 1).evaluate(&TargetContext::new(&p, &enemy, &[])), Some(true));
    }

    #[test]
    fn a_skill_can_be_found_on_either_bar() {
        let mut p = player(); // 10, 11 on the front bar, 20, 21 on the back bar, front active
        let anywhere = |id| Condition::AbilitySlottedAnyBar { ability_id: id };
        assert_eq!(holds(anywhere(10), &p), Some(true));
        assert_eq!(holds(anywhere(20), &p), Some(true), "on the back bar, still found");
        assert_eq!(holds(Condition::AbilitySlotted { ability_id: 20 }, &p), Some(false), "the active-bar question is unchanged");
        assert_eq!(holds(anywhere(99), &p), Some(false));
        p.active_bar = ActiveBar::Backup;
        assert_eq!(holds(anywhere(10), &p), Some(true), "a bar swap does not lose it");
        let enemy = dummy(0.0, 0.0, None);
        assert_eq!(anywhere(10).evaluate(&TargetContext::new(&p, &enemy, &[])), Some(true));
    }

    fn dummy(max_health: f64, current_health: f64, debuff: Option<(u32, u32)>) -> Enemy {
        let mut active_effects = ActiveEffects::new();
        if let Some((id, stacks)) = debuff {
            active_effects.set(id, stacks);
        }
        Enemy { id: 0, active_effects, base_channels: ChannelSet::new(), channels: ChannelSet::new(), player_buffs: vec![], max_health, current_health }
    }

    #[test]
    fn the_target_pass_adds_health_debuffs_and_kind_and_keeps_the_player() {
        let p = player();
        let enemy = dummy(1000.0, 249.0, Some((7, 3)));
        let kinds = [TargetKind::Monster, TargetKind::Boss];
        let ctx = TargetContext::new(&p, &enemy, &kinds);
        let yes = |c: Condition| assert_eq!(c.evaluate(&ctx), Some(true), "{c}");
        let no = |c: Condition| assert_eq!(c.evaluate(&ctx), Some(false), "{c}");
        yes(Condition::TargetHealthBelowPct(25)); // 24.9 % is below 25 %
        no(Condition::TargetHealthBelowPct(24));
        no(Condition::TargetHealthAbovePct(25));
        yes(Condition::TargetDebuffActive(7));
        no(Condition::TargetDebuffActive(8));
        yes(Condition::TargetDebuffStacks { ability_id: 7, at_least: 3 });
        no(Condition::TargetDebuffStacks { ability_id: 7, at_least: 4 });
        yes(Condition::TargetIs(TargetKind::Boss));
        no(Condition::TargetIs(TargetKind::Player));
        // the player's own answers pass straight through
        yes(Condition::WeaponEquipped(WeaponKind::DualWield));
        yes(Condition::SetPieces { set_id: ANSUULS_TORMENT_SET_ID, at_least: 3 });
        // still not answered: the player's combat state and the target's afflictions
        assert_eq!(Condition::InCombat.evaluate(&ctx), None);
        assert_eq!(Condition::TargetAfflicted(Affliction::Snared).evaluate(&ctx), None);
    }

    #[test]
    fn the_target_pass_does_not_guess_what_the_target_is_or_its_health() {
        let p = player();
        let enemy = dummy(0.0, 0.0, None);
        let ctx = TargetContext::new(&p, &enemy, &[]);
        assert_eq!(Condition::TargetIs(TargetKind::Monster).evaluate(&ctx), None);
        assert_eq!(Condition::TargetHealthBelowPct(25).evaluate(&ctx), None);
        assert_eq!(Condition::TargetDebuffActive(7).evaluate(&ctx), Some(false));
    }
}
