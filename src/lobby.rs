use bevy::prelude::*;
use bevy_matchbox::matchbox_socket::{PeerId, PeerState};

use crate::{
    common::{despawn_screen, AddressedEvent, AppState, Event, MyAssets, Socket},
    player::Player,
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
    rooms: Vec<Room>,
}

impl Lobby {
    pub fn new() -> Self {
        Self { rooms: vec![] }
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn remove_room_by_peer(&mut self, peer: PeerId) {
        self.rooms.retain(|room| {
            if room.owner.id == peer && room.players.len() <= 1 {
                false
            } else {
                true
            }
        })
    }
}

impl Plugin for LobbyComponent {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lobby), setup_lobby)
            .add_systems(
                Update,
                (update_lobby, lobby_button_press_system, receive_events)
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
    player: Res<Player>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LobbyButton::EnterRoom => {
                    // 加入房间 没有房间则创建 有则加入
                    let rooms = lobby.rooms.to_owned();
                    if let Some(room) = rooms
                        .iter()
                        .find(|r| r.players.iter().filter(|p| p.is_none()).count() > 0)
                    {
                        println!("请求加入房间{:?}", room);
                        socket.send_unreliable(
                            AddressedEvent {
                                src: player.clone(),
                                event: Event::JoinRoom,
                            },
                            room.players
                                .iter()
                                .filter(|p| p.is_some())
                                .map(|v| v.clone().unwrap().id)
                                .collect::<Vec<PeerId>>(),
                        );
                    } else {
                        // 创建房间 通知其他客户端房间信息
                        let room = Room::new(player.clone());
                        lobby.add_room(room.clone());
                        commands.insert_resource(room.clone());
                        state.set(AppState::InRoom);
                    }
                }
                LobbyButton::CreateRoom => {
                    // 创建房间 通知其他客户端房间信息
                    let room = Room::new(player.clone());
                    lobby.add_room(room.clone());
                    commands.insert_resource(room.clone());
                    state.set(AppState::InRoom);
                }
            }
        }
    }
}

pub fn update_lobby(mut socket: ResMut<Socket>, mut lobby: ResMut<Lobby>) {
    for (peer, state) in socket.update_peers_unreliable() {
        match state {
            PeerState::Disconnected => lobby.remove_room_by_peer(peer),
            _ => {}
        }
    }
}

pub fn receive_events(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
    player: Res<Player>,
) {
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
                let p = Some(player.clone());
                if room.players.contains(&p) {
                    state.set(AppState::InRoom);
                }
            }
            Event::Test(_) => todo!(),
            _ => {}
        }
    }
}
