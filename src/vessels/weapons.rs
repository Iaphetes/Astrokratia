use std::f32::consts::PI;

use bevy::prelude::*;

use super::vessels::{VesselDefinition, VesselID};

#[derive(Clone)]
pub struct Hardpoint {
    pub transform: Transform,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WeaponsType {
    Plasma,
}

struct Target(Entity);
#[derive(Clone, Component)]
pub struct WeaponStats {
    pub weapons_type: WeaponsType,
    pub color: LinearRgba,
    pub velocity: Vec3,
}

#[derive(Event)]
pub struct WeaponsFireEvent(pub (VesselID, WeaponStats));

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponsFireEvent>()
            .add_systems(Update, (fire_weapon, move_projectile));
    }
}
fn calculate_launch_transform(
    vessel_transform: &Transform,
    hardpoint_transform: &Transform,
) -> Transform {
    let mut absolute_hardpoint_transform = hardpoint_transform.clone();
    absolute_hardpoint_transform.translation += vessel_transform.translation;
    absolute_hardpoint_transform
        .rotate_around(vessel_transform.translation, vessel_transform.rotation);

    absolute_hardpoint_transform.rotate_local_z(PI / 2.0);
    absolute_hardpoint_transform
}

fn fire_weapon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shots_fired: EventReader<WeaponsFireEvent>,
    vessels: Query<(&VesselID, &VesselDefinition, &Transform)>,
) {
    for shot_fired in shots_fired.read() {
        let (firing_vessel_id, weapon_stats) = shot_fired.0.clone();
        if let Some((vessel_definition, vessel_transform)) = vessels
            .iter()
            .filter(|(vessel_id, _, _)| **vessel_id == firing_vessel_id)
            .map(|(_, vessel_definition, vessel_transform)| (vessel_definition, vessel_transform))
            .into_iter()
            .next()
        {
            if let Some((_, relevant_hardpoints)) = vessel_definition
                .hardpoints
                .iter()
                .filter(|(weapons_type, _)| **weapons_type == weapon_stats.clone().weapons_type)
                .into_iter()
                .next()
            {
                for hardpoint in relevant_hardpoints {
                    let launch_point =
                        calculate_launch_transform(vessel_transform, &hardpoint.transform);
                    match weapon_stats.weapons_type {
                        // let launch_position = calculate_launch_transform(vessel_transform, )
                        WeaponsType::Plasma => {
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cylinder::new(0.01, 1.0)),
                                    material: materials.add(StandardMaterial {
                                        emissive: weapon_stats.color.into(), // 4. Put something bright in a dark environment to see the effect
                                        ..default()
                                    }),
                                    transform: launch_point,
                                    ..default()
                                },
                                weapon_stats.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn move_projectile(mut projectiles: Query<(&mut Transform, &WeaponStats)>) {
    for (mut projectile_transform, weapon_stats) in projectiles.iter_mut() {
        let mut directed_velocity = Transform::from_translation(weapon_stats.velocity);
        let mut rotation = projectile_transform.rotation * Quat::from_rotation_z(-PI / 2.);

        directed_velocity.rotate_around(Vec3::ZERO, rotation);
        directed_velocity.rotate_local_z(-PI / 2.0);
        println!("{:?}", directed_velocity.translation);
        println!("{:?}", projectile_transform.rotation);

        projectile_transform.translation += directed_velocity.translation
    }
}
