#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CoreCombatAbility {
    Bash,
    Block,
    BreakFree,
    DodgeRoll,
    Sprint,
    Sneak,
}