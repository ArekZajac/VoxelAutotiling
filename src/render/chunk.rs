use bevy::prelude::*;

use super::autotile::neighbors;
use super::classify::classify;
use super::tileset::Tileset;
use crate::world::{Coord, Voxel, WORLD_SIZE, World, WorldChanged};

#[derive(Component)]
pub struct ChunkRoot;

#[derive(Component)]
pub struct TileInstance;

pub fn spawn_chunk(mut commands: Commands) {
    commands.spawn((
        ChunkRoot,
        Transform::IDENTITY,
        GlobalTransform::IDENTITY,
        Visibility::default(),
    ));
}

pub fn remesh_on_world_changed(
    mut ev: MessageReader<WorldChanged>,
    world: Res<World>,
    tileset: Res<Tileset>,
    mut commands: Commands,
    q_root: Query<Entity, With<ChunkRoot>>,
    q_tiles: Query<Entity, With<TileInstance>>,
) {
    if ev.is_empty() {
        return;
    }

    if !tileset.ready {
        return;
    }

    info!(
        "Remesh triggered. tileset_ready={}, tiles={}",
        tileset.ready,
        tileset.tiles.len()
    );

    ev.clear();

    let Ok(root) = q_root.single() else {
        return;
    };

    // Clear previous render instances
    for e in &q_tiles {
        commands.entity(e).despawn();
    }

    // Spawn tiles
    for y in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let c = Coord::new(x, y, z);
                if world.get(&c) == Voxel::Air {
                    continue;
                }

                let nei = neighbors(&world, c);
                let (kind, rot) = classify(nei);

                let pos = voxel_min_world(c) + Vec3::splat(0.5);

                let Some(tile) = tileset.tiles.get(&kind).cloned() else {
                    continue;
                };

                commands.entity(root).with_children(|p| {
                    p.spawn((
                        TileInstance,
                        Mesh3d(tile.mesh),
                        MeshMaterial3d(tile.material),
                        Transform {
                            translation: pos,
                            rotation: rot,
                            scale: Vec3::ONE,
                        },
                        GlobalTransform::IDENTITY,
                        Visibility::default(),
                    ));
                });
            }
        }
    }
}

fn voxel_min_world(c: Coord) -> Vec3 {
    let half = WORLD_SIZE as f32 / 2.0;
    Vec3::new(c.x as f32 - half, c.y as f32 - half, c.z as f32 - half)
}
