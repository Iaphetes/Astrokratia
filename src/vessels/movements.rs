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
                }
            }
        }
    }
}
pub fn apply_velocity(time: Res<Time>, mut vessels: Query<(&mut Transform, &mut VelocityVector)>) {
    for (mut vessel_transform, vessel_velocity) in vessels.iter_mut() {
        vessel_transform.translation.x += vessel_velocity.linear_velocity.x * time.delta_seconds();
        vessel_transform.translation.y += vessel_velocity.linear_velocity.y * time.delta_seconds();
        vessel_transform.translation.z += vessel_velocity.linear_velocity.z * time.delta_seconds();
    }
}
