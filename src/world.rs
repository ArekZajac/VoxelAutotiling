use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WorldChanged>()
            .insert_resource(World::new())
            .add_systems(Startup, seed_world);
    }
}

// ---------- COORD ----------

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

// ---------- VOXEL ----------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Voxel {
    Air,
    Brick,
}

// ---------- WORLD ----------

pub const WORLD_SIZE: i32 = 8;

#[derive(Resource)]
pub struct World {
    contents: Vec<Voxel>,
}

#[derive(Message, Default)]
pub struct WorldChanged;

impl World {
    pub fn new() -> Self {
        Self {
            contents: vec![Voxel::Air; (WORLD_SIZE * WORLD_SIZE * WORLD_SIZE) as usize],
        }
    }

    pub fn set(&mut self, coord: &Coord, voxel: Voxel) {
        if Self::in_bounds(coord) {
            self.contents[Self::idx(coord)] = voxel;
        }
    }

    pub fn get(&self, coord: &Coord) -> Voxel {
        if Self::in_bounds(coord) {
            self.contents[Self::idx(coord)]
        } else {
            Voxel::Air
        }
    }

    fn idx(coord: &Coord) -> usize {
        (coord.y * WORLD_SIZE * WORLD_SIZE + coord.z * WORLD_SIZE + coord.x) as usize
    }

    pub fn in_bounds(coord: &Coord) -> bool {
        coord.x >= 0
            && coord.x < WORLD_SIZE
            && coord.y >= 0
            && coord.y < WORLD_SIZE
            && coord.z >= 0
            && coord.z < WORLD_SIZE
    }
}

pub fn seed_world(mut world: ResMut<World>, mut message: MessageWriter<WorldChanged>) {
    let center = WORLD_SIZE / 2;
    world.set(&Coord::new(center, center, center), Voxel::Brick);
    message.write_default();
}
