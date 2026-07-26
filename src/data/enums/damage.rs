#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ResistableDamageType {
    Physical,
    Flame,
    Shock,
    Frost,
    Earth,
    Magic,
    Disease,
    Poison,
    Bleed,

    Martial,
    Spell,
    All,
    Fall,
}