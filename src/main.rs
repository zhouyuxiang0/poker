// #![windows_subsystem = "windows"]
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowMode, WindowResolution},
};

mod common;
mod lobby;
mod player;
mod room;
mod start_menu;
mod card;

use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier2d::prelude::*;
use lobby::LobbyComponent;
use room::RoomUIComponent;
use start_menu::StartMenuPlugin;

use common::{AppState, MyAssets};

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "斗地主".into(),
                    fit_canvas_to_parent: true,
                    resizable: false,
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin::default()),))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::StartMenu),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (setup, setup_rapier))
        .add_plugins(StartMenuPlugin)
        .add_plugins(LobbyComponent)
        .add_plugins(RoomUIComponent)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
