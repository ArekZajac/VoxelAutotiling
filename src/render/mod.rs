use bevy::prelude::*;

pub mod camera;
pub mod chunk;
pub mod meshing;
pub mod picking;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (camera::spawn_camera, chunk::spawn_chunk))
            .add_systems(
                Update,
                (
                    camera::orbit_camera,
                    picking::mouse_edit_voxels,
                    chunk::remesh_on_world_changed,
                ),
            );
    }
}
