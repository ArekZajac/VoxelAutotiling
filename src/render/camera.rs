use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitCamera {
    pub target: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

pub fn spawn_camera(mut commands: Commands) {
    let target = Vec3::ZERO;
    let radius = 18.0;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, radius).looking_at(target, Vec3::Y),
        OrbitCamera {
            target,
            radius,
            yaw: 0.0,
            pitch: 0.4,
            sensitivity: 0.008,
        },
    ));
}

pub fn orbit_camera(
    mut mouse_motion: MessageReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cam: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    if !buttons.pressed(MouseButton::Middle) {
        mouse_motion.clear();
        return;
    }

    let mut delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        delta += ev.delta;
    }
    if delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut orbit) in &mut cam {
        orbit.yaw += delta.x * orbit.sensitivity;
        orbit.pitch += delta.y * orbit.sensitivity;

        let max_pitch = 1.55;
        orbit.pitch = orbit.pitch.clamp(-max_pitch, max_pitch);

        let (sy, cy) = orbit.yaw.sin_cos();
        let (sp, cp) = orbit.pitch.sin_cos();

        let offset = Vec3::new(cy * cp, sp, sy * cp) * orbit.radius;
        transform.translation = orbit.target + offset;
        transform.look_at(orbit.target, Vec3::Y);
    }
}
