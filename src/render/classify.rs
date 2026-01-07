use super::autotile::Neighbors;
use super::tile_kind::TileKind;
use bevy::prelude::*;

fn conn_dirs(nei: Neighbors) -> Vec<IVec3> {
    let mut v = Vec::new();
    if nei.nx {
        v.push(IVec3::new(-1, 0, 0));
    }
    if nei.px {
        v.push(IVec3::new(1, 0, 0));
    }
    if nei.ny {
        v.push(IVec3::new(0, -1, 0));
    }
    if nei.py {
        v.push(IVec3::new(0, 1, 0));
    }
    if nei.nz {
        v.push(IVec3::new(0, 0, -1));
    }
    if nei.pz {
        v.push(IVec3::new(0, 0, 1));
    }
    v
}

fn is_opposite_pair(a: IVec3, b: IVec3) -> bool {
    a == -b
}

fn is_straight(conns: &[IVec3]) -> bool {
    conns.len() == 2 && is_opposite_pair(conns[0], conns[1])
}

fn kind_from_conns(conns: &[IVec3]) -> TileKind {
    match conns.len() {
        0 => TileKind::Solo,
        1 => TileKind::End,
        2 => {
            if is_straight(conns) {
                TileKind::Straight
            } else {
                TileKind::Corner
            }
        }
        3 => TileKind::Tee,
        _ => TileKind::Cross,
    }
}

// Blender Z <-> Bevy Y
fn canonical_conns(kind: TileKind) -> Vec<IVec3> {
    let nx = IVec3::new(-1, 0, 0);
    let px = IVec3::new(1, 0, 0);
    let ny = IVec3::new(0, -1, 0);
    let py = IVec3::new(0, 1, 0);

    match kind {
        TileKind::Solo => vec![],
        TileKind::End => vec![ny],               // Blender: -Z -> Bevy: -Y
        TileKind::Straight => vec![ny, py],      // Blender: -Z,+Z -> Bevy: -Y,+Y
        TileKind::Corner => vec![ny, px],        // Blender: -Z,+X -> Bevy: -Y,+X
        TileKind::Tee => vec![ny, px, nx],       // Blender: -Z,+X,-X -> Bevy: -Y,+X,-X
        TileKind::Cross => vec![ny, py, px, nx], // Blender: -Z,+Z,+X,-X -> Bevy: -Y,+Y,+X,-X
    }
}

fn all_cube_rotations() -> Vec<Quat> {
    let forwards = [Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y, Vec3::Z, -Vec3::Z];
    let ups = [Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y, Vec3::Z, -Vec3::Z];

    let mut rots: Vec<Quat> = Vec::new();

    for f in forwards {
        for u in ups {
            if f.dot(u).abs() > 0.001 {
                continue;
            }

            let r = Quat::from_mat3(&Mat3::from_cols(
                u.cross(f), // Right
                u,          // Up
                f,          // Forward
            ));

            if !rots.iter().any(|q: &Quat| q.dot(r).abs() > 0.9999) {
                rots.push(r);
            }
        }
    }

    rots
}

fn rotate_dir(q: Quat, d: IVec3) -> IVec3 {
    let v = q * d.as_vec3();
    IVec3::new(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
}

fn set_eq(mut a: Vec<IVec3>, mut b: Vec<IVec3>) -> bool {
    a.sort_by_key(|v| (v.x, v.y, v.z));
    b.sort_by_key(|v| (v.x, v.y, v.z));
    a == b
}

fn find_rotation(canon: &[IVec3], actual: &[IVec3]) -> Quat {
    if canon.is_empty() && actual.is_empty() {
        return Quat::IDENTITY;
    }

    for r in all_cube_rotations() {
        let rotated: Vec<IVec3> = canon.iter().copied().map(|d| rotate_dir(r, d)).collect();
        if set_eq(rotated, actual.to_vec()) {
            return r;
        }
    }

    Quat::IDENTITY
}

pub fn classify(nei: Neighbors) -> (TileKind, Quat) {
    let actual_conns = conn_dirs(nei);
    let kind = kind_from_conns(&actual_conns);
    let canon = canonical_conns(kind);
    let rot = find_rotation(&canon, &actual_conns);
    (kind, rot)
}
