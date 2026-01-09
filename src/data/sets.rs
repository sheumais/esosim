use crate::models::player::ActiveSet;

pub enum SetBonusType {
    Health(Option<u32>),
    Stamina(Option<u32>),
    Magicka(Option<u32>),
    HealingTaken(Option<u32>),
    HealingDone(Option<u32>),
    Power(Option<u32>),
    Armour(Option<u32>),
    HealthRecovery(Option<u32>),
    StaminaRecovery(Option<u32>),
    MagickaRecovery(Option<u32>),
    CriticalChance(Option<u32>),
    MinorAegis,
    MinorSlayer,
    Penetration(Option<u32>),
    ReducePlayerDamageTaken(Option<u32>),
    CriticalResistance(Option<u32>),
}

impl SetBonusType {
    fn get_value(&self) -> u32 {
        match self {
            SetBonusType::Health(o) => o.unwrap_or(SET_HEALTH_DEFAULT),
            SetBonusType::Stamina(o) => o.unwrap_or(SET_STAMINA_DEFAULT),
            SetBonusType::Magicka(o) => o.unwrap_or(SET_MAGICKA_DEFAULT),
            SetBonusType::HealingTaken(o) => o.unwrap_or(SET_HEALING_TAKEN_DEFAULT),
            SetBonusType::HealingDone(o) => o.unwrap_or(SET_HEALING_DONE_DEFAULT),
            SetBonusType::Power(o) => o.unwrap_or(SET_POWER_DEFAULT),
            SetBonusType::Armour(o) => o.unwrap_or(SET_ARMOUR_DEFAULT),
            SetBonusType::HealthRecovery(o) => o.unwrap_or(SET_HEALTH_RECOVERY_DEFAULT),
            SetBonusType::StaminaRecovery(o) => o.unwrap_or(SET_STAMINA_RECOVERY_DEFAULT),
            SetBonusType::MagickaRecovery(o) => o.unwrap_or(SET_MAGICKA_RECOVERY_DEFAULT),
            SetBonusType::CriticalChance(o) => o.unwrap_or(SET_CRITICAL_CHANCE_DEFAULT),
            SetBonusType::MinorAegis => SET_MINOR_AEGIS_DEFAULT,
            SetBonusType::MinorSlayer => SET_MINOR_SLAYER_DEFAULT,
            SetBonusType::Penetration(o) => o.unwrap_or(SET_PENETRATION_DEFAULT),
            SetBonusType::ReducePlayerDamageTaken(o) => {
                o.unwrap_or(SET_REDUCE_PLAYER_DAMAGE_TAKEN_DEFAULT)
            }
            SetBonusType::CriticalResistance(o) => {
                o.unwrap_or(SET_CRITICAL_RESISTANCE_DEFAULT)
            }
        }
    }

    fn same_kind(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub struct Set {
    bonuses: &'static [&'static [SetBonusType]],
}

pub fn get_total_bonus(set_details: &ActiveSet, bonus: &SetBonusType) -> u32 {
    if let Some(set_bonuses) = SET_BONUSES.get(&set_details.set_id) {
        return set_bonuses
            .bonuses
            .iter()
            .take(set_details.count as usize)
            .flat_map(|group| group.iter())
            .filter(|s| s.same_kind(bonus))
            .map(|b| b.get_value())
            .sum();
    }
    0
}

// todo: fix scaling
pub const SET_HEALTH_DEFAULT: u32 = 1206;
pub const SET_STAMINA_DEFAULT: u32 = 1096;
pub const SET_MAGICKA_DEFAULT: u32 = 1096;
pub const SET_POWER_DEFAULT: u32 = 129;
pub const SET_ARMOUR_DEFAULT: u32 = 1487;
pub const SET_CRITICAL_CHANCE_DEFAULT: u32 = 657;
pub const SET_PENETRATION_DEFAULT: u32 = 1487;
pub const SET_CRITICAL_RESISTANCE_DEFAULT: u32 = 424;
pub const SET_REDUCE_PLAYER_DAMAGE_TAKEN_DEFAULT: u32 = 3;
pub const SET_MINOR_AEGIS_DEFAULT: u32 = 1;
pub const SET_MINOR_SLAYER_DEFAULT: u32 = 1;
pub const SET_MAGICKA_RECOVERY_DEFAULT: u32 = 129;
pub const SET_STAMINA_RECOVERY_DEFAULT: u32 = 129;
pub const SET_HEALTH_RECOVERY_DEFAULT: u32 = 129;
pub const SET_HEALING_DONE_DEFAULT: u32 = 4;
pub const SET_HEALING_TAKEN_DEFAULT: u32 = 4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reawakened_hierophant_magicka_bonus_is_summed_correctly() {
        let active_set = ActiveSet {
            set_id: 722,
            count: 5,
        };

        let bonus = SetBonusType::Magicka(None);

        let total = get_total_bonus(&active_set, &bonus);

        assert_eq!(total, 731 * 3);
    }
}

// Big thank you to UESP!
// https://esoitem.uesp.net/viewlog.php?record=setSummary

