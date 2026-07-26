use eso_skill_data::enums::skill_line::SkillLine;

use crate::{data::enums::gear::ItemType, entity::player::Player};

pub enum GrantCondition {
    EquippedItemTypeCount(ItemType),
    ActiveSkillsFromLine(SkillLine),
    EquippedSetCount { set_id: u16, threshold: u8 },
    Always,
}

pub struct GrantRule { pub effect_id: u32, pub condition: GrantCondition }

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