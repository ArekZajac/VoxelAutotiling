use bevy::prelude::*;

use crate::render::meshing::build_chunk_mesh;
use crate::world::{World, WorldChanged};

#[derive(Component)]
pub struct ChunkMesh {
    pub mesh: Handle<Mesh>,
}

pub fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<World>,
) {
    let mesh_handle = meshes.add(build_chunk_mesh(&world));

    let mat_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.45, 0.35),
        perceptual_roughness: 0.9,
        metallic: 0.0,
        // keep default culling ON (correct windings)
        ..Default::default()
    });

    commands.spawn((
        ChunkMesh {
            mesh: mesh_handle.clone(),
        },
        Mesh3d(mesh_handle),
        MeshMaterial3d(mat_handle),
        Transform::IDENTITY,
    ));
}

pub fn remesh_on_world_changed(
    mut ev: MessageReader<WorldChanged>,
    world: Res<World>,
    q: Query<&ChunkMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if ev.is_empty() {
        return;
    }
    ev.clear();

    let Ok(chunk) = q.single() else {
        return;
    };
    let Some(mesh) = meshes.get_mut(&chunk.mesh) else {
        return;
    };

    *mesh = build_chunk_mesh(&world);
}
