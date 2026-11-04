//! Id-level summaries of what a sync changed. Generated files are line-per-record JSON that
//! starts `{"id":N,` and a constants file of `pub const NAME: u16 = N;`, so plain string
//! handling is enough — no JSON dependency.

use std::collections::BTreeMap;

#[derive(Default, Debug, PartialEq)]
pub struct Delta {
    pub added: Vec<u32>,
    pub removed: Vec<u32>,
    pub changed: Vec<u32>,
}

fn jsonl_by_id(text: &str) -> BTreeMap<u32, &str> {
    text.lines()
        .filter_map(|l| {
            let rest = l.strip_prefix("{\"id\":")?;
            let end = rest.find(|c: char| !c.is_ascii_digit())?;
            Some((rest[..end].parse().ok()?, l))
        })
        .collect()
}

pub fn jsonl(old: &str, new: &str) -> Delta {
    let (o, n) = (jsonl_by_id(old), jsonl_by_id(new));
    let mut d = Delta::default();
    for (id, line) in &n {
        match o.get(id) {
            None => d.added.push(*id),
            Some(prev) if prev != line => d.changed.push(*id),
            _ => {}
        }
    }
    d.removed = o.keys().filter(|id| !n.contains_key(id)).copied().collect();
    d
}

/// `(id, NAME)` pairs from a generated `set_ids.rs`.
pub fn consts(text: &str) -> BTreeMap<u32, String> {
    text.lines()
        .filter_map(|l| {
            let rest = l.trim().strip_prefix("pub const ")?;
            let (name, tail) = rest.split_once(':')?;
            let id = tail.split_once('=')?.1.trim().trim_end_matches(';').trim();
            Some((id.parse().ok()?, name.trim().to_string()))
        })
        .collect()
}

/// Constants whose name changed for the same id (the `ARKASIS_GENIUS` -> `ARKASISS_GENIUS`
/// problem, roadmap 1.4): returns `(id, old, new)`.
pub fn renamed(old: &str, new: &str) -> Vec<(u32, String, String)> {
    let (o, n) = (consts(old), consts(new));
    n.iter()
        .filter_map(|(id, name)| match o.get(id) {
            Some(prev) if prev != name => Some((*id, prev.clone(), name.clone())),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jsonl_delta_reports_added_removed_changed() {
        let old = "{\"id\":1,\"a\":1}\n{\"id\":2,\"a\":1}\n{\"id\":10,\"a\":1}\n";
        let new = "{\"id\":1,\"a\":1}\n{\"id\":2,\"a\":9}\n{\"id\":11,\"a\":1}\n";
        let d = jsonl(old, new);
        assert_eq!(d, Delta { added: vec![11], removed: vec![10], changed: vec![2] });
    }

    #[test]
    fn identical_files_are_an_empty_delta() {
        let t = "{\"id\":5,\"x\":[]}\n";
        assert_eq!(jsonl(t, t), Delta::default());
    }

    #[test]
    fn renames_are_found_by_id() {
        let old = "// h\npub const ARKASIS_GENIUS_SET_ID: u16 = 7;\npub const SAME_SET_ID: u16 = 8;\n";
        let new = "pub const ARKASISS_GENIUS_SET_ID: u16 = 7;\npub const SAME_SET_ID: u16 = 8;\npub const NEW_SET_ID: u16 = 9;\n";
        assert_eq!(
            renamed(old, new),
            vec![(7, "ARKASIS_GENIUS_SET_ID".to_string(), "ARKASISS_GENIUS_SET_ID".to_string())]
        );
    }
}