pub static SET_BONUSES: phf::Map<u32, &'static Set> = phf::phf_map! {
    19 => &VESTMENTS_OF_THE_WARLOCK,
    20 => &WITCHMAN_ARMOR,
    21 => &AKAVIRI_DRAGONGUARD,
    22 => &DREAMERS_MANTLE,
    23 => &ARCHERS_MIND,
    24 => &FOOTMANS_FORTUNE,
    25 => &DESERT_ROSE,
    26 => &PRISONERS_RAGS,
    27 => &FIORDS_LEGACY,
    28 => &BARKSKIN,
    29 => &SERGEANTS_MAIL,
    30 => &THUNDERBUGS_CARAPACE,
    31 => &SILKS_OF_THE_SUN,
    32 => &HEALERS_HABIT,
    33 => &VIPERS_STING,
    34 => &NIGHT_MOTHERS_EMBRACE,
    35 => &KNIGHTMARE,
    36 => &ARMOR_OF_THE_VEILED_HERITANCE,
    37 => &DEATHS_WIND,
    38 => &TWILIGHTS_EMBRACE,
    39 => &ALESSIAN_ORDER,
    40 => &NIGHTS_SILENCE,
    41 => &WHITESTRAKES_RETRIBUTION,
    43 => &ARMOR_OF_THE_SEDUCER,
    44 => &VAMPIRES_KISS,
    46 => &NOBLE_DUELISTS_SILKS,
    47 => &ROBES_OF_THE_WITHERED_HAND,
    48 => &MAGNUS_GIFT,
    49 => &SHADOW_OF_THE_RED_MOUNTAIN,
    50 => &THE_MORAG_TONG,
    51 => &NIGHT_MOTHERS_GAZE,
    52 => &BECKONING_STEEL,
    53 => &THE_ICE_FURNACE,
    54 => &ASHEN_GRIP,
    55 => &PRAYER_SHAWL,
    56 => &STENDARRS_EMBRACE,
    57 => &SYRABANES_GRIP,
    58 => &HIDE_OF_THE_WEREWOLF,
    59 => &KYNES_KISS,
    60 => &DARKSTRIDE,
    61 => &DREUGH_KING_SLAYER,
    62 => &HATCHLINGS_SHELL,
    63 => &THE_JUGGERNAUT,
    64 => &SHADOW_DANCERS_RAIMENT,
    65 => &BLOODTHORNS_TOUCH,
    66 => &ROBES_OF_THE_HIST,
    67 => &SHADOW_WALKER,
    68 => &STYGIAN,
    69 => &RANGERS_GAIT,
    70 => &SEVENTH_LEGION_BRUTE,
    71 => &DUROKS_BANE,
    72 => &NIKULAS_HEAVY_ARMOR,
    73 => &OBLIVIONS_FOE,
    74 => &SPECTRES_EYE,
    75 => &TORUGS_PACT,
    76 => &ROBES_OF_ALTERATION_MASTERY,
    77 => &CRUSADER,
    78 => &HIST_BARK,
    79 => &WILLOWS_PATH,
    80 => &HUNDINGS_RAGE,
    81 => &SONG_OF_LAMAE,
    82 => &ALESSIAS_BULWARK,
    83 => &ELF_BANE,
    84 => &ORGNUMS_SCALES,
    85 => &ALMALEXIAS_MERCY,
    86 => &QUEENS_ELEGANCE,
    87 => &EYES_OF_MARA,
    88 => &ROBES_OF_DESTRUCTION_MASTERY,
    89 => &SENTRY,
    90 => &SENCHES_BITE,
    91 => &OBLIVIONS_EDGE,
    92 => &KAGRENACS_HOPE,
    93 => &STORM_KNIGHTS_PLATE,
    94 => &MERIDIAS_BLESSED_ARMOR,
    95 => &SHALIDORS_CURSE,
    96 => &ARMOR_OF_TRUTH,
    97 => &THE_ARCH_MAGE,
    98 => &NECROPOTENCE,
    99 => &SALVATION,
    100 => &HAWKS_EYE,
    101 => &AFFLICTION,
    102 => &DUNERIPPERS_SCALES,
    103 => &MAGICKA_FURNACE,
    104 => &CURSE_EATER,
    105 => &TWIN_SISTERS,
    106 => &WILDERQUEENS_ARCH,
    107 => &WYRD_TREES_BLESSING,
    108 => &RAVAGER,
    109 => &LIGHT_OF_CYRODIIL,
    110 => &SANCTUARY,
    111 => &WARD_OF_CYRODIIL,
    112 => &NIGHT_TERROR,
    113 => &CREST_OF_CYRODIIL,
    114 => &SOULSHINE,
    116 => &THE_DESTRUCTION_SUITE,
    117 => &RELICS_OF_THE_PHYSICIAN_ANSUR,
    118 => &TREASURES_OF_THE_EARTHFORGE,
    119 => &RELICS_OF_THE_REBELLION,
    120 => &ARMS_OF_INFERNACE,
    121 => &ARMS_OF_THE_ANCESTORS,
    122 => &EBON_ARMORY,
    123 => &HIRCINES_VENEER,
    124 => &THE_WORMS_RAIMENT,
    125 => &WRATH_OF_THE_IMPERIUM,
    126 => &GRACE_OF_THE_ANCIENTS,
    127 => &DEADLY_STRIKE,
    128 => &BLESSING_OF_THE_POTENTATES,
    129 => &VENGEANCE_LEECH,
    130 => &EAGLE_EYE,
    131 => &BASTION_OF_THE_HEARTLAND,
    132 => &SHIELD_OF_THE_VALIANT,
    133 => &BUFFER_OF_THE_SWIFT,
    134 => &SHROUD_OF_THE_LICH,
    135 => &DRAUGRS_HERITAGE,
    136 => &IMMORTAL_WARRIOR,
    137 => &BERSERKING_WARRIOR,
    138 => &DEFENDING_WARRIOR,
    139 => &WISE_MAGE,
    140 => &DESTRUCTIVE_MAGE,
    141 => &HEALING_MAGE,
    142 => &QUICK_SERPENT,
    143 => &POISONOUS_SERPENT,
    144 => &TWICE_FANGED_SERPENT,
    145 => &WAY_OF_FIRE,
    146 => &WAY_OF_AIR,
    147 => &WAY_OF_MARTIAL_KNOWLEDGE,
    148 => &WAY_OF_THE_ARENA,
    155 => &UNDAUNTED_BASTION,
    156 => &UNDAUNTED_INFILTRATOR,
    157 => &UNDAUNTED_UNWEAVER,
    158 => &EMBERSHIELD,
    159 => &SUNDERFLAME,
    160 => &BURNING_SPELLWEAVE,
    161 => &TWICE_BORN_STAR,
    162 => &SPAWN_OF_MEPHALA,
    163 => &BLOODSPAWN,
    164 => &LORD_WARDEN,
    165 => &SCOURGE_HARVESTER,
    166 => &ENGINE_GUARDIAN,
    167 => &NIGHTFLAME,
    168 => &NERIENETH,
    169 => &VALKYN_SKORIA,
    170 => &MAW_OF_THE_INFERNAL,
    171 => &ETERNAL_WARRIOR,
    172 => &INFALLIBLE_MAGE,
    173 => &VICIOUS_SERPENT,
    176 => &NOBLES_CONQUEST,
    177 => &REDISTRIBUTOR,
    178 => &ARMOR_MASTER,
    179 => &BLACK_ROSE,
    180 => &POWERFUL_ASSAULT,
    181 => &MERITORIOUS_SERVICE,
    183 => &MOLAG_KENA,
    184 => &BRANDS_OF_IMPERIUM,
    185 => &SPELL_POWER_CURE,
    186 => &JOLTING_ARMS,
    187 => &SWAMP_RAIDER,
    188 => &STORM_MASTER,
    190 => &SCATHING_MAGE,
    193 => &OVERWHELMING_SURGE,
    194 => &COMBAT_PHYSICIAN,
    195 => &SHEER_VENOM,
    196 => &LEECHING_PLATE,
    197 => &TORMENTOR,
    198 => &ESSENCE_THIEF,
    199 => &SHIELD_BREAKER,
    200 => &PHOENIX,
    201 => &REACTIVE_ARMOR,
    204 => &ENDURANCE,
    205 => &WILLPOWER,
    206 => &AGILITY,
    207 => &LAW_OF_JULIANOS,
    208 => &TRIAL_BY_FIRE,
    209 => &ARMOR_OF_THE_CODE,
    210 => &MARK_OF_THE_PARIAH,
    211 => &PERMAFROST,
    212 => &BRIARHEART,
    213 => &GLORIOUS_DEFENDER,
    214 => &PARA_BELLUM,
    215 => &ELEMENTAL_SUCCESSION,
    216 => &HUNT_LEADER,
    217 => &WINTERBORN,
    218 => &TRINIMACS_VALOR,
    219 => &MORKULDIN,
    224 => &TAVAS_FAVOR,
    225 => &CLEVER_ALCHEMIST,
    226 => &ETERNAL_HUNT,
    227 => &BAHRAHAS_CURSE,
    228 => &SYVARRAS_SCALES,
    229 => &TWILIGHT_REMEDY,
    230 => &MOONDANCER,
    231 => &LUNAR_BASTION,
    232 => &ROAR_OF_ALKOSH,
    234 => &MARKSMANS_CREST,
    235 => &ROBES_OF_TRANSMUTATION,
    236 => &VICIOUS_DEATH,
    237 => &LEKIS_FOCUS,
    238 => &FASALLAS_GUILE,
    239 => &WARRIORS_FURY,
    240 => &KVATCH_GLADIATOR,
    241 => &VARENS_LEGACY,
    242 => &PELINALS_WRATH,
    243 => &HIDE_OF_MORIHAUS,
    244 => &FLANKING_STRATEGIST,
    245 => &SITHIS_TOUCH,
    246 => &GALERIONS_REVENGE,
    247 => &VICECANON_OF_VENOM,
    248 => &THEWS_OF_THE_HARBINGER,
    253 => &IMPERIAL_PHYSIQUE,
    256 => &MIGHTY_CHUDAN,
    257 => &VELIDRETH,
    258 => &AMBER_PLASM,
    259 => &HEEM_JAS_RETRIBUTION,
    260 => &ASPECT_OF_MAZZATUN,
    261 => &GOSSAMER,
    262 => &WIDOWMAKER,
    263 => &HAND_OF_MEPHALA,
    264 => &GIANT_SPIDER,
    265 => &SHADOWREND,
    266 => &KRAGH,
    267 => &SWARM_MOTHER,
    268 => &SENTINEL_OF_RKUGAMZ,
    269 => &CHOKETHORN,
    270 => &SLIMECRAW,
    271 => &SELLISTRIX,
    272 => &INFERNAL_GUARDIAN,
    273 => &ILAMBRIS,
    274 => &ICEHEART,
    275 => &STORMFIST,
    276 => &TREMORSCALE,
    277 => &PIRATE_SKELETON,
    278 => &THE_TROLL_KING,
    279 => &SELENE,
    280 => &GROTHDARR,
    281 => &ARMOR_OF_THE_TRAINEE,
    282 => &VAMPIRE_CLOAK,
    283 => &SWORD_SINGER,
    284 => &ORDER_OF_DIAGNA,
    285 => &VAMPIRE_LORD,
    286 => &SPRIGGANS_THORNS,
    287 => &GREEN_PACT,
    288 => &BEEKEEPERS_GEAR,
    289 => &SPINNERS_GARMENTS,
    290 => &SKOOMA_SMUGGLER,
    291 => &SHALK_EXOSKELETON,
    292 => &MOTHERS_SORROW,
    293 => &PLAGUE_DOCTOR,
    294 => &YSGRAMORS_BIRTHRIGHT,
    295 => &JAILBREAKER,
    296 => &SPELUNKER,
    297 => &SPIDER_CULTIST_COWL,
    298 => &LIGHT_SPEAKER,
    299 => &TOOTHROW,
    300 => &NETCHS_TOUCH,
    301 => &STRENGTH_OF_THE_AUTOMATON,
    302 => &LEVIATHAN,
    303 => &LAMIAS_SONG,
    304 => &MEDUSA,
    305 => &TREASURE_HUNTER,
    307 => &DRAUGR_HULK,
    308 => &BONE_PIRATES_TATTERS,
    309 => &KNIGHT_ERRANTS_MAIL,
    310 => &SWORD_DANCER,
    311 => &RATTLECAGE,
    313 => &TITANIC_CLEAVE,
    314 => &PUNCTURING_REMEDY,
    315 => &STINGING_SLASHES,
    316 => &CAUSTIC_ARROW,
    317 => &DESTRUCTIVE_IMPACT,
    318 => &GRAND_REJUVENATION,
    320 => &WAR_MAIDEN,
    321 => &DEFILER,
    322 => &WARRIOR_POET,
    323 => &ASSASSINS_GUILE,
    324 => &DAEDRIC_TRICKERY,
    325 => &SHACKLEBREAKER,
    326 => &VANGUARDS_CHALLENGE,
    327 => &COWARDS_GEAR,
    328 => &KNIGHT_SLAYER,
    329 => &WIZARDS_RIPOSTE,
    330 => &AUTOMATED_DEFENSE,
    331 => &WAR_MACHINE,
    332 => &MASTER_ARCHITECT,
    333 => &INVENTORS_GUARD,
    334 => &IMPREGNABLE_ARMOR,
    335 => &DRAUGRS_REST,
    336 => &PILLAR_OF_NIRN,
    337 => &IRONBLOOD,
    338 => &FLAME_BLOSSOM,
    339 => &BLOODDRINKER,
    340 => &HAGRAVENS_GARDEN,
    341 => &EARTHGORE,
    342 => &DOMIHAUS,
    343 => &CALUURIONS_LEGACY,
    344 => &TRAPPINGS_OF_INVIGORATION,
    345 => &ULFNORS_FAVOR,
    346 => &JORVULDS_GUIDANCE,
    347 => &PLAGUE_SLINGER,
    348 => &CURSE_OF_DOYLEMISH,
    349 => &THURVOKUN,
    350 => &ZAAN,
    351 => &INNATE_AXIOM,
    352 => &FORTIFIED_BRASS,
    353 => &MECHANICAL_ACUITY,
    354 => &MAD_TINKERER,
    355 => &UNFATHOMABLE_DARKNESS,
    356 => &LIVEWIRE,
    357 => &PERFECTED_DISCIPLINED_SLASH,
    358 => &PERFECTED_DEFENSIVE_POSITION,
    359 => &PERFECTED_CHAOTIC_WHIRLWIND,
    360 => &PERFECTED_PIERCING_SPRAY,
    361 => &PERFECTED_CONCENTRATED_FORCE,
    362 => &PERFECTED_TIMELESS_BLESSING,
    363 => &DISCIPLINED_SLASH,
    364 => &DEFENSIVE_POSITION,
    365 => &CHAOTIC_WHIRLWIND,
    366 => &PIERCING_SPRAY,
    367 => &CONCENTRATED_FORCE,
    368 => &TIMELESS_BLESSING,
    369 => &MERCILESS_CHARGE,
    370 => &RAMPAGING_SLASH,
    371 => &CRUEL_FLURRY,
    372 => &THUNDEROUS_VOLLEY,
    373 => &CRUSHING_WALL,
    374 => &PRECISE_REGENERATION,
    380 => &PROPHETS,
    381 => &BROKEN_SOUL,
    382 => &GRACE_OF_GLOOM,
    383 => &GRYPHONS_FEROCITY,
    384 => &WISDOM_OF_VANUS,
    385 => &ADEPT_RIDER,
    386 => &SLOADS_SEMBLANCE,
    387 => &NOCTURNALS_FAVOR,
    388 => &AEGIS_OF_GALENWE,
    389 => &ARMS_OF_RELEQUEN,
    390 => &MANTLE_OF_SIRORIA,
    391 => &VESTMENT_OF_OLORIME,
    392 => &PERFECTED_AEGIS_OF_GALENWE,
    393 => &PERFECTED_ARMS_OF_RELEQUEN,
    394 => &PERFECTED_MANTLE_OF_SIRORIA,
    395 => &PERFECTED_VESTMENT_OF_OLORIME,
    397 => &BALORGH,
    398 => &VYKOSA,
    399 => &HANUS_COMPASSION,
    400 => &BLOOD_MOON,
    401 => &HAVEN_OF_URSUS,
    402 => &MOON_HUNTER,
    403 => &SAVAGE_WEREWOLF,
    404 => &JAILERS_TENACITY,
    405 => &BRIGHT_THROATS_BOAST,
    406 => &DEAD_WATERS_GUILE,
    407 => &CHAMPION_OF_THE_HIST,
    408 => &GRAVE_STAKE_COLLECTOR,
    409 => &NAGA_SHAMAN,
    410 => &MIGHT_OF_THE_LOST_LEGION,
    411 => &GALLANT_CHARGE,
    412 => &RADIAL_UPPERCUT,
    413 => &SPECTRAL_CLOAK,
    414 => &VIRULENT_SHOT,
    415 => &WILD_IMPULSE,
    416 => &MENDERS_WARD,
    417 => &INDOMITABLE_FURY,
    418 => &SPELL_STRATEGIST,
    419 => &BATTLEFIELD_ACROBAT,
    420 => &SOLDIER_OF_ANGUISH,
    421 => &STEADFAST_HERO,
    422 => &BATTALION_DEFENDER,
    423 => &PERFECTED_GALLANT_CHARGE,
    424 => &PERFECTED_RADIAL_UPPERCUT,
    425 => &PERFECTED_SPECTRAL_CLOAK,
    426 => &PERFECTED_VIRULENT_SHOT,
    427 => &PERFECTED_WILD_IMPULSE,
    428 => &PERFECTED_MENDERS_WARD,
    429 => &MIGHTY_GLACIER,
    430 => &TZOGVINS_WARBAND,
    431 => &ICY_CONJURER,
    432 => &STONEKEEPER,
    433 => &FROZEN_WATCHER,
    434 => &SCAVENGING_DEMISE,
    435 => &AURORANS_THUNDER,
    436 => &SYMPHONY_OF_BLADES,
    437 => &COLDHARBOURS_FAVORITE,
    438 => &SENCHE_RAHTS_GRIT,
    439 => &VASTARIES_TUTELAGE,
    440 => &CRAFTY_ALFIQ,
    441 => &VESTURE_OF_DARLOC_BRAE,
    442 => &CALL_OF_THE_UNDERTAKER,
    443 => &EYE_OF_NAHVIINTAAS,
    444 => &FALSE_GODS_DEVOTION,
    445 => &TOOTH_OF_LOKKESTIIZ,
    446 => &CLAW_OF_YOLNAHKRIIN,
    448 => &PERFECTED_EYE_OF_NAHVIINTAAS,
    449 => &PERFECTED_FALSE_GODS_DEVOTION,
    450 => &PERFECTED_TOOTH_OF_LOKKESTIIZ,
    451 => &PERFECTED_CLAW_OF_YOLNAHKRIIN,
    452 => &HOLLOWFANG_THIRST,
    453 => &DROZAKARS_CLAWS,
    454 => &RENALDS_RESOLVE,
    455 => &ZENS_REDRESS,
    456 => &AZUREBLIGHT_REAPER,
    457 => &DRAGONS_DEFILEMENT,
    458 => &GRUNDWULF,
    459 => &MAARSELOK,
    465 => &SENCHAL_DEFENDER,
    466 => &MARAUDERS_HASTE,
    467 => &DRAGONGUARD_ELITE,
    468 => &DARING_CORSAIR,
    469 => &ANCIENT_DRAGONGUARD,
    470 => &NEW_MOON_ACOLYTE,
    471 => &HITIS_HEARTH,
    472 => &TITANBORN_STRENGTH,
    473 => &BANIS_TORMENT,
    474 => &DRAUGRKINS_GRIP,
    475 => &AEGIS_CALLER,
    476 => &GRAVE_GUARDIAN,
    478 => &MOTHER_CIANNAIT,
    479 => &KJALNARS_NIGHTMARE,
    480 => &CRITICAL_RIPOSTE,
    481 => &UNCHAINED_AGGRESSOR,
    482 => &DAUNTLESS_COMBATANT,
    487 => &WINTERS_RESPITE,
    488 => &VENOMOUS_SMITE,
    489 => &ETERNAL_VIGOR,
    490 => &STUHNS_FAVOR,
    491 => &DRAGONS_APPETITE,
    492 => &KYNES_WIND,
    493 => &PERFECTED_KYNES_WIND,
    494 => &VROLS_COMMAND,
    495 => &PERFECTED_VROLS_COMMAND,
    496 => &ROARING_OPPORTUNIST,
    497 => &PERFECTED_ROARING_OPPORTUNIST,
    498 => &YANDIRS_MIGHT,
    499 => &PERFECTED_YANDIRS_MIGHT,
    501 => &THRASSIAN_STRANGLERS,
    503 => &RING_OF_THE_WILD_HUNT,
    505 => &TORC_OF_TONAL_CONSTANCY,
    506 => &SPELL_PARASITE,
    513 => &TALFYGS_TREACHERY,
    514 => &UNLEASHED_TERROR,
    515 => &CRIMSON_TWILIGHT,
    516 => &ELEMENTAL_CATALYST,
    517 => &KRAGLENS_HOWL,
    518 => &ARKASIS_GENIUS,
    519 => &SNOW_TREADERS,
    520 => &MALACATHS_BAND_OF_BRUTALITY,
    521 => &BLOODLORDS_EMBRACE,
    522 => &PERFECTED_MERCILESS_CHARGE,
    523 => &PERFECTED_RAMPAGING_SLASH,
    524 => &PERFECTED_CRUEL_FLURRY,
    525 => &PERFECTED_THUNDEROUS_VOLLEY,
    526 => &PERFECTED_CRUSHING_WALL,
    527 => &PERFECTED_PRECISE_REGENERATION,
    528 => &PERFECTED_TITANIC_CLEAVE,
    529 => &PERFECTED_PUNCTURING_REMEDY,
    530 => &PERFECTED_STINGING_SLASHES,
    531 => &PERFECTED_CAUSTIC_ARROW,
    532 => &PERFECTED_DESTRUCTIVE_IMPACT,
    533 => &PERFECTED_GRAND_REJUVENATION,
    534 => &STONE_HUSK,
    535 => &LADY_THORN,
    536 => &RADIANT_BASTION,
    537 => &VOIDCALLER,
    538 => &WITCH_KNIGHTS_DEFIANCE,
    539 => &RED_EAGLES_FURY,
    540 => &LEGACY_OF_KARTH,
    541 => &AETHERIAL_ASCENSION,
    542 => &HEX_SIPHON,
    543 => &PESTILENT_HOST,
    544 => &EXPLOSIVE_REBUKE,
    557 => &EXECUTIONERS_BLADE,
    558 => &VOID_BASH,
    559 => &FRENZIED_MOMENTUM,
    560 => &POINT_BLANK_SNIPE,
    561 => &WRATH_OF_ELEMENTS,
    562 => &FORCE_OVERFLOW,
    563 => &PERFECTED_EXECUTIONERS_BLADE,
    564 => &PERFECTED_VOID_BASH,
    565 => &PERFECTED_FRENZIED_MOMENTUM,
    566 => &PERFECTED_POINT_BLANK_SNIPE,
    567 => &PERFECTED_WRATH_OF_ELEMENTS,
    568 => &PERFECTED_FORCE_OVERFLOW,
    569 => &TRUE_SWORN_FURY,
    570 => &KINRAS_WRATH,
    571 => &DRAKES_RUSH,
    572 => &UNLEASHED_RITUALIST,
    573 => &DAGONS_DOMINION,
    574 => &FOOLKILLERS_WARD,
    575 => &RING_OF_THE_PALE_ORDER,
    576 => &PEARLS_OF_EHLNOFEY,
    577 => &ENCRATIS_BEHEMOTH,
    578 => &BARON_ZAUDRUS,
    579 => &FROSTBITE,
    580 => &DEADLANDS_ASSASSIN,
    581 => &BOG_RAIDER,
    582 => &HIST_WHISPERER,
    583 => &HEARTLAND_CONQUEROR,
    584 => &DIAMONDS_VICTORY,
    585 => &SAXHLEEL_CHAMPION,
    586 => &SUL_XANS_TORMENT,
    587 => &BAHSEIS_MANIA,
    588 => &STONE_TALKERS_OATH,
    589 => &PERFECTED_SAXHLEEL_CHAMPION,
    590 => &PERFECTED_SUL_XANS_TORMENT,
    591 => &PERFECTED_BAHSEIS_MANIA,
    592 => &PERFECTED_STONE_TALKERS_OATH,
    593 => &GAZE_OF_SITHIS,
    594 => &HARPOONERS_WADING_KILT,
    596 => &DEATH_DEALERS_FETE,
    597 => &SHAPESHIFTERS_CHAIN,
    598 => &ZOAL_THE_EVER_WAKEFUL,
    599 => &IMMOLATOR_CHARR,
    600 => &GLORGOLOCH_THE_DESTROYER,
    602 => &CRIMSON_OATHS_RIVE,
    603 => &SCORIONS_FEAST,
    604 => &RUSH_OF_AGONY,
    605 => &SILVER_ROSE_VIGIL,
    606 => &THUNDER_CALLER,
    607 => &GRISLY_GOURMET,
    608 => &PRIOR_THIERRIC,
    609 => &MAGMA_INCARNATE,
    610 => &WRETCHED_VITALITY,
    611 => &DEADLANDS_DEMOLISHER,
    612 => &IRON_FLASK,
    613 => &EYE_OF_THE_GRASP,
    614 => &HEXOS_WARD,
    615 => &KYNMARCHERS_CRUELTY,
    616 => &DARK_CONVERGENCE,
    617 => &PLAGUEBREAK,
    618 => &HROTHGARS_CHILL,
    619 => &MALIGALIGS_MAELSTROM,
    620 => &GRYPHONS_REPRISAL,
    621 => &GLACIAL_GUARDIAN,
    622 => &TURNING_TIDE,
    623 => &STORM_CURSEDS_REVENGE,
    624 => &SPRIGGANS_VIGOR,
    625 => &MARKYN_RING_OF_MAJESTY,
    626 => &BELHARZAS_BAND,
    627 => &SPAULDER_OF_RUIN,
    629 => &RALLYING_CRY,
    630 => &HEW_AND_SUNDER,
    631 => &ENERVATING_AURA,
    632 => &KARGAEDA,
    633 => &NAZARAY,
    634 => &NUNATAK,
    635 => &LADY_MALYGDA,
    636 => &BARON_THIRSK,
    640 => &ORDERS_WRATH,
    641 => &SERPENTS_DISDAIN,
    642 => &DRUIDS_BRAID,
    643 => &BLESSING_OF_HIGH_ISLE,
    644 => &STEADFASTS_METTLE,
    645 => &SYSTRES_SCOWL,
    646 => &WHORL_OF_THE_DEPTHS,
    647 => &CORAL_RIPTIDE,
    648 => &PEARLESCENT_WARD,
    649 => &PILLAGERS_PROFIT,
    650 => &PERFECTED_PILLAGERS_PROFIT,
    651 => &PERFECTED_PEARLESCENT_WARD,
    652 => &PERFECTED_CORAL_RIPTIDE,
    653 => &PERFECTED_WHORL_OF_THE_DEPTHS,
    654 => &MORAS_WHISPERS,
    655 => &DOV_RHA_SABATONS,
    656 => &LEFTHANDERS_AEGIS_BELT,
    657 => &SEA_SERPENTS_COIL,
    658 => &OAKENSOUL_RING,
    660 => &DEEPROOT_ZEAL,
    661 => &STONES_ACCORD,
    662 => &RAGE_OF_THE_URSAUK,
    663 => &PANGRIT_DENMOTHER,
    664 => &GRAVE_INEVITABILITY,
    665 => &PHYLACTERYS_GRASP,
    666 => &ARCHDRUID_DEVYRIC,
    667 => &EUPHOTIC_GATEKEEPER,
    668 => &LANGUOR_OF_PERYITE,
    669 => &NOCTURNALS_PLOY,
    670 => &MARAS_BALM,
    671 => &BACK_ALLEY_GOURMAND,
    672 => &PHOENIX_MOTH_THEURGE,
    673 => &BASTION_OF_THE_DRAOIFE,
    674 => &FAUNS_LARK_CLADDING,
    675 => &STORMWEAVERS_CAVORT,
    676 => &SYRABANES_WARD,
    677 => &CHIMERAS_REBUKE,
    678 => &OLD_GROWTH_BREWER,
    679 => &CLAW_OF_THE_FOREST_WRAITH,
    680 => &RITEMASTERS_BOND,
    681 => &NIX_HOUNDS_HOWL,
    682 => &TELVANNI_ENFORCER,
    683 => &ROKSA_THE_WARPED,
    684 => &RUNECARVERS_BLAZE,
    685 => &APOCRYPHAL_INSPIRATION,
    686 => &ABYSSAL_BRACE,
    687 => &OZEZAN_THE_INFERNO,
    688 => &SNAKE_IN_THE_STARS,
    689 => &SHELL_SPLITTER,
    690 => &JUDGMENT_OF_AKATOSH,
    691 => &CRYPTCANON_VESTMENTS,
    692 => &ESOTERIC_ENVIRONMENT_GREAVES,
    693 => &TORC_OF_THE_LAST_AYLEID_KING,
    694 => &VELOTHI_UR_MAGES_AMULET,
    695 => &SHATTERED_FATE,
    696 => &TELVANNI_EFFICIENCY,
    697 => &SEEKER_SYNTHESIS,
    698 => &VIVECS_DUALITY,
    699 => &CAMONNA_TONG,
    700 => &ADAMANT_LURKER,
    701 => &PEACE_AND_SERENITY,
    702 => &ANSUULS_TORMENT,
    703 => &TEST_OF_RESOLVE,
    704 => &TRANSFORMATIVE_HOPE,
    705 => &PERFECTED_TRANSFORMATIVE_HOPE,
    706 => &PERFECTED_TEST_OF_RESOLVE,
    707 => &PERFECTED_ANSUULS_TORMENT,
    708 => &PERFECTED_PEACE_AND_SERENITY,
    711 => &COLOVIAN_HIGHLANDS_GENERAL,
    712 => &JERALL_MOUNTAINS_WARCHIEF,
    713 => &NIBENAY_BAY_BATTLEREEVE,
    722 => &REAWAKENED_HIEROPHANT,
    723 => &BASALT_BLOODED_WARRIOR,
    724 => &NOBILITY_IN_DECAY,
    726 => &SOULCLEAVER,
    727 => &MONOLITH_OF_STORMS,
    728 => &WRATHSUN,
    729 => &GARDENER_OF_SEASONS,
    730 => &CINDERS_OF_ANTHELMIR,
    731 => &SLUTHRUGS_HUNGER,
    732 => &BLACK_GLOVE_GROUNDING,
    734 => &ANTHELMIRS_CONSTRUCT,
    735 => &BLIND_PATH_INDUCTION,
    736 => &TARNISHED_NIGHTMARE,
    737 => &REFLECTED_FURY,
    738 => &THE_BLIND,
    754 => &OAKFATHERS_RETRIBUTION,
    755 => &BLUNTED_BLADES,
    756 => &BAAN_DARS_BLESSING,
    757 => &SYMMETRY_OF_THE_WEALD,
    758 => &MACABRE_VINTAGE,
    759 => &AYLEID_REFUGE,
    760 => &ROURKEN_STEAMGUARDS,
    761 => &THE_SHADOW_QUEENS_COWL,
    762 => &THE_SAINT_AND_THE_SEDUCER,
    763 => &THARRIKERS_STRIKE,
    764 => &HIGHLAND_SENTINEL,
    765 => &THREADS_OF_WAR,
    766 => &MORA_SCRIBES_THESIS,
    767 => &SLIVERS_OF_THE_NULL_ARCA,
    768 => &LUCENT_ECHOES,
    769 => &XORYNS_MASTERPIECE,
    770 => &PERFECTED_XORYNS_MASTERPIECE,
    771 => &PERFECTED_LUCENT_ECHOES,
    772 => &PERFECTED_SLIVERS_OF_THE_NULL_ARCA,
    773 => &PERFECTED_MORA_SCRIBES_THESIS,
    775 => &SPATTERING_DISJUNCTION,
    776 => &PYREBRAND,
    777 => &CORPSEBURSTER,
    778 => &UMBRAL_EDGE,
    779 => &BEACON_OF_OBLIVION,
    780 => &AETHERIC_LANCER,
    781 => &AERIES_CRY,
    782 => &TRACKERS_LASH,
    783 => &SHARED_PAIN,
    784 => &SIEGEMASTERS_FOCUS,
    791 => &BULWARK_RUINATION,
    792 => &FARSTRIDER,
    793 => &NETCH_OIL,
    794 => &VANDORALLENS_RESONANCE,
    795 => &JERENSIS_BLADESTORM,
    796 => &LUCILLAS_WINDSHIELD,
    797 => &SQUALL_OF_RETRIBUTION,
    798 => &HEROIC_UNITY,
    799 => &FLEDGLINGS_NEST,
    800 => &NOXIOUS_BOULDER,
    801 => &ORPHEON_THE_TACTICIAN,
    802 => &ARKAYS_CHARITY,
    803 => &LAMP_KNIGHTS_ART,
    804 => &BLACKFEATHER_FLIGHT,
    805 => &THREE_QUEENS_WELLSPRING,
    806 => &DEATH_DANCER,
    807 => &FULL_BELLY_BARRICADE,
    808 => &SHARED_BURDEN,
    809 => &TIDE_BORN_WILDSTALKER,
    810 => &FELLOWSHIPS_FORTITUDE,
    811 => &MAD_GODS_DANCING_SHOES,
    812 => &RAKKHATS_VOIDMANTLE,
    813 => &MONOMYTH_REFORGED,
    814 => &HARMONY_IN_CHAOS,
    815 => &KAZPIANS_CRUEL_SIGNET,
    816 => &DOLOROUS_ARENA,
    817 => &RECOVERY_CONVERGENCE,
    818 => &PERFECTED_RECOVERY_CONVERGENCE,
    819 => &PERFECTED_DOLOROUS_ARENA,
    820 => &PERFECTED_KAZPIANS_CRUEL_SIGNET,
    821 => &PERFECTED_HARMONY_IN_CHAOS,
    822 => &LUSTROUS_SOULWELL,
    823 => &VYKANDS_SOULFURY,
    824 => &BLACK_FOUNDRY_STEEL,
    825 => &XANMEER_SPELLWEAVER,
    826 => &TOOLS_OF_THE_TRAPMASTER,
    827 => &STONEHULK_DOMINATION,
    828 => &BLACK_GEM_MONSTROSITY,
    829 => &BAR_SAKKA,
    830 => &SPELLSHREDDER,
    831 => &COUP_DE_GRACE,
    832 => &UNFLINCHING_ULTIMATE,
    845 => &HUNTSMANS_WARMASK,
    846 => &XANMEER_GENESIS,
};

