use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use crate::player::camera::enable_camera;

pub struct SkyboxPlugin;
impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, skybox_setup.after(enable_camera))
            .add_systems(Update, asset_loaded);
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}
fn skybox_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_entities: Query<Entity, With<Camera>>,
) {
    let skybox_handle: Handle<Image> = asset_server.load("Textures/Skybox/stacked.png");
    for camera_entity in camera_entities.iter() {
        commands.entity(camera_entity).insert((Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },));
    }
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}
fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(&cubemap.image_handle) == Some(LoadState::Loaded)
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
