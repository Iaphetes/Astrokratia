use bevy::prelude::*;

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
    for movement_event in movement_events.read() {
        for (mut vessel_velocity, vessel_id, vessel_definition) in vessels.iter_mut() {
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
                    }
                    MovementType::TurnRight => {
                        vessel_velocity.angular_velocity.y -= time.delta_seconds()
                            * vessel_definition.movement_properties.angular_acceleration.y
                    }
                }
            }
        }
    }
}
pub fn apply_velocity(time: Res<Time>, mut vessels: Query<(&mut Transform, &mut VelocityVector)>) {
    for (mut vessel_transform, vessel_velocity) in vessels.iter_mut() {
        let x_direction = vessel_transform.local_x();
        let y_direction = vessel_transform.local_y();
        let z_direction = vessel_transform.local_z();

        let global_velocity_vector = x_direction * vessel_velocity.linear_velocity.x
            + y_direction * vessel_velocity.linear_velocity.y
            + z_direction * vessel_velocity.linear_velocity.z;

        println!("{:?}", global_velocity_vector);
        vessel_transform.translation += global_velocity_vector * time.delta_seconds();

        vessel_transform.rotate_local_y(vessel_velocity.angular_velocity.y * time.delta_seconds());
    }
}
