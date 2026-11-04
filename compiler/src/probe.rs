//! Diagnostics that answer the open questions in PLAN.md §10 from real data,
//! so the answers don't have to be guessed.

use crate::corpus::Corpus;
use eso_skill_data::SkillData34;
use eso_skill_data::enums::major_minor::MajorMinorBuff;
use std::collections::BTreeMap;

struct Probe {
    name: &'static str,
    extract: fn(&SkillData34) -> Vec<u32>,
}

/// Every u32-valued field that *might* hold an ability id. The first block is
/// already used as edges; the second block is "unknown meaning" — if one of
/// those shows a high valid-id rate in `probe-edges`, promote it to an edge.
fn probes() -> Vec<Probe> {
    vec![
        // --- already edges (sanity: these should be ~100% valid) ---
        Probe { name: "causes_ids", extract: |s| s.causes_ids.clone() },
        Probe { name: "base_data.caused_by", extract: |s| vec![s.base_data.caused_by] },
        Probe { name: "base_data.related_ability_id", extract: |s| vec![s.base_data.related_ability_id] },
        Probe { name: "u8c (replaces)", extract: |s| vec![s.u8c] },
        Probe { name: "list25[].base_ability_id", extract: |s| s.list25.iter().map(|e| e.base_ability_id).collect() },
        Probe { name: "tooltip_data[].tooltip_ids", extract: |s| s.tooltip_data.iter().flat_map(|t| t.tooltip_ids.iter().copied()).collect() },
        Probe { name: "u13.u33c_ability_id", extract: |s| vec![s.u13.u33c_ability_id] },
        Probe { name: "u15.u34c_ability_id", extract: |s| vec![s.u15.u34c_ability_id] },
        Probe { name: "u18.u374_ability_id", extract: |s| vec![s.u18.u374_ability_id] },
        Probe { name: "u22.u39c_ability_id", extract: |s| vec![s.u22.u39c_ability_id] },
        // --- unknown meaning ---
        Probe { name: "list26[].u2", extract: |s| s.list26.iter().flat_map(|l| l.u2.iter().copied()).collect() },
        Probe { name: "list26[].list3", extract: |s| s.list26.iter().flat_map(|l| l.list3.iter().copied()).collect() },
        Probe { name: "list5[].d5a", extract: |s| s.list5.iter().flat_map(|e| e.d5a.iter().copied()).collect() },
        Probe { name: "list5[].d5b", extract: |s| s.list5.iter().flat_map(|e| e.d5b.iter().copied()).collect() },
        Probe { name: "list6a", extract: |s| s.list6a.clone() },
        Probe { name: "list7", extract: |s| s.list7.clone() },
        Probe { name: "list8", extract: |s| s.list8.clone() },
        Probe { name: "list9[].list9a", extract: |s| s.list9.iter().flat_map(|e| e.list9a.iter().copied()).collect() },
        Probe { name: "list9[].list9b", extract: |s| s.list9.iter().flat_map(|e| e.list9b.iter().copied()).collect() },
        Probe { name: "list11[].list11a", extract: |s| s.list11.iter().flat_map(|e| e.list11a.iter().copied()).collect() },
        Probe { name: "list11[].list11b", extract: |s| s.list11.iter().flat_map(|e| e.list11b.iter().copied()).collect() },
        Probe { name: "base_data.value0 (>0)", extract: |s| positive(s.base_data.value0) },
        Probe { name: "base_data.value1 (>0)", extract: |s| positive(s.base_data.value1) },
        Probe { name: "base_data.value2 (>0)", extract: |s| positive(s.base_data.value2) },
    ]
}

fn positive(v: i32) -> Vec<u32> {
    if v > 0 { vec![v as u32] } else { Vec::new() }
}

fn pct(part: usize, whole: usize) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 * 100.0 / whole as f64 }
}

/// For every candidate field: how often is a non-zero value a real ability id?
/// Compare against the printed base rate — a field far above it is an edge.
pub fn probe_edges(c: &Corpus) {
    let max_id = c.skills.keys().next_back().copied().unwrap_or(0) as f64;
    let base = c.skills.len() as f64 * 100.0 / (max_id + 1.0);
    println!(
        "{} records, max id {}. A random number in 0..=max is a valid id {:.1}% of the time.\n\
         The '>=255' columns exclude small constants (PLAN.md §10 heuristic).\n",
        c.skills.len(),
        max_id as u64,
        base
    );
    println!(
        "{:<32} {:>9} {:>9} {:>7} | {:>9} {:>9} {:>7}",
        "field", "nonzero", "valid", "rate", ">=255", "valid", "rate"
    );
    for p in probes() {
        let (mut nz, mut ok, mut nz_big, mut ok_big) = (0usize, 0usize, 0usize, 0usize);
        for s in c.skills.values() {
            for v in (p.extract)(s) {
                if v == 0 {
                    continue;
                }
                nz += 1;
                let valid = c.skills.contains_key(&v);
                if valid {
                    ok += 1;
                }
                if v >= 255 {
                    nz_big += 1;
                    if valid {
                        ok_big += 1;
                    }
                }
            }
        }
        println!(
            "{:<32} {:>9} {:>9} {:>6.1}% | {:>9} {:>9} {:>6.1}%",
            p.name, nz, ok, pct(ok, nz), nz_big, ok_big, pct(ok_big, nz_big)
        );
    }
}

/// Group records by their `major_minor_id` code and show the most common names
/// per code. Fills the gaps in `MajorMinorBuff` (e.g. which code is Minor Slayer?)
/// directly from the data. Codes with no enum entry are flagged.
pub fn major_minor_report(c: &Corpus) {
    let mut by_code: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
    for (id, s) in &c.skills {
        if s.major_minor_id != 0 {
            by_code.entry(s.major_minor_id).or_default().push(*id);
        }
    }
    println!("{:>4}  {:<24} {:>7}  most common names (count, example id)", "code", "MajorMinorBuff", "records");
    for (code, ids) in &by_code {
        let known = MajorMinorBuff::from_id(&(*code as u32)).map(|m| m.as_str());
        let mut names: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        for id in ids {
            names.entry(c.name_of(*id)).or_default().push(*id);
        }
        let mut ranked: Vec<(&String, &Vec<u32>)> = names.iter().collect();
        ranked.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
        println!(
            "{:>4}  {:<24} {:>7}{}",
            code,
            known.unwrap_or("<<NOT IN ENUM>>"),
            ids.len(),
            if known.is_none() { "   <-- add to eso_skill_data" } else { "" }
        );
        for (n, v) in ranked.iter().take(4) {
            println!("        {:>5}x {:<34} e.g. {}", v.len(), n, v[0]);
        }
    }
}

/// `who-causes <id>`: every record that references `id`, and how.
pub fn who_causes(c: &Corpus, g: &crate::graph::Graph, id: u32) {
    println!("{} ({})", id, c.name_of(id));
    match g.incoming.get(&id) {
        None => println!("  nothing references this id"),
        Some(refs) => {
            println!("  referenced by {} record(s):", refs.len());
            for r in refs {
                let lt = r.link_type.map_or(String::new(), |t| format!(" link_type={t}"));
                println!("    {:>7}  {:<40} via {:?}{}", r.from, c.name_of(r.from), r.kind, lt);
            }
        }
    }
}
