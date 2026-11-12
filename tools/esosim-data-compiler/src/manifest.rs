//! Reading the game's data straight out of `eso.mnf` (and the DAT archives next to it).
//!
//! The stable identity of a file is its **manifest file id**, the true file index in the MNF
//! (`MnfFileEntry::true_index`). The number in an exported file name changes every patch; these
//! do not. Nothing is written to disk and nothing is searched for by content.
//!
//! | file                 | id           |
//! |----------------------|--------------|
//! | ability (skill) data | `0x60000000` |
//! | item table           | `0x60000004` |
//! | set table            | `0x60000016` |
//! | champion point table | `0x600001AB` |
//! | lang                 | `0x01000014` |
//!
//! Needs the DAT archives next to the MNF and, for version 3 archives, the Oodle library.

use std::path::Path;

use eso_common::lang::EsoLangFile;
use eso_common::mnf::{MnfFile, MnfFileEntry};
use eso_skill_data::champion_points::{ChampionPointData, load_champion_point_data};
use eso_skill_data::{SkillData34, load_skill_data};

use crate::corpus::LangTexts;
use crate::items;

pub const ABILITY_MANIFEST_ID: u32 = 0x6000_0000;
pub const ITEM_MANIFEST_ID: u32 = 0x6000_0004;
pub const SET_MANIFEST_ID: u32 = 0x6000_0016;
pub const CP_MANIFEST_ID: u32 = 0x6000_01AB;
pub const LANG_MANIFEST_ID: u32 = 0x0100_0014;

/// Raw game data, decoded but not yet interpreted.
pub struct GameData {
    pub skills: Vec<SkillData34>,
    /// The set table in its container layout (what the old `*_Uncompressed.EsoFileData` held).
    pub set_table: Vec<u8>,
    /// Every record of the champion point table, in file order.
    pub cp: Vec<ChampionPointData>,
    pub lang: LangTexts,
    /// `item_types.csv`, already decoded from the item table (roadmap 2.8).
    pub item_types_csv: String,
}

fn entry_for(mnf: &MnfFile, id: u32, what: &str) -> Result<MnfFileEntry, String> {
    let hits: Vec<&MnfFileEntry> = mnf.file_table().iter().filter(|e| e.true_index() == id).collect();
    match hits.as_slice() {
        [e] => Ok(**e),
        [] => Err(format!("no file with manifest id 0x{id:X} ({what}) in the MNF")),
        many => {
            Ok(**many.last().unwrap())
        }
    }
}

/// Read the manifest at `mnf_path` and decode the ability table, the set table and the lang.
pub fn read(mnf_path: &Path) -> Result<GameData, String> {
    let oodle = eso_common::oodle::load_oodle_lib();
    let hint = if oodle { "" } else { " (Oodle library not found)" };

    let path = mnf_path.to_str().ok_or("MNF path is not valid UTF-8")?;
    let mut mnf = MnfFile::new();
    mnf.load(path).map_err(|e| format!("loading '{path}': {e:?}"))?;

    // EsoFileData tables, in the same combined layout the exporter wrote.
    let table = |id: u32, what: &str| -> Result<Vec<u8>, String> {
        let entry = entry_for(&mnf, id, what)?;
        let data = mnf
            .read_eso_file_data(&entry)
            .map_err(|e| format!("reading manifest id 0x{id:X} ({what}): {e:?}{hint}"))?;
        Ok(data.to_combined_bytes())
    };

    let abilities = table(ABILITY_MANIFEST_ID, "abilities")?;
    let set_table = table(SET_MANIFEST_ID, "sets")?;
    let cp_table = table(CP_MANIFEST_ID, "champion points")?;
    let item_table = table(ITEM_MANIFEST_ID, "items")?;
    let item_types_csv = items::item_types_from_table(&item_table)
        .map_err(|e| format!("manifest id 0x{ITEM_MANIFEST_ID:X} is not a valid item table: {e}"))?;

    let lang_entry = entry_for(&mnf, LANG_MANIFEST_ID, "lang")?;
    let lang_bytes = mnf
        .read_entry(&lang_entry, None)
        .map_err(|e| format!("reading manifest id 0x{LANG_MANIFEST_ID:X} (lang): {e:?}{hint}"))?;
    let mut lang = EsoLangFile::new();
    lang.parse_data(&lang_bytes)
        .map_err(|e| format!("parsing manifest id 0x{LANG_MANIFEST_ID:X} (lang): {e:?}"))?;

    Ok(GameData {
        skills: parse_abilities(&abilities)?,
        set_table,
        cp: parse_champion_points(&cp_table)?,
        lang: LangTexts::from_lang(&lang),
        item_types_csv,
    })
}

/// Run `load` on `bytes` written to a temporary file that is removed again before this returns.
///
/// `eso_skill_data` only offers path-based loaders, so this is the one place that needs replacing
/// when the parser can read from memory.
fn with_temp_file<T>(
    bytes: &[u8],
    what: &str,
    load: impl FnOnce(&str) -> Result<T, String>,
) -> Result<T, String> {
    struct Temp(std::path::PathBuf);
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp = Temp(std::env::temp_dir().join(format!(
        "esosim-{what}-{}-{nonce}_Uncompressed.EsoFileData",
        std::process::id()
    )));
    std::fs::write(&tmp.0, bytes).map_err(|e| format!("writing '{}': {e}", tmp.0.display()))?;
    let path = tmp.0.to_str().ok_or("temporary path is not valid UTF-8")?;
    load(path)
}

/// Decode the ability table.
fn parse_abilities(bytes: &[u8]) -> Result<Vec<SkillData34>, String> {
    with_temp_file(bytes, "abilities", |path| {
        let state = load_skill_data(path)
            .map_err(|e| format!("manifest id 0x{ABILITY_MANIFEST_ID:X} is not a valid ability table: {e}"))?;
        Ok(state.skills)
    })
}

/// Decode the champion point table. The parser only prints a warning when the number of records
/// it read differs from the file's header; that is an error here, because it means records were
/// skipped or misread and the values cannot be trusted.
fn parse_champion_points(bytes: &[u8]) -> Result<Vec<ChampionPointData>, String> {
    with_temp_file(bytes, "champion-points", |path| {
        let state = load_champion_point_data(path)
            .map_err(|e| format!("manifest id 0x{CP_MANIFEST_ID:X} is not a valid champion point table: {e}"))?;
        eprintln!(
            "champion points: {} bytes, header says {} record(s), parser read {}",
            state.size_of_file,
            state.num_records_header,
            state.champion_points.len()
        );
        if state.champion_points.len() != state.num_records_header as usize {
            return Err(format!(
                "champion point table (manifest id 0x{CP_MANIFEST_ID:X}): header says {} records, the parser read {}",
                state.num_records_header,
                state.champion_points.len()
            ));
        }
        Ok(state.champion_points)
    })
}
