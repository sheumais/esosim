use phf::{Map, phf_map};

use crate::{data::enums::damage::ResistableDamageType, engine::effect::*};

pub struct ActiveSet {
    pub set_id: u32,
    pub count: u8,
}

#[derive(Debug)]
pub struct SetBonus {
    pub channel: Channel,
    pub value: f64,
}

#[derive(Debug)]
pub struct Set {
    pub bonuses: &'static [&'static [SetBonus]],
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
    use crate::{data::enums::gear::{GearSlot, ItemQuality, ItemType}, engine::aggregator::ChannelSet, entity::{gear::GearPiece, player::Player}};
    use super::*;

    #[test]
    fn reawakened_hierophant_magicka_bonus_is_summed_correctly() {
        let mut channels = ChannelSet::new();
        let mut player = Player::new();
        let hierophant_item = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(722), None);
        let hierophant_weapon= GearPiece::new(ItemType::LightningStaff, 66, None, ItemQuality::Legendary, Some(722), None);
        player.set_gear_piece(GearSlot::Chest, hierophant_item.clone());
        player.set_gear_piece(GearSlot::Legs, hierophant_item.clone());
        player.set_gear_piece(GearSlot::Hands, hierophant_item.clone());
        player.set_gear_piece(GearSlot::MainHand, hierophant_weapon);
        player.apply_set_bonuses(&mut channels);
        assert_eq!(channels.get(Channel::Resource(ResourceKind::Magicka, AggKind::Additive)), 731 * 3);
    }
}

