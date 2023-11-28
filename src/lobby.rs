use bevy::prelude::*;
use bevy_matchbox::{
    matchbox_socket::{PeerId, PeerState, SingleChannel},
    MatchboxSocket,
};

use crate::{
    common::{despawn_screen, AppState, MyAssets},
    room::{Player, Rooms},
};

#[derive(Component)]
pub struct LobbyPlugin;

#[derive(Component)]
pub enum LobbyButton {
    EnterRoom,
    CreateRoom,
}

#[derive(Resource)]
pub struct Lobby {
    pub rooms: Rooms,
    pub wait_players: Vec<Player>,
    socket: MatchboxSocket<SingleChannel>,
}

impl Lobby {
    pub fn new(socket: MatchboxSocket<SingleChannel>) -> Self {
        Self {
            rooms: Rooms::default(),
            wait_players: vec![],
            socket,
        }
    }
}

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), setup_lobby)
            .add_systems(
                Update,
                (lobby_button_press_system, lobby_system).run_if(in_state(AppState::Lobby)),
            )
            .add_systems(OnExit(AppState::Lobby), (despawn_screen::<LobbyPlugin>,));
    }
}

pub fn setup_lobby(mut commands: Commands, asset: Res<MyAssets>) {
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
            LobbyPlugin,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset.bg_login.clone().into(),
                ..default()
            });
            parent.spawn(ImageBundle {
                image: asset.you_qing_girl.clone().into(),
                style: Style {
                    width: Val::Px(390.),
                    height: Val::Px(370.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(95.),
                    left: Val::Px(320.),
                    ..default()
                },
                ..Default::default()
            });
            parent.spawn(ImageBundle {
                image: asset.you_qing_boy.clone().into(),
                style: Style {
                    width: Val::Px(390.),
                    height: Val::Px(370.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(95.),
                    left: Val::Px(825.),
                    ..default()
                },
                ..Default::default()
            });
            parent
                .spawn(ButtonBundle {
                    image: asset.btn_enter_room.clone().into(),
                    style: Style {
                        width: Val::Px(390.),
                        height: Val::Px(160.),
                        position_type: PositionType::Absolute,
                        top: Val::Px(395.),
                        left: Val::Px(315.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LobbyButton::EnterRoom);
            parent.spawn(ImageBundle {
                image: asset.btn_create_room.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(395.),
                    left: Val::Px(820.),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(ImageBundle {
                image: asset.tip.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(220.),
                    left: Val::Px(-60.),
                    ..Default::default()
                },
                transform: Transform {
                    rotation: Quat::from_rotation_z(-1.57),
                    // rotation: Quat::from_array([20., 0., 0., 0.]),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn lobby_button_press_system(
    query: Query<(&Interaction, &LobbyButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LobbyButton::EnterRoom => {
                    state.set(AppState::InRoom);
                }
                LobbyButton::CreateRoom => {
                    // println!("weixin");
                }
            }
        }
    }
}

pub fn lobby_system(mut lobby: ResMut<Lobby>) {
    for (peer, new_state) in lobby.socket.update_peers() {
        // you can also handle the specific dis(connections) as they occur:
        match new_state {
            PeerState::Connected => {
                let mut payload = Vec::new();
                ciborium::ser::into_writer(&lobby, &mut payload).unwrap();
                lobby.socket.send(lobby, peer);
                info!("peer {peer} connected");
            }
            PeerState::Disconnected => info!("peer {peer} disconnected"),
        }
    }
    let connected_peers = lobby.socket.connected_peers();
    let peer_id = connected_peers.collect::<Vec<PeerId>>();
    // socket.send(Box::new(b"packet"), peer_id[0]);
    // println!("在线人数{} {:?}", peer_id.len() + 1, peer_id);
}
