use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub mod autotile;
pub mod camera;
pub mod chunk;
pub mod classify;
pub mod picking;
pub mod tile_kind;
pub mod tileset;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                camera::spawn_camera,
                chunk::spawn_chunk,
                tileset::load_debug_tileset,
            ),
        )
        .add_systems(
            Update,
            (
                tileset::populate_tileset,
                chunk::remesh_on_world_changed.after(tileset::populate_tileset),
                camera::orbit_camera,
                picking::mouse_edit_voxels,
            ),
        );
    }
}