// Big thank you to UESP!
// https://esoitem.uesp.net/viewlog.php?record=setSummary
pub static SET_BONUSES: Map<u16, &'static Set> = phf_map!{
    19u16 => &VESTMENTS_OF_THE_WARLOCK,
    20u16 => &WITCHMAN_ARMOR,
    21u16 => &AKAVIRI_DRAGONGUARD,
    22u16 => &DREAMERS_MANTLE,
    23u16 => &ARCHERS_MIND,
    24u16 => &FOOTMANS_FORTUNE,
    25u16 => &DESERT_ROSE,
    26u16 => &PRISONERS_RAGS,
    27u16 => &FIORDS_LEGACY,
    28u16 => &BARKSKIN,
    29u16 => &SERGEANTS_MAIL,
    30u16 => &THUNDERBUGS_CARAPACE,
    31u16 => &SILKS_OF_THE_SUN,
    32u16 => &HEALERS_HABIT,
    33u16 => &VIPERS_STING,
    34u16 => &NIGHT_MOTHERS_EMBRACE,
    35u16 => &KNIGHTMARE,
    36u16 => &ARMOR_OF_THE_VEILED_HERITANCE,
    37u16 => &DEATHS_WIND,
    38u16 => &TWILIGHTS_EMBRACE,
    39u16 => &ALESSIAN_ORDER,
    40u16 => &NIGHTS_SILENCE,
    41u16 => &WHITESTRAKES_RETRIBUTION,
    43u16 => &ARMOR_OF_THE_SEDUCER,
    44u16 => &VAMPIRES_KISS,
    46u16 => &NOBLE_DUELISTS_SILKS,
    47u16 => &ROBES_OF_THE_WITHERED_HAND,
    48u16 => &MAGNUS_GIFT,
    49u16 => &SHADOW_OF_THE_RED_MOUNTAIN,
    50u16 => &THE_MORAG_TONG,
    51u16 => &NIGHT_MOTHERS_GAZE,
    52u16 => &BECKONING_STEEL,
    53u16 => &THE_ICE_FURNACE,
    54u16 => &ASHEN_GRIP,
    55u16 => &PRAYER_SHAWL,
    56u16 => &STENDARRS_EMBRACE,
    57u16 => &SYRABANES_GRIP,
    58u16 => &HIDE_OF_THE_WEREWOLF,
    59u16 => &KYNES_KISS,
    60u16 => &DARKSTRIDE,
    61u16 => &DREUGH_KING_SLAYER,
    62u16 => &HATCHLINGS_SHELL,
    63u16 => &THE_JUGGERNAUT,
    64u16 => &SHADOW_DANCERS_RAIMENT,
    65u16 => &BLOODTHORNS_TOUCH,
    66u16 => &ROBES_OF_THE_HIST,
    67u16 => &SHADOW_WALKER,
    68u16 => &STYGIAN,
    69u16 => &RANGERS_GAIT,
    70u16 => &SEVENTH_LEGION_BRUTE,
    71u16 => &DUROKS_BANE,
    72u16 => &NIKULAS_HEAVY_ARMOR,
    73u16 => &OBLIVIONS_FOE,
    74u16 => &SPECTRES_EYE,
    75u16 => &TORUGS_PACT,
    76u16 => &ROBES_OF_ALTERATION_MASTERY,
    77u16 => &CRUSADER,
    78u16 => &HIST_BARK,
    79u16 => &WILLOWS_PATH,
    80u16 => &HUNDINGS_RAGE,
    81u16 => &SONG_OF_LAMAE,
    82u16 => &ALESSIAS_BULWARK,
    83u16 => &ELF_BANE,
    84u16 => &ORGNUMS_SCALES,
    85u16 => &ALMALEXIAS_MERCY,
    86u16 => &QUEENS_ELEGANCE,
    87u16 => &EYES_OF_MARA,
    88u16 => &ROBES_OF_DESTRUCTION_MASTERY,
    89u16 => &SENTRY,
    90u16 => &SENCHES_BITE,
    91u16 => &OBLIVIONS_EDGE,
    92u16 => &KAGRENACS_HOPE,
    93u16 => &STORM_KNIGHTS_PLATE,
    94u16 => &MERIDIAS_BLESSED_ARMOR,
    95u16 => &SHALIDORS_CURSE,
    96u16 => &ARMOR_OF_TRUTH,
    97u16 => &THE_ARCH_MAGE,
    98u16 => &NECROPOTENCE,
    99u16 => &SALVATION,
    100u16 => &HAWKS_EYE,
    101u16 => &AFFLICTION,
    102u16 => &DUNERIPPERS_SCALES,
    103u16 => &MAGICKA_FURNACE,
    104u16 => &CURSE_EATER,
    105u16 => &TWIN_SISTERS,
    106u16 => &WILDERQUEENS_ARCH,
    107u16 => &WYRD_TREES_BLESSING,
    108u16 => &RAVAGER,
    109u16 => &LIGHT_OF_CYRODIIL,
    110u16 => &SANCTUARY,
    111u16 => &WARD_OF_CYRODIIL,
    112u16 => &NIGHT_TERROR,
    113u16 => &CREST_OF_CYRODIIL,
    114u16 => &SOULSHINE,
    116u16 => &THE_DESTRUCTION_SUITE,
    117u16 => &RELICS_OF_THE_PHYSICIAN_ANSUR,
    118u16 => &TREASURES_OF_THE_EARTHFORGE,
    119u16 => &RELICS_OF_THE_REBELLION,
    120u16 => &ARMS_OF_INFERNACE,
    121u16 => &ARMS_OF_THE_ANCESTORS,
    122u16 => &EBON_ARMORY,
    123u16 => &HIRCINES_VENEER,
    124u16 => &THE_WORMS_RAIMENT,
    125u16 => &WRATH_OF_THE_IMPERIUM,
    126u16 => &GRACE_OF_THE_ANCIENTS,
    127u16 => &DEADLY_STRIKE,
    128u16 => &BLESSING_OF_THE_POTENTATES,
    129u16 => &VENGEANCE_LEECH,
    130u16 => &EAGLE_EYE,
    131u16 => &BASTION_OF_THE_HEARTLAND,
    132u16 => &SHIELD_OF_THE_VALIANT,
    133u16 => &BUFFER_OF_THE_SWIFT,
    134u16 => &SHROUD_OF_THE_LICH,
    135u16 => &DRAUGRS_HERITAGE,
    136u16 => &IMMORTAL_WARRIOR,
    137u16 => &BERSERKING_WARRIOR,
    138u16 => &DEFENDING_WARRIOR,
    139u16 => &WISE_MAGE,
    140u16 => &DESTRUCTIVE_MAGE,
    141u16 => &HEALING_MAGE,
    142u16 => &QUICK_SERPENT,
    143u16 => &POISONOUS_SERPENT,
    144u16 => &TWICE_FANGED_SERPENT,
    145u16 => &WAY_OF_FIRE,
    146u16 => &WAY_OF_AIR,
    147u16 => &WAY_OF_MARTIAL_KNOWLEDGE,
    148u16 => &WAY_OF_THE_ARENA,
    155u16 => &UNDAUNTED_BASTION,
    156u16 => &UNDAUNTED_INFILTRATOR,
    157u16 => &UNDAUNTED_UNWEAVER,
    158u16 => &EMBERSHIELD,
    159u16 => &SUNDERFLAME,
    160u16 => &BURNING_SPELLWEAVE,
    161u16 => &TWICE_BORN_STAR,
    162u16 => &SPAWN_OF_MEPHALA,
    163u16 => &BLOODSPAWN,
    164u16 => &LORD_WARDEN,
    165u16 => &SCOURGE_HARVESTER,
    166u16 => &ENGINE_GUARDIAN,
    167u16 => &NIGHTFLAME,
    168u16 => &NERIENETH,
    169u16 => &VALKYN_SKORIA,
    170u16 => &MAW_OF_THE_INFERNAL,
    171u16 => &ETERNAL_WARRIOR,
    172u16 => &INFALLIBLE_MAGE,
    173u16 => &VICIOUS_SERPENT,
    176u16 => &NOBLES_CONQUEST,
    177u16 => &REDISTRIBUTOR,
    178u16 => &ARMOR_MASTER,
    179u16 => &BLACK_ROSE,
    180u16 => &POWERFUL_ASSAULT,
    181u16 => &MERITORIOUS_SERVICE,
    183u16 => &MOLAG_KENA,
    184u16 => &BRANDS_OF_IMPERIUM,
    185u16 => &SPELL_POWER_CURE,
    186u16 => &JOLTING_ARMS,
    187u16 => &SWAMP_RAIDER,
    188u16 => &STORM_MASTER,
    190u16 => &SCATHING_MAGE,
    193u16 => &OVERWHELMING_SURGE,
    194u16 => &COMBAT_PHYSICIAN,
    195u16 => &SHEER_VENOM,
    196u16 => &LEECHING_PLATE,
    197u16 => &TORMENTOR,
    198u16 => &ESSENCE_THIEF,
    199u16 => &SHIELD_BREAKER,
    200u16 => &PHOENIX,
    201u16 => &REACTIVE_ARMOR,
    204u16 => &ENDURANCE,
    205u16 => &WILLPOWER,
    206u16 => &AGILITY,
    207u16 => &LAW_OF_JULIANOS,
    208u16 => &TRIAL_BY_FIRE,
    209u16 => &ARMOR_OF_THE_CODE,
    210u16 => &MARK_OF_THE_PARIAH,
    211u16 => &PERMAFROST,
    212u16 => &BRIARHEART,
    213u16 => &GLORIOUS_DEFENDER,
    214u16 => &PARA_BELLUM,
    215u16 => &ELEMENTAL_SUCCESSION,
    216u16 => &HUNT_LEADER,
    217u16 => &WINTERBORN,
    218u16 => &TRINIMACS_VALOR,
    219u16 => &MORKULDIN,
    224u16 => &TAVAS_FAVOR,
    225u16 => &CLEVER_ALCHEMIST,
    226u16 => &ETERNAL_HUNT,
    227u16 => &BAHRAHAS_CURSE,
    228u16 => &SYVARRAS_SCALES,
    229u16 => &TWILIGHT_REMEDY,
    230u16 => &MOONDANCER,
    231u16 => &LUNAR_BASTION,
    232u16 => &ROAR_OF_ALKOSH,
    234u16 => &MARKSMANS_CREST,
    235u16 => &ROBES_OF_TRANSMUTATION,
    236u16 => &VICIOUS_DEATH,
    237u16 => &LEKIS_FOCUS,
    238u16 => &FASALLAS_GUILE,
    239u16 => &WARRIORS_FURY,
    240u16 => &KVATCH_GLADIATOR,
    241u16 => &VARENS_LEGACY,
    242u16 => &PELINALS_WRATH,
    243u16 => &HIDE_OF_MORIHAUS,
    244u16 => &FLANKING_STRATEGIST,
    245u16 => &SITHIS_TOUCH,
    246u16 => &GALERIONS_REVENGE,
    247u16 => &VICECANON_OF_VENOM,
    248u16 => &THEWS_OF_THE_HARBINGER,
    253u16 => &IMPERIAL_PHYSIQUE,
    256u16 => &MIGHTY_CHUDAN,
    257u16 => &VELIDRETH,
    258u16 => &AMBER_PLASM,
    259u16 => &HEEM_JAS_RETRIBUTION,
    260u16 => &ASPECT_OF_MAZZATUN,
    261u16 => &GOSSAMER,
    262u16 => &WIDOWMAKER,
    263u16 => &HAND_OF_MEPHALA,
    264u16 => &GIANT_SPIDER,
    265u16 => &SHADOWREND,
    266u16 => &KRAGH,
    267u16 => &SWARM_MOTHER,
    268u16 => &SENTINEL_OF_RKUGAMZ,
    269u16 => &CHOKETHORN,
    270u16 => &SLIMECRAW,
    271u16 => &SELLISTRIX,
    272u16 => &INFERNAL_GUARDIAN,
    273u16 => &ILAMBRIS,
    274u16 => &ICEHEART,
    275u16 => &STORMFIST,
    276u16 => &TREMORSCALE,
    277u16 => &PIRATE_SKELETON,
    278u16 => &THE_TROLL_KING,
    279u16 => &SELENE,
    280u16 => &GROTHDARR,
    281u16 => &ARMOR_OF_THE_TRAINEE,
    282u16 => &VAMPIRE_CLOAK,
    283u16 => &SWORD_SINGER,
    284u16 => &ORDER_OF_DIAGNA,
    285u16 => &VAMPIRE_LORD,
    286u16 => &SPRIGGANS_THORNS,
    287u16 => &GREEN_PACT,
    288u16 => &BEEKEEPERS_GEAR,
    289u16 => &SPINNERS_GARMENTS,
    290u16 => &SKOOMA_SMUGGLER,
    291u16 => &SHALK_EXOSKELETON,
    292u16 => &MOTHERS_SORROW,
    293u16 => &PLAGUE_DOCTOR,
    294u16 => &YSGRAMORS_BIRTHRIGHT,
    295u16 => &JAILBREAKER,
    296u16 => &SPELUNKER,
    297u16 => &SPIDER_CULTIST_COWL,
    298u16 => &LIGHT_SPEAKER,
    299u16 => &TOOTHROW,
    300u16 => &NETCHS_TOUCH,
    301u16 => &STRENGTH_OF_THE_AUTOMATON,
    302u16 => &LEVIATHAN,
    303u16 => &LAMIAS_SONG,
    304u16 => &MEDUSA,
    305u16 => &TREASURE_HUNTER,
    307u16 => &DRAUGR_HULK,
    308u16 => &BONE_PIRATES_TATTERS,
    309u16 => &KNIGHT_ERRANTS_MAIL,
    310u16 => &SWORD_DANCER,
    311u16 => &RATTLECAGE,
    313u16 => &TITANIC_CLEAVE,
    314u16 => &PUNCTURING_REMEDY,
    315u16 => &STINGING_SLASHES,
    316u16 => &CAUSTIC_ARROW,
    317u16 => &DESTRUCTIVE_IMPACT,
    318u16 => &GRAND_REJUVENATION,
    320u16 => &WAR_MAIDEN,
    321u16 => &DEFILER,
    322u16 => &WARRIOR_POET,
    323u16 => &ASSASSINS_GUILE,
    324u16 => &DAEDRIC_TRICKERY,
    325u16 => &SHACKLEBREAKER,
    326u16 => &VANGUARDS_CHALLENGE,
    327u16 => &COWARDS_GEAR,
    328u16 => &KNIGHT_SLAYER,
    329u16 => &WIZARDS_RIPOSTE,
    330u16 => &AUTOMATED_DEFENSE,
    331u16 => &WAR_MACHINE,
    332u16 => &MASTER_ARCHITECT,
    333u16 => &INVENTORS_GUARD,
    334u16 => &IMPREGNABLE_ARMOR,
    335u16 => &DRAUGRS_REST,
    336u16 => &PILLAR_OF_NIRN,
    337u16 => &IRONBLOOD,
    338u16 => &FLAME_BLOSSOM,
    339u16 => &BLOODDRINKER,
    340u16 => &HAGRAVENS_GARDEN,
    341u16 => &EARTHGORE,
    342u16 => &DOMIHAUS,
    343u16 => &CALUURIONS_LEGACY,
    344u16 => &TRAPPINGS_OF_INVIGORATION,
    345u16 => &ULFNORS_FAVOR,
    346u16 => &JORVULDS_GUIDANCE,
    347u16 => &PLAGUE_SLINGER,
    348u16 => &CURSE_OF_DOYLEMISH,
    349u16 => &THURVOKUN,
    350u16 => &ZAAN,
    351u16 => &INNATE_AXIOM,
    352u16 => &FORTIFIED_BRASS,
    353u16 => &MECHANICAL_ACUITY,
    354u16 => &MAD_TINKERER,
    355u16 => &UNFATHOMABLE_DARKNESS,
    356u16 => &LIVEWIRE,
    357u16 => &PERFECTED_DISCIPLINED_SLASH,
    358u16 => &PERFECTED_DEFENSIVE_POSITION,
    359u16 => &PERFECTED_CHAOTIC_WHIRLWIND,
    360u16 => &PERFECTED_PIERCING_SPRAY,
    361u16 => &PERFECTED_CONCENTRATED_FORCE,
    362u16 => &PERFECTED_TIMELESS_BLESSING,
    363u16 => &DISCIPLINED_SLASH,
    364u16 => &DEFENSIVE_POSITION,
    365u16 => &CHAOTIC_WHIRLWIND,
    366u16 => &PIERCING_SPRAY,
    367u16 => &CONCENTRATED_FORCE,
    368u16 => &TIMELESS_BLESSING,
    369u16 => &MERCILESS_CHARGE,
    370u16 => &RAMPAGING_SLASH,
    371u16 => &CRUEL_FLURRY,
    372u16 => &THUNDEROUS_VOLLEY,
    373u16 => &CRUSHING_WALL,
    374u16 => &PRECISE_REGENERATION,
    380u16 => &PROPHETS,
    381u16 => &BROKEN_SOUL,
    382u16 => &GRACE_OF_GLOOM,
    383u16 => &GRYPHONS_FEROCITY,
    384u16 => &WISDOM_OF_VANUS,
    385u16 => &ADEPT_RIDER,
    386u16 => &SLOADS_SEMBLANCE,
    387u16 => &NOCTURNALS_FAVOR,
    388u16 => &AEGIS_OF_GALENWE,
    389u16 => &ARMS_OF_RELEQUEN,
    390u16 => &MANTLE_OF_SIRORIA,
    391u16 => &VESTMENT_OF_OLORIME,
    392u16 => &PERFECTED_AEGIS_OF_GALENWE,
    393u16 => &PERFECTED_ARMS_OF_RELEQUEN,
    394u16 => &PERFECTED_MANTLE_OF_SIRORIA,
    395u16 => &PERFECTED_VESTMENT_OF_OLORIME,
    397u16 => &BALORGH,
    398u16 => &VYKOSA,
    399u16 => &HANUS_COMPASSION,
    400u16 => &BLOOD_MOON,
    401u16 => &HAVEN_OF_URSUS,
    402u16 => &MOON_HUNTER,
    403u16 => &SAVAGE_WEREWOLF,
    404u16 => &JAILERS_TENACITY,
    405u16 => &BRIGHT_THROATS_BOAST,
    406u16 => &DEAD_WATERS_GUILE,
    407u16 => &CHAMPION_OF_THE_HIST,
    408u16 => &GRAVE_STAKE_COLLECTOR,
    409u16 => &NAGA_SHAMAN,
    410u16 => &MIGHT_OF_THE_LOST_LEGION,
    411u16 => &GALLANT_CHARGE,
    412u16 => &RADIAL_UPPERCUT,
    413u16 => &SPECTRAL_CLOAK,
    414u16 => &VIRULENT_SHOT,
    415u16 => &WILD_IMPULSE,
    416u16 => &MENDERS_WARD,
    417u16 => &INDOMITABLE_FURY,
    418u16 => &SPELL_STRATEGIST,
    419u16 => &BATTLEFIELD_ACROBAT,
    420u16 => &SOLDIER_OF_ANGUISH,
    421u16 => &STEADFAST_HERO,
    422u16 => &BATTALION_DEFENDER,
    423u16 => &PERFECTED_GALLANT_CHARGE,
    424u16 => &PERFECTED_RADIAL_UPPERCUT,
    425u16 => &PERFECTED_SPECTRAL_CLOAK,
    426u16 => &PERFECTED_VIRULENT_SHOT,
    427u16 => &PERFECTED_WILD_IMPULSE,
    428u16 => &PERFECTED_MENDERS_WARD,
    429u16 => &MIGHTY_GLACIER,
    430u16 => &TZOGVINS_WARBAND,
    431u16 => &ICY_CONJURER,
    432u16 => &STONEKEEPER,
    433u16 => &FROZEN_WATCHER,
    434u16 => &SCAVENGING_DEMISE,
    435u16 => &AURORANS_THUNDER,
    436u16 => &SYMPHONY_OF_BLADES,
    437u16 => &COLDHARBOURS_FAVORITE,
    438u16 => &SENCHE_RAHTS_GRIT,
    439u16 => &VASTARIES_TUTELAGE,
    440u16 => &CRAFTY_ALFIQ,
    441u16 => &VESTURE_OF_DARLOC_BRAE,
    442u16 => &CALL_OF_THE_UNDERTAKER,
    443u16 => &EYE_OF_NAHVIINTAAS,
    444u16 => &FALSE_GODS_DEVOTION,
    445u16 => &TOOTH_OF_LOKKESTIIZ,
    446u16 => &CLAW_OF_YOLNAHKRIIN,
    448u16 => &PERFECTED_EYE_OF_NAHVIINTAAS,
    449u16 => &PERFECTED_FALSE_GODS_DEVOTION,
    450u16 => &PERFECTED_TOOTH_OF_LOKKESTIIZ,
    451u16 => &PERFECTED_CLAW_OF_YOLNAHKRIIN,
    452u16 => &HOLLOWFANG_THIRST,
    453u16 => &DROZAKARS_CLAWS,
    454u16 => &RENALDS_RESOLVE,
    455u16 => &ZENS_REDRESS,
    456u16 => &AZUREBLIGHT_REAPER,
    457u16 => &DRAGONS_DEFILEMENT,
    458u16 => &GRUNDWULF,
    459u16 => &MAARSELOK,
    465u16 => &SENCHAL_DEFENDER,
    466u16 => &MARAUDERS_HASTE,
    467u16 => &DRAGONGUARD_ELITE,
    468u16 => &DARING_CORSAIR,
    469u16 => &ANCIENT_DRAGONGUARD,
    470u16 => &NEW_MOON_ACOLYTE,
    471u16 => &HITIS_HEARTH,
    472u16 => &TITANBORN_STRENGTH,
    473u16 => &BANIS_TORMENT,
    474u16 => &DRAUGRKINS_GRIP,
    475u16 => &AEGIS_CALLER,
    476u16 => &GRAVE_GUARDIAN,
    478u16 => &MOTHER_CIANNAIT,
    479u16 => &KJALNARS_NIGHTMARE,
    480u16 => &CRITICAL_RIPOSTE,
    481u16 => &UNCHAINED_AGGRESSOR,
    482u16 => &DAUNTLESS_COMBATANT,
    487u16 => &WINTERS_RESPITE,
    488u16 => &VENOMOUS_SMITE,
    489u16 => &ETERNAL_VIGOR,
    490u16 => &STUHNS_FAVOR,
    491u16 => &DRAGONS_APPETITE,
    492u16 => &KYNES_WIND,
    493u16 => &PERFECTED_KYNES_WIND,
    494u16 => &VROLS_COMMAND,
    495u16 => &PERFECTED_VROLS_COMMAND,
    496u16 => &ROARING_OPPORTUNIST,
    497u16 => &PERFECTED_ROARING_OPPORTUNIST,
    498u16 => &YANDIRS_MIGHT,
    499u16 => &PERFECTED_YANDIRS_MIGHT,
    501u16 => &THRASSIAN_STRANGLERS,
    503u16 => &RING_OF_THE_WILD_HUNT,
    505u16 => &TORC_OF_TONAL_CONSTANCY,
    506u16 => &SPELL_PARASITE,
    513u16 => &TALFYGS_TREACHERY,
    514u16 => &UNLEASHED_TERROR,
    515u16 => &CRIMSON_TWILIGHT,
    516u16 => &ELEMENTAL_CATALYST,
    517u16 => &KRAGLENS_HOWL,
    518u16 => &ARKASIS_GENIUS,
    519u16 => &SNOW_TREADERS,
    520u16 => &MALACATHS_BAND_OF_BRUTALITY,
    521u16 => &BLOODLORDS_EMBRACE,
    522u16 => &PERFECTED_MERCILESS_CHARGE,
    523u16 => &PERFECTED_RAMPAGING_SLASH,
    524u16 => &PERFECTED_CRUEL_FLURRY,
    525u16 => &PERFECTED_THUNDEROUS_VOLLEY,
    526u16 => &PERFECTED_CRUSHING_WALL,
    527u16 => &PERFECTED_PRECISE_REGENERATION,
    528u16 => &PERFECTED_TITANIC_CLEAVE,
    529u16 => &PERFECTED_PUNCTURING_REMEDY,
    530u16 => &PERFECTED_STINGING_SLASHES,
    531u16 => &PERFECTED_CAUSTIC_ARROW,
    532u16 => &PERFECTED_DESTRUCTIVE_IMPACT,
    533u16 => &PERFECTED_GRAND_REJUVENATION,
    534u16 => &STONE_HUSK,
    535u16 => &LADY_THORN,
    536u16 => &RADIANT_BASTION,
    537u16 => &VOIDCALLER,
    538u16 => &WITCH_KNIGHTS_DEFIANCE,
    539u16 => &RED_EAGLES_FURY,
    540u16 => &LEGACY_OF_KARTH,
    541u16 => &AETHERIAL_ASCENSION,
    542u16 => &HEX_SIPHON,
    543u16 => &PESTILENT_HOST,
    544u16 => &EXPLOSIVE_REBUKE,
    557u16 => &EXECUTIONERS_BLADE,
    558u16 => &VOID_BASH,
    559u16 => &FRENZIED_MOMENTUM,
    560u16 => &POINT_BLANK_SNIPE,
    561u16 => &WRATH_OF_ELEMENTS,
    562u16 => &FORCE_OVERFLOW,
    563u16 => &PERFECTED_EXECUTIONERS_BLADE,
    564u16 => &PERFECTED_VOID_BASH,
    565u16 => &PERFECTED_FRENZIED_MOMENTUM,
    566u16 => &PERFECTED_POINT_BLANK_SNIPE,
    567u16 => &PERFECTED_WRATH_OF_ELEMENTS,
    568u16 => &PERFECTED_FORCE_OVERFLOW,
    569u16 => &TRUE_SWORN_FURY,
    570u16 => &KINRAS_WRATH,
    571u16 => &DRAKES_RUSH,
    572u16 => &UNLEASHED_RITUALIST,
    573u16 => &DAGONS_DOMINION,
    574u16 => &FOOLKILLERS_WARD,
    575u16 => &RING_OF_THE_PALE_ORDER,
    576u16 => &PEARLS_OF_EHLNOFEY,
    577u16 => &ENCRATIS_BEHEMOTH,
    578u16 => &BARON_ZAUDRUS,
    579u16 => &FROSTBITE,
    580u16 => &DEADLANDS_ASSASSIN,
    581u16 => &BOG_RAIDER,
    582u16 => &HIST_WHISPERER,
    583u16 => &HEARTLAND_CONQUEROR,
    584u16 => &DIAMONDS_VICTORY,
    585u16 => &SAXHLEEL_CHAMPION,
    586u16 => &SUL_XANS_TORMENT,
    587u16 => &BAHSEIS_MANIA,
    588u16 => &STONE_TALKERS_OATH,
    589u16 => &PERFECTED_SAXHLEEL_CHAMPION,
    590u16 => &PERFECTED_SUL_XANS_TORMENT,
    591u16 => &PERFECTED_BAHSEIS_MANIA,
    592u16 => &PERFECTED_STONE_TALKERS_OATH,
    593u16 => &GAZE_OF_SITHIS,
    594u16 => &HARPOONERS_WADING_KILT,
    596u16 => &DEATH_DEALERS_FETE,
    597u16 => &SHAPESHIFTERS_CHAIN,
    598u16 => &ZOAL_THE_EVER_WAKEFUL,
    599u16 => &IMMOLATOR_CHARR,
    600u16 => &GLORGOLOCH_THE_DESTROYER,
    602u16 => &CRIMSON_OATHS_RIVE,
    603u16 => &SCORIONS_FEAST,
    604u16 => &RUSH_OF_AGONY,
    605u16 => &SILVER_ROSE_VIGIL,
    606u16 => &THUNDER_CALLER,
    607u16 => &GRISLY_GOURMET,
    608u16 => &PRIOR_THIERRIC,
    609u16 => &MAGMA_INCARNATE,
    610u16 => &WRETCHED_VITALITY,
    611u16 => &DEADLANDS_DEMOLISHER,
    612u16 => &IRON_FLASK,
    613u16 => &EYE_OF_THE_GRASP,
    614u16 => &HEXOS_WARD,
    615u16 => &KYNMARCHERS_CRUELTY,
    616u16 => &DARK_CONVERGENCE,
    617u16 => &PLAGUEBREAK,
    618u16 => &HROTHGARS_CHILL,
    619u16 => &MALIGALIGS_MAELSTROM,
    620u16 => &GRYPHONS_REPRISAL,
    621u16 => &GLACIAL_GUARDIAN,
    622u16 => &TURNING_TIDE,
    623u16 => &STORM_CURSEDS_REVENGE,
    624u16 => &SPRIGGANS_VIGOR,
    625u16 => &MARKYN_RING_OF_MAJESTY,
    626u16 => &BELHARZAS_BAND,
    627u16 => &SPAULDER_OF_RUIN,
    629u16 => &RALLYING_CRY,
    630u16 => &HEW_AND_SUNDER,
    631u16 => &ENERVATING_AURA,
    632u16 => &KARGAEDA,
    633u16 => &NAZARAY,
    634u16 => &NUNATAK,
    635u16 => &LADY_MALYGDA,
    636u16 => &BARON_THIRSK,
    640u16 => &ORDERS_WRATH,
    641u16 => &SERPENTS_DISDAIN,
    642u16 => &DRUIDS_BRAID,
    643u16 => &BLESSING_OF_HIGH_ISLE,
    644u16 => &STEADFASTS_METTLE,
    645u16 => &SYSTRES_SCOWL,
    646u16 => &WHORL_OF_THE_DEPTHS,
    647u16 => &CORAL_RIPTIDE,
    648u16 => &PEARLESCENT_WARD,
    649u16 => &PILLAGERS_PROFIT,
    650u16 => &PERFECTED_PILLAGERS_PROFIT,
    651u16 => &PERFECTED_PEARLESCENT_WARD,
    652u16 => &PERFECTED_CORAL_RIPTIDE,
    653u16 => &PERFECTED_WHORL_OF_THE_DEPTHS,
    654u16 => &MORAS_WHISPERS,
    655u16 => &DOV_RHA_SABATONS,
    656u16 => &LEFTHANDERS_AEGIS_BELT,
    657u16 => &SEA_SERPENTS_COIL,
    658u16 => &OAKENSOUL_RING,
    660u16 => &DEEPROOT_ZEAL,
    661u16 => &STONES_ACCORD,
    662u16 => &RAGE_OF_THE_URSAUK,
    663u16 => &PANGRIT_DENMOTHER,
    664u16 => &GRAVE_INEVITABILITY,
    665u16 => &PHYLACTERYS_GRASP,
    666u16 => &ARCHDRUID_DEVYRIC,
    667u16 => &EUPHOTIC_GATEKEEPER,
    668u16 => &LANGUOR_OF_PERYITE,
    669u16 => &NOCTURNALS_PLOY,
    670u16 => &MARAS_BALM,
    671u16 => &BACK_ALLEY_GOURMAND,
    672u16 => &PHOENIX_MOTH_THEURGE,
    673u16 => &BASTION_OF_THE_DRAOIFE,
    674u16 => &FAUNS_LARK_CLADDING,
    675u16 => &STORMWEAVERS_CAVORT,
    676u16 => &SYRABANES_WARD,
    677u16 => &CHIMERAS_REBUKE,
    678u16 => &OLD_GROWTH_BREWER,
    679u16 => &CLAW_OF_THE_FOREST_WRAITH,
    680u16 => &RITEMASTERS_BOND,
    681u16 => &NIX_HOUNDS_HOWL,
    682u16 => &TELVANNI_ENFORCER,
    683u16 => &ROKSA_THE_WARPED,
    684u16 => &RUNECARVERS_BLAZE,
    685u16 => &APOCRYPHAL_INSPIRATION,
    686u16 => &ABYSSAL_BRACE,
    687u16 => &OZEZAN_THE_INFERNO,
    688u16 => &SNAKE_IN_THE_STARS,
    689u16 => &SHELL_SPLITTER,
    690u16 => &JUDGMENT_OF_AKATOSH,
    691u16 => &CRYPTCANON_VESTMENTS,
    692u16 => &ESOTERIC_ENVIRONMENT_GREAVES,
    693u16 => &TORC_OF_THE_LAST_AYLEID_KING,
    694u16 => &VELOTHI_UR_MAGES_AMULET,
    695u16 => &SHATTERED_FATE,
    696u16 => &TELVANNI_EFFICIENCY,
    697u16 => &SEEKER_SYNTHESIS,
    698u16 => &VIVECS_DUALITY,
    699u16 => &CAMONNA_TONG,
    700u16 => &ADAMANT_LURKER,
    701u16 => &PEACE_AND_SERENITY,
    702u16 => &ANSUULS_TORMENT,
    703u16 => &TEST_OF_RESOLVE,
    704u16 => &TRANSFORMATIVE_HOPE,
    705u16 => &PERFECTED_TRANSFORMATIVE_HOPE,
    706u16 => &PERFECTED_TEST_OF_RESOLVE,
    707u16 => &PERFECTED_ANSUULS_TORMENT,
    708u16 => &PERFECTED_PEACE_AND_SERENITY,
    711u16 => &COLOVIAN_HIGHLANDS_GENERAL,
    712u16 => &JERALL_MOUNTAINS_WARCHIEF,
    713u16 => &NIBENAY_BAY_BATTLEREEVE,
    722u16 => &REAWAKENED_HIEROPHANT,
    723u16 => &BASALT_BLOODED_WARRIOR,
    724u16 => &NOBILITY_IN_DECAY,
    726u16 => &SOULCLEAVER,
    727u16 => &MONOLITH_OF_STORMS,
    728u16 => &WRATHSUN,
    729u16 => &GARDENER_OF_SEASONS,
    730u16 => &CINDERS_OF_ANTHELMIR,
    731u16 => &SLUTHRUGS_HUNGER,
    732u16 => &BLACK_GLOVE_GROUNDING,
    734u16 => &ANTHELMIRS_CONSTRUCT,
    735u16 => &BLIND_PATH_INDUCTION,
    736u16 => &TARNISHED_NIGHTMARE,
    737u16 => &REFLECTED_FURY,
    738u16 => &THE_BLIND,
    754u16 => &OAKFATHERS_RETRIBUTION,
    755u16 => &BLUNTED_BLADES,
    756u16 => &BAAN_DARS_BLESSING,
    757u16 => &SYMMETRY_OF_THE_WEALD,
    758u16 => &MACABRE_VINTAGE,
    759u16 => &AYLEID_REFUGE,
    760u16 => &ROURKEN_STEAMGUARDS,
    761u16 => &THE_SHADOW_QUEENS_COWL,
    762u16 => &THE_SAINT_AND_THE_SEDUCER,
    763u16 => &THARRIKERS_STRIKE,
    764u16 => &HIGHLAND_SENTINEL,
    765u16 => &THREADS_OF_WAR,
    766u16 => &MORA_SCRIBES_THESIS,
    767u16 => &SLIVERS_OF_THE_NULL_ARCA,
    768u16 => &LUCENT_ECHOES,
    769u16 => &XORYNS_MASTERPIECE,
    770u16 => &PERFECTED_XORYNS_MASTERPIECE,
    771u16 => &PERFECTED_LUCENT_ECHOES,
    772u16 => &PERFECTED_SLIVERS_OF_THE_NULL_ARCA,
    773u16 => &PERFECTED_MORA_SCRIBES_THESIS,
    775u16 => &SPATTERING_DISJUNCTION,
    776u16 => &PYREBRAND,
    777u16 => &CORPSEBURSTER,
    778u16 => &UMBRAL_EDGE,
    779u16 => &BEACON_OF_OBLIVION,
    780u16 => &AETHERIC_LANCER,
    781u16 => &AERIES_CRY,
    782u16 => &TRACKERS_LASH,
    783u16 => &SHARED_PAIN,
    784u16 => &SIEGEMASTERS_FOCUS,
    791u16 => &BULWARK_RUINATION,
    792u16 => &FARSTRIDER,
    793u16 => &NETCH_OIL,
    794u16 => &VANDORALLENS_RESONANCE,
    795u16 => &JERENSIS_BLADESTORM,
    796u16 => &LUCILLAS_WINDSHIELD,
    797u16 => &SQUALL_OF_RETRIBUTION,
    798u16 => &HEROIC_UNITY,
    799u16 => &FLEDGLINGS_NEST,
    800u16 => &NOXIOUS_BOULDER,
    801u16 => &ORPHEON_THE_TACTICIAN,
    802u16 => &ARKAYS_CHARITY,
    803u16 => &LAMP_KNIGHTS_ART,
    804u16 => &BLACKFEATHER_FLIGHT,
    805u16 => &THREE_QUEENS_WELLSPRING,
    806u16 => &DEATH_DANCER,
    807u16 => &FULL_BELLY_BARRICADE,
    808u16 => &SHARED_BURDEN,
    809u16 => &TIDE_BORN_WILDSTALKER,
    810u16 => &FELLOWSHIPS_FORTITUDE,
    811u16 => &MAD_GODS_DANCING_SHOES,
    812u16 => &RAKKHATS_VOIDMANTLE,
    813u16 => &MONOMYTH_REFORGED,
    814u16 => &HARMONY_IN_CHAOS,
    815u16 => &KAZPIANS_CRUEL_SIGNET,
    816u16 => &DOLOROUS_ARENA,
    817u16 => &RECOVERY_CONVERGENCE,
    818u16 => &PERFECTED_RECOVERY_CONVERGENCE,
    819u16 => &PERFECTED_DOLOROUS_ARENA,
    820u16 => &PERFECTED_KAZPIANS_CRUEL_SIGNET,
    821u16 => &PERFECTED_HARMONY_IN_CHAOS,
    822u16 => &LUSTROUS_SOULWELL,
    823u16 => &VYKANDS_SOULFURY,
    824u16 => &BLACK_FOUNDRY_STEEL,
    825u16 => &XANMEER_SPELLWEAVER,
    826u16 => &TOOLS_OF_THE_TRAPMASTER,
    827u16 => &STONEHULK_DOMINATION,
    828u16 => &BLACK_GEM_MONSTROSITY,
    829u16 => &BAR_SAKKA,
    830u16 => &SPELLSHREDDER,
    831u16 => &COUP_DE_GRACE,
    832u16 => &UNFLINCHING_ULTIMATE,
    845u16 => &HUNTSMANS_WARMASK,
    846u16 => &XANMEER_GENESIS,
    848u16 => &SHATTERED_PATHS_SIGNET,
    849u16 => &GLITTERING_GOAD,
    850u16 => &THOUSAND_EYES,
    851u16 => &THE_RUCKUS,
    855u16 => &GORETHIEF,
};

