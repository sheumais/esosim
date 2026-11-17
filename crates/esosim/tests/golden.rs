//! Golden fixtures (roadmap 3.1, Appendix B): a character as the GoldenFixture addon exported it, plus the numbers
//! the game's own character sheet showed for it. `every_golden_fixture_matches_the_game` runs every `*.json` in
//! `tests/golden/`: it turns the fixture's `build` into a [`Build`], derives `Build::sheet`, and compares each number
//! of `expected.sheet` with the engine's. All differences of all fixtures are reported together, not just the first.
//!
//! To add a fixture: run `/goldenfixture` in game, save the output as a new file in `tests/golden/`, run
//! `cargo test -p esosim --test golden -- --nocapture`. "Every new data source ships with a fixture that includes it."
//!
//! # What the harness adds to the addon's `build`
//!
//! The addon writes a `build` that is nearly a serialised [`Build`] but not quite; [`build_of`] closes the gap and says
//! so in the report (`note:` lines) every time it has to:
//!
//! * `class` is taken from `extra.character.class_id` when `build.class` is missing (addon 1.0.4 leaves it out).
//! * An `enchant` that only names its `glyph` gets the piece's `effective_level` and `quality`.
//! * `subclass` is not exported by the addon, so a build without one is a pure class to the engine and a subclassed
//!   character loses the passives of the lines it took (an Arcanist with Assassination has no Pressure Points).
//!   [`infer_subclass`] works the swaps out from the skill lines of the passives and slotted abilities.
//! * Buffs that were on the character when the numbers were read (`extra.buffs`) are added to `external_buffs` when the
//!   engine has a `Buff` effect for them (see [`MERGE_OBSERVED_BUFFS`]). The engine cannot yet derive Major Brutality from
//!   the skill that gave it in game, and the sheet it is compared with includes it. Buffs the engine has no effect for
//!   (food, vampirism, ...) are listed as `unmodelled`: they are in the game's numbers and not in the engine's.
//!
//! # What is compared
//!
//! Every number in `expected.sheet` (or in `expected` itself, for a flat hand-written fixture) that [`engine_value`]
//! knows. The tolerance is exact for a whole number and [`FRACTION_TOLERANCE`] for a fraction; `tolerance` in the
//! fixture overrides it per key. A key listed in `hidden` is shown and not asserted: the game's sheet leaves out a bonus
//! the engine applies (Appendix D). A key the engine has no stat for is listed as `not compared` and not asserted.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

use serde_json::Value;

use esosim::data::bundle::Bundle;
use esosim::data::tables::sets::SETS;
use esosim::engine::effect::{AggKind, Bucket, Channel, EffectSource, ResistableDamageType as Dmg, ResourceKind};
use esosim::entity::build::Build;
use esosim::entity::player::Player;
use esosim::stats::sheet::StatSheet;
use esosim_types::{PlayerClass, SubclassSwap, subclass::MAX_SUBCLASS_SWAPS};

const FIXTURE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");

/// Add the buffs the character had in game to the build's `external_buffs` (those the engine has a `Buff` effect for).
/// Set to `false` to compare the build exactly as the addon wrote it.
const MERGE_OBSERVED_BUFFS: bool = true;

/// Under each compared number, list every source the engine counted for it (see [`sources_of`]).
const SHOW_SOURCES: bool = true;

/// With [`SHOW_SOURCES`]: only list the sources of a number that DIFFERS, to keep a passing run short.
const SOURCES_ONLY_FOR_DIFFERENCES: bool = false;

/// Absolute tolerance for a fraction (crit chance), as in Appendix B. A whole number must match exactly.
const FRACTION_TOLERANCE: f64 = 1e-4;

/// `GetUnitClassId` -> `PlayerClass` (as `{:?}` writes it, which is what `Build` reads).
fn class_name(class_id: u64) -> Option<&'static str> {
    Some(match class_id {
        1 => "Dragonknight",
        2 => "Sorcerer",
        3 => "Nightblade",
        4 => "Warden",
        5 => "Necromancer",
        6 => "Templar",
        117 => "Arcanist",
        _ => return None,
    })
}

struct Fixture {
    file: String,
    name: String,
    patch: String,
    build: Build,
    notes: Vec<String>,
    expected: Vec<(String, f64)>,
    tolerance: BTreeMap<String, f64>,
    hidden: BTreeSet<String>,
}

