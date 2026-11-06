use eso_skill_data::enums::skill_line::SkillLine;

use crate::{data::{enums::gear::ItemType, skill_data::major_minor::*, tables::sets::*}, entity::player::Player};

pub enum GrantCondition {
    EquippedItemTypeCount(ItemType),
    ActiveSkillsFromLine(SkillLine),
    EquippedSetCount { set_id: u16, threshold: u8 },
    Always,
}

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

pub static SLAYER_SETS: &[GrantRule] = &[
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: ANSUULS_TORMENT_SET_ID,           threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: ARMS_OF_RELEQUEN_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: BAHSEIS_MANIA_SET_ID,             threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: CORAL_RIPTIDE_SET_ID,             threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: FALSE_GODS_DEVOTION_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: HARMONY_IN_CHAOS_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: INFALLIBLE_MAGE_SET_ID,           threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: KAZPIANS_CRUEL_SIGNET_SET_ID,     threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: MANTLE_OF_SIRORIA_SET_ID,         threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: MASTER_ARCHITECT_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: MOONDANCER_SET_ID,                threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: MORA_SCRIBES_THESIS_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: PEACE_AND_SERENITY_SET_ID,        threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: ROAR_OF_ALKOSH_SET_ID,            threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: ROARING_OPPORTUNIST_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: SLIVERS_OF_THE_NULL_ARCA_SET_ID,  threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: SUL_XANS_TORMENT_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: TOOTH_OF_LOKKESTIIZ_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: VICIOUS_SERPENT_SET_ID,           threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: WAR_MACHINE_SET_ID,               threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: WHORL_OF_THE_DEPTHS_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: SLAYER_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: YANDIRS_MIGHT_SET_ID,             threshold: 3 }},
];

pub static AEGIS_SETS: &[GrantRule] = &[
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: AEGIS_OF_GALENWE_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: AUTOMATED_DEFENSE_SET_ID,      threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: CLAW_OF_YOLNAHKRIIN_SET_ID,    threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: DOLOROUS_ARENA_SET_ID,         threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: ETERNAL_WARRIOR_SET_ID,        threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: EYE_OF_NAHVIINTAAS_SET_ID,     threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: INVENTORS_GUARD_SET_ID,        threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: KYNES_WIND_SET_ID,             threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: LUCENT_ECHOES_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: LUNAR_BASTION_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: PEARLESCENT_WARD_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: PILLAGERS_PROFIT_SET_ID,       threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: RECOVERY_CONVERGENCE_SET_ID,   threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: SAXHLEEL_CHAMPION_SET_ID,      threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: STONE_TALKERS_OATH_SET_ID,     threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: TEST_OF_RESOLVE_SET_ID,        threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: TRANSFORMATIVE_HOPE_SET_ID,    threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: TWILIGHT_REMEDY_SET_ID,        threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: VESTMENT_OF_OLORIME_SET_ID,    threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: VROLS_COMMAND_SET_ID,          threshold: 3 }},
    GrantRule { effect_id: AEGIS_MINOR_ID, condition: GrantCondition::EquippedSetCount { set_id: XORYNS_MASTERPIECE_SET_ID,     threshold: 3 }},
];