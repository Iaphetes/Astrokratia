use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

pub struct SolarSystemPlugin;
impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, solar_system_setup);
    }
}

fn solar_system_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 3200.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
    commands.spawn((SceneBundle {
        scene: asset_server.load("3D/environment/planet.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 2.0, 6_772_000.0)
            .with_rotation(Quat::from_rotation_y((75.0_f32).to_radians()))
            .with_scale(Vec3::splat(1000.0)),
        ..default()
    },));
    let parent: Entity = commands
        .spawn((SceneBundle {
            scene: asset_server.load("3D/environment/asteroid_01.glb#Scene0"),
            transform: Transform::from_xyz(-5000.0, 2.0, 5.0).with_scale(Vec3::splat(10.0)),
            // transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },))
        .id();

    let parent: Entity = commands
        .spawn((SceneBundle {
            scene: asset_server.load("3D/environment/sun.glb#Scene0"),
            transform: Transform::from_xyz(150_000_000_000_000.0, 2.0, 5.0)
                .with_scale(Vec3::splat(100000000.0)),

            // transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },))
        .id();
}