static ABYSSAL_BRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(Some(1710))],
    ],
};

static ADAMANT_LURKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static ADEPT_RIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static AEGIS_CALLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static AEGIS_OF_GALENWE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static AERIES_CRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static AETHERIAL_ASCENSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(Some(7377))],
    ],
};

static AETHERIC_LANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static AFFLICTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static AGILITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(Some(1752))],
        &[SetBonusType::Power(Some(206))]
    ],
};

static AKAVIRI_DRAGONGUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static ALESSIAS_BULWARK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static ALESSIAN_ORDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static ALMALEXIAS_MERCY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static AMBER_PLASM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MagickaRecovery(Some(245)), SetBonusType::StaminaRecovery(Some(245)), SetBonusType::HealthRecovery(Some(245))],
    ],
};

static ANCIENT_DRAGONGUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ANSUULS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
    ],
};

static ANTHELMIRS_CONSTRUCT: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static APOCRYPHAL_INSPIRATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static ARCHDRUID_DEVYRIC: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static ARCHERS_MIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ARKASIS_GENIUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static ARKAYS_CHARITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ARMOR_MASTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static ARMOR_OF_THE_CODE: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ARMOR_OF_THE_SEDUCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static ARMOR_OF_THE_TRAINEE: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(Some(1454))],
        &[SetBonusType::Magicka(Some(1454))],
        &[SetBonusType::Stamina(Some(1454))],
    ],
};

