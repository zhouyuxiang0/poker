use bevy::prelude::*;

use crate::{
    common::{despawn_screen, AppState, MenuButton, MyAssets, Socket},
    lobby::Lobby,
    player::Player,
};
use bevy_matchbox::prelude::*;

#[derive(Component)]
pub(crate) struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), setup_start_menu)
            .add_systems(
                Update,
                (menu_button_press_system).run_if(in_state(AppState::StartMenu)),
            )
            .add_systems(
                OnExit(AppState::StartMenu),
                despawn_screen::<StartMenuPlugin>,
            );
    }
}

pub fn setup_start_menu(mut commands: Commands, assets: Res<MyAssets>) {
    let room_url = "ws://47.108.130.232:3536/poker";
    info!("connecting to matchbox server: {room_url}");
    let socket = MatchboxSocket::new_ggrs(room_url);
    let socket = Socket::new(socket);
    commands.insert_resource(socket);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..Default::default()
            },
            StartMenuPlugin,
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    StartMenuPlugin,
                    ImageBundle {
                        image: assets.loading_bg.clone().into(),
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder
                        .spawn(ButtonBundle {
                            image: assets.btn_weixin.clone().into(),
                            style: Style {
                                width: Val::Percent(16.),
                                height: Val::Percent(9.),
                                position_type: PositionType::Absolute,
                                top: Val::Percent(41.),
                                left: Val::Percent(2.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(MenuButton::Weixin);
                    builder
                        .spawn(ButtonBundle {
                            image: assets.btn_traveler.clone().into(),
                            style: Style {
                                width: Val::Percent(16.),
                                height: Val::Percent(9.),
                                position_type: PositionType::Absolute,
                                top: Val::Percent(30.),
                                left: Val::Percent(2.),
                                ..Default::default()
                            },
                            ..default()
                        })
                        .insert(MenuButton::Traveler);
                    builder.spawn(ButtonBundle {
                        image: assets.yonghuxieyi.clone().into(),
                        style: Style {
                            width: Val::Percent(34.),
                            height: Val::Percent(6.),
                            position_type: PositionType::Absolute,
                            top: Val::Percent(85.5),
                            left: Val::Percent(32.5),
                            ..Default::default()
                        },
                        ..default()
                    });
                    builder.spawn(ImageBundle {
                        image: assets.check_mark.clone().into(),
                        style: Style {
                            width: Val::Percent(5.),
                            height: Val::Percent(7.),
                            position_type: PositionType::Absolute,
                            top: Val::Percent(85.),
                            left: Val::Percent(34.),
                            ..Default::default()
                        },
                        ..default()
                    });
                });
        });
    // commands.spawn(AudioBundle {
    //     source: assets.login_bg.clone(),
    //     // settings: PlaybackSettings::LOOP,
    //     settings: PlaybackSettings::ONCE,
    // });
}

pub fn menu_button_press_system(
    mut commands: Commands,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Traveler => {
                    if let Some(peer) = socket.unreliable_id() {
                        let lobby = Lobby::new();
                        commands.insert_resource(lobby);
                        commands.insert_resource(Player::new(peer));
                        state.set(AppState::Lobby);
                    }
                }
                MenuButton::Weixin => {
                    // println!("weixin");
                }
            }
        }
    }
}
