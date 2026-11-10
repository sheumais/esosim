//! The ten playable races. Only the name lives here; what a race gives (racial passives) is data,
//! see roadmap 2.4.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
    Altmer,
    Argonian,
    Bosmer,
    Breton,
    Dunmer,
    Imperial,
    Khajiit,
    Nord,
    Orc,
    Redguard,
}
