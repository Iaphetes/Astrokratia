use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::camera,
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
    offset: f32,
    camera_rotation: CameraRotation,
    velocity_offset: f32,
    focal_point: Vec3,
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
            offset: -30.0,
            camera_rotation: CameraRotation::MatchY,
            velocity_offset: 50.0,
            focal_point: Vec3 {
                x: 0.0,
                y: 4.0,
                z: 0.0,
            },
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
fn calculate_focussing_point(
    vessel_velocity: &VelocityVector,
    camera_behaviour: &CameraBehaviour,
    vessel_transform: &Transform,
) -> Vec3 {
    let focussing_distance: f32 = logistic_interpolation(
        vessel_velocity.linear_velocity.length().abs(),
        camera_behaviour.velocity_offset,
        0.01,
        1.0,
    );
    let focussing_point = vessel_transform.translation
        + Transform::from_xyz(focussing_distance, 0.0, 0.0)
            .with_rotation(vessel_transform.rotation)
            .translation;
    let mut rotated_focussing_point = vessel_transform.clone();
    if vessel_velocity.angular_velocity.length() == 0.0 {
        return focussing_point;
    }
    let turn_circle_circumference = -2.0 * PI * vessel_velocity.turn_radius;
    let mut turn_circle_center = vessel_transform.clone();
    turn_circle_center.translation.z += vessel_velocity.turn_radius;
    rotated_focussing_point.rotate_around(
        turn_circle_center.translation,
        Quat::from_rotation_y(2.0 * PI * (focussing_distance / turn_circle_circumference)),
    );

    let rotation_interpolation_factor = logistic_interpolation(
        vessel_velocity.angular_velocity.length().abs(),
        2.0,
        0.1,
        1.0,
    );

    focussing_point.lerp(
        rotated_focussing_point.translation,
        rotation_interpolation_factor,
    )
}

fn calculate_camera_position(
    vessel_transform: &Transform,
    camera_behaviour: &CameraBehaviour,
    focussing_point: &Vec3,
) -> Vec3 {
    let focal_point: Vec3 = vessel_transform.translation + camera_behaviour.focal_point;
    return focal_point + (*focussing_point - focal_point).normalize() * camera_behaviour.offset;
}

fn track_camera(
    controlled_vessels: Query<(&Transform, &VesselID, &VelocityVector)>,
    mut cameras: Query<(&mut Transform, &CameraBehaviour), Without<VesselID>>,
) {
    let Ok((mut camera_transform, camera_behaviour)) = cameras.get_single_mut() else {
        return;
    };
    for (vessel_transform, vessel_id, velocity_vector) in controlled_vessels.iter() {
        if vessel_id.player == Player::Host {
            let focussing_point: Vec3 =
                calculate_focussing_point(&velocity_vector, &camera_behaviour, &vessel_transform);
            let mut new_camera_transform = camera_transform.clone();
            new_camera_transform.translation =
                calculate_camera_position(&vessel_transform, &camera_behaviour, &focussing_point);
            match camera_behaviour.camera_rotation {
                CameraRotation::MatchY => {
                    new_camera_transform.look_at(focussing_point, Vec3::Y);
                }
            };
            new_camera_transform
                .rotate_around(vessel_transform.translation, vessel_transform.rotation);

            *camera_transform = new_camera_transform;
        }
    }
}
