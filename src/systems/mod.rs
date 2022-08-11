mod player_input;
mod map_render;
mod entity_renderer;
use crate::prelude::*;

pub fn build_scbheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .build()
}
