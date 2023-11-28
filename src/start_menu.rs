use bevy::prelude::*;

use crate::{
    common::{despawn_screen, AppState, MenuButton, MyAssets},
    lobby::Lobby,
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
            StartMenuPlugin,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.loading_bg.clone().into(),
                ..default()
            });
            parent
                .spawn(ButtonBundle {
                    image: assets.btn_weixin.clone().into(),
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
                .spawn(ButtonBundle {
                    image: assets.btn_traveler.clone().into(),
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
                })
                .insert(MenuButton::Traveler);
            parent.spawn(ButtonBundle {
                image: assets.yonghuxieyi.clone().into(),
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
            });
            parent.spawn(ImageBundle {
                image: assets.check_mark.clone().into(),
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
            });
        });
    commands.spawn(AudioBundle {
        source: assets.login_bg.clone(),
        // settings: PlaybackSettings::LOOP,
        settings: PlaybackSettings::ONCE,
    });
}

pub fn menu_button_press_system(
    mut commands: Commands,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Traveler => {
                    let room_url = "ws://127.0.0.1:3536/poker";
                    // let (socket, message_loop) = WebRtcSocket::builder(room_url)
                    //     .add_unreliable_channel()
                    //     .add_reliable_channel()
                    //     .build();
                    info!("connecting to matchbox server: {room_url}");
                    let socket = MatchboxSocket::new_ggrs(room_url);
                    let lobby = Lobby::new(socket);
                    commands.insert_resource(lobby);
                    state.set(AppState::Lobby);
                }
                MenuButton::Weixin => {
                    // println!("weixin");
                }
            }
        }
    }
}
