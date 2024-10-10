use bevy::prelude::Component;

use crate::player::player::Player;

use super::movements::MovementProperties;

#[derive(Component, PartialEq, Eq)]
pub struct VesselID {
    pub player: Player,
    pub id: u32,
}
#[derive(Clone)]
pub enum VesselClass {
    Cruiser,
}
#[derive(Clone)]
pub enum Faction {
    Greek,
}

#[derive(Component, Clone)]
pub struct VesselDefinition {
    pub class: VesselClass,
    pub faction: Faction,
    pub model_path: String,
    pub movement_properties: MovementProperties,
}
