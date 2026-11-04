//! Game-fundamentals checks (your "ensure game fundamentals through automated
//! tests" TODO). Run with `verify`; exits non-zero on any FAIL so it can gate CI
//! after each patch extract.
//!
//! The big one: `src/data/skill_data/major_minor.rs` holds ability ids typed in by
//! hand (from Healer's Haven). Here each one is checked against the extracted
//! data three ways: the record exists, its language name matches, and its
//! `major_minor_id` code maps (via `MajorMinorBuff`) to the same buff.

use crate::corpus::Corpus;
use eso_skill_data::enums::major_minor::MajorMinorBuff;
use std::collections::BTreeSet;

/// (ability id, expected name) — generated from src/data/skill_data/major_minor.rs.
/// Regenerate rather than hand-edit if that file changes.
pub const EXPECTED_BUFFS: &[(u32, &str)] = &[
    (93123, "Major Aegis"),             // AEGIS_MAJOR_ID
    (61745, "Major Berserk"),           // BERSERK_MAJOR_ID
    (61665, "Major Brutality"),         // BRUTALITY_MAJOR_ID
    (109966, "Major Courage"),          // COURAGE_MAJOR_ID
    (61705, "Major Endurance"),         // ENDURANCE_MAJOR_ID
    (61716, "Major Evasion"),           // EVASION_MAJOR_ID
    (61736, "Major Expedition"),        // EXPEDITION_MAJOR_ID
    (61747, "Major Force"),             // FORCE_MAJOR_ID
    (61698, "Major Fortitude"),         // FORTITUDE_MAJOR_ID
    (61709, "Major Heroism"),           // HEROISM_MAJOR_ID
    (61707, "Major Intellect"),         // INTELLECT_MAJOR_ID
    (61711, "Major Mending"),           // MENDING_MAJOR_ID
    (61689, "Major Prophecy"),          // PROPHECY_MAJOR_ID
    (61722, "Major Protection"),        // PROTECTION_MAJOR_ID
    (61694, "Major Resolve"),           // RESOLVE_MAJOR_ID
    (61667, "Major Savagery"),          // SAVAGERY_MAJOR_ID
    (93109, "Major Slayer"),            // SLAYER_MAJOR_ID
    (61687, "Major Sorcery"),           // SORCERY_MAJOR_ID
    (61713, "Major Vitality"),          // VITALITY_MAJOR_ID
    (76618, "Minor Aegis"),             // AEGIS_MINOR_ID
    (61744, "Minor Berserk"),           // BERSERK_MINOR_ID
    (61662, "Minor Brutality"),         // BRUTALITY_MINOR_ID
    (147417, "Minor Courage"),          // COURAGE_MINOR_ID
    (61704, "Minor Endurance"),         // ENDURANCE_MINOR_ID
    (61715, "Minor Evasion"),           // EVASION_MINOR_ID
    (61735, "Minor Expedition"),        // EXPEDITION_MINOR_ID
    (61746, "Minor Force"),             // FORCE_MINOR_ID
    (61697, "Minor Fortitude"),         // FORTITUDE_MINOR_ID
    (63569, "Gallop"),                  // GALLOP_MINOR_ID
    (61708, "Minor Heroism"),           // HEROISM_MINOR_ID
    (61706, "Minor Intellect"),         // INTELLECT_MINOR_ID
    (61710, "Minor Mending"),           // MENDING_MINOR_ID
    (61691, "Minor Prophecy"),          // PROPHECY_MINOR_ID
    (61721, "Minor Protection"),        // PROTECTION_MINOR_ID
    (61693, "Minor Resolve"),           // RESOLVE_MINOR_ID
    (61666, "Minor Savagery"),          // SAVAGERY_MINOR_ID
    (76617, "Minor Slayer"),            // SLAYER_MINOR_ID
    (61685, "Minor Sorcery"),           // SORCERY_MINOR_ID
    (88490, "Minor Toughness"),         // TOUGHNESS_MINOR_ID
    (61549, "Minor Vitality"),          // VITALITY_MINOR_ID
    (61743, "Major Breach"),            // BREACH_MAJOR_ID
    (145977, "Major Brittle"),          // BRITTLE_MAJOR_ID
    (147643, "Major Cowardice"),        // COWARDICE_MAJOR_ID
    (61727, "Major Defile"),            // DEFILE_MAJOR_ID
    (61725, "Major Maim"),              // MAIM_MAJOR_ID
    (106754, "Major Vulnerability"),    // VULNERABILITY_MAJOR_ID
    (61742, "Minor Breach"),            // BREACH_MINOR_ID
    (145975, "Minor Brittle"),          // BRITTLE_MINOR_ID
    (79867, "Minor Cowardice"),         // COWARDICE_MINOR_ID
    (61726, "Minor Defile"),            // DEFILE_MINOR_ID
    (79907, "Minor Enervation"),        // ENERVATION_MINOR_ID
    (86304, "Minor Lifesteal"),         // LIFESTEAL_MINOR_ID
    (88401, "Minor Magickasteal"),      // MAGICKASTEAL_MINOR_ID
    (61723, "Minor Maim"),              // MAIM_MINOR_ID
    (61733, "Minor Mangle"),            // MANGLE_MINOR_ID
    (140699, "Minor Timidity"),         // TIMIDITY_MINOR_ID
    (79895, "Minor Uncertainty"),       // UNCERTAINTY_MINOR_ID
    (79717, "Minor Vulnerability"),     // VULNERABILITY_MINOR_ID
];

