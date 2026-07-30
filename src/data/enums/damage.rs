#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ResistableDamageType {
    /* The four "magical" damage types */
    Flame,
    Shock,
    Frost,
    Earth,
    Magic,

    /* The four "martial" damage types */
    Physical, // Confusingly named- separate to the "Physical" resistance et cetera.
    Disease,
    Poison,
    Bleed,

    Martial, // Affects martial
    Spell, // Affects magical
    All, // Affects Magical and Martial and their sub-categories

    /// Not included in All
    Fall,
}