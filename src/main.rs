//! A simple 3D scene with light shining over a cube sitting on a plane.
mod environment;
mod player;
mod vessels;
use std::collections::HashMap;

use bevy::prelude::*;
use environment::{skybox::SkyboxPlugin, solar_system::SolarSystemPlugin};
use player::{camera::FlightCameraPlugin, input::InputParser};
use vessels::{
    movements::{MovementEvent, MovementProperties, VelocityVector, VesselMovement},
    spawn::spawn_vessel,
    vessels::{VesselDefinition, VesselID},
    weapons::{Hardpoint, WeaponsPlugin, WeaponsType},
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlightCameraPlugin,
            VesselMovement,
            InputParser,
            SkyboxPlugin,
            SolarSystemPlugin,
            WeaponsPlugin,
        ))
        .add_event::<MovementEvent>()
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cube
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
                x: 100.0,
                y: 5.0,
                z: 5.0,
            },
            angular_acceleration: Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
        },
        hardpoints: vec![(
            WeaponsType::Plasma,
            vec![
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 10.0,
                        y: 0.0,
                        z: -1.80,
                    }),
                },
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 9.50,
                        y: 0.0,
                        z: -2.1,
                    }),
                },
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 9.0,
                        y: 0.0,
                        z: -2.40,
                    }),
                },
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 10.0,
                        y: 0.0,
                        z: 1.80,
                    }),
                },
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 9.50,
                        y: 0.0,
                        z: 2.10,
                    }),
                },
                Hardpoint {
                    transform: Transform::from_translation(Vec3 {
                        x: 9.0,
                        y: 0.0,
                        z: 2.40,
                    }),
                },
            ],
        )]
        .iter()
        .cloned()
        .collect(),
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
