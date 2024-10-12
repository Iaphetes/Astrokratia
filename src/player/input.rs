use bevy::prelude::*;

use crate::vessels::{
    movements::{MovementEvent, MovementType},
    vessels::VesselID,
};

use super::player::Player;

pub struct InputParser;
impl Plugin for InputParser {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_input);
    }
}
pub fn movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_events: EventWriter<MovementEvent>,
) {
    if keys.pressed(KeyCode::KeyW) {
        println!("sending forward event");
        movement_events.send(MovementEvent {
            movement_type: MovementType::Forward,
            vessel_id: VesselID {
                player: Player::Host,
                id: 0,
            },
        }); // W is being held down
    }
    if keys.pressed(KeyCode::KeyS) {
        movement_events.send(MovementEvent {
            movement_type: MovementType::Backward,
            vessel_id: VesselID {
                player: Player::Host,
                id: 0,
            },
        }); // W is being held down
    }
    if keys.pressed(KeyCode::KeyA) {
        movement_events.send(MovementEvent {
            movement_type: MovementType::TurnLeft,
            vessel_id: VesselID {
                player: Player::Host,
                id: 0,
            },
        });
    }
    if keys.pressed(KeyCode::KeyD) {
        movement_events.send(MovementEvent {
            movement_type: MovementType::TurnRight,
            vessel_id: VesselID {
                player: Player::Host,
                id: 0,
            },
        });
    }
}
