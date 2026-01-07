use bevy::prelude::*;

mod render;
mod world;

fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: "assets".into(),
        ..default()
    }));

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);

    app.add_plugins((world::WorldPlugin, render::RenderPlugin))
        .add_systems(Startup, light_scene)
        .run();
}

pub fn light_scene(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::linear_rgb(0.35, 0.35, 0.35),
        brightness: 100.0,
        affects_lightmapped_meshes: false,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 60_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(20.0, 50.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
