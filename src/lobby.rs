use bevy::prelude::*;

use crate::common::{GameSounds, GameTextureAtlasHandles};

#[derive(Component)]
pub struct Lobby;

pub fn setup_lobby(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_sounds: Res<GameSounds>,
    game_texture_atlas: Res<GameTextureAtlasHandles>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..Default::default()
            },
            Lobby,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset_server.load("bg_login.jpg").into(),
                ..default()
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: game_texture_atlas.you_qing.to_owned(),
                style: Style {
                    width: Val::Px(390.),
                    height: Val::Px(310.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(95.),
                    left: Val::Px(320.),
                    ..default()
                },
                ..Default::default()
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: game_texture_atlas.you_qing.to_owned(),
                texture_atlas_image: UiTextureAtlasImage {
                    index: 1,
                    ..Default::default()
                },
                style: Style {
                    width: Val::Px(390.),
                    height: Val::Px(310.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(195.),
                    left: Val::Px(700.),
                    ..default()
                },
                ..Default::default()
            });
            parent.spawn(ImageBundle {
                image: asset_server.load("image/btn_enter_room.png").into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(395.),
                    left: Val::Px(315.),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
