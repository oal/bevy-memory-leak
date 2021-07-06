use bevy::prelude::*;
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use rand::prelude::*;

// Texture width / height for each tile
const SIZE: isize = 128;

#[derive(Debug)]
struct TileData {
    data: Vec<u8>,
}

impl TileData {
    pub fn from_data(data: Vec<u8>) -> Self {
        TileData {
            data
        }
    }
}

fn update_textures(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut query: Query<(&TileData, &Handle<ColorMaterial>)>,
) {
    for (tile, mat_handle) in query.iter_mut() {
        let material = materials.get_mut(mat_handle).unwrap();
        if let Some(tex_handle) = &material.texture {
            if let Some(texture) = textures.get_mut(tex_handle) {
                // Update texture data. Memory leaks even when the line below is commented out.
                texture.data = tile.data.clone();
            }
        } else {
            // This happens the first time something has changed for this tile (texture is not yet
            // created).
            let tx = Texture::new_fill(
                Extent3d::new(SIZE as u32, SIZE as u32, 1),
                TextureDimension::D2,
                &tile.data,
                TextureFormat::R8Unorm,
            );
            let texture_handle = textures.add(tx);
            material.texture = texture_handle.into();
        }
    };
}

fn modify_tile(mut query: Query<&mut TileData>) {
    // Modify one random pixel in the tile data.
    let mut rng = rand::thread_rng();
    for mut tile in query.iter_mut() {
        let random: f64 = rng.gen();
        let idx: f64 = random * (SIZE * SIZE) as f64;
        tile.data[idx as usize] = 255;
    }
}

fn init_tiles(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 0.8;
    commands.spawn_bundle(camera_bundle);


    // Generate a few tiles
    const OFFSET: f32 = 1.025; // Space between tiles.
    const WORLD_SIZE: isize = 2; // -2 to +2 in both directions, so 5x5 tiles.
    for ty in -WORLD_SIZE..WORLD_SIZE + 1 {
        for tx in -WORLD_SIZE..WORLD_SIZE + 1 {
            // Creates a subtle gradient from black to red
            let data = {
                let mut val = vec![0; (SIZE * SIZE) as usize];
                for y in 0..SIZE {
                    for x in 0..SIZE {
                        let idx = (y * SIZE + x) as usize;
                        let gx = tx * SIZE + x;
                        let gy = -ty * SIZE + y;
                        val[idx] = (gx as f32 / 25.0 + gy as f32 / 25.0) as u8;
                    }
                }
                val
            };

            // Create a tile with the data generated above, as well as a Sprite bundle so it
            // will be positioned and rendered.
            let material = ColorMaterial::color(Color::WHITE);
            let mat_handle = materials.add(material);
            commands
                .spawn_bundle(
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new((tx * SIZE) as f32 * OFFSET, (ty * SIZE) as f32, 0.0) * OFFSET),
                        material: mat_handle,
                        ..Default::default()
                    })
                .insert(TileData::from_data(data));
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(init_tiles.system())
        .add_system(modify_tile.system())
        .add_system(update_textures.system())

        .run();
}