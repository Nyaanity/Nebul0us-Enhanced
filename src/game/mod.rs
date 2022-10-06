#[allow(non_camel_case_types)]
pub mod enums {
    mod base_event;
    mod clan_rank;
    mod click_type;
    mod color_cycle;
    mod connection_state;
    mod difficulty;
    mod game_mode;
    mod packet;
    mod server;
    mod split_multiplier;
    mod world_size;

    pub use {
        base_event::BaseEvent, clan_rank::ClanRank, click_type::ClickType, color_cycle::ColorCycle,
        connection_state::ConnectionState, difficulty::Difficulty, game_mode::GameMode,
        server::Server, split_multiplier::SplitMultiplier, world_size::WorldSize,
    };
}

pub mod packets;
pub mod rng_crypt;
