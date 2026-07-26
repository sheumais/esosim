use crate::data::enums::damage::ResistableDamageType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Health,
    Magicka,
    Stamina,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggKind {
    Additive,
    Multiplicative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Armour(ResistableDamageType),
    CriticalChance,
    CriticalDamage,
    CriticalResistance,
    Power,
    Penetration(ResistableDamageType),
    Resource(ResourceKind, AggKind),
    Recovery(ResourceKind, AggKind),
    HealingDone,
    HealingTaken,
    DamageTakenFromPlayers,
}

impl Channel {
    /// The doc's §4.2 sketch has `Aggregator` carry both an `additive` and
    /// a `multiplicative_bps` field, and `Channel::Resource` carries an
    /// explicit `AggKind` — but nothing ever reads that tag. This is the
    /// missing link: every channel routes through here to decide which
    /// side of the `Aggregator` it lands on.
    ///
    /// Resource is the only channel that's genuinely either/or in-game (a
    /// flat "+3000 Max Magicka" enchant vs. a "+8% Max Magicka" CP star
    /// both exist and must combine correctly). Every other channel here
    /// is additive-only at *this* layer — percentage effects that depend
    /// on hit-time target state are `ConditionalEffect`s (§8.2), not
    /// `Effect`s, and never reach `ChannelSet` at all.
    pub fn agg_kind(&self) -> AggKind {
        match self {
            Channel::Resource(_, kind) => *kind,
            _ => AggKind::Additive,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub id: u32,
    pub name: &'static str,
    pub value: f64,
    pub value_per_stack: f64,
    pub channels: &'static [Channel],
}

pub static FORTIFIED: Effect = Effect {
    id: 142035,
    name: "Fortified",
    value: 0.0,
    value_per_stack: 34.62,
    channels: &[Channel::Armour(ResistableDamageType::All)],
};