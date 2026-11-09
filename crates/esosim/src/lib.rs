pub mod data {
    pub mod enums {
        pub mod gear;
        pub mod damage;
        pub mod cca;
    }
    pub mod bundle;
    pub mod effects {
    }
    pub mod skill_data {
        pub mod major_minor;
    }
    pub mod tables {
        pub mod armour;
        pub mod enchant;
        pub mod item_type;
        pub mod power;
        pub use esosim_data::set_ids; // generated; lives in crates/esosim-data
        pub mod item_set;
        pub mod set_kind;
        pub mod sets;
        pub mod traits;
    }
}

pub mod engine {
    pub mod active_effects;
    pub mod aggregator;
    pub mod champion;
    pub mod conditional;
    pub mod effect;
    pub mod grant_rule;
    pub mod mundus;
    pub mod registry;
    pub mod set_descriptor;
}

pub mod entity {
    pub mod enemy;
    pub mod gear;
    pub mod player;
    pub mod set_issues;
}

pub mod stats {
    pub mod armour;
    pub mod coef;
    pub mod critical;
    pub mod power;
    pub mod resource;
    pub mod sheet;
}