use bevy::prelude::*;
use bevy_matchbox::{
    matchbox_socket::{PeerId, PeerState, SingleChannel},
    MatchboxSocket,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{despawn_screen, AppState, Event, MyAssets},
    room::{Room, Rooms},
};

#[derive(Component)]
pub struct LobbyPlugin;

#[derive(Component)]
pub enum LobbyButton {
    EnterRoom,
    CreateRoom,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddressedEvent {
    src: PeerId,
    event: Event,
}

#[derive(Resource)]
pub struct Lobby {
    pub wait_players: Vec<PeerId>,
    socket: MatchboxSocket<SingleChannel>,
    pub rooms: Vec<Room>,
}

impl Lobby {
    pub fn new(mut socket: MatchboxSocket<SingleChannel>) -> Self {
        Self {
            wait_players: vec![],
            socket,
            rooms: vec![],
        }
    }

    fn receive(&mut self) -> Vec<AddressedEvent> {
        self.socket
            .receive()
            .iter()
            .map(|(_, payload)| payload)
            .filter_map(|payload| ciborium::de::from_reader(&payload[..]).ok())
            .collect()
    }

    fn send(&mut self, event: AddressedEvent) {
        let mut payload = Vec::new();
        ciborium::ser::into_writer(&event, &mut payload).unwrap();
        let peers: Vec<_> = self.socket.connected_peers().collect();
        for peer in peers {
            self.socket.send(payload.clone().into(), peer);
        }
    }

    fn join(&mut self, peer_id: PeerId) {
        self.wait_players.push(peer_id);
    }

    fn remove_player(&mut self, p: PeerId) {
        self.wait_players.retain(|peer| peer == &p);
    }

    fn contact_rooms(&mut self, rooms: Vec<Room>) {
        let mut new_rooms = rooms
            .iter()
            .filter(|&room| !self.rooms.contains(room))
            .map(|v| v.to_owned())
            .collect::<Vec<Room>>();
        self.rooms.append(&mut new_rooms);
    }
}

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), setup_lobby)
            .add_systems(
                Update,
                (lobby_button_press_system, receive_events, lobby_system)
                    .run_if(in_state(AppState::Lobby)),
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
            parent
                .spawn(ButtonBundle {
                    image: asset.btn_create_room.clone().into(),
                    style: Style {
                        width: Val::Px(390.),
                        height: Val::Px(160.),
                        position_type: PositionType::Absolute,
                        top: Val::Px(395.),
                        left: Val::Px(820.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LobbyButton::CreateRoom);
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
    mut commands: Commands,
    query: Query<(&Interaction, &LobbyButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
    mut lobby: ResMut<Lobby>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LobbyButton::EnterRoom => {
                    state.set(AppState::InRoom);
                }
                LobbyButton::CreateRoom => {
                    let room = Room::new(lobby.socket.id().unwrap());
                    commands.insert_resource(room);
                    state.set(AppState::InRoom);
                }
            }
        }
    }
}

pub fn lobby_system(mut lobby: ResMut<Lobby>) {
    let mut add_wait_players = vec![];
    let mut remove_wait_players = vec![];
    for (peer, new_state) in lobby.socket.update_peers() {
        match new_state {
            PeerState::Connected => {
                add_wait_players.push(peer);
            }
            PeerState::Disconnected => {
                remove_wait_players.push(peer);
            }
        }
    }
    if lobby.socket.id().is_some() {
        let src = lobby.socket.id().unwrap();
        let rooms = lobby.rooms.to_owned();
        let wait_players = lobby.wait_players.to_owned();
        lobby.send(AddressedEvent {
            src,
            event: Event::SyncLobby {
                wait_players,
                add_wait_players,
                remove_wait_players,
                rooms,
                // add_rooms: todo!(),
                // remove_rooms: todo!(),
            },
        });
    }
}

pub fn receive_events(mut lobby: ResMut<Lobby>) {
    let binding = lobby.receive();
    let events = Vec::from_iter(
        binding
            .iter()
            .filter(|e| e.src != lobby.socket.id().unwrap()),
    );
    // for AddressedEvent { src, event } in events {
    //     match event {
    //         Event::SyncLobby {
    //             wait_players,
    //             rooms,
    //             add_wait_players,
    //             remove_wait_players,
    //         } => {
    //             lobby.contact_rooms(rooms.to_vec());
    //             // lobby.wait_players.
    //         }
    //     }
    // }
}
