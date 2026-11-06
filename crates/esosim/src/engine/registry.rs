use std::collections::HashMap;

use super::effect::EffectDef;
use crate::data::bundle::Bundle;

/// Owned lookup of effect definitions, built from a [`Bundle`] (no `&'static` statics).
#[derive(Default, Debug, Clone)]
pub struct EffectRegistry {
    by_id: HashMap<u32, EffectDef>,
}

impl EffectRegistry {
    pub fn from_bundle(bundle: &Bundle) -> Self {
        Self { by_id: bundle.effects.clone() }
    }

    /// Everything compiled into the binary.
    pub fn embedded() -> Self {
        Self::from_bundle(Bundle::embedded())
    }

    pub fn from_defs(defs: impl IntoIterator<Item = EffectDef>) -> Self {
        let mut by_id = HashMap::new();
        for def in defs {
            let (id, name) = (def.id, def.name.clone());
            let prev = by_id.insert(id, def);
            debug_assert!(prev.is_none(), "duplicate effect id {id} ({name}) registered twice");
        }
        Self { by_id }
    }

    pub fn get(&self, id: u32) -> Option<&EffectDef> {
        self.by_id.get(&id)
    }

    pub fn contains(&self, id: u32) -> bool {
        self.by_id.contains_key(&id)
    }
}
