use phf::{Map, phf_map};
use crate::data::{StatBuff as Buff, critical_damage::{HARPOONERS_WADING_KILT_ID, MORA_SCRIBES_THESIS_ID}, major_minor::*};

pub static SAVAGERY_MAJOR: Buff = Buff { id: SAVAGERY_MAJOR_ID, value: 2629f64, value_per_stack: 0f64};
pub static SAVAGERY_MINOR: Buff = Buff { id: SAVAGERY_MINOR_ID, value: 1314f64, value_per_stack: 0f64};
pub static PROPHECY_MAJOR: Buff = Buff { id: PROPHECY_MAJOR_ID, value: 2629f64, value_per_stack: 0f64};
pub static PROPHECY_MINOR: Buff = Buff { id: PROPHECY_MINOR_ID, value: 1314f64, value_per_stack: 0f64};
pub static THIEF_MUNDUS: Buff = Buff { id: 13975, value: 1212f64, value_per_stack: 0f64};
pub static PRODIGY: Buff = Buff { id: 45561, value: 0f64, value_per_stack: 219f64};
pub static PRECISION: Buff = Buff { id: 141898, value: 0f64, value_per_stack: 160f64}; // CP
pub static ACCURACY: Buff = Buff { id: 45492, value: 1314f64, value_per_stack: 0f64};
pub static AURIELS_BOW_ACCURACY: Buff = Buff { id: 118506, value: 1314f64, value_per_stack: 0f64};
pub static CONCENTRATION: Buff = Buff { id: 45562, value: 0f64, value_per_stack: 939f64};

pub static BESERKING_WARRIOR: Buff = Buff { id: 50978, value: 0f64, value_per_stack: 298f64};
// https://en.uesp.net/wiki/Online:Brawling_Advantage
// https://en.uesp.net/wiki/Online:Claw_of_the_Forest_Wraith
// https://en.uesp.net/wiki/Online:Death-Dancer
// https://en.uesp.net/wiki/Online:Dragonguard_Elite
// https://en.uesp.net/wiki/Online:Eagle_Eye
pub static HARPOONERS_WADING_KILT: Buff = Buff { id: HARPOONERS_WADING_KILT_ID, value: 0f64, value_per_stack: 110f64};
pub static HIGHLAND_SENTINEL: Buff = Buff { id: 219681, value: 0.0f64, value_per_stack: 443f64};
// https://en.uesp.net/wiki/Online:Lethal_Sorcery
pub static MASTER_ASSASSIN: Buff = Buff { id: 45038, value: 1448f64, value_per_stack: 0f64};
pub static MECHANICAL_ACUITY: Buff = Buff { id: 99204, value: 0f64, value_per_stack: 20f64};
pub static MORA_SCRIBES_THESIS: Buff = Buff { id: MORA_SCRIBES_THESIS_ID, value: 0f64, value_per_stack: 128f64};
// https://en.uesp.net/wiki/Online:Mora's_Whispers
pub static PRESSURE_POINTS: Buff = Buff { id: 45053, value: 0f64, value_per_stack: 548f64};
pub static SUL_XAN_SOULBOUND: Buff = Buff { id: 154737, value: 2160f64, value_per_stack: 0f64};
// https://en.uesp.net/wiki/Online:True-Sworn_Fury
pub static TWIN_BLADE_AND_BLUNT_DAGGER: Buff = Buff { id: 45482, value: 0f64, value_per_stack: 657f64};
// https://en.uesp.net/wiki/Online:Tzogvin's_Warband
pub static UNCERTAINTY_MINOR: Buff = Buff { id: UNCERTAINTY_MINOR_ID, value: -1314f64, value_per_stack: 0f64};
pub static VINEDUSK_TRAINING: Buff = Buff { id: 45494, value: 1314f64, value_per_stack: 0f64};
pub static VOLENDRUNGS_POTENCY: Buff = Buff { id: 117965, value: 1314f64, value_per_stack: 0f64};

pub static SKORMONGONDARS_SAVAGERY: Buff = Buff { id: 139696, value: 5478f64, value_per_stack: 0f64}; // 25% equivalent

pub static DEATH_KNELL: Buff = Buff { id: 116198, value: 4382f64, value_per_stack: 0f64};

pub static CRITICAL_CHANCE_BOTH: Map<u32, &'static Buff> = phf_map! {
    13975 => &THIEF_MUNDUS,
    45561 => &PRODIGY,
    141898 => &PRECISION,
    45492 => &ACCURACY,
    118506 => &AURIELS_BOW_ACCURACY,
    117965 => &VOLENDRUNGS_POTENCY,
    139696 => &SKORMONGONDARS_SAVAGERY,
    45562 => &CONCENTRATION,

    50978 => &BESERKING_WARRIOR,
    155150 => &HARPOONERS_WADING_KILT,
    219681 => &HIGHLAND_SENTINEL,
    45038 => &MASTER_ASSASSIN,
    99204 => &MECHANICAL_ACUITY,
    220315 => &MORA_SCRIBES_THESIS,
    45053 => &PRESSURE_POINTS,
    154737 => &SUL_XAN_SOULBOUND,
    45482 => &TWIN_BLADE_AND_BLUNT_DAGGER,
    79895 => &UNCERTAINTY_MINOR,
    45494 => &VINEDUSK_TRAINING,
    // 116198 => &DEATH_KNELL,
};

pub static CRITICAL_CHANCE_SPELL: Map<u32, &'static Buff> = phf_map! {
    61689 => &PROPHECY_MAJOR,
    61691 => &PROPHECY_MINOR,
};

pub static CRITICAL_CHANCE_WEAPON: Map<u32, &'static Buff> = phf_map! {
    61667 => &SAVAGERY_MAJOR,
    61666 => &SAVAGERY_MINOR,
};