static ARMOR_OF_THE_VEILED_HERITANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static ARMOR_OF_TRUTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static ARMS_OF_INFERNACE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonusType::Power(None)],
    ],
};

static ARMS_OF_RELEQUEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
    ],
};

static ARMS_OF_THE_ANCESTORS: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonusType::Penetration(None)],
    ],
};

static ASHEN_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ASPECT_OF_MAZZATUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ASSASSINS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static AURORANS_THUNDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static AUTOMATED_DEFENSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static AYLEID_REFUGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static AZUREBLIGHT_REAPER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static BAAN_DARS_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static BACK_ALLEY_GOURMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static BAHRAHAS_CURSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BAHSEIS_MANIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static BALORGH: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static BANIS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BAR_SAKKA: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static BARKSKIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static BARON_THIRSK: Set = Set {
    bonuses: &[
        &[SetBonusType::StaminaRecovery(None), SetBonusType::MagickaRecovery(None)],
    ],
};

static BARON_ZAUDRUS: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(Some(548)), SetBonusType::Magicka(Some(548)), SetBonusType::Health(Some(603))],
    ],
};

static BASALT_BLOODED_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static BASTION_OF_THE_DRAOIFE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BASTION_OF_THE_HEARTLAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::ReducePlayerDamageTaken(None)],
    ],
};