/// The swaps a character evidently made, from the skill lines of its passives and of the abilities on its bars: a
/// line of another class is one it took (a character cannot hold or slot what it has not got), and an own line with
/// no evidence is one it gave up. They are paired in the class's own line order. Which own line was given up is a
/// guess, but a harmless one: the lines with no passive unlocked and nothing slotted have nothing to give the sheet,
/// and any swap removes Class Mastery. `Ok(vec![])` for a pure class; `Err` when the lines cannot be a legal
/// subclassing (more foreign lines than lines that could have been given up, or than the game allows).
fn infer_subclass(build: &Build, bundle: &Bundle) -> Result<Vec<SubclassSwap>, String> {
    let mut evidence = Vec::new();
    for id in build.passives.iter().chain(&build.front_bar).chain(&build.back_bar) {
        if let Some(line) = bundle.skill_line_of(*id) {
            if !evidence.contains(&line) {
                evidence.push(line);
            }
        }
    }
    let foreign: Vec<_> = evidence
        .iter()
        .copied()
        .filter(|line| PlayerClass::of_skill_line(*line).is_some_and(|class| class != build.class))
        .collect();
    let given_up: Vec<_> =
        build.class.skill_lines().iter().copied().filter(|line| !evidence.contains(line)).collect();
    if foreign.len() > MAX_SUBCLASS_SWAPS.min(given_up.len()) {
        let names: Vec<_> = foreign.iter().map(|line| line.as_str()).collect();
        return Err(format!("{} lines of other classes ({}) do not fit a legal subclassing", foreign.len(), names.join(", ")));
    }
    Ok(given_up.into_iter().zip(foreign).map(|(replaces, with)| SubclassSwap::new(replaces, with)).collect())
}

/// The addon's `build` as a [`Build`], and what had to be added to get there.
fn build_of(root: &Value, bundle: &Bundle) -> Result<(Build, Vec<String>), String> {
    let mut notes = Vec::new();
    let mut build = root
        .get("build")
        .and_then(Value::as_object)
        .cloned()
        .ok_or("the fixture has no \"build\" object")?;

    if !build.contains_key("class") {
        let id = root.pointer("/extra/character/class_id").and_then(Value::as_u64);
        match id.and_then(class_name) {
            Some(name) => {
                build.insert("class".to_string(), Value::from(name));
                notes.push(format!("class {name} taken from extra.character.class_id"));
            }
            None => return Err("build has no class and extra.character.class_id is missing or unknown".to_string()),
        }
    }

    if let Some(Value::Object(gear)) = build.get_mut("gear") {
        for (_slot, piece) in gear.iter_mut() {
            let Some(piece) = piece.as_object_mut() else { continue };
            let level = piece.get("effective_level").cloned();
            let quality = piece.get("quality").cloned();
            if let Some(Value::Object(enchant)) = piece.get_mut("enchant") {
                if !enchant.contains_key("effective_level") {
                    if let Some(level) = level {
                        enchant.insert("effective_level".to_string(), level);
                    }
                }
                if !enchant.contains_key("quality") {
                    if let Some(quality) = quality {
                        enchant.insert("quality".to_string(), quality);
                    }
                }
            }
        }
    }

    let subclass_given = build.contains_key("subclass");
    let mut external: Vec<u32> = build
        .get("external_buffs")
        .and_then(Value::as_array)
        .map(|ids| ids.iter().filter_map(Value::as_u64).map(|id| id as u32).collect())
        .unwrap_or_default();
    let mut unmodelled = Vec::new();
    let observed = root.pointer("/extra/buffs").and_then(Value::as_array).cloned().unwrap_or_default();
    for buff in &observed {
        let Some(id) = buff.get("ability_id").and_then(Value::as_u64).map(|id| id as u32) else { continue };
        let name = buff.get("name").and_then(Value::as_str).unwrap_or("?");
        if buff.get("mundus").and_then(Value::as_bool) == Some(true) {
            continue; // the build's own mundus loadout
        }
        let engine_buff = matches!(bundle.effect(id).map(|effect| &effect.source), Some(EffectSource::Buff { .. }));
        if !engine_buff {
            unmodelled.push(format!("{name} ({id})"));
        } else if MERGE_OBSERVED_BUFFS && !external.contains(&id) {
            external.push(id);
            notes.push(format!("{name} ({id}) was on in game: added to external_buffs"));
        }
    }
    if !unmodelled.is_empty() {
        notes.push(format!("unmodelled, in the game's numbers and not in the engine's: {}", unmodelled.join(", ")));
    }
    build.insert("external_buffs".to_string(), Value::from(external));

    let mut build = serde_json::from_value::<Build>(Value::Object(build)).map_err(|e| format!("build: {e}"))?;
    if !subclass_given {
        match infer_subclass(&build, bundle) {
            Ok(swaps) if swaps.is_empty() => {}
            Ok(swaps) => {
                let listed: Vec<_> =
                    swaps.iter().map(|swap| format!("{} -> {}", swap.replaces.as_str(), swap.with.as_str())).collect();
                notes.push(format!("subclass inferred from the passives and bars (the addon does not export it): {}", listed.join(", ")));
                build.subclass = swaps;
            }
            Err(why) => notes.push(format!("subclass not inferred: {why}")),
        }
    }
    Ok((build, notes))
}