static ABYSSAL_BRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1710 as f64 }],
    ],
};

static ADAMANT_LURKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static ADEPT_RIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static AEGIS_CALLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static AEGIS_OF_GALENWE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};
// GRANT_STUB: ADAMANT_LURKER -> MinorMinorAegis @ 3pc
static AERIES_CRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static AETHERIAL_ASCENSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 7377 as f64 }],
    ],
};

static AETHERIC_LANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static AFFLICTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static AGILITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1752 as f64 }],
        &[SetBonus { channel: Channel::Power, value: 206 as f64 }]
    ],
};

static AKAVIRI_DRAGONGUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static ALESSIAS_BULWARK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static ALESSIAN_ORDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static ALMALEXIAS_MERCY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static AMBER_PLASM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: 245 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 245 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: 245 as f64 }],
    ],
};

static ANCIENT_DRAGONGUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ANSUULS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};
// GRANT_STUB: AERIES_CRY -> MinorMinorSlayer @ 3pc
static ANTHELMIRS_CONSTRUCT: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static APOCRYPHAL_INSPIRATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ARCHDRUID_DEVYRIC: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static ARCHERS_MIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ARKASIS_GENIUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static ARKAYS_CHARITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ARMOR_MASTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static ARMOR_OF_THE_CODE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ARMOR_OF_THE_SEDUCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static ARMOR_OF_THE_TRAINEE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1454 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1454 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1454 as f64 }],
    ],
};