static BATTALION_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static BATTLEFIELD_ACROBAT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BEACON_OF_OBLIVION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
    ],
};

static BECKONING_STEEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static BEEKEEPERS_GEAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(Some(900))],
    ],
};

static BELHARZAS_BAND: Set = Set {
    bonuses: &[
    ],
};

static BERSERKING_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static BLACK_FOUNDRY_STEEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BLACK_GEM_MONSTROSITY: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static BLACK_ROSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(Some(176))]
    ],
};

static BLACK_GLOVE_GROUNDING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static BLACKFEATHER_FLIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static BLESSING_OF_HIGH_ISLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static BLESSING_OF_THE_POTENTATES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::ReducePlayerDamageTaken(None)],
    ],
};

static BLIND_PATH_INDUCTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static BLOOD_MOON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static BLOODDRINKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static BLOODLORDS_EMBRACE: Set = Set {
    bonuses: &[
    ],
};

static BLOODSPAWN: Set = Set {
    bonuses: &[
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static BLOODTHORNS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static BLUNTED_BLADES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static BOG_RAIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BONE_PIRATES_TATTERS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static BRANDS_OF_IMPERIUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static BRIARHEART: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static BRIGHT_THROATS_BOAST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static BROKEN_SOUL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
    ],
};

static BUFFER_OF_THE_SWIFT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static BULWARK_RUINATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static BURNING_SPELLWEAVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static CALL_OF_THE_UNDERTAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Health(None)],
    ],
};

static CALUURIONS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static CAMONNA_TONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static CAUSTIC_ARROW: Set = Set {
    bonuses: &[
        &[],
    ],
};

static CHAMPION_OF_THE_HIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(Some(1600))],
    ],
};

static CHAOTIC_WHIRLWIND: Set = Set {
    bonuses: &[
        &[],
    ],
};

static CHIMERAS_REBUKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static CHOKETHORN: Set = Set {
    bonuses: &[
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static CINDERS_OF_ANTHELMIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static CLAW_OF_THE_FOREST_WRAITH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static CLAW_OF_YOLNAHKRIIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Stamina(None)],
    ],
};

static CLEVER_ALCHEMIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static COLDHARBOURS_FAVORITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static COLOVIAN_HIGHLANDS_GENERAL: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static COMBAT_PHYSICIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static CONCENTRATED_FORCE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static CORAL_RIPTIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static CORPSEBURSTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static COUP_DE_GRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static COWARDS_GEAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(Some(250))]
    ],
};

static CRAFTY_ALFIQ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(Some(2550))],
    ],
};

static CREST_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static CRIMSON_OATHS_RIVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static CRIMSON_TWILIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static CRITICAL_RIPOSTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalResistance(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::CriticalResistance(None)],
    ],
};

static CRUEL_FLURRY: Set = Set {
    bonuses: &[
        &[],
    ],
};

static CRUSADER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static CRUSHING_WALL: Set = Set {
    bonuses: &[
        &[],
    ],
};

static CRYPTCANON_VESTMENTS: Set = Set {
    bonuses: &[
    ],
};

static CURSE_EATER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static CURSE_OF_DOYLEMISH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DAEDRIC_TRICKERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static DAGONS_DOMINION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DARING_CORSAIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static DARK_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DARKSTRIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static DAUNTLESS_COMBATANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DEAD_WATERS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DEADLANDS_ASSASSIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(Some(200))],
    ],
};

static DEADLANDS_DEMOLISHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static DEADLY_STRIKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DEATH_DEALERS_FETE: Set = Set {
    bonuses: &[
    ],
};

static DEATHS_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DEATH_DANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static DEEPROOT_ZEAL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DEFENDING_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DEFENSIVE_POSITION: Set = Set {
    bonuses: &[
        &[],
    ],
};

static DEFILER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static DESERT_ROSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static DESTRUCTIVE_IMPACT: Set = Set {
    bonuses: &[
        &[],
    ],
};

static DESTRUCTIVE_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DIAMONDS_VICTORY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DISCIPLINED_SLASH: Set = Set {
    bonuses: &[
        &[],
    ],
};

static DOLOROUS_ARENA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Armour(None)],
    ],
};

static DOMIHAUS: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None), SetBonusType::Magicka(None)],
    ],
};

static DOV_RHA_SABATONS: Set = Set {
    bonuses: &[
    ],
};

static DRAGONS_APPETITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static DRAGONS_DEFILEMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DRAGONGUARD_ELITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static DRAKES_RUSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static DRAUGR_HULK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(Some(2550))],
    ],
};

static DRAUGRS_HERITAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static DRAUGRS_REST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static DRAUGRKINS_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static DREAMERS_MANTLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static DREUGH_KING_SLAYER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static DROZAKARS_CLAWS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static DRUIDS_BRAID: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(Some(1565))],
        &[SetBonusType::Stamina(Some(1565))],
        &[SetBonusType::Health(Some(1722))],
        &[SetBonusType::Magicka(Some(1565))],
        &[SetBonusType::Stamina(Some(1565))],
        &[SetBonusType::Health(Some(1722))],
        &[SetBonusType::Magicka(Some(1565))],
        &[SetBonusType::Stamina(Some(1565))],
    ],
};

static DUNERIPPERS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static DUROKS_BANE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static EAGLE_EYE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static EARTHGORE: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingDone(None)],
    ],
};

