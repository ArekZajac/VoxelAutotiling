use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::world::{Coord, Voxel, WORLD_SIZE, World};

pub fn build_chunk_mesh(world: &World) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut next = 0u32;

    let mut push_face = |base: Vec3, face: Face| {
        let (p, n, uv) = face_vertices(base, face);

        positions.extend(p);
        normals.extend(n);
        uvs.extend(uv);

        indices.extend([next, next + 1, next + 2, next, next + 2, next + 3]);
        next += 4;
    };

    for y in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let c = Coord::new(x, y, z);
                if world.get(&c) == Voxel::Air {
                    continue;
                }

                let base = voxel_min_world(c);

                if world.get(&Coord::new(x - 1, y, z)) == Voxel::Air {
                    push_face(base, Face::NegX);
                }
                if world.get(&Coord::new(x + 1, y, z)) == Voxel::Air {
                    push_face(base, Face::PosX);
                }
                if world.get(&Coord::new(x, y - 1, z)) == Voxel::Air {
                    push_face(base, Face::NegY);
                }
                if world.get(&Coord::new(x, y + 1, z)) == Voxel::Air {
                    push_face(base, Face::PosY);
                }
                if world.get(&Coord::new(x, y, z - 1)) == Voxel::Air {
                    push_face(base, Face::NegZ);
                }
                if world.get(&Coord::new(x, y, z + 1)) == Voxel::Air {
                    push_face(base, Face::PosZ);
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn voxel_min_world(c: Coord) -> Vec3 {
    let half = WORLD_SIZE as f32 / 2.0;
    Vec3::new(c.x as f32 - half, c.y as f32 - half, c.z as f32 - half)
}

#[derive(Clone, Copy)]
enum Face {
    NegX,
    PosX,
    NegY,
    PosY,
    NegZ,
    PosZ,
}

fn face_vertices(base: Vec3, face: Face) -> ([[f32; 3]; 4], [[f32; 3]; 4], [[f32; 2]; 4]) {
    let (p0, p1, p2, p3, n) = match face {
        Face::NegX => (
            base + Vec3::new(0.0, 0.0, 0.0),
            base + Vec3::new(0.0, 0.0, 1.0),
            base + Vec3::new(0.0, 1.0, 1.0),
            base + Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
        ),
        Face::PosX => (
            base + Vec3::new(1.0, 0.0, 0.0),
            base + Vec3::new(1.0, 1.0, 0.0),
            base + Vec3::new(1.0, 1.0, 1.0),
            base + Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        Face::NegY => (
            base + Vec3::new(0.0, 0.0, 0.0),
            base + Vec3::new(1.0, 0.0, 0.0),
            base + Vec3::new(1.0, 0.0, 1.0),
            base + Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, -1.0, 0.0),
        ),
        Face::PosY => (
            base + Vec3::new(0.0, 1.0, 1.0),
            base + Vec3::new(1.0, 1.0, 1.0),
            base + Vec3::new(1.0, 1.0, 0.0),
            base + Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        Face::NegZ => (
            base + Vec3::new(0.0, 0.0, 0.0),
            base + Vec3::new(0.0, 1.0, 0.0),
            base + Vec3::new(1.0, 1.0, 0.0),
            base + Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        Face::PosZ => (
            base + Vec3::new(1.0, 0.0, 1.0),
            base + Vec3::new(1.0, 1.0, 1.0),
            base + Vec3::new(0.0, 1.0, 1.0),
            base + Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
    };

    let positions = [p0.to_array(), p1.to_array(), p2.to_array(), p3.to_array()];
    let normals = [n.to_array(), n.to_array(), n.to_array(), n.to_array()];
    let uvs = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

    (positions, normals, uvs)
}
