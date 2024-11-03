use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::vessels::{
    movements::{MovementEvent, MovementType},
    vessels::VesselID,
    weapons::{WeaponStats, WeaponsFireEvent, WeaponsType},
};

use super::player::Player;

pub struct InputParser;
impl Plugin for InputParser {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (weapons_input, movement_input));
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
pub fn weapons_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut weapons_fire_event: EventWriter<WeaponsFireEvent>,
) {
    if keys.pressed(KeyCode::Space) {
        weapons_fire_event.send(WeaponsFireEvent((
            VesselID {
                player: Player::Host,
                id: 0,
            },
            WeaponStats {
                weapons_type: WeaponsType::Plasma,
                color: LinearRgba::rgb(0.0, 255.0, 0.0),
                velocity: Vec3 {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        )));
    }
}
