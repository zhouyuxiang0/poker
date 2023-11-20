use bevy::prelude::*;

use crate::common::{AppState, GameSounds, GameTextureAtlasHandles, MenuButton};

#[derive(Component)]
pub struct OnStartMenuScreen;

#[derive(Component)]
pub struct OnStartMenuScreenMultiplayerModeFlag;

pub fn setup_start_menu(
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
            OnStartMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset_server.load("loading_bg.png").into(),
                ..default()
            });
            parent
                .spawn(ButtonBundle {
                    image: asset_server.load("image/button/btn_weixin.png").into(),
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        margin: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        top: Val::Px(290.),
                        left: Val::Px(30.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MenuButton::Weixin);
            parent
                .spawn((
                    ButtonBundle {
                        image: asset_server.load("image/button/btn_ traveler.png").into(),
                        style: Style {
                            width: Val::Px(200.),
                            height: Val::Px(60.),
                            margin: UiRect::all(Val::Px(10.0)),
                            position_type: PositionType::Absolute,
                            top: Val::Px(200.),
                            left: Val::Px(30.),
                            ..Default::default()
                        },
                        ..default()
                    },
                    OnStartMenuScreenMultiplayerModeFlag,
                ))
                .insert(MenuButton::Traveler);
            parent.spawn((
                ButtonBundle {
                    image: asset_server.load("image/yonghuxieyi.png").into(),
                    style: Style {
                        width: Val::Px(400.),
                        height: Val::Px(50.),
                        margin: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        top: Val::Px(600.),
                        left: Val::Px(400.),
                        ..Default::default()
                    },
                    ..default()
                },
                OnStartMenuScreenMultiplayerModeFlag,
            ));
            parent.spawn((
                ImageBundle {
                    image: asset_server.load("image/check_mark.png").into(),
                    style: Style {
                        width: Val::Px(70.),
                        height: Val::Px(50.),
                        margin: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        top: Val::Px(600.),
                        left: Val::Px(395.),
                        ..Default::default()
                    },
                    ..default()
                },
                OnStartMenuScreenMultiplayerModeFlag,
            ));
        });
    commands.spawn(AudioBundle {
        source: game_sounds.login_bg.clone(),
        // settings: PlaybackSettings::LOOP,
        settings: PlaybackSettings::ONCE,
    });
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// pub fn start_game(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
//     if keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
//         info!("Switch app state to lobby");
//         app_state.set(AppState::Lobby);
//     }
// }