/// `expected.sheet` (the addon) or `expected` itself (Appendix B): the numbers in it, by key.
fn expected_of(root: &Value) -> Result<Vec<(String, f64)>, String> {
    let expected = root
        .get("expected")
        .and_then(Value::as_object)
        .ok_or("the fixture has no \"expected\" object")?;
    let numbers = match expected.get("sheet") {
        Some(Value::Object(sheet)) => sheet,
        _ => expected,
    };
    let numbers: Vec<(String, f64)> = numbers
        .iter()
        .filter_map(|(key, value)| value.as_f64().map(|n| (key.clone(), n)))
        .collect();
    if numbers.is_empty() {
        return Err("expected holds no numbers".to_string());
    }
    Ok(numbers)
}

fn load(path: &Path, bundle: &Bundle) -> Result<Fixture, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("cannot read: {e}"))?;
    let root: Value = serde_json::from_str(&text).map_err(|e| format!("not JSON: {e}"))?;
    let (build, notes) = build_of(&root, bundle)?;
    let text_of = |key: &str| root.get(key).and_then(Value::as_str).unwrap_or("?").to_string();
    let tolerance: BTreeMap<String, f64> = root
        .get("tolerance")
        .and_then(Value::as_object)
        .map(|map| map.iter().filter_map(|(k, v)| v.as_f64().map(|n| (k.clone(), n))).collect())
        .unwrap_or_default();
    let hidden: BTreeSet<String> = root
        .get("hidden")
        .and_then(Value::as_array)
        .map(|keys| keys.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default();
    Ok(Fixture {
        file: path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
        name: text_of("name"),
        patch: text_of("patch"),
        build,
        notes,
        expected: expected_of(&root)?,
        tolerance,
        hidden,
    })
}

