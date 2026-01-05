use bevy::math::Dir3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::world::{Coord, Voxel, WORLD_SIZE, World, WorldChanged};

pub fn mouse_edit_voxels(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut world: ResMut<World>,
    mut changed: MessageWriter<WorldChanged>,
) {
    let add = buttons.just_pressed(MouseButton::Left);
    let remove = buttons.just_pressed(MouseButton::Right);
    if !add && !remove {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok((camera, cam_xform)) = q_cam.single() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(cam_xform, cursor) else {
        return;
    };

    let Some(hit) = raycast_voxels(&world, ray.origin, ray.direction) else {
        return;
    };

    if remove {
        world.set(&hit.coord, Voxel::Air);
        changed.write_default();
        return;
    }

    if add {
        let place = Coord::new(
            hit.coord.x + hit.normal.x,
            hit.coord.y + hit.normal.y,
            hit.coord.z + hit.normal.z,
        );

        if World::in_bounds(&place) && world.get(&place) == Voxel::Air {
            world.set(&place, Voxel::Brick);
            changed.write_default();
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RayHit {
    coord: Coord,
    normal: IVec3, // face normal in voxel coords
    t: f32,
}

fn raycast_voxels(world: &World, origin: Vec3, dir: Dir3) -> Option<RayHit> {
    let mut best: Option<RayHit> = None;

    for y in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let c = Coord::new(x, y, z);
                if world.get(&c) == Voxel::Air {
                    continue;
                }

                let min = voxel_min_world(c);
                let max = min + Vec3::ONE;

                if let Some((t, n)) = ray_aabb(origin, dir.as_vec3(), min, max) {
                    if t >= 0.0 {
                        match best {
                            None => {
                                best = Some(RayHit {
                                    coord: c,
                                    normal: n,
                                    t,
                                })
                            }
                            Some(b) if t < b.t => {
                                best = Some(RayHit {
                                    coord: c,
                                    normal: n,
                                    t,
                                })
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    best
}

fn voxel_min_world(c: Coord) -> Vec3 {
    let half = WORLD_SIZE as f32 / 2.0;
    Vec3::new(c.x as f32 - half, c.y as f32 - half, c.z as f32 - half)
}

// Returns (t_entry, face_normal)
fn ray_aabb(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3) -> Option<(f32, IVec3)> {
    let mut tmin = -f32::INFINITY;
    let mut tmax = f32::INFINITY;
    let mut n = IVec3::ZERO;

    // X
    if dir.x.abs() < 1e-8 {
        if origin.x < min.x || origin.x > max.x {
            return None;
        }
    } else {
        let inv = 1.0 / dir.x;
        let mut t1 = (min.x - origin.x) * inv;
        let mut t2 = (max.x - origin.x) * inv;
        let mut axis_n = IVec3::new(-1, 0, 0);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            axis_n = IVec3::new(1, 0, 0);
        }
        if t1 > tmin {
            tmin = t1;
            n = axis_n;
        }
        tmax = tmax.min(t2);
        if tmin > tmax {
            return None;
        }
    }

    // Y
    if dir.y.abs() < 1e-8 {
        if origin.y < min.y || origin.y > max.y {
            return None;
        }
    } else {
        let inv = 1.0 / dir.y;
        let mut t1 = (min.y - origin.y) * inv;
        let mut t2 = (max.y - origin.y) * inv;
        let mut axis_n = IVec3::new(0, -1, 0);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            axis_n = IVec3::new(0, 1, 0);
        }
        if t1 > tmin {
            tmin = t1;
            n = axis_n;
        }
        tmax = tmax.min(t2);
        if tmin > tmax {
            return None;
        }
    }

    // Z
    if dir.z.abs() < 1e-8 {
        if origin.z < min.z || origin.z > max.z {
            return None;
        }
    } else {
        let inv = 1.0 / dir.z;
        let mut t1 = (min.z - origin.z) * inv;
        let mut t2 = (max.z - origin.z) * inv;
        let mut axis_n = IVec3::new(0, 0, -1);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            axis_n = IVec3::new(0, 0, 1);
        }
        if t1 > tmin {
            tmin = t1;
            n = axis_n;
        }
        tmax = tmax.min(t2);
        if tmin > tmax {
            return None;
        }
    }

    Some((tmin, n))
}
