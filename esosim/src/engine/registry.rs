use std::collections::HashMap;

use super::effect::Effect;

pub struct EffectRegistry {
    by_id: HashMap<u32, &'static Effect>,
}

impl EffectRegistry {
    pub fn from_statics(effects: &[&'static Effect]) -> Self {
        let mut by_id = HashMap::with_capacity(effects.len());
        for &fx in effects {
            let prev = by_id.insert(fx.id, fx);
            debug_assert!(
                prev.is_none(),
                "duplicate effect id {} ({}) registered twice",
                fx.id,
                fx.name
            );
        }
        Self { by_id }
    }

    pub fn get(&self, id: u32) -> Option<&'static Effect> {
        self.by_id.get(&id).copied()
    }

    pub fn contains(&self, id: u32) -> bool {
        self.by_id.contains_key(&id)
    }
}