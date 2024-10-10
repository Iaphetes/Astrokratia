use std::f32::consts::PI;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings, core_3d::Camera3dDepthTextureUsage, tonemapping::Tonemapping,
    },
    prelude::*,
    reflect::DynamicTypePath,
};

use crate::vessels::{movements::apply_velocity, vessels::VesselID};

use super::player::Player;
enum CameraRotation {
    MatchY,
}
#[derive(Component)]
struct CameraBehaviour {
    offset: Vec3,
    camera_rotation: CameraRotation,
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
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // 3. Enable bloom for the camera
        BloomSettings::NATURAL,
        CameraBehaviour {
            offset: Vec3 {
                x: 0.0,
                y: 10.0,
                z: -30.0,
            },
            camera_rotation: CameraRotation::MatchY,
        },
    ));
}

fn track_camera(
    controlled_vessels: Query<(&Transform, &VesselID)>,
    mut cameras: Query<(&mut Transform, &CameraBehaviour), Without<VesselID>>,
) {
    let Ok((mut camera_transform, camera_behaviour)) = cameras.get_single_mut() else {
        return;
    };
    for (transform, vessel_id) in controlled_vessels.iter() {
        if vessel_id.player == Player::Host {
            let mut new_camera_transform = camera_transform.clone();
            new_camera_transform.translation = transform.translation + camera_behaviour.offset;
            match camera_behaviour.camera_rotation {
                CameraRotation::MatchY => {
                    new_camera_transform.look_at(transform.translation, Vec3::Y);
                }
            };

            *camera_transform = new_camera_transform;
        }
    }
}
