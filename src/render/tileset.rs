use bevy::{gltf::GltfMesh, prelude::*};
use std::collections::HashMap;

use crate::world::WorldChanged;

use super::tile_kind::TileKind;

#[derive(Clone)]
pub struct TileAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct Tileset {
    pub gltf: Handle<Gltf>,
    pub tiles: HashMap<TileKind, TileAsset>,
    pub ready: bool,
}

pub fn load_debug_tileset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf: Handle<Gltf> = asset_server.load("tiles/debug.glb");
    commands.insert_resource(Tileset {
        gltf,
        tiles: HashMap::new(),
        ready: false,
    });
}

pub fn populate_tileset(
    mut tileset: ResMut<Tileset>,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut changed: MessageWriter<WorldChanged>,
) {
    if tileset.ready {
        return;
    }

    if !asset_server.is_loaded_with_dependencies(&tileset.gltf) {
        return;
    }

    let Some(gltf) = gltfs.get(&tileset.gltf) else {
        return;
    };

    let mut inserted_this_frame = 0usize;

    for (name, gltf_mesh_handle) in gltf.named_meshes.iter() {
        let Some(kind) = TileKind::from_name(name) else {
            continue;
        };
        if tileset.tiles.contains_key(&kind) {
            continue;
        }

        let Some(gltf_mesh) = gltf_meshes.get(gltf_mesh_handle) else {
            continue;
        };
        let Some(prim) = gltf_mesh.primitives.first() else {
            continue;
        };

        let mesh = prim.mesh.clone();
        let material = prim.material.clone().unwrap_or_else(|| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                ..Default::default()
            })
        });

        tileset.tiles.insert(kind, TileAsset { mesh, material });
        inserted_this_frame += 1;
    }

    if inserted_this_frame > 0 {
        info!("Tileset progress: {}/6", tileset.tiles.len());
    }

    if tileset.tiles.len() == 6 {
        tileset.ready = true;
        info!("Tileset ready!");
        changed.write_default();
    }
}
