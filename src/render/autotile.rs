use crate::world::{Coord, Voxel, World};

#[derive(Clone, Copy)]
pub struct Neighbors {
    pub nx: bool,
    pub px: bool,
    pub ny: bool,
    pub py: bool,
    pub nz: bool,
    pub pz: bool,
}

pub fn neighbors(world: &World, c: Coord) -> Neighbors {
    Neighbors {
        nx: world.get(&Coord::new(c.x - 1, c.y, c.z)) != Voxel::Air,
        px: world.get(&Coord::new(c.x + 1, c.y, c.z)) != Voxel::Air,
        ny: world.get(&Coord::new(c.x, c.y - 1, c.z)) != Voxel::Air,
        py: world.get(&Coord::new(c.x, c.y + 1, c.z)) != Voxel::Air,
        nz: world.get(&Coord::new(c.x, c.y, c.z - 1)) != Voxel::Air,
        pz: world.get(&Coord::new(c.x, c.y, c.z + 1)) != Voxel::Air,
    }
}
