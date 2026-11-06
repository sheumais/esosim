//! Runtime view of the effect data (roadmap 1.5, Appendix A.3).
//!
//! For now this only holds `EffectDef`s; abilities, CP nodes and mundus data join it in 2.0.
//! Everything is compiled in with `include_str!`, so it works in WASM with no file IO.

use std::collections::HashMap;
use std::sync::OnceLock;

use esosim_types::EffectDef;

/// Every effect file compiled into the binary, in load order. Ids must be unique across them.
const EFFECT_FILES: [(&str, &str); 2] = [
    ("core.jsonl", include_str!("effects/core.jsonl")),
    ("legacy.jsonl", include_str!("effects/legacy.jsonl")),
];

#[derive(Default, Debug)]
pub struct Bundle {
    pub effects: HashMap<u32, EffectDef>,
}

impl Bundle {
    /// All effect files compiled into the binary.
    pub fn embedded() -> &'static Bundle {
        static EMBEDDED: OnceLock<Bundle> = OnceLock::new();
        EMBEDDED.get_or_init(|| {
            let mut bundle = Bundle::default();
            for (name, text) in EFFECT_FILES {
                bundle.add_jsonl(text).unwrap_or_else(|e| panic!("embedded effect file {name}: {e}"));
            }
            bundle
        })
    }

    /// One `EffectDef` per line; blank lines and lines starting with `//` are skipped.
    /// A duplicate id is an error.
    pub fn from_jsonl(effects: &str) -> Result<Self, String> {
        let mut bundle = Bundle::default();
        bundle.add_jsonl(effects)?;
        Ok(bundle)
    }

    pub fn add_jsonl(&mut self, effects: &str) -> Result<(), String> {
        for (i, line) in effects.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            let def: EffectDef =
                serde_json::from_str(line).map_err(|e| format!("effects line {}: {e}", i + 1))?;
            let id = def.id;
            if let Some(prev) = self.effects.insert(id, def) {
                return Err(format!("effects line {}: duplicate id {id} (`{}`)", i + 1, prev.name));
            }
        }
        Ok(())
    }

    pub fn effect(&self, id: u32) -> Option<&EffectDef> {
        self.effects.get(&id)
    }

    /// Ids that a condition refers to (`SelfBuffActive`, `TargetDebuffActive`) but that have
    /// no effect in the bundle. Sorted and de-duplicated.
    pub fn dangling_ids(&self) -> Vec<u32> {
        let mut referenced = Vec::new();
        for def in self.effects.values() {
            def.condition.referenced_effect_ids(&mut referenced);
        }
        referenced.retain(|id| !self.effects.contains_key(id));
        referenced.sort_unstable();
        referenced.dedup();
        referenced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_data_loads_and_has_no_dangling_ids() {
        let b = Bundle::embedded();
        assert!(b.effect(142035).is_some());
        assert!(b.dangling_ids().is_empty());
    }

    fn channel_total(b: &Bundle) -> usize {
        b.effects.values().map(|d| d.channels.len()).sum()
    }

    /// Roadmap 1.6: all legacy rows are in the data. The old tables had 74 live `Buff` rows
    /// (the roadmap's 75 counted a commented-out stub); FORTIFIED is in core.jsonl, the rest
    /// in legacy.jsonl, where Argonian Resistance and Wood Elf's Resist Affliction each cover
    /// two channels (Poison + Disease), so 74 rows = 76 channel entries.
    #[test]
    fn every_legacy_row_is_in_the_effect_data() {
        let b = Bundle::embedded();
        assert_eq!(channel_total(b), 76);
        assert_eq!(b.effects.len(), 67); // 66 legacy effects + FORTIFIED
    }

    #[test]
    fn major_savagery_is_2629_weapon_crit_rating() {
        use esosim_types::Channel;
        let d = Bundle::embedded().effect(61667).expect("Major Savagery (61667)");
        assert_eq!(d.name, "Major Savagery");
        assert_eq!(d.value_at(Channel::WeaponCriticalChance, 1), 2629.0);
    }

    #[test]
    fn legacy_comments_survive_as_notes() {
        let d = Bundle::embedded().effect(17906).expect("Crusher");
        assert_eq!(d.note.as_deref(), Some("Assumes infused. todo"));
    }

    #[test]
    fn rows_sharing_an_id_are_one_effect_with_several_channels() {
        use esosim_types::{Channel, ResistableDamageType};
        let d = Bundle::embedded().effect(45482).expect("Twin Blade and Blunt");
        assert_eq!(d.value_at(Channel::Penetration(ResistableDamageType::All), 1), 1487.0);
        assert_eq!(d.value_at(Channel::CriticalChance, 2), 1314.0);
        assert_eq!(d.value_at(Channel::CriticalDamage, 2), 12.0);
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let line = include_str!("effects/core.jsonl").lines().next().unwrap();
        let err = Bundle::from_jsonl(&format!("{line}\n{line}\n")).unwrap_err();
        assert!(err.contains("duplicate id 142035"), "{err}");
    }

    #[test]
    fn bad_json_names_the_line() {
        let err = Bundle::from_jsonl("// comment\n{nope}\n").unwrap_err();
        assert!(err.starts_with("effects line 2:"), "{err}");
    }
}
