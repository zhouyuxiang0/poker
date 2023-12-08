use bevy::prelude::*;
use bevy_matchbox::matchbox_socket::{PeerId, PeerState};

use crate::{
    common::{despawn_screen, AddressedEvent, AppState, Event, MyAssets, Socket},
    room::Room,
};

#[derive(Component)]
pub struct LobbyComponent;

#[derive(Component)]
pub enum LobbyButton {
    EnterRoom,
    CreateRoom,
}

#[derive(Resource)]
pub struct Lobby {
    wait_players: Vec<PeerId>,
    rooms: Vec<Room>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            wait_players: vec![],
            rooms: vec![],
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

    fn contact_rooms(&mut self, rooms: Vec<Room>) {
        let mut new_rooms = rooms
            .iter()
            .filter(|&room| !self.rooms.contains(room))
            .map(|v| v.to_owned())
            .collect::<Vec<Room>>();
        self.rooms.append(&mut new_rooms);
    }
}

impl Plugin for LobbyComponent {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), setup_lobby)
            .add_systems(
                Update,
                (lobby_system, lobby_button_press_system, receive_events)
                    .run_if(in_state(AppState::Lobby)),
            )
            .add_systems(OnExit(AppState::Lobby), (despawn_screen::<LobbyComponent>,));
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
            LobbyComponent,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: asset.bg_login.clone().into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..default()
            });
            parent.spawn(ImageBundle {
                image: asset.you_qing_girl.clone().into(),
                style: Style {
                    width: Val::Percent(30.),
                    height: Val::Percent(50.),
                    // width: Val::P(390.),
                    // height: Val::Px(370.),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(14.),
                    left: Val::Percent(25.),
                    ..default()
                },
                ..Default::default()
            });
            parent.spawn(ImageBundle {
                image: asset.you_qing_boy.clone().into(),
                style: Style {
                    width: Val::Percent(30.),
                    height: Val::Percent(50.),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(14.),
                    left: Val::Percent(65.),
                    ..default()
                },
                ..Default::default()
            });
            parent
                .spawn(ButtonBundle {
                    image: asset.btn_enter_room.clone().into(),
                    style: Style {
                        width: Val::Percent(32.),
                        height: Val::Percent(23.),
                        position_type: PositionType::Absolute,
                        top: Val::Percent(57.),
                        left: Val::Percent(24.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LobbyButton::EnterRoom);
            parent
                .spawn(ButtonBundle {
                    image: asset.btn_create_room.clone().into(),
                    style: Style {
                        width: Val::Percent(32.),
                        height: Val::Percent(23.),
                        position_type: PositionType::Absolute,
                        top: Val::Percent(57.),
                        left: Val::Percent(63.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LobbyButton::CreateRoom);
            parent.spawn(ImageBundle {
                image: asset.tip.clone().into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(23.),
                    left: Val::Percent(3.),
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
    mut socket: ResMut<Socket>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LobbyButton::EnterRoom => {
                    if let Some(peer) = socket.unreliable_id() {
                        // 加入房间 没有房间则创建 有则加入
                        let rooms = lobby.rooms.to_owned();
                        if let Some(room) = rooms
                            .iter()
                            .find(|r| r.player2.is_none() || r.player3.is_none())
                        {
                            println!("请求加入房间{:?}", room);
                            socket.send_unreliable(
                                AddressedEvent {
                                    src: peer,
                                    event: Event::JoinRoom,
                                },
                                vec![room.id],
                            );
                        } else {
                            // create
                        }
                    }
                }
                LobbyButton::CreateRoom => {
                    // 创建房间 通知其他客户端房间信息
                    if let Some(peer) = socket.unreliable_id() {
                        let room = Room::new(peer);
                        commands.insert_resource(room);
                        lobby.add_room(room);
                        // 与其他客户端同步room信息
                        let peers = socket
                            .unreliable_connected_peers()
                            .collect::<Vec<PeerId>>()
                            .to_owned();
                        socket.send_unreliable(
                            AddressedEvent {
                                src: peer,
                                event: Event::SyncRoom(room),
                            },
                            peers,
                        );
                        state.set(AppState::InRoom);
                    }
                }
            }
        }
    }
}

pub fn lobby_system(mut lobby: ResMut<Lobby>, mut socket: ResMut<Socket>) {
    for (peer, new_state) in socket.update_peers_unreliable() {
        match new_state {
            PeerState::Connected => {
                lobby.join(peer);
            }
            PeerState::Disconnected => {
                lobby.remove_player(peer);
            }
        }
    }
    if let Some(local_peer) = socket.unreliable_id() {
        if !lobby.wait_players.contains(&local_peer) {
            lobby.join(local_peer);
        }
    }
}

pub fn receive_events(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
) {
    // 接收room消息 将room收集为rooms
    for AddressedEvent { src: _, event } in socket.receive_unreliable() {
        match event {
            Event::SyncRoom(room) => {
                if !lobby.rooms.contains(&room) {
                    println!("add new room {:?}", room);
                    lobby.add_room(room.to_owned());
                }
            }
            Event::JoinRoomSuccess(room) => {
                commands.insert_resource(room.to_owned());
                lobby.add_room(room);
                state.set(AppState::InRoom);
            }
            Event::Test(_) => todo!(),
            _ => {}
        }
    }
}