static ARMOR_OF_THE_VEILED_HERITANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static ARMOR_OF_TRUTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ARMS_OF_INFERNACE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ARMS_OF_RELEQUEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static ARMS_OF_THE_ANCESTORS: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static ASHEN_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ASPECT_OF_MAZZATUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ASSASSINS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static AURORANS_THUNDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static AUTOMATED_DEFENSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static AYLEID_REFUGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static AZUREBLIGHT_REAPER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BAAN_DARS_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static BACK_ALLEY_GOURMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BAHRAHAS_CURSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BAHSEIS_MANIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static BALORGH: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BANIS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BAR_SAKKA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static BARKSKIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static BARON_THIRSK: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static BARON_ZAUDRUS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 548 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 548 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 603 as f64 }],
    ],
};

static BASALT_BLOODED_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BASTION_OF_THE_DRAOIFE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BASTION_OF_THE_HEARTLAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::DamageTakenFromPlayers, value: SET_REDUCE_PLAYER_DAMAGE_TAKEN_DEFAULT as f64 }],
    ],
};

static BATTALION_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static BATTLEFIELD_ACROBAT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BEACON_OF_OBLIVION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static BECKONING_STEEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static BEEKEEPERS_GEAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: 900 as f64 }],
    ],
};

static BELHARZAS_BAND: Set = Set {
    bonuses: &[
    ],
};

