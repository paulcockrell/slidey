use bevy::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

pub struct AsciiPlugin;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tilemap_packed.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(TILE_SIZE), 12, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(AsciiSheet(texture_atlas_handle));
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of ascii range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .id()
}
