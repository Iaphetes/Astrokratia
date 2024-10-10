//! A simple 3D scene with light shining over a cube sitting on a plane.
mod environment;
mod player;
mod vessels;
use bevy::prelude::*;
use environment::skybox::SkyboxPlugin;
use player::{camera::FlightCameraPlugin, input::InputParser};
use vessels::{
    movements::{MovementEvent, MovementProperties, VelocityVector, VesselMovement},
    spawn::spawn_vessel,
    vessels::{VesselDefinition, VesselID},
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlightCameraPlugin,
            VesselMovement,
            InputParser,
            SkyboxPlugin,
        ))
        .add_event::<MovementEvent>()
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(0.01, 1.0)),
        material: materials.add(StandardMaterial {
            emissive: LinearRgba::rgb(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_x(90.0)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    let player_vessel = VesselDefinition {
        class: vessels::vessels::VesselClass::Cruiser,
        faction: vessels::vessels::Faction::Greek,
        model_path: "3D/vessels/greek/Myrmidon_Leviathan/vessel.glb".to_owned(),
        movement_properties: MovementProperties {
            linear_acceleration: Vec3 {
                x: 20.0,
                y: 5.0,
                z: 5.0,
            },
            angular_acceleration: Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
        },
    };
    spawn_vessel(
        &mut commands,
        &asset_server,
        &player_vessel,
        VesselID {
            player: player::player::Player::Host,
            id: 0,
        },
        VelocityVector::default(),
    );
}