static BERSERKING_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static BLACK_FOUNDRY_STEEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BLACK_GEM_MONSTROSITY: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static BLACK_ROSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 176 as f64 }]
    ],
};

static BLACK_GLOVE_GROUNDING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static BLACKFEATHER_FLIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static BLESSING_OF_HIGH_ISLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static BLESSING_OF_THE_POTENTATES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::DamageTakenFromPlayers, value: SET_REDUCE_PLAYER_DAMAGE_TAKEN_DEFAULT as f64 }],
    ],
};

static BLIND_PATH_INDUCTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static BLOOD_MOON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static BLOODDRINKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static BLOODLORDS_EMBRACE: Set = Set {
    bonuses: &[
    ],
};

static BLOODSPAWN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static BLOODTHORNS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static BLUNTED_BLADES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BOG_RAIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BONE_PIRATES_TATTERS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static BRANDS_OF_IMPERIUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static BRIARHEART: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static BRIGHT_THROATS_BOAST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static BROKEN_SOUL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static BUFFER_OF_THE_SWIFT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static BULWARK_RUINATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static BURNING_SPELLWEAVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static CALL_OF_THE_UNDERTAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static CALUURIONS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static CAMONNA_TONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1600 as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static CHOKETHORN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static CINDERS_OF_ANTHELMIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static CLAW_OF_THE_FOREST_WRAITH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static CLAW_OF_YOLNAHKRIIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static CLEVER_ALCHEMIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static COLDHARBOURS_FAVORITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static COLOVIAN_HIGHLANDS_GENERAL: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static COMBAT_PHYSICIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static CORPSEBURSTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static COUP_DE_GRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static COWARDS_GEAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 250 as f64 }]
    ],
};

