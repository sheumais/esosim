//! Reading the game's data straight out of `eso.mnf` (and the DAT archives next to it).
//!
//! The stable identity of a file is its **manifest file id**, the true file index in the MNF
//! (`MnfFileEntry::true_index`). The number in an exported file name changes every patch; these
//! do not. Nothing is written to disk and nothing is searched for by content.
//!
//! | file                 | id           |
//! |----------------------|--------------|
//! | ability (skill) data | `0x0`        |
//! | set table            | `0x16`       |
//! | lang                 | `0x01000014` |
//!
//! Needs the DAT archives next to the MNF and, for version 3 archives, the Oodle library.

use std::path::Path;

use eso_common::lang::EsoLangFile;
use eso_common::mnf::{MnfFile, MnfFileEntry};
use eso_skill_data::{SkillData34, load_skill_data};

use crate::corpus::LangTexts;

pub const ABILITY_MANIFEST_ID: u32 = 0x6000_0000;
pub const SET_MANIFEST_ID: u32 = 0x6000_0016;
pub const LANG_MANIFEST_ID: u32 = 0x0100_0014;

/// Raw game data, decoded but not yet interpreted.
pub struct GameData {
    pub skills: Vec<SkillData34>,
    /// The set table in its container layout (what the old `*_Uncompressed.EsoFileData` held).
    pub set_table: Vec<u8>,
    pub lang: LangTexts,
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
        lang: LangTexts::from_lang(&lang),
    })
}

/// Decode the ability table.
///
/// `eso_skill_data` only offers a path-based loader (`load_skill_data`), so the bytes take a
/// short trip through a temporary file that is removed again before this returns. This is the
/// one place that needs replacing when the parser can read from memory.
fn parse_abilities(bytes: &[u8]) -> Result<Vec<SkillData34>, String> {
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
        "esosim-abilities-{}-{nonce}_Uncompressed.EsoFileData",
        std::process::id()
    )));
    std::fs::write(&tmp.0, bytes).map_err(|e| format!("writing '{}': {e}", tmp.0.display()))?;
    let path = tmp.0.to_str().ok_or("temporary path is not valid UTF-8")?;
    let state = load_skill_data(path)
        .map_err(|e| format!("manifest id 0x{ABILITY_MANIFEST_ID:X} is not a valid ability table: {e}"))?;
    Ok(state.skills)
}
