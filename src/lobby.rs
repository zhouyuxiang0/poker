use bevy::prelude::*;
use bevy_matchbox::{
    matchbox_socket::{PeerId, PeerState, SingleChannel},
    MatchboxSocket,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{despawn_screen, AppState, Event, MyAssets},
    room::Room,
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
        self.wait_players.retain(|peer| peer != &p);
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
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
    mut commands: Commands,
    query: Query<(&Interaction, &LobbyButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
    mut lobby: ResMut<Lobby>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LobbyButton::EnterRoom => {
                    // 加入房间 没有房间则创建 有则加入
                    state.set(AppState::InRoom);
                }
                LobbyButton::CreateRoom => {
                    // 创建房间 通知其他客户端房间信息
                    if let Some(peer) = lobby.socket.id() {
                        let room = Room::new(peer);
                        commands.insert_resource(room);
                        lobby.add_room(room);
                        // 与其他客户端同步room信息
                        lobby.send(AddressedEvent {
                            src: peer,
                            event: Event::SyncRoom(room),
                        });
                    }
                }
            }
        }
    }
}

pub fn lobby_system(mut lobby: ResMut<Lobby>) {
    for (peer, new_state) in lobby.socket.update_peers() {
        match new_state {
            PeerState::Connected => {
                lobby.join(peer);
            }
            PeerState::Disconnected => {
                lobby.remove_player(peer);
            }
        }
    }
    if let Some(local_peer) = lobby.socket.id() {
        if !lobby.wait_players.contains(&local_peer) {
            lobby.join(local_peer);
        }
    }
    // info!("{:?}", lobby.wait_players);
}

pub fn receive_events(mut lobby: ResMut<Lobby>) {
    // 接收room消息 将room收集为rooms
    let binding = lobby.receive();
    let events = Vec::from_iter(
        binding
            .iter()
            .filter(|e| e.src != lobby.socket.id().unwrap()),
    );
    for AddressedEvent { src, event } in events {
        match event {
            Event::SyncRoom(room) => {
                if !lobby.rooms.contains(room) {
                    println!("add new room {:?}", room);
                    lobby.add_room(room.to_owned());
                }
            }
        }
    }
}