static CRAFTY_ALFIQ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 2550 as f64 }],
    ],
};

static CREST_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static CRIMSON_OATHS_RIVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static CRIMSON_TWILIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static CRITICAL_RIPOSTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalResistance, value: SET_CRITICAL_RESISTANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalResistance, value: SET_CRITICAL_RESISTANCE_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static CURSE_OF_DOYLEMISH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DAEDRIC_TRICKERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static DAGONS_DOMINION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DARING_CORSAIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static DARK_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DARKSTRIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static DAUNTLESS_COMBATANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static DEAD_WATERS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DEADLANDS_ASSASSIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 200 as f64 }],
    ],
};

static DEADLANDS_DEMOLISHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static DEADLY_STRIKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DEATH_DEALERS_FETE: Set = Set {
    bonuses: &[
    ],
};

static DEATHS_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static DEATH_DANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static DEEPROOT_ZEAL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static DEFENDING_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static DESERT_ROSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DIAMONDS_VICTORY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static DOMIHAUS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static DOV_RHA_SABATONS: Set = Set {
    bonuses: &[
    ],
};

static DRAGONS_APPETITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static DRAGONS_DEFILEMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static DRAGONGUARD_ELITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static DRAKES_RUSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static DRAUGR_HULK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 2550 as f64 }],
    ],
};

static DRAUGRS_HERITAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static DRAUGRS_REST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static DRAUGRKINS_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static DREAMERS_MANTLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static DREUGH_KING_SLAYER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static DROZAKARS_CLAWS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static DRUIDS_BRAID: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1565 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1565 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1722 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1565 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1565 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1722 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1565 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1565 as f64 }],
    ],
};

static DUNERIPPERS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static DUROKS_BANE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static EAGLE_EYE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static EARTHGORE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static EBON_ARMORY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static ELEMENTAL_CATALYST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ELEMENTAL_SUCCESSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ELF_BANE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static EMBERSHIELD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static ENCRATIS_BEHEMOTH: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ENDURANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 1928 as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: 618 as f64 }],
    ],
};

static ENERVATING_AURA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static ENGINE_GUARDIAN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static ESOTERIC_ENVIRONMENT_GREAVES: Set = Set {
    bonuses: &[
    ],
};

static ESSENCE_THIEF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ETERNAL_HUNT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static ETERNAL_VIGOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static ETERNAL_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static EUPHOTIC_GATEKEEPER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static EYE_OF_NAHVIINTAAS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static EYE_OF_THE_GRASP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static EYES_OF_MARA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static FALSE_GODS_DEVOTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static FARSTRIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static FASALLAS_GUILE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 7425 as f64 }],
        &[],
        &[],
        &[],
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 6020 as f64 }],
    ],
};

static FIORDS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static FLAME_BLOSSOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static FLANKING_STRATEGIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static FLEDGLINGS_NEST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static FOOLKILLERS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static FOOTMANS_FORTUNE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 3460 as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static FROZEN_WATCHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static FULL_BELLY_BARRICADE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static GALERIONS_REVENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static GAZE_OF_SITHIS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 3276 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: 1025 as f64 }, SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 4000 as f64 }]
    ],
};

static GIANT_SPIDER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static GLACIAL_GUARDIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static GLITTERING_GOAD: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static GLORGOLOCH_THE_DESTROYER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static GLORIOUS_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static GORETHIEF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static GOSSAMER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static GRACE_OF_GLOOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static GRACE_OF_THE_ANCIENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 2406 as f64 }]
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static GRAVE_INEVITABILITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static GRAVE_STAKE_COLLECTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static GREEN_PACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static GRISLY_GOURMET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 526 as f64 }],
    ],
};

static GROTHDARR: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static GRUNDWULF: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static GRYPHONS_FEROCITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static GRYPHONS_REPRISAL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static HAGRAVENS_GARDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static HAND_OF_MEPHALA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static HANUS_COMPASSION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1963 as f64 }],
    ],
};

static HARMONY_IN_CHAOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HARPOONERS_WADING_KILT: Set = Set {
    bonuses: &[
    ],
};

static HATCHLINGS_SHELL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static HAVEN_OF_URSUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static HAWKS_EYE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HEALERS_HABIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HEALING_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static HEARTLAND_CONQUEROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static HEEM_JAS_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HEROIC_UNITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static HEW_AND_SUNDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HEX_SIPHON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static HEXOS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static HIDE_OF_MORIHAUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static HIDE_OF_THE_WEREWOLF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HIGHLAND_SENTINEL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: 986 as f64 }],
    ],
};

static HIRCINES_VENEER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static HIST_BARK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static HIST_WHISPERER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static HITIS_HEARTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static HOLLOWFANG_THIRST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static HROTHGARS_CHILL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static HUNDINGS_RAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 300 as f64 }],
    ],
};

static HUNT_LEADER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static HUNTSMANS_WARMASK: Set = Set {
    bonuses: &[
    ],
};

static ICEHEART: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ICY_CONJURER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static ILAMBRIS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static IMMOLATOR_CHARR: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static IMMORTAL_WARRIOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static IMPERIAL_PHYSIQUE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static IMPREGNABLE_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalResistance, value: 1650 as f64 }]
    ],
};

static INDOMITABLE_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static INFALLIBLE_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static INFERNAL_GUARDIAN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static INNATE_AXIOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static INVENTORS_GUARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static IRON_FLASK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static IRONBLOOD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static JAILBREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 142 as f64 }],
    ],
};

static JAILERS_TENACITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static JERALL_MOUNTAINS_WARCHIEF: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static JERENSIS_BLADESTORM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static JOLTING_ARMS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static JORVULDS_GUIDANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static JUDGMENT_OF_AKATOSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 2291 as f64 }]
    ],
};

static KAGRENACS_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 222 as f64 }],
    ],
};

static KARGAEDA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 731 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 731 as f64 }]
    ],
};

static KAZPIANS_CRUEL_SIGNET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static KINRAS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static KJALNARS_NIGHTMARE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static KNIGHT_SLAYER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static KNIGHT_ERRANTS_MAIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static KNIGHTMARE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static KRAGH: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static KRAGLENS_HOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static KVATCH_GLADIATOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static KYNES_KISS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static KYNES_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static KYNMARCHERS_CRUELTY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static LADY_MALYGDA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static LADY_THORN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static LAMIAS_SONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static LAMP_KNIGHTS_ART: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 100_000 as f64 }]
    ],
};

static LANGUOR_OF_PERYITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalResistance, value: SET_CRITICAL_RESISTANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static LAW_OF_JULIANOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 300 as f64 }],
    ],
};

static LEECHING_PLATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static LEFTHANDERS_AEGIS_BELT: Set = Set {
    bonuses: &[
    ],
};

static LEGACY_OF_KARTH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static LEKIS_FOCUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static LEVIATHAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: 1528 as f64 }],
    ],
};

static LIGHT_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static LIGHT_SPEAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static LIVEWIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static LORD_WARDEN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static LUCENT_ECHOES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static LUCILLAS_WINDSHIELD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static LUNAR_BASTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static LUSTROUS_SOULWELL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static MAARSELOK: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static MACABRE_VINTAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 150 as f64 }],
    ],
};

static MAD_GODS_DANCING_SHOES: Set = Set {
    bonuses: &[
    ],
};

static MAD_TINKERER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MAGICKA_FURNACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MAGMA_INCARNATE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static MAGNUS_GIFT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MALACATHS_BAND_OF_BRUTALITY: Set = Set {
    bonuses: &[
    ],
};

static MALIGALIGS_MAELSTROM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MANTLE_OF_SIRORIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MARAS_BALM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalResistance, value: SET_CRITICAL_RESISTANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static MARAUDERS_HASTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static MARK_OF_THE_PARIAH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static MARKSMANS_CREST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MARKYN_RING_OF_MAJESTY: Set = Set {
    bonuses: &[
    ],
};