static EBON_ARMORY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static ELEMENTAL_CATALYST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ELEMENTAL_SUCCESSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ELF_BANE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static EMBERSHIELD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static ENCRATIS_BEHEMOTH: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static ENDURANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(Some(1928))],
        &[SetBonusType::HealthRecovery(Some(618))],
    ],
};

static ENERVATING_AURA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static ENGINE_GUARDIAN: Set = Set {
    bonuses: &[
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static ESOTERIC_ENVIRONMENT_GREAVES: Set = Set {
    bonuses: &[
    ],
};

static ESSENCE_THIEF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static ETERNAL_HUNT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static ETERNAL_VIGOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static ETERNAL_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static EUPHOTIC_GATEKEEPER: Set = Set {
    bonuses: &[
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static EXECUTIONERS_BLADE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static EXPLOSIVE_REBUKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static EYE_OF_NAHVIINTAAS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
    ],
};

static EYE_OF_THE_GRASP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static EYES_OF_MARA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static FALSE_GODS_DEVOTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static FARSTRIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static FASALLAS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static FAUNS_LARK_CLADDING: Set = Set {
    bonuses: &[
    ],
};

static FELLOWSHIPS_FORTITUDE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[],
        &[],
        &[SetBonusType::Armour(Some(7425))],
        &[],
        &[],
        &[],
        &[],
        &[SetBonusType::Health(Some(6020))],
    ],
};

static FIORDS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static FLAME_BLOSSOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static FLANKING_STRATEGIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static FLEDGLINGS_NEST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static FOOLKILLERS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static FOOTMANS_FORTUNE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static FORCE_OVERFLOW: Set = Set {
    bonuses: &[
        &[],
    ],
};

static FORTIFIED_BRASS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(Some(3460))],
    ],
};

static FRENZIED_MOMENTUM: Set = Set {
    bonuses: &[
        &[],
    ],
};

static FROSTBITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static FROZEN_WATCHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static FULL_BELLY_BARRICADE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static GALERIONS_REVENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static GALLANT_CHARGE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static GARDENER_OF_SEASONS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static GAZE_OF_SITHIS: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(Some(3276)), SetBonusType::HealthRecovery(Some(1025)), SetBonusType::Armour(Some(4000))]
    ],
};

static GIANT_SPIDER: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingTaken(None)],
    ],
};

static GLACIAL_GUARDIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static GLORGOLOCH_THE_DESTROYER: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static GLORIOUS_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static GOSSAMER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static GRACE_OF_GLOOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static GRACE_OF_THE_ANCIENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(Some(2406))]
    ],
};

static GRAND_REJUVENATION: Set = Set {
    bonuses: &[
        &[],
    ],
};

static GRAVE_GUARDIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static GRAVE_INEVITABILITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static GRAVE_STAKE_COLLECTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
    ],
};

static GREEN_PACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::Health(None)],
    ],
};

static GRISLY_GOURMET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Stamina(Some(526))],
    ],
};

static GROTHDARR: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static GRUNDWULF: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static GRYPHONS_FEROCITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static GRYPHONS_REPRISAL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static HAGRAVENS_GARDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static HAND_OF_MEPHALA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static HANUS_COMPASSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(Some(1963))],
    ],
};

static HARMONY_IN_CHAOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static HARPOONERS_WADING_KILT: Set = Set {
    bonuses: &[
    ],
};

static HATCHLINGS_SHELL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static HAVEN_OF_URSUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static HAWKS_EYE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HEALERS_HABIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HEALING_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static HEARTLAND_CONQUEROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static HEEM_JAS_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HEROIC_UNITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static HEW_AND_SUNDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HEX_SIPHON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static HEXOS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static HIDE_OF_MORIHAUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static HIDE_OF_THE_WEREWOLF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HIGHLAND_SENTINEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(Some(986))],
    ],
};

static HIRCINES_VENEER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static HIST_BARK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
    ],
};

static HIST_WHISPERER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static HITIS_HEARTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static HOLLOWFANG_THIRST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static HROTHGARS_CHILL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static HUNDINGS_RAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(Some(300))],
    ],
};

static HUNT_LEADER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static HUNTSMANS_WARMASK: Set = Set {
    bonuses: &[
    ],
};

static ICEHEART: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ICY_CONJURER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static ILAMBRIS: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static IMMOLATOR_CHARR: Set = Set {
    bonuses: &[
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static IMMORTAL_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static IMPERIAL_PHYSIQUE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static IMPREGNABLE_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalResistance(Some(1650))]
    ],
};

static INDOMITABLE_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static INFALLIBLE_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static INFERNAL_GUARDIAN: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static INNATE_AXIOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None), SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static INVENTORS_GUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingDone(None)],
    ],
};

static IRON_FLASK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static IRONBLOOD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static JAILBREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(Some(142))],
    ],
};

static JAILERS_TENACITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static JERALL_MOUNTAINS_WARCHIEF: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static JERENSIS_BLADESTORM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static JOLTING_ARMS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static JORVULDS_GUIDANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static JUDGMENT_OF_AKATOSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(Some(2291))]
    ],
};

static KAGRENACS_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(Some(222))],
    ],
};

static KARGAEDA: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(Some(731)), SetBonusType::Stamina(Some(731))]
    ],
};

static KAZPIANS_CRUEL_SIGNET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
    ],
};

static KINRAS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static KJALNARS_NIGHTMARE: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static KNIGHT_SLAYER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
    ],
};

static KNIGHT_ERRANTS_MAIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static KNIGHTMARE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static KRAGH: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static KRAGLENS_HOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static KVATCH_GLADIATOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static KYNES_KISS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static KYNES_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingDone(None)],
    ],
};

static KYNMARCHERS_CRUELTY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static LADY_MALYGDA: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static LADY_THORN: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static LAMIAS_SONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static LAMP_KNIGHTS_ART: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(Some(100_000))]
    ],
};

static LANGUOR_OF_PERYITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalResistance(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static LAW_OF_JULIANOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(Some(300))],
    ],
};

static LEECHING_PLATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static LEFTHANDERS_AEGIS_BELT: Set = Set {
    bonuses: &[
    ],
};

static LEGACY_OF_KARTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static LEKIS_FOCUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static LEVIATHAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(Some(1528))],
    ],
};

static LIGHT_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static LIGHT_SPEAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static LIVEWIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static LORD_WARDEN: Set = Set {
    bonuses: &[
        &[SetBonusType::Armour(None)],
    ],
};

static LUCENT_ECHOES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
    ],
};

static LUCILLAS_WINDSHIELD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static LUNAR_BASTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
    ],
};

static LUSTROUS_SOULWELL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static MAARSELOK: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None)],
    ],
};

static MACABRE_VINTAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(Some(150))],
    ],
};

static MAD_GODS_DANCING_SHOES: Set = Set {
    bonuses: &[
    ],
};

static MAD_TINKERER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MAGICKA_FURNACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MAGMA_INCARNATE: Set = Set {
    bonuses: &[
        &[SetBonusType::MagickaRecovery(None), SetBonusType::StaminaRecovery(None)],
    ],
};

static MAGNUS_GIFT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MALACATHS_BAND_OF_BRUTALITY: Set = Set {
    bonuses: &[
    ],
};

static MALIGALIGS_MAELSTROM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MANTLE_OF_SIRORIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static MARAS_BALM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::CriticalResistance(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static MARAUDERS_HASTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static MARK_OF_THE_PARIAH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static MARKSMANS_CREST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MARKYN_RING_OF_MAJESTY: Set = Set {
    bonuses: &[
    ],
};

static MASTER_ARCHITECT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static MAW_OF_THE_INFERNAL: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static MECHANICAL_ACUITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MEDUSA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(Some(892))],
    ],
};

static MENDERS_WARD: Set = Set {
    bonuses: &[
        &[],
    ],
};

static MERCILESS_CHARGE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static MERIDIAS_BLESSED_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static MERITORIOUS_SERVICE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MIGHT_OF_THE_LOST_LEGION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
    ],
};

static MIGHTY_CHUDAN: Set = Set {
    bonuses: &[
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static MIGHTY_GLACIER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::StaminaRecovery(None), SetBonusType::MagickaRecovery(None)],
    ],
};

static MOLAG_KENA: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static MONOLITH_OF_STORMS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MONOMYTH_REFORGED: Set = Set {
    bonuses: &[
    ],
};

static MOON_HUNTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static MOONDANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static MORA_SCRIBES_THESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static MORAS_WHISPERS: Set = Set {
    bonuses: &[
    ],
};

static MORKULDIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static MOTHER_CIANNAIT: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static MOTHERS_SORROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(Some(1528))],
    ],
};

static NAGA_SHAMAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static NAZARAY: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static NECROPOTENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static NERIENETH: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static NETCH_OIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static NETCHS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static NEW_MOON_ACOLYTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(Some(401))],
    ],
};

static NIBENAY_BAY_BATTLEREEVE: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalResistance(None)]
    ],
};

static NIGHT_MOTHERS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static NIGHT_MOTHERS_GAZE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static NIGHT_TERROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
    ],
};

static NIGHTS_SILENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static NIGHTFLAME: Set = Set {
    bonuses: &[
        &[SetBonusType::Magicka(None)],
    ],
};

static NIKULAS_HEAVY_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static NIX_HOUNDS_HOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static NOBILITY_IN_DECAY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static NOBLE_DUELISTS_SILKS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static NOBLES_CONQUEST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static NOCTURNALS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static NOCTURNALS_PLOY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static NOXIOUS_BOULDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static NUNATAK: Set = Set {
    bonuses: &[
        &[SetBonusType::Armour(None)],
    ],
};

static OAKENSOUL_RING: Set = Set {
    bonuses: &[
    ],
};

static OAKFATHERS_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static OBLIVIONS_EDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(Some(258))],
    ],
};

