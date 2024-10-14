use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::rapier::pipeline::DebugColor;

use crate::player::input::movement_input;

use super::vessels::{VesselDefinition, VesselID};

pub struct VesselMovement;
impl Plugin for VesselMovement {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_velocity.after(movement_input),
                apply_velocity.after(change_velocity),
            ),
        );
    }
}
pub enum MovementType {
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
}
#[derive(Event)]
pub struct MovementEvent {
    pub movement_type: MovementType,
    pub vessel_id: VesselID,
}
#[derive(Component, Default)]
pub struct VelocityVector {
    linear_velocity: Vec3,
    angular_velocity: Vec3,
}

#[derive(Clone)]
pub struct MovementProperties {
    pub linear_acceleration: Vec3,
    pub angular_acceleration: Vec3,
}
fn change_velocity(
    time: Res<Time>,
    mut movement_events: EventReader<MovementEvent>,
    mut vessels: Query<(&mut VelocityVector, &VesselID, &VesselDefinition)>,
) {
    for (mut vessel_velocity, vessel_id, vessel_definition) in vessels.iter_mut() {
        for movement_event in movement_events.read() {
            if movement_event.vessel_id == *vessel_id {
                match movement_event.movement_type {
                    MovementType::Forward => {
                        vessel_velocity.linear_velocity.x += time.delta_seconds()
                            * vessel_definition.movement_properties.linear_acceleration.x
                    }
                    MovementType::Backward => {
                        vessel_velocity.linear_velocity.x -= time.delta_seconds()
                            * vessel_definition.movement_properties.linear_acceleration.x
                    }
                    MovementType::TurnLeft => {
                        vessel_velocity.angular_velocity.y += time.delta_seconds()
                            * vessel_definition.movement_properties.angular_acceleration.y
                            * 2.0
                    }
                    MovementType::TurnRight => {
                        vessel_velocity.angular_velocity.y -= time.delta_seconds()
                            * vessel_definition.movement_properties.angular_acceleration.y
                            * 2.0
                    }
                }
            }
        }
        let angular_velocity_change =
            time.delta_seconds() * vessel_definition.movement_properties.angular_acceleration.y;
        if vessel_velocity.angular_velocity.y.abs() <= angular_velocity_change {
            vessel_velocity.angular_velocity.y = 0.0
        } else {
            if vessel_velocity.angular_velocity.y > 0.0 {
                vessel_velocity.angular_velocity.y -= angular_velocity_change;
            } else if vessel_velocity.angular_velocity.y < 0.0 {
                vessel_velocity.angular_velocity.y += angular_velocity_change;
            }
        }
    }
}
// currently only considers rotation around the y axis
fn calculate_turn_circle_center(
    transform: &Transform,
    vessel_velocity: &VelocityVector,
) -> (f32, Vec3) {
    let turn_radius = vessel_velocity.linear_velocity.x / vessel_velocity.angular_velocity.y * 10.0;

    let mut turn_circle_center = transform.clone();
    turn_circle_center.translation.z -= turn_radius;

    turn_circle_center.rotate_around(transform.translation, transform.rotation);
    (turn_radius, turn_circle_center.translation)
}
#[derive(Component)]
pub struct DebugCube;
pub fn apply_velocity(
    mut commands: Commands,
    time: Res<Time>,
    mut vessels: Query<(&mut Transform, &mut VelocityVector)>,
    mut debug_cube_query: Query<
        (Entity, &mut Transform),
        (With<DebugCube>, Without<VelocityVector>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut vessel_transform, vessel_velocity) in vessels.iter_mut() {
        let mut debug_cube_instance = debug_cube_query.get_single_mut();
        if vessel_velocity.angular_velocity == Vec3::ZERO {
            if let Ok((debug_cube_entity, _debug_cube)) = debug_cube_instance {
                commands.entity(debug_cube_entity).despawn();
            }
            let x_direction = vessel_transform.local_x();
            let y_direction = vessel_transform.local_y();
            let z_direction = vessel_transform.local_z();

            let global_velocity_vector = x_direction * vessel_velocity.linear_velocity.x
                + y_direction * vessel_velocity.linear_velocity.y
                + z_direction * vessel_velocity.linear_velocity.z;

            vessel_transform.translation += global_velocity_vector * time.delta_seconds();
        } else {
            let (turn_radius, turn_circle_center) =
                calculate_turn_circle_center(&vessel_transform, &vessel_velocity);
            if turn_radius == 0.0 {
                vessel_transform
                    .rotate_local_y(vessel_velocity.angular_velocity.y * time.delta_seconds());
            } else {
                println!("Turn Radius: {:?}", turn_radius);

                match debug_cube_instance {
                    Ok((_, mut debug_cube_transform)) => {
                        debug_cube_transform.translation = turn_circle_center;
                    }
                    Err(_) => {
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::default()),

                                material: materials.add(StandardMaterial {
                                    emissive: LinearRgba::rgb(2.0, 13.99, 5.32),
                                    ..default()
                                }),
                                transform: Transform::from_translation(turn_circle_center),
                                ..default()
                            },
                            DebugCube,
                        ));
                    }
                }
                let turn_circle_circumference = 2.0 * PI * turn_radius;
                let path_travelled = vessel_velocity.linear_velocity.x * time.delta_seconds();
                vessel_transform.rotate_around(
                    turn_circle_center,
                    Quat::from_rotation_y(2.0 * PI * (path_travelled / turn_circle_circumference)),
                )
            }
        }
    }
}