static MASTER_ARCHITECT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MAW_OF_THE_INFERNAL: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MECHANICAL_ACUITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MEDUSA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: 892 as f64 }],
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
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static MERITORIOUS_SERVICE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MIGHT_OF_THE_LOST_LEGION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static MIGHTY_CHUDAN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static MIGHTY_GLACIER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static MOLAG_KENA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MONOLITH_OF_STORMS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MONOMYTH_REFORGED: Set = Set {
    bonuses: &[
    ],
};

static MOON_HUNTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static MOONDANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MORA_SCRIBES_THESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MORAS_WHISPERS: Set = Set {
    bonuses: &[
    ],
};

static MORKULDIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static MOTHER_CIANNAIT: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static MOTHERS_SORROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: 1528 as f64 }],
    ],
};

static NAGA_SHAMAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static NAZARAY: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static NECROPOTENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static NERIENETH: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static NETCH_OIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static NETCHS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static NEW_MOON_ACOLYTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 401 as f64 }],
    ],
};

static NIBENAY_BAY_BATTLEREEVE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalResistance, value: SET_CRITICAL_RESISTANCE_DEFAULT as f64 }]
    ],
};

static NIGHT_MOTHERS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static NIGHT_MOTHERS_GAZE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static NIGHT_TERROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static NIGHTS_SILENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static NIGHTFLAME: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static NIKULAS_HEAVY_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static NIX_HOUNDS_HOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static NOBILITY_IN_DECAY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static NOBLE_DUELISTS_SILKS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static NOBLES_CONQUEST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static NOCTURNALS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static NOCTURNALS_PLOY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static NOXIOUS_BOULDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static NUNATAK: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static OAKENSOUL_RING: Set = Set {
    bonuses: &[
    ],
};

static OAKFATHERS_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static OBLIVIONS_EDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 258 as f64 }],
    ],
};

static OBLIVIONS_FOE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static OLD_GROWTH_BREWER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static ORDER_OF_DIAGNA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: 5 as f64 }],
    ],
};

static ORDERS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: 943 as f64 }],
    ],
};

static ORGNUMS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static ORPHEON_THE_TACTICIAN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static OVERWHELMING_SURGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static OZEZAN_THE_INFERNO: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PANGRIT_DENMOTHER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 1890 as f64 }],
    ],
};

static PARA_BELLUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static PEACE_AND_SERENITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PEARLESCENT_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static PEARLS_OF_EHLNOFEY: Set = Set {
    bonuses: &[
    ],
};

static PELINALS_WRATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_AEGIS_OF_GALENWE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static PERFECTED_ANSUULS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_ARMS_OF_RELEQUEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_BAHSEIS_MANIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_CAUSTIC_ARROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: 103 as f64 }]
    ],
};

static PERFECTED_CHAOTIC_WHIRLWIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_CLAW_OF_YOLNAHKRIIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static PERFECTED_CONCENTRATED_FORCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: 103 as f64 }]
    ],
};

static PERFECTED_CORAL_RIPTIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_CRUEL_FLURRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: 103 as f64 }]
    ],
};

static PERFECTED_CRUSHING_WALL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_DEFENSIVE_POSITION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 103 as f64 }]
    ],
};

static PERFECTED_DESTRUCTIVE_IMPACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: 103 as f64 }]
    ],
};

static PERFECTED_DISCIPLINED_SLASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 877 as f64 }]
    ],
};

static PERFECTED_DOLOROUS_ARENA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static PERFECTED_EXECUTIONERS_BLADE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_EYE_OF_NAHVIINTAAS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PERFECTED_FALSE_GODS_DEVOTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_FORCE_OVERFLOW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 877 as f64 }]
    ],
};

static PERFECTED_FRENZIED_MOMENTUM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 877 as f64 }]
    ],
};

static PERFECTED_GALLANT_CHARGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_GRAND_REJUVENATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 877 as f64 }]
    ],
};

static PERFECTED_HARMONY_IN_CHAOS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_KAZPIANS_CRUEL_SIGNET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_KYNES_WIND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PERFECTED_LUCENT_ECHOES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static PERFECTED_MANTLE_OF_SIRORIA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_MENDERS_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: 103 as f64 }]
    ],
};

static PERFECTED_MERCILESS_CHARGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_MORA_SCRIBES_THESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_PEACE_AND_SERENITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_PEARLESCENT_WARD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static PERFECTED_PIERCING_SPRAY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_PILLAGERS_PROFIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PERFECTED_POINT_BLANK_SNIPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: 103 as f64 }]
    ],
};

static PERFECTED_PRECISE_REGENERATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_PUNCTURING_REMEDY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: 3 as f64 }]
    ],
};

static PERFECTED_RADIAL_UPPERCUT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_RAMPAGING_SLASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: 77 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 77 as f64 }]
    ],
};

static PERFECTED_RECOVERY_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static PERFECTED_ROARING_OPPORTUNIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PERFECTED_SAXHLEEL_CHAMPION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static PERFECTED_SLIVERS_OF_THE_NULL_ARCA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERFECTED_SPECTRAL_CLOAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 103 as f64 }]
    ],
};

static PERFECTED_STINGING_SLASHES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_STONE_TALKERS_OATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PERFECTED_SUL_XANS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_TEST_OF_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static PERFECTED_THUNDEROUS_VOLLEY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_TIMELESS_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 877 as f64 }]
    ],
};

static PERFECTED_TITANIC_CLEAVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_TOOTH_OF_LOKKESTIIZ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_TRANSFORMATIVE_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static PERFECTED_VESTMENT_OF_OLORIME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PERFECTED_VIRULENT_SHOT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 526 as f64 }]
    ],
};

static PERFECTED_VOID_BASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 965 as f64 }]
    ],
};

static PERFECTED_VROLS_COMMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static PERFECTED_WHORL_OF_THE_DEPTHS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static PERFECTED_WILD_IMPULSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_WRATH_OF_ELEMENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1190 as f64 }]
    ],
};

static PERFECTED_XORYNS_MASTERPIECE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PERFECTED_YANDIRS_MIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PERMAFROST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PESTILENT_HOST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static PHOENIX: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PHOENIX_MOTH_THEURGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static PHYLACTERYS_GRASP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static PILLAR_OF_NIRN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PIRATE_SKELETON: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static PLAGUE_DOCTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 2804 as f64 }],
    ],
};

static PLAGUE_SLINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PLAGUEBREAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static POWERFUL_ASSAULT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PRAYER_SHAWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PRECISE_REGENERATION: Set = Set {
    bonuses: &[
        &[],
    ],
};

static PRIOR_THIERRIC: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static PRISONERS_RAGS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static PROPHETS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static QUEENS_ELEGANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static QUICK_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static RAGE_OF_THE_URSAUK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static RAKKHATS_VOIDMANTLE: Set = Set {
    bonuses: &[
    ],
};

static RALLYING_CRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static RATTLECAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static RAVAGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static REACTIVE_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static REAWAKENED_HIEROPHANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 731 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 731 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 731 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 731 as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 731 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 731 as f64 }],
    ],
};

static RECOVERY_CONVERGENCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static RED_EAGLES_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static REDISTRIBUTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static REFLECTED_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static RELICS_OF_THE_PHYSICIAN_ANSUR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static RELICS_OF_THE_REBELLION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static RENALDS_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ROAR_OF_ALKOSH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ROARING_OPPORTUNIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ROBES_OF_ALTERATION_MASTERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ROBES_OF_DESTRUCTION_MASTERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ROBES_OF_THE_HIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ROBES_OF_THE_WITHERED_HAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static ROBES_OF_TRANSMUTATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ROKSA_THE_WARPED: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: 70 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: 70 as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: 70 as f64 }]
    ],
};

static ROURKEN_STEAMGUARDS: Set = Set {
    bonuses: &[
    ],
};

static RUNECARVERS_BLAZE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static RUSH_OF_AGONY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static SALVATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SANCTUARY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SAVAGE_WEREWOLF: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static SAXHLEEL_CHAMPION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SCATHING_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SCAVENGING_DEMISE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SCORIONS_FEAST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SCOURGE_HARVESTER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static SEA_SERPENTS_COIL: Set = Set {
    bonuses: &[
    ],
};

static SEEKER_SYNTHESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SELENE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SELLISTRIX: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SENCHAL_DEFENDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SENCHES_BITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SENCHE_RAHTS_GRIT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static SENTINEL_OF_RKUGAMZ: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static SENTRY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SERGEANTS_MAIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SERPENTS_DISDAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SEVENTH_LEGION_BRUTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SHACKLEBREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 2065 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 2065 as f64 }],
    ],
};

static SHADOW_DANCERS_RAIMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SHADOW_OF_THE_RED_MOUNTAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SHADOW_WALKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static SHADOWREND: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static SHALIDORS_CURSE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static SHALK_EXOSKELETON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static SHAPESHIFTERS_CHAIN: Set = Set {
    bonuses: &[
    ],
};