static OBLIVIONS_FOE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static OLD_GROWTH_BREWER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static ORDER_OF_DIAGNA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::HealingTaken(Some(5))],
    ],
};

static ORDERS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(Some(943))],
    ],
};

static ORGNUMS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static ORPHEON_THE_TACTICIAN: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static OVERWHELMING_SURGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static OZEZAN_THE_INFERNO: Set = Set {
    bonuses: &[
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static PANGRIT_DENMOTHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::Stamina(Some(1890))],
    ],
};

static PARA_BELLUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static PEACE_AND_SERENITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static PEARLESCENT_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static PEARLS_OF_EHLNOFEY: Set = Set {
    bonuses: &[
    ],
};

static PELINALS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_AEGIS_OF_GALENWE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static PERFECTED_ANSUULS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_ARMS_OF_RELEQUEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_BAHSEIS_MANIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_CAUSTIC_ARROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(Some(103))]
    ],
};

static PERFECTED_CHAOTIC_WHIRLWIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_CLAW_OF_YOLNAHKRIIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
    ],
};

static PERFECTED_CONCENTRATED_FORCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(Some(103))]
    ],
};

static PERFECTED_CORAL_RIPTIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_CRUEL_FLURRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(Some(103))]
    ],
};

static PERFECTED_CRUSHING_WALL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_DEFENSIVE_POSITION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(Some(103))]
    ],
};

static PERFECTED_DESTRUCTIVE_IMPACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(Some(103))]
    ],
};

static PERFECTED_DISCIPLINED_SLASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(Some(877))]
    ],
};

static PERFECTED_DOLOROUS_ARENA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static PERFECTED_EXECUTIONERS_BLADE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_EYE_OF_NAHVIINTAAS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static PERFECTED_FALSE_GODS_DEVOTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_FORCE_OVERFLOW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(Some(877))]
    ],
};

static PERFECTED_FRENZIED_MOMENTUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(Some(877))]
    ],
};

static PERFECTED_GALLANT_CHARGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(Some(1190))]
    ],
};

static PERFECTED_GRAND_REJUVENATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(Some(877))]
    ],
};

static PERFECTED_HARMONY_IN_CHAOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_KAZPIANS_CRUEL_SIGNET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_KYNES_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static PERFECTED_LUCENT_ECHOES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
    ],
};

static PERFECTED_MANTLE_OF_SIRORIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_MENDERS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(Some(103))]
    ],
};

static PERFECTED_MERCILESS_CHARGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_MORA_SCRIBES_THESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_PEACE_AND_SERENITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_PEARLESCENT_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static PERFECTED_PIERCING_SPRAY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_PILLAGERS_PROFIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static PERFECTED_POINT_BLANK_SNIPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(Some(103))]
    ],
};

static PERFECTED_PRECISE_REGENERATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_PUNCTURING_REMEDY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(Some(3))]
    ],
};

static PERFECTED_RADIAL_UPPERCUT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_RAMPAGING_SLASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(Some(77)), SetBonusType::StaminaRecovery(Some(77))]
    ],
};

static PERFECTED_RECOVERY_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static PERFECTED_ROARING_OPPORTUNIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static PERFECTED_SAXHLEEL_CHAMPION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static PERFECTED_SLIVERS_OF_THE_NULL_ARCA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERFECTED_SPECTRAL_CLOAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(Some(103))]
    ],
};

static PERFECTED_STINGING_SLASHES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_STONE_TALKERS_OATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static PERFECTED_SUL_XANS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_TEST_OF_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static PERFECTED_THUNDEROUS_VOLLEY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_TIMELESS_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(Some(877))]
    ],
};

static PERFECTED_TITANIC_CLEAVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_TOOTH_OF_LOKKESTIIZ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_TRANSFORMATIVE_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealingDone(None)],
    ],
};

static PERFECTED_VESTMENT_OF_OLORIME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static PERFECTED_VIRULENT_SHOT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(Some(526))]
    ],
};

static PERFECTED_VOID_BASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(Some(965))]
    ],
};

static PERFECTED_VROLS_COMMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static PERFECTED_WHORL_OF_THE_DEPTHS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PERFECTED_WILD_IMPULSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_WRATH_OF_ELEMENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(Some(1190))]
    ],
};

static PERFECTED_XORYNS_MASTERPIECE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static PERFECTED_YANDIRS_MIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PERMAFROST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static PESTILENT_HOST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static PHOENIX: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PHOENIX_MOTH_THEURGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static PHYLACTERYS_GRASP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static PIERCING_SPRAY: Set = Set {
    bonuses: &[
        &[],
    ],
};

static PILLAGERS_PROFIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
    ],
};

static PILLAR_OF_NIRN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PIRATE_SKELETON: Set = Set {
    bonuses: &[
        &[SetBonusType::Armour(None)],
    ],
};

static PLAGUE_DOCTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(Some(2804))],
    ],
};

static PLAGUE_SLINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PLAGUEBREAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static POINT_BLANK_SNIPE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static POISONOUS_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static POWERFUL_ASSAULT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static PRAYER_SHAWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static PRECISE_REGENERATION: Set = Set {
    bonuses: &[
        &[],
    ],
};

static PRIOR_THIERRIC: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static PRISONERS_RAGS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static PROPHETS: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static PUNCTURING_REMEDY: Set = Set {
    bonuses: &[
        &[],
    ],
};

static PYREBRAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static QUEENS_ELEGANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static QUICK_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static RADIAL_UPPERCUT: Set = Set {
    bonuses: &[
        &[],
    ],
};

static RADIANT_BASTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
    ],
};

static RAGE_OF_THE_URSAUK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static RAKKHATS_VOIDMANTLE: Set = Set {
    bonuses: &[
    ],
};

static RALLYING_CRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static RAMPAGING_SLASH: Set = Set {
    bonuses: &[
        &[],
    ],
};

static RANGERS_GAIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static RATTLECAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static RAVAGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
    ],
};

static REACTIVE_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Health(None)],
    ],
};

static REAWAKENED_HIEROPHANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(Some(731)), SetBonusType::Stamina(Some(731))],
        &[SetBonusType::Magicka(Some(731)), SetBonusType::Stamina(Some(731))],
        &[SetBonusType::Magicka(Some(731)), SetBonusType::Stamina(Some(731))],
    ],
};

static RECOVERY_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Magicka(None)],
    ],
};

static RED_EAGLES_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static REDISTRIBUTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static REFLECTED_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static RELICS_OF_THE_PHYSICIAN_ANSUR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
    ],
};

static RELICS_OF_THE_REBELLION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
    ],
};

static RENALDS_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static RING_OF_THE_PALE_ORDER: Set = Set {
    bonuses: &[
    ],
};

static RING_OF_THE_WILD_HUNT: Set = Set {
    bonuses: &[
    ],
};

static RITEMASTERS_BOND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ROAR_OF_ALKOSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static ROARING_OPPORTUNIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static ROBES_OF_ALTERATION_MASTERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ROBES_OF_DESTRUCTION_MASTERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static ROBES_OF_THE_HIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ROBES_OF_THE_WITHERED_HAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
    ],
};

static ROBES_OF_TRANSMUTATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ROKSA_THE_WARPED: Set = Set {
    bonuses: &[
        &[SetBonusType::StaminaRecovery(Some(70)), SetBonusType::MagickaRecovery(Some(70)), SetBonusType::HealthRecovery(Some(70))]
    ],
};

static ROURKEN_STEAMGUARDS: Set = Set {
    bonuses: &[
    ],
};

static RUNECARVERS_BLAZE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static RUSH_OF_AGONY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static SALVATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static SANCTUARY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SAVAGE_WEREWOLF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static SAXHLEEL_CHAMPION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Stamina(None)],
    ],
};

static SCATHING_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SCAVENGING_DEMISE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SCORIONS_FEAST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SCOURGE_HARVESTER: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static SEA_SERPENTS_COIL: Set = Set {
    bonuses: &[
    ],
};

static SEEKER_SYNTHESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static SELENE: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static SELLISTRIX: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None)],
    ],
};

static SENCHAL_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SENCHES_BITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static SENCHE_RAHTS_GRIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static SENTINEL_OF_RKUGAMZ: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingDone(None)],
    ],
};

static SENTRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SERGEANTS_MAIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SERPENTS_DISDAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SEVENTH_LEGION_BRUTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::HealthRecovery(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SHACKLEBREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(Some(2065)), SetBonusType::Magicka(Some(2065))],
    ],
};

static SHADOW_DANCERS_RAIMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SHADOW_OF_THE_RED_MOUNTAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SHADOW_WALKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static SHADOWREND: Set = Set {
    bonuses: &[
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static SHALIDORS_CURSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Health(None)],
    ],
};

static SHALK_EXOSKELETON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static SHAPESHIFTERS_CHAIN: Set = Set {
    bonuses: &[
    ],
};

static SHARED_BURDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SHARED_PAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SHATTERED_FATE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[],
        &[],
        &[SetBonusType::Penetration(Some(7918))],
        &[],
        &[],
        &[],
        &[],
        &[SetBonusType::Power(Some(687))],
        &[],
        &[SetBonusType::CriticalChance(Some(1528))],
    ],
};

static SHEER_VENOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SHELL_SPLITTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static SHIELD_BREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SHIELD_OF_THE_VALIANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::ReducePlayerDamageTaken(None)],
    ],
};

static SHROUD_OF_THE_LICH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static SIEGEMASTERS_FOCUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static SILKS_OF_THE_SUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SILVER_ROSE_VIGIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static SITHIS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SKOOMA_SMUGGLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SLIMECRAW: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SLIVERS_OF_THE_NULL_ARCA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SLOADS_SEMBLANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SLUTHRUGS_HUNGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static SNAKE_IN_THE_STARS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static SNOW_TREADERS: Set = Set {
    bonuses: &[
    ],
};

