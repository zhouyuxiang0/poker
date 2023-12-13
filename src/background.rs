use bevy::prelude::*;

use crate::common::{AppState, MyAssets};

#[derive(Component)]
pub(crate) struct Background;

impl Plugin for Background {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), setup_start_menu_background);
    }
}

pub fn setup_start_menu_background(mut commands: Commands, asset: Res<MyAssets>) {
    commands.spawn(SpriteBundle {
        texture: asset.loading_bg.clone(),
        ..Default::default()
    });
}
