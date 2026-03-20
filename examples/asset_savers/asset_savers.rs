use bevy::asset::processor::LoadTransformAndSave;
use bevy::asset::transformer::IdentityAssetTransformer;
use bevy::prelude::*;
use bevy_common_assets::cbor::{CborAssetPlugin, CborAssetSaver};
use bevy_common_assets::json::{JsonAssetLoader, JsonAssetPlugin, JsonAssetSaver};
use bevy_common_assets::msgpack::{MsgPackAssetPlugin, MsgPackAssetSaver};
use bevy_common_assets::postcard::{PostcardAssetPlugin, PostcardAssetSaver};
use bevy_common_assets::ron::{RonAssetLoader, RonAssetPlugin, RonAssetSaver};
use bevy_common_assets::toml::{TomlAssetPlugin, TomlAssetSaver};
use bevy_common_assets::yaml::{YamlAssetPlugin, YamlAssetSaver};
use serde::{Deserialize, Serialize};

/// This example processes source asset files into various binary and text formats using asset
/// savers, then loads and renders the processed assets.
///
/// When you run the example, `examples/asset_savers/imported_assets` is created and populated
/// with the processed files. Edit any source file in `examples/asset_savers/assets/` and
/// re-run to see the processor rebuild it.
///
/// Source files and their processing pipelines:
///
/// | Source file      | Source format | Processed format    |
/// |------------------|---------------|---------------------|
/// | trees.postlevel  | JSON          | Postcard (binary)   |
/// | trees.cborlevel  | JSON          | CBOR (binary)       |
/// | trees.msglevel   | JSON          | MessagePack (binary)|
/// | trees.ronlevel   | JSON          | RON                 |
/// | trees.jsonlevel  | RON           | JSON                |
/// | trees.tomllevel  | JSON          | TOML                |
/// | trees.yamllevel  | JSON          | YAML                |
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                mode: AssetMode::Processed,
                file_path: "examples/asset_savers/assets".to_string(),
                processed_file_path: "examples/asset_savers/imported_assets/Default".to_string(),
                ..default()
            }),
            // Each plugin registers the loader for the processed output extension
            PostcardAssetPlugin::<Level>::new(&["postlevel"]),
            CborAssetPlugin::<Level>::new(&["cborlevel"]),
            MsgPackAssetPlugin::<Level>::new(&["msglevel"]),
            RonAssetPlugin::<Level>::new(&["ronlevel"]),
            JsonAssetPlugin::<Level>::new(&["jsonlevel"]),
            TomlAssetPlugin::<Level>::new(&["tomllevel"]),
            YamlAssetPlugin::<Level>::new(&["yamllevel"]),
        ))
        // JSON source → Postcard binary
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            PostcardAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            PostcardAssetSaver::default(),
        ))
        // JSON source → CBOR binary
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            CborAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            CborAssetSaver::default(),
        ))
        // JSON source → MessagePack binary
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            MsgPackAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            MsgPackAssetSaver::default(),
        ))
        // JSON source → RON text
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            RonAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            RonAssetSaver::default(),
        ))
        // RON source → JSON text  (avoids a trivial JSON→JSON round-trip)
        .register_asset_processor::<LoadTransformAndSave<
            RonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            JsonAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            JsonAssetSaver::default(),
        ))
        // JSON source → TOML text
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            TomlAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            TomlAssetSaver::default(),
        ))
        // JSON source → YAML text
        .register_asset_processor::<LoadTransformAndSave<
            JsonAssetLoader<Level>,
            IdentityAssetTransformer<Level>,
            YamlAssetSaver<Level>,
        >>(LoadTransformAndSave::new(
            IdentityAssetTransformer::default(),
            YamlAssetSaver::default(),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_trees.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LevelHandles {
        postcard: asset_server.load("trees.postlevel"),
        cbor: asset_server.load("trees.cborlevel"),
        msgpack: asset_server.load("trees.msglevel"),
        ron: asset_server.load("trees.ronlevel"),
        json: asset_server.load("trees.jsonlevel"),
        toml: asset_server.load("trees.tomllevel"),
        yaml: asset_server.load("trees.yamllevel"),
    });
    commands.insert_resource(ImageHandle(asset_server.load("tree.png")));
    commands.spawn((Camera2d, Msaa::Off));
}

fn spawn_trees(
    mut commands: Commands,
    handles: Res<LevelHandles>,
    tree: Res<ImageHandle>,
    levels: Res<Assets<Level>>,
    mut state: ResMut<NextState<AppState>>,
) {
    let format_offsets: [(&Handle<Level>, Vec2); 7] = [
        (&handles.postcard, Vec2::new(-525., 75.)),
        (&handles.cbor, Vec2::new(-175., 75.)),
        (&handles.msgpack, Vec2::new(175., 75.)),
        (&handles.ron, Vec2::new(525., 75.)),
        (&handles.json, Vec2::new(-350., -75.)),
        (&handles.toml, Vec2::new(0., -75.)),
        (&handles.yaml, Vec2::new(350., -75.)),
    ];

    for (handle, _) in &format_offsets {
        if levels.get(handle.id()).is_none() {
            return;
        }
    }

    for (handle, offset) in &format_offsets {
        let level = levels.get(handle.id()).unwrap();
        for position in &level.positions {
            commands.spawn((
                Sprite::from_image(tree.0.clone()),
                Transform::from_translation(Vec3::new(
                    position[0] + offset.x,
                    position[1] + offset.y,
                    position[2],
                )),
            ));
        }
    }

    state.set(AppState::Level);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Level,
}

#[derive(Resource)]
struct ImageHandle(Handle<Image>);

#[derive(Resource)]
struct LevelHandles {
    postcard: Handle<Level>,
    cbor: Handle<Level>,
    msgpack: Handle<Level>,
    ron: Handle<Level>,
    json: Handle<Level>,
    toml: Handle<Level>,
    yaml: Handle<Level>,
}

#[derive(Deserialize, Serialize, Asset, TypePath)]
struct Level {
    positions: Vec<[f32; 3]>,
}