/// Add your own spot checks here: (ability id, expected name, must have an edge to ids).
pub const SPOT_CHECKS: &[(u32, &str, &[u32])] = &[
    (142035, "Fortified", &[]), // engine::effect::FORTIFIED
    // e.g. (<Crushing Shock id>, "Crushing Shock", &[<Burning id>, <Concussion id>]),
];

#[derive(Default)]
pub struct Report {
    pub fails: usize,
    pub warns: usize,
}

impl Report {
    fn fail(&mut self, msg: String) {
        self.fails += 1;
        println!("FAIL  {msg}");
    }
    fn warn(&mut self, msg: String) {
        self.warns += 1;
        println!("WARN  {msg}");
    }
}

pub fn verify(c: &Corpus, g: &crate::graph::Graph) -> Report {
    let mut r = Report::default();

    // Table sanity: no id listed twice.
    let mut seen = BTreeSet::new();
    for (id, name) in EXPECTED_BUFFS {
        if !seen.insert(*id) {
            r.fail(format!("EXPECTED_BUFFS lists id {id} ({name}) twice"));
        }
    }

    for (id, expected) in EXPECTED_BUFFS {
        let Some(s) = c.skills.get(id) else {
            r.fail(format!("{expected}: id {id} does not exist in the extracted data"));
            continue;
        };

        let lang = c.name_of(*id);
        if !lang.eq_ignore_ascii_case(expected) {
            r.warn(format!("{expected}: id {id} is named '{lang}' in the language file"));
        }

        let code = s.major_minor_id;
        if code == 0 {
            r.warn(format!("{expected}: id {id} has no major_minor_id code"));
            continue;
        }
        match MajorMinorBuff::from_id(&(code as u32)) {
            Some(b) if b.as_str().eq_ignore_ascii_case(expected) => {}
            Some(b) => r.fail(format!(
                "{expected}: id {id} has code {code} which MajorMinorBuff says is '{}'",
                b.as_str()
            )),
            None => println!(
                "NOTE  {expected}: id {id} has code {code}, missing from MajorMinorBuff -> add `{} = {code}`",
                expected.replace(' ', "")
            ),
        }
    }

    for (id, expected, edges_to) in SPOT_CHECKS {
        let Some(_) = c.skills.get(id) else {
            r.fail(format!("spot check: id {id} ({expected}) does not exist"));
            continue;
        };
        let lang = c.name_of(*id);
        if !lang.eq_ignore_ascii_case(expected) {
            r.fail(format!("spot check: id {id} expected '{expected}', data says '{lang}'"));
        }
        let out = g.out.get(id).map(|v| v.as_slice()).unwrap_or(&[]);
        for target in *edges_to {
            if !out.iter().any(|e| e.to == *target) {
                r.fail(format!("spot check: {expected} ({id}) has no edge to {target}"));
            }
        }
    }

    println!(
        "\n{} buff ids + {} spot checks: {} fail, {} warn",
        EXPECTED_BUFFS.len(),
        SPOT_CHECKS.len(),
        r.fails,
        r.warns
    );
    r
}
