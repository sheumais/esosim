//! serde adapters for the two `eso_skill_data` enums that have no serde impls.
//!
//! Used through `#[serde(with = "...")]` on the `Channel` variants that carry a
//! [`Mechanic`] or a [`SkillLine`]. Both are written as their display name (what the game
//! calls them: `"Ultimate"`, `"Restoration Staff"`), which is stable and readable in
//! `bundle.jsonl` / `sets.jsonl`, and read back by looking the name up among the known ids.
//! An unknown name is an error, never a silent default.

macro_rules! by_name {
    ($module:ident, $ty:path, $id:ty, $ids:expr, $what:literal) => {
        pub mod $module {
            use $ty as T;
            use serde::{de, Deserialize, Deserializer, Serializer};

            pub fn serialize<S: Serializer>(v: &T, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(v.as_str())
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<T, D::Error> {
                let name = String::deserialize(d)?;
                super::find::<T, $id>($ids, |i| T::from_id(&i), |v| v.as_str(), &name)
                    .ok_or_else(|| de::Error::custom(format!("unknown {} `{name}`", $what)))
            }
        }
    };
}

fn find<T, I>(ids: impl Iterator<Item = I>, from_id: impl Fn(I) -> Option<T>, name_of: impl Fn(&T) -> &'static str, name: &str) -> Option<T> {
    ids.filter_map(from_id).find(|v| name_of(v) == name)
}

by_name!(mechanic, eso_skill_data::enums::mechanic::Mechanic, u8, 0u8..=u8::MAX, "mechanic");
by_name!(skill_line, eso_skill_data::enums::skill_line::SkillLine, u32, 0u32..4096, "skill line");

#[cfg(test)]
mod tests {
    use eso_skill_data::enums::{mechanic::Mechanic, skill_line::SkillLine};

    #[test]
    fn every_known_name_is_found_again() {
        for id in 0u8..=u8::MAX {
            if let Some(m) = Mechanic::from_id(&id) {
                assert_eq!(super::find(0u8..=u8::MAX, |i| Mechanic::from_id(&i), |v| v.as_str(), m.as_str()), Some(m));
            }
        }
        for id in 0u32..4096 {
            if let Some(l) = SkillLine::from_id(&id) {
                assert_eq!(super::find(0u32..4096, |i| SkillLine::from_id(&i), |v| v.as_str(), l.as_str()), Some(l));
            }
        }
    }

    #[test]
    fn unknown_names_are_not_found() {
        assert_eq!(super::find(0u8..=u8::MAX, |i| Mechanic::from_id(&i), |v| v.as_str(), "Wibble"), None);
    }
}
