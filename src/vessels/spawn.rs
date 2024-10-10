use crate::{player::player::Player, vessels::vessels::VesselDefinition};
use bevy::prelude::*;

use super::{movements::VelocityVector, vessels::VesselID};

pub fn spawn_vessel(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    vessel_definition: &VesselDefinition,
    vessel_id: VesselID,
    velocity_vector: VelocityVector,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(
                GltfAssetLabel::Scene(0).from_asset(vessel_definition.model_path.clone()),
                // GltfAssetLabel::Scene(0).from_asset(),
            ),
            ..default()
        },
        vessel_id,
        vessel_definition.clone(),
        velocity_vector,
    ));
}