static SHARED_BURDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SHARED_PAIN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SHATTERED_FATE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[],
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 7918 as f64 }],
        &[],
        &[],
        &[],
        &[],
        &[SetBonus { channel: Channel::Power, value: 687 as f64 }],
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: 1528 as f64 }],
    ],
};

static SHATTERED_PATHS_SIGNET: Set = Set {
    bonuses: &[
        &[],
    ],
};

static SHEER_VENOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SHELL_SPLITTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static SHIELD_BREAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SHIELD_OF_THE_VALIANT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::DamageTakenFromPlayers, value: SET_REDUCE_PLAYER_DAMAGE_TAKEN_DEFAULT as f64 }],
    ],
};

static SHROUD_OF_THE_LICH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static SIEGEMASTERS_FOCUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static SILKS_OF_THE_SUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SILVER_ROSE_VIGIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SITHIS_TOUCH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SKOOMA_SMUGGLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SLIMECRAW: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SLIVERS_OF_THE_NULL_ARCA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SLOADS_SEMBLANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SLUTHRUGS_HUNGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static SNAKE_IN_THE_STARS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static SNOW_TREADERS: Set = Set {
    bonuses: &[
    ],
};

static SOLDIER_OF_ANGUISH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SONG_OF_LAMAE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static SOULCLEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SOULSHINE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPATTERING_DISJUNCTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPAULDER_OF_RUIN: Set = Set {
    bonuses: &[
    ],
};

static SPAWN_OF_MEPHALA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static SPELL_PARASITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SPELL_POWER_CURE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPELL_STRATEGIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPELLSHREDDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SPELUNKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPIDER_CULTIST_COWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SPINNERS_GARMENTS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 3460 as f64 }],
    ],
};

static SPRIGGANS_THORNS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 3460 as f64 }],
    ],
};

static SPRIGGANS_VIGOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SQUALL_OF_RETRIBUTION: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static STEADFAST_HERO: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static STEADFASTS_METTLE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static STENDARRS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static STINGING_SLASHES: Set = Set {
    bonuses: &[
        &[],
    ],
};

static STONE_HUSK: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static STONES_ACCORD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static STONE_TALKERS_OATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static STONEHULK_DOMINATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static STONEKEEPER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: 548 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 548 as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: 603 as f64 }],
    ],
};

static STORM_KNIGHTS_PLATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static STORM_MASTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static STORM_CURSEDS_REVENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static STORMFIST: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static STORMWEAVERS_CAVORT: Set = Set {
    bonuses: &[
    ],
};

static STRENGTH_OF_THE_AUTOMATON: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static STUHNS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static STYGIAN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static SUL_XANS_TORMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SUNDERFLAME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SWAMP_RAIDER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SWARM_MOTHER: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SWORD_DANCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static SWORD_SINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SYMMETRY_OF_THE_WEALD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static SYMPHONY_OF_BLADES: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static SYRABANES_GRIP: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static SYRABANES_WARD: Set = Set {
    bonuses: &[
    ],
};

static SYSTRES_SCOWL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static SYVARRAS_SCALES: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static TALFYGS_TREACHERY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static TARNISHED_NIGHTMARE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static TAVAS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static TELVANNI_EFFICIENCY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static TELVANNI_ENFORCER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static TEST_OF_RESOLVE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static THARRIKERS_STRIKE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static THE_ARCH_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static THE_BLIND: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static THE_DESTRUCTION_SUITE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static THE_ICE_FURNACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static THE_JUGGERNAUT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static THE_MORAG_TONG: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static THE_RUCKUS: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
    ],
};

static THE_WORMS_RAIMENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static THEWS_OF_THE_HARBINGER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static THOUSAND_EYES: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static THRASSIAN_STRANGLERS: Set = Set {
    bonuses: &[
    ],
};

static THREADS_OF_WAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static THREE_QUEENS_WELLSPRING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static THUNDER_CALLER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static THUNDERBUGS_CARAPACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static THUNDEROUS_VOLLEY: Set = Set {
    bonuses: &[
        &[],
    ],
};

static THURVOKUN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static TIDE_BORN_WILDSTALKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static TOOTH_OF_LOKKESTIIZ: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static TOOTHROW: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static TORC_OF_THE_LAST_AYLEID_KING: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: 500 as f64 }]
    ],
};

static TORC_OF_TONAL_CONSTANCY: Set = Set {
    bonuses: &[
    ],
};

static TORMENTOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static TORUGS_PACT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static TRACKERS_LASH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static TRANSFORMATIVE_HOPE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::HealingDone, value: SET_HEALING_DONE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static TRAPPINGS_OF_INVIGORATION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static TREASURE_HUNTER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static TREASURES_OF_THE_EARTHFORGE: Set = Set {
    bonuses: &[
        &[],
        &[],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static TREMORSCALE: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static TRIAL_BY_FIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static TRINIMACS_VALOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static TRUE_SWORN_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static TURNING_TIDE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static TWICE_BORN_STAR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static TWICE_FANGED_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static TWILIGHT_REMEDY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static TWILIGHTS_EMBRACE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static TWIN_SISTERS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static TZOGVINS_WARBAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static ULFNORS_FAVOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static UMBRAL_EDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static UNCHAINED_AGGRESSOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static UNDAUNTED_BASTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static UNDAUNTED_INFILTRATOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static UNDAUNTED_UNWEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static UNFATHOMABLE_DARKNESS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static UNFLINCHING_ULTIMATE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static UNLEASHED_RITUALIST: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static UNLEASHED_TERROR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static VALKYN_SKORIA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static VAMPIRE_CLOAK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 171 as f64 }],
    ],
};

static VAMPIRE_LORD: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static VAMPIRES_KISS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }, SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VANDORALLENS_RESONANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VANGUARDS_CHALLENGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static VARENS_LEGACY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static VASTARIES_TUTELAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }, SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static VELIDRETH: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VELOTHI_UR_MAGES_AMULET: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: 1650 as f64 }]
    ],
};

static VENGEANCE_LEECH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static VENOMOUS_SMITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VESTMENT_OF_OLORIME: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static VESTMENTS_OF_THE_WARLOCK: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static VESTURE_OF_DARLOC_BRAE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static VICECANON_OF_VENOM: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static VICIOUS_DEATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
    ],
};

static VICIOUS_SERPENT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VIPERS_STING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VROLS_COMMAND: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static VYKANDS_SOULFURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Penetration(ResistableDamageType::All), value: SET_PENETRATION_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static VYKOSA: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::HealingTaken, value: SET_HEALING_TAKEN_DEFAULT as f64 }],
    ],
};

static WAR_MACHINE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WAR_MAIDEN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WARD_OF_CYRODIIL: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static WARRIORS_FURY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WARRIOR_POET: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
    ],
};

static WAY_OF_AIR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
    ],
};

static WAY_OF_FIRE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static WAY_OF_MARTIAL_KNOWLEDGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WAY_OF_THE_ARENA: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 165 as f64 }]
    ],
};

static WHITESTRAKES_RETRIBUTION: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Armour(ResistableDamageType::All), value: SET_ARMOUR_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static WHORL_OF_THE_DEPTHS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WIDOWMAKER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
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
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};

static WILLOWS_PATH: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_RECOVERY_DEFAULT as f64 }],
    ],
};

static WILLPOWER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: 1752 as f64 }],
        &[SetBonus { channel: Channel::Power, value: 206 as f64 }]
    ],
};

static WINTERS_RESPITE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static WINTERBORN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WISDOM_OF_VANUS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WISE_MAGE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WITCH_KNIGHTS_DEFIANCE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WITCHMAN_ARMOR: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
    ],
};

static WIZARDS_RIPOSTE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalResistance, value: 660 as f64 }]
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
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: 325 as f64 }]
    ],
};

static WRATHSUN: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WRETCHED_VITALITY: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static WYRD_TREES_BLESSING: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static XANMEER_GENESIS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Health, AggKind::Additive), value: SET_HEALTH_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static XANMEER_SPELLWEAVER: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static XORYNS_MASTERPIECE: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorAegis], */
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
    ],
};

static YANDIRS_MIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        /* TODO(grant_rule): &[SetBonusType::MinorSlayer], */
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static YSGRAMORS_BIRTHRIGHT: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
    ],
};

static ZENS_REDRESS: Set = Set {
    bonuses: &[
        &[],
        &[SetBonus { channel: Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_RECOVERY_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Power, value: SET_POWER_DEFAULT as f64 }],
        &[SetBonus { channel: Channel::Resource(ResourceKind::Magicka, AggKind::Additive), value: SET_MAGICKA_DEFAULT as f64 }],
    ],
};

static ZAAN: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::CriticalChance, value: SET_CRITICAL_CHANCE_DEFAULT as f64 }],
    ],
};

static ZOAL_THE_EVER_WAKEFUL: Set = Set {
    bonuses: &[
        &[SetBonus { channel: Channel::Resource(ResourceKind::Stamina, AggKind::Additive), value: SET_STAMINA_DEFAULT as f64 }],
    ],
};