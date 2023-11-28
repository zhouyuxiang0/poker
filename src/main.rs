use bevy::prelude::*;

mod common;
mod lobby;
mod room;
mod start_menu;

use bevy_asset_loader::prelude::*;
use lobby::LobbyPlugin;
use room::{setup_room, wait_for_players};
use start_menu::StartMenuPlugin;

use common::{AppState, MyAssets};

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "poker".into(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::StartMenu),
        )
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_plugins(StartMenuPlugin)
        .add_plugins(LobbyPlugin)
        .add_systems(OnEnter(AppState::InRoom), setup_room)
        .add_systems(Update, wait_for_players.run_if(in_state(AppState::InRoom)))
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