/// The engine's number for a key of the fixture's sheet. Weapon and spell damage / critical are one stat in the
/// engine (as in the game: every bonus to one is a bonus to the other), so both spellings read the same field.
fn engine_value(sheet: &StatSheet, key: &str) -> Option<f64> {
    Some(match key {
        "max_health" => sheet.max_health as f64,
        "max_magicka" => sheet.max_magicka as f64,
        "max_stamina" => sheet.max_stamina as f64,
        "power" | "spell_power" | "weapon_power" => sheet.power as f64,
        "crit_rating" => sheet.crit_rating as f64,
        "crit_chance" | "spell_crit_chance" | "weapon_crit_chance" => sheet.crit_chance as f64,
        "crit_damage_pct" => sheet.crit_damage_pct as f64,
        "crit_healing_pct" => sheet.crit_healing_pct as f64,
        "critical_resistance" => sheet.critical_resistance as f64,
        "physical_resistance" => sheet.physical_resistance as f64,
        "spell_resistance" => sheet.spell_resistance as f64,
        "physical_penetration" => sheet.physical_penetration as f64,
        "spell_penetration" => sheet.spell_penetration as f64,
        "health_recovery" => sheet.health_recovery as f64,
        "magicka_recovery" => sheet.magicka_recovery as f64,
        "stamina_recovery" => sheet.stamina_recovery as f64,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Verdict {
    Pass,
    Fail,
    /// Listed in the fixture's `hidden`: shown, not asserted.
    Hidden,
    /// The engine has no stat of this name.
    NotCompared,
}

struct Row {
    key: String,
    game: f64,
    engine: Option<f64>,
    tolerance: f64,
    verdict: Verdict,
}

fn compare(
    expected: &[(String, f64)],
    tolerance: &BTreeMap<String, f64>,
    hidden: &BTreeSet<String>,
    sheet: &StatSheet,
) -> Vec<Row> {
    expected
        .iter()
        .map(|(key, game)| {
            let game = *game;
            let engine = engine_value(sheet, key);
            let tol = tolerance
                .get(key)
                .copied()
                .unwrap_or(if game.fract() == 0.0 { 0.0 } else { FRACTION_TOLERANCE });
            let verdict = match engine {
                None => Verdict::NotCompared,
                Some(_) if hidden.contains(key) => Verdict::Hidden,
                Some(engine) if (engine - game).abs() <= tol => Verdict::Pass,
                Some(_) => Verdict::Fail,
            };
            Row { key: key.clone(), game, engine, tolerance: tol, verdict }
        })
        .collect()
}

fn number(n: f64) -> String {
    if n.fract() == 0.0 { format!("{n:.0}") } else { format!("{n:.6}") }
}

fn signed(n: f64) -> String {
    if n > 0.0 { format!("+{}", number(n)) } else { number(n) }
}

/// The channels the engine reads for a sheet key, and the part of the number that comes from no source (level,
/// attribute points, the game's defaults). Mirrors `stats/{resource,power,critical,armour}.rs`: if a formula there
/// changes, change this too. `None` for a key the engine has no stat for.
fn channels_of(key: &str, build: &Build) -> Option<(Vec<Channel>, String)> {
    use AggKind::{Additive, Multiplicative};
    use ResourceKind::{Health, Magicka, Stamina};
    let level = build.level as i64;
    let attrs = build.attributes;
    let pool = |kind: ResourceKind| {
        let (level_coeff, attr_coeff, points) = match kind {
            Health => (300, 122, attrs.health),
            Magicka => (220, 111, attrs.magicka),
            Stamina => (220, 111, attrs.stamina),
        };
        let base = level_coeff * level + 1000 + attr_coeff * points as i64;
        (
            vec![Channel::Resource(kind, Additive), Channel::Resource(kind, Multiplicative)],
            format!("{base} = {level_coeff} x level {level} + 1000 + {attr_coeff} x {points} attribute points"),
        )
    };
    let recovery = |kind: ResourceKind| {
        (vec![Channel::Recovery(kind, Additive), Channel::Recovery(kind, Multiplicative)], "0".to_string())
    };
    let layered = |channel: fn(Dmg) -> Channel, group: Dmg| (vec![channel(Dmg::All), channel(group)], "0".to_string());
    Some(match key {
        "max_health" => pool(Health),
        "max_magicka" => pool(Magicka),
        "max_stamina" => pool(Stamina),
        "power" | "spell_power" | "weapon_power" => (
            vec![
                Channel::Power(esosim::engine::effect::PowerContext::All, Additive),
                Channel::Power(esosim::engine::effect::PowerContext::All, Multiplicative),
            ],
            format!("{} = 20 x level {level}", 20 * level),
        ),
        "crit_rating" => (vec![Channel::CriticalChance], "0".to_string()),
        "crit_chance" | "spell_crit_chance" | "weapon_crit_chance" => {
            (
                vec![Channel::CriticalChance],
                "base rating 2181; the sources below are added with their fractions, the sum is floored once, and (2181 + that) is divided by 21912 (level 66)"
                    .to_string(),
            )
        }
        "crit_damage_pct" => (vec![Channel::CriticalDamage], "50".to_string()),
        "crit_healing_pct" => (vec![Channel::CriticalHealing], "50".to_string()),
        "physical_resistance" => layered(Channel::Armour, Dmg::Martial),
        "spell_resistance" => layered(Channel::Armour, Dmg::Spell),
        "physical_penetration" => layered(esosim::stats::armour::penetration_channel, Dmg::Martial),
        "spell_penetration" => layered(esosim::stats::armour::penetration_channel, Dmg::Spell),
        "health_recovery" => recovery(Health),
        "magicka_recovery" => recovery(Magicka),
        "stamina_recovery" => recovery(Stamina),
        _ => return None,
    })
}

/// A source as a person would name it: the set and its piece count, the champion star, the buff or passive by name.
fn describe(source: &EffectSource, bundle: &Bundle) -> String {
    let effect = |id: u32| bundle.effect(id).map(|e| e.name.clone()).unwrap_or_else(|| "?".to_string());
    match source {
        EffectSource::Set { set_id, pieces_required } => {
            let name = SETS.get(*set_id).map(|s| s.name.as_str()).unwrap_or("?");
            format!("set {name} [{set_id}], {pieces_required}-piece row")
        }
        EffectSource::ChampionPoint { cp_id } => {
            let name = bundle.stars.get(cp_id).map(|s| s.name.as_str()).unwrap_or("?");
            format!("champion star {name} [{cp_id}]")
        }
        EffectSource::Mundus { ability_id } => format!("mundus {} [{ability_id}]", effect(*ability_id)),
        EffectSource::Race { name, ability_id } => format!("race passive {} [{ability_id}] ({name})", effect(*ability_id)),
        EffectSource::Class { name, ability_id } => format!("class passive {} [{ability_id}] ({name})", effect(*ability_id)),
        EffectSource::Passive { ability_id } => format!("passive {} [{ability_id}]", effect(*ability_id)),
        EffectSource::ActiveSkill { ability_id } => format!("skill {} [{ability_id}]", effect(*ability_id)),
        EffectSource::Buff { ability_id } => format!("buff {} [{ability_id}]", effect(*ability_id)),
        EffectSource::Debuff { ability_id } => format!("debuff {} [{ability_id}]", effect(*ability_id)),
        EffectSource::Food => "food".to_string(),
        EffectSource::Potion => "potion".to_string(),
        EffectSource::Gear => "gear (armour value / weapon power of the pieces)".to_string(),
        EffectSource::GearTrait => "gear traits".to_string(),
        EffectSource::Enchant => "enchants".to_string(),
        EffectSource::CoreCombat => "core combat".to_string(),
    }
}

/// Every source the engine counted for `key`, one line each, in the order the engine gathered them, so the list can
/// be held against the game's tooltip: a source missing from it is under-contributing, one that should not be there
/// is over-contributing.
fn sources_of(key: &str, build: &Build, player: &Player, bundle: &Bundle) -> Option<String> {
    let (channels, base) = channels_of(key, build)?;
    let mut out = format!("      sources of {key} (engine base {base}):\n");
    let (mut additive, mut multiplicative) = (0.0, 0.0);
    let mut any = false;
    for channel in channels {
        let percent = matches!(channel.bucket(), Bucket::Multiplicative);
        for (source, value) in player.explain(channel) {
            any = true;
            let (shown, unit) = if percent { (value * 100.0, "%") } else { (value, "") };
            if percent { multiplicative += shown } else { additive += shown }
            out.push_str(&format!("        {:>+12}{unit:<1}  {}  <{channel:?}>\n", number_signed(shown), describe(&source, bundle)));
        }
    }
    if !any {
        out.push_str("        (no source contributes)\n");
    } else {
        out.push_str(&format!("        total: {} flat, {:+.2}% multiplicative\n", number(additive), multiplicative));
    }
    Some(out)
}

fn number_signed(n: f64) -> String {
    if n.fract() == 0.0 { format!("{n:+.0}") } else { format!("{n:+.4}") }
}

fn report(fixture: &Fixture, rows: &[Row], player: Option<&Player>, bundle: &Bundle) -> String {
    let mut out = format!("{} [{}, patch {}]\n", fixture.name, fixture.file, fixture.patch);
    for note in &fixture.notes {
        out.push_str(&format!("  note: {note}\n"));
    }
    for row in rows {
        let line = match (row.verdict, row.engine) {
            (Verdict::NotCompared, _) | (_, None) => {
                format!("  not compared  {:<22} game {:>12}  (the engine has no such stat)\n", row.key, number(row.game))
            }
            (verdict, Some(engine)) => {
                let tag = match verdict {
                    Verdict::Pass => "ok          ",
                    Verdict::Fail => "DIFFERS     ",
                    _ => "hidden      ",
                };
                format!(
                    "  {tag}  {:<22} game {:>12}  engine {:>12}  diff {:>11}  tolerance {}\n",
                    row.key,
                    number(row.game),
                    number(engine),
                    signed(engine - row.game),
                    number(row.tolerance),
                )
            }
        };
        out.push_str(&line);
        if SHOW_SOURCES && !(SOURCES_ONLY_FOR_DIFFERENCES && row.verdict != Verdict::Fail) {
            if let Some(sources) = player.and_then(|p| sources_of(&row.key, &fixture.build, p, bundle)) {
                out.push_str(&sources);
            }
        }
    }
    out
}

fn fixture_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(FIXTURE_DIR)
        .unwrap_or_else(|e| panic!("{FIXTURE_DIR}: {e}"))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    files.sort();
    files
}

#[test]
fn every_golden_fixture_matches_the_game() {
    let bundle = Bundle::embedded();
    let files = fixture_files();
    assert!(!files.is_empty(), "no fixture in {FIXTURE_DIR}");

    let mut failures = Vec::new();
    for path in &files {
        let file = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let fixture = match load(path, bundle) {
            Ok(fixture) => fixture,
            Err(e) => {
                failures.push(format!("{file}: {e}\n"));
                continue;
            }
        };
        let sheet = match catch_unwind(AssertUnwindSafe(|| fixture.build.sheet(bundle))) {
            Ok(sheet) => sheet,
            Err(_) => {
                failures.push(format!("{file}: Build::sheet panicked\n"));
                continue;
            }
        };
        let rows = compare(&fixture.expected, &fixture.tolerance, &fixture.hidden, &sheet);
        let player = catch_unwind(AssertUnwindSafe(|| fixture.build.to_player(bundle))).ok();
        let text = report(&fixture, &rows, player.as_ref(), bundle);
        println!("{text}");
        if rows.iter().any(|row| row.verdict == Verdict::Fail) {
            failures.push(text);
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} golden fixture(s) differ from the game:\n\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
}

#[cfg(test)]
mod harness {
    use super::*;
    use esosim::data::enums::gear::ItemQuality;

    #[test]
    fn the_adapter_fills_in_what_the_addon_leaves_out() {
        let root: Value = serde_json::from_str(
            r#"{
                "name": "adapter", "patch": "U52",
                "build": {
                    "race": "Khajiit", "level": 50,
                    "attributes": {"health": 32, "magicka": 32, "stamina": 0},
                    "external_buffs": [],
                    "gear": {"chest": {
                        "item_id": 1, "item_type": "Medium", "effective_level": 66, "gear_trait": "ArmorDivines",
                        "quality": "Legendary", "set_id": null, "enchant": {"glyph": "Magicka"}, "name": "extra keys are ignored"
                    }}
                },
                "extra": {
                    "character": {"class_id": 117},
                    "buffs": [
                        {"ability_id": 61665, "name": "Major Brutality"},
                        {"ability_id": 84720, "name": "Eye Scream"},
                        {"ability_id": 13981, "name": "Boon: The Lover", "mundus": true}
                    ]
                },
                "expected": {"sheet": {"max_health": 1}}
            }"#,
        )
        .unwrap();
        let (build, notes) = build_of(&root, Bundle::embedded()).unwrap();
        assert_eq!(build.class, PlayerClass::Arcanist);
        let enchant = build.gear.chest.unwrap().enchant.unwrap();
        assert_eq!((enchant.effective_level, enchant.quality), (66, ItemQuality::Legendary));
        // a buff the engine knows is added, the mundus is the build's own, food is reported and not added
        assert_eq!(build.external_buffs, vec![61665]);
        assert!(notes.iter().any(|n| n.contains("Eye Scream (84720)")), "{notes:?}");
        assert!(!notes.iter().any(|n| n.contains("13981")), "{notes:?}");
    }

    /// An Arcanist whose passives and bars hold Assassination ones has taken that line: the fixture's own build says
    /// nothing about it, and without the swap Pressure Points (an Assassination passive) is dropped.
    #[test]
    fn a_subclassed_character_has_its_swaps_inferred() {
        let bundle = Bundle::embedded();
        let path = Path::new(FIXTURE_DIR).join("arcanist-khajiit-cp2120-front-bar.json");
        let fixture = load(&path, bundle).unwrap();
        let taken: Vec<_> = fixture.build.subclass.iter().map(|swap| swap.with.as_str()).collect();
        assert!(taken.contains(&"Assassination"), "{taken:?} {:?}", fixture.notes);
        assert!(fixture.notes.iter().any(|n| n.starts_with("subclass inferred")), "{:?}", fixture.notes);
        assert_eq!(fixture.build.subclass_issues(bundle), [], "the inferred swaps are a legal subclassing");
    }

    #[test]
    fn a_pure_class_or_a_stated_subclass_is_left_alone() {
        // Pressure Points (45053) is an Assassination passive
        let build_of_json = |class: &str, extra: &str| {
            let root: Value = serde_json::from_str(&format!(
                r#"{{"build": {{"race": "Nord", "class": "{class}", "level": 50,
                    "attributes": {{"health": 0, "magicka": 0, "stamina": 0}}, "passives": [45053] {extra}}}}}"#
            ))
            .unwrap();
            build_of(&root, Bundle::embedded()).unwrap()
        };
        // nothing foreign to a Nightblade, nothing to infer
        let (build, notes) = build_of_json("Nightblade", "");
        assert!(build.subclass.is_empty() && notes.is_empty(), "{notes:?}");
        // an Arcanist with that passive took Assassination, unless the build says otherwise
        let (build, notes) = build_of_json("Arcanist", "");
        assert_eq!(build.subclass.iter().map(|swap| swap.with.as_str()).collect::<Vec<_>>(), ["Assassination"]);
        assert!(notes.iter().any(|n| n.starts_with("subclass inferred")), "{notes:?}");
        let (build, notes) = build_of_json("Arcanist", r#", "subclass": []"#);
        assert!(build.subclass.is_empty() && notes.is_empty(), "{notes:?}");
    }

    #[test]
    fn a_build_without_a_class_anywhere_is_an_error() {
        let root: Value = serde_json::from_str(
            r#"{"build": {"race": "Nord", "level": 50, "attributes": {"health": 0, "magicka": 0, "stamina": 0}}}"#,
        )
        .unwrap();
        assert!(build_of(&root, Bundle::embedded()).unwrap_err().contains("class"));
    }

    #[test]
    fn a_wrong_number_fails_and_hidden_and_unknown_keys_do_not() {
        let sheet = StatSheet { max_health: 100, power: 5, crit_chance: 0.3741, ..Default::default() };
        let expected = vec![
            ("max_health".to_string(), 101.0),
            ("crit_chance".to_string(), 0.37413),
            ("some_unlisted_stat".to_string(), 1320.0),
            ("power".to_string(), 9999.0),
        ];
        let hidden = BTreeSet::from(["power".to_string()]);
        let verdicts: Vec<Verdict> =
            compare(&expected, &BTreeMap::new(), &hidden, &sheet).iter().map(|row| row.verdict).collect();
        assert_eq!(verdicts, [Verdict::Fail, Verdict::Pass, Verdict::NotCompared, Verdict::Hidden]);
    }

    #[test]
    fn a_fixture_tolerance_overrides_the_default() {
        let sheet = StatSheet { max_health: 100, ..Default::default() };
        let expected = vec![("max_health".to_string(), 103.0)];
        let loose = BTreeMap::from([("max_health".to_string(), 5.0)]);
        assert_eq!(compare(&expected, &BTreeMap::new(), &BTreeSet::new(), &sheet)[0].verdict, Verdict::Fail);
        assert_eq!(compare(&expected, &loose, &BTreeSet::new(), &sheet)[0].verdict, Verdict::Pass);
    }

    #[test]
    fn the_flat_shape_of_appendix_b_is_read_too() {
        let root: Value = serde_json::from_str(r#"{"expected": {"max_health": 33456, "crit_chance": 0.4843, "note": "x"}}"#).unwrap();
        let mut keys: Vec<String> = expected_of(&root).unwrap().into_iter().map(|(k, _)| k).collect();
        keys.sort();
        assert_eq!(keys, ["crit_chance", "max_health"]);
    }
}