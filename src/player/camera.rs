use std::f32::consts::PI;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings, core_3d::Camera3dDepthTextureUsage, tonemapping::Tonemapping,
    },
    prelude::*,
    reflect::DynamicTypePath,
};

use crate::vessels::{
    movements::{apply_velocity, VelocityVector},
    vessels::VesselID,
};

use super::player::Player;
enum CameraRotation {
    MatchY,
}
#[derive(Component)]
struct CameraBehaviour {
    offset: Vec3,
    camera_rotation: CameraRotation,
    velocity_offset: f32,
}
pub struct FlightCameraPlugin;
impl Plugin for FlightCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enable_camera)
            .add_systems(Update, track_camera.after(apply_velocity));
    }
}
pub fn enable_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(-10.0, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // 3. Enable bloom for the camera
        BloomSettings::NATURAL,
        CameraBehaviour {
            offset: Vec3 {
                x: -30.0,
                y: 10.0,
                z: 0.0,
            },
            camera_rotation: CameraRotation::MatchY,
            velocity_offset: 50.0,
        },
    ));
    // commands.spawn((
    //     Camera3dBundle {
    //         camera: Camera {
    //             hdr: true, // 1. HDR is required for bloom
    //             ..default()
    //         },
    //         tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
    //         transform: Transform::from_xyz(0.0, 500.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     // 3. Enable bloom for the camera
    //     BloomSettings::NATURAL,
    //     CameraBehaviour {
    //         offset: Vec3 {
    //             x: 0.0,
    //             y: 10.0,
    //             z: -30.0,
    //         },
    //         camera_rotation: CameraRotation::MatchY,
    //     },
    // ));
}
fn logistic_interpolation(x: f32, supremum: f32, steepness: f32, offset: f32) -> f32 {
    return supremum / (1.0 + std::f32::consts::E.powf(-steepness * (x - offset)));
}

fn track_camera(
    controlled_vessels: Query<(&Transform, &VesselID, &VelocityVector)>,
    mut cameras: Query<(&mut Transform, &CameraBehaviour), Without<VesselID>>,
) {
    let Ok((mut camera_transform, camera_behaviour)) = cameras.get_single_mut() else {
        return;
    };
    for (transform, vessel_id, velocity_vector) in controlled_vessels.iter() {
        if vessel_id.player == Player::Host {
            let mut new_camera_transform = camera_transform.clone();
            new_camera_transform.translation = transform.translation + camera_behaviour.offset;
            let focussing_distance: f32 = logistic_interpolation(
                velocity_vector.linear_velocity.length().abs(),
                camera_behaviour.velocity_offset,
                0.01,
                1.0,
            );
            println!("focussing_distance {:?}", focussing_distance);
            println!("velocity_vector {:?}", velocity_vector.linear_velocity);
            let focussing_point: Vec3 = transform.translation
                + Transform::from_xyz(focussing_distance, 0.0, 0.0)
                    .with_rotation(transform.rotation)
                    .translation;
            match camera_behaviour.camera_rotation {
                CameraRotation::MatchY => {
                    new_camera_transform.look_at(focussing_point, Vec3::Y);
                }
            };
            new_camera_transform.rotate_around(transform.translation, transform.rotation);

            *camera_transform = new_camera_transform;
        }
    }
}