static SOLDIER_OF_ANGUISH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SONG_OF_LAMAE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static SOULCLEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SOULSHINE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPATTERING_DISJUNCTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPAULDER_OF_RUIN: Set = Set {
    bonuses: &[
    ],
};

static SPAWN_OF_MEPHALA: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None)],
    ],
};

static SPECTRAL_CLOAK: Set = Set {
    bonuses: &[
        &[],
    ],
};

static SPECTRES_EYE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Health(None)],
    ],
};

static SPELL_PARASITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SPELL_POWER_CURE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPELL_STRATEGIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPELLSHREDDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SPELUNKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPIDER_CULTIST_COWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SPINNERS_GARMENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(Some(3460))],
    ],
};

static SPRIGGANS_THORNS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(Some(3460))],
    ],
};

static SPRIGGANS_VIGOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SQUALL_OF_RETRIBUTION: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static STEADFAST_HERO: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static STEADFASTS_METTLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static STENDARRS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static STINGING_SLASHES: Set = Set {
    bonuses: &[
        &[],
    ],
};

static STONE_HUSK: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static STONES_ACCORD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static STONE_TALKERS_OATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static STONEHULK_DOMINATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static STONEKEEPER: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(Some(548)), SetBonusType::Magicka(Some(548)), SetBonusType::Health(Some(603))],
    ],
};

static STORM_KNIGHTS_PLATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static STORM_MASTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static STORM_CURSEDS_REVENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static STORMFIST: Set = Set {
    bonuses: &[
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static STORMWEAVERS_CAVORT: Set = Set {
    bonuses: &[
    ],
};

static STRENGTH_OF_THE_AUTOMATON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static STUHNS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static STYGIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static SUL_XANS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SUNDERFLAME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SWAMP_RAIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SWARM_MOTHER: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None), SetBonusType::Magicka(None)],
    ],
};

static SWORD_DANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static SWORD_SINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SYMMETRY_OF_THE_WEALD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static SYMPHONY_OF_BLADES: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingDone(None)],
    ],
};

static SYRABANES_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static SYRABANES_WARD: Set = Set {
    bonuses: &[
    ],
};

static SYSTRES_SCOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static SYVARRAS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static TALFYGS_TREACHERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static TARNISHED_NIGHTMARE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static TAVAS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static TELVANNI_EFFICIENCY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static TELVANNI_ENFORCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static TEST_OF_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
    ],
};

static THARRIKERS_STRIKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static THE_ARCH_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static THE_BLIND: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static THE_DESTRUCTION_SUITE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonusType::Armour(None)],
    ],
};

static THE_ICE_FURNACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static THE_JUGGERNAUT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
    ],
};

static THE_MORAG_TONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static THE_SAINT_AND_THE_SEDUCER: Set = Set {
    bonuses: &[
    ],
};

static THE_SHADOW_QUEENS_COWL: Set = Set {
    bonuses: &[
    ],
};

static THE_TROLL_KING: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingDone(None)],
    ],
};

static THE_WORMS_RAIMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static THEWS_OF_THE_HARBINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static THRASSIAN_STRANGLERS: Set = Set {
    bonuses: &[
    ],
};

static THREADS_OF_WAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static THREE_QUEENS_WELLSPRING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static THUNDER_CALLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static THUNDERBUGS_CARAPACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Power(None)],
    ],
};

static THUNDEROUS_VOLLEY: Set = Set {
    bonuses: &[
        &[],
    ],
};

static THURVOKUN: Set = Set {
    bonuses: &[
        &[SetBonusType::Health(None)],
    ],
};

static TIDE_BORN_WILDSTALKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static TIMELESS_BLESSING: Set = Set {
    bonuses: &[
        &[],
    ],
};

static TITANBORN_STRENGTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static TITANIC_CLEAVE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static TOOLS_OF_THE_TRAPMASTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static TOOTH_OF_LOKKESTIIZ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static TOOTHROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static TORC_OF_THE_LAST_AYLEID_KING: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(Some(1337)), SetBonusType::MagickaRecovery(Some(500)), SetBonusType::StaminaRecovery(Some(500))]
    ],
};

static TORC_OF_TONAL_CONSTANCY: Set = Set {
    bonuses: &[
    ],
};

static TORMENTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static TORUGS_PACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static TRACKERS_LASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static TRANSFORMATIVE_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::HealingDone(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static TRAPPINGS_OF_INVIGORATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static TREASURE_HUNTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static TREASURES_OF_THE_EARTHFORGE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonusType::Armour(None)],
    ],
};

static TREMORSCALE: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None)],
    ],
};

static TRIAL_BY_FIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static TRINIMACS_VALOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static TRUE_SWORN_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static TURNING_TIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
    ],
};

static TWICE_BORN_STAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static TWICE_FANGED_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static TWILIGHT_REMEDY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Power(None)],
    ],
};

static TWILIGHTS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static TWIN_SISTERS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static TZOGVINS_WARBAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static ULFNORS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::MagickaRecovery(None), SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static UMBRAL_EDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static UNCHAINED_AGGRESSOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static UNDAUNTED_BASTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static UNDAUNTED_INFILTRATOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static UNDAUNTED_UNWEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static UNFATHOMABLE_DARKNESS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static UNFLINCHING_ULTIMATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static UNLEASHED_RITUALIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static UNLEASHED_TERROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static VALKYN_SKORIA: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(None)],
    ],
};

static VAMPIRE_CLOAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(Some(171))],
    ],
};

static VAMPIRE_LORD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static VAMPIRES_KISS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None), SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VANDORALLENS_RESONANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VANGUARDS_CHALLENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static VARENS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::HealingTaken(None)],
    ],
};

static VASTARIES_TUTELAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None), SetBonusType::StaminaRecovery(None)],
    ],
};

static VELIDRETH: Set = Set {
    bonuses: &[
        &[SetBonusType::Power(None)],
    ],
};

static VELOTHI_UR_MAGES_AMULET: Set = Set {
    bonuses: &[
        &[SetBonusType::Penetration(Some(1650))]
    ],
};

static VENGEANCE_LEECH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
    ],
};

static VENOMOUS_SMITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VESTMENT_OF_OLORIME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static VESTMENTS_OF_THE_WARLOCK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static VESTURE_OF_DARLOC_BRAE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static VICECANON_OF_VENOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static VICIOUS_DEATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
    ],
};

static VICIOUS_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VIPERS_STING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static VIRULENT_SHOT: Set = Set {
    bonuses: &[
        &[],
    ],
};

static VIVECS_DUALITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static VOID_BASH: Set = Set {
    bonuses: &[
        &[],
    ],
};

static VOIDCALLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VROLS_COMMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::Health(None)],
    ],
};

static VYKANDS_SOULFURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Penetration(None)],
        &[SetBonusType::Power(None)],
    ],
};

static VYKOSA: Set = Set {
    bonuses: &[
        &[SetBonusType::HealingTaken(None)],
    ],
};

static WAR_MACHINE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static WAR_MAIDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WARD_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static WARRIORS_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WARRIOR_POET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::Armour(None)],
    ],
};

static WAY_OF_AIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
    ],
};

static WAY_OF_FIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::CriticalChance(None)],
    ],
};

static WAY_OF_MARTIAL_KNOWLEDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WAY_OF_THE_ARENA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Power(Some(165))]
    ],
};

static WHITESTRAKES_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Armour(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static WHORL_OF_THE_DEPTHS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static WIDOWMAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static WILD_IMPULSE: Set = Set {
    bonuses: &[
        &[],
    ],
};

static WILDERQUEENS_ARCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Stamina(None)],
    ],
};

static WILLOWS_PATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::HealthRecovery(None)],
    ],
};

static WILLPOWER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(Some(1752))],
        &[SetBonusType::Power(Some(206))]
    ],
};

static WINTERS_RESPITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static WINTERBORN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WISDOM_OF_VANUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WISE_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WITCH_KNIGHTS_DEFIANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WITCHMAN_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Health(None)],
    ],
};

static WIZARDS_RIPOSTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::CriticalResistance(Some(660))]
    ],
};

static WRATH_OF_ELEMENTS: Set = Set {
    bonuses: &[
        &[],
    ],
};

static WRATH_OF_THE_IMPERIUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(Some(325))]
    ],
};

static WRATHSUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WRETCHED_VITALITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::StaminaRecovery(None)],
        &[SetBonusType::Power(None)],
    ],
};

static WYRD_TREES_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static XANMEER_GENESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Stamina(None)],
        &[SetBonusType::Health(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static XANMEER_SPELLWEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Power(None)],
    ],
};

static XORYNS_MASTERPIECE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::MinorAegis],
        &[SetBonusType::MagickaRecovery(None)],
    ],
};

static YANDIRS_MIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::Power(None)],
        &[SetBonusType::MinorSlayer],
        &[SetBonusType::Power(None)],
    ],
};

static YSGRAMORS_BIRTHRIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::CriticalChance(None)],
        &[SetBonusType::Magicka(None)],
        &[SetBonusType::Power(None)],
    ],
};

static ZENS_REDRESS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonusType::MagickaRecovery(None)],
        &[SetBonusType::Power(None)],
        &[SetBonusType::Magicka(None)],
    ],
};

static ZAAN: Set = Set {
    bonuses: &[
        &[SetBonusType::CriticalChance(None)],
    ],
};

static ZOAL_THE_EVER_WAKEFUL: Set = Set {
    bonuses: &[
        &[SetBonusType::Stamina(None)],
    ],
};