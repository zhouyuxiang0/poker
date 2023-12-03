use bevy::{prelude::*, utils::HashMap};
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    common::{despawn_screen, AddressedEvent, AppState, Event, MyAssets, Socket},
    lobby::Lobby,
};

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
struct CollabId(u16);
struct Peer {
    chalk: Entity,
    cursor: Entity,
}
struct Peers(HashMap<CollabId, Peer>);
#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Room {
    pub id: PeerId,
    pub player1: PeerId,
    pub player2: Option<PeerId>,
    pub player3: Option<PeerId>,
}

impl Room {
    pub fn join(&mut self, peer: PeerId) {
        if self.player2.is_none() {
            self.player2 = Some(peer)
        } else if self.player3.is_none() {
            self.player3 = Some(peer)
        }
    }
}

#[derive(Component)]
pub struct RoomComponent;

impl Room {
    pub fn new(peer: PeerId) -> Self {
        Self {
            id: peer,
            player1: peer,
            player2: None,
            player3: None,
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Room {}

#[derive(Resource)]
pub struct Rooms(Vec<Room>);

impl Plugin for RoomComponent {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InRoom), setup_room)
            .add_systems(
                Update,
                (publish_room, receive_events).run_if(in_state(AppState::InRoom)),
            )
            .add_systems(OnExit(AppState::InRoom), despawn_screen::<RoomComponent>);
    }
}

pub fn setup_room(
    mut commands: Commands,
    assets: Res<MyAssets>,
    room: ResMut<Room>,
    mut socket: ResMut<Socket>,
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
            RoomComponent,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.table_bg_1.clone().into(),
                style: Style {
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                ..default()
            });
            let local_player = socket.unreliable_id();
            let play_name = if room.player1 == local_player.unwrap() {
                "player 1"
            } else if room.player2 == local_player {
                "player 2"
            } else if room.player3 == local_player {
                "player 3"
            } else {
                unreachable!()
            };
            parent.spawn(TextBundle {
                text: Text::from_section(
                    play_name,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 24.0,
                        color: Color::GOLD,
                        ..Default::default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(525.),
                    left: Val::Px(70.),
                    ..Default::default()
                },
                ..default()
            });
            parent.spawn(ImageBundle {
                image: assets.room_touxiang.clone().into(),
                style: Style {
                    // width: Val::Px(70.),
                    // height: Val::Px(50.),
                    margin: UiRect::all(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(550.),
                    left: Val::Px(50.),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn publish_room(_lobby: ResMut<Lobby>, room: ResMut<Room>, mut socket: ResMut<Socket>) {
    let peers = socket
        .unreliable_connected_peers()
        .collect::<Vec<PeerId>>()
        .to_owned();
    socket.send_unreliable(
        AddressedEvent {
            src: room.player1,
            event: Event::SyncRoom(*room),
        },
        peers,
    );
}

pub fn receive_events(
    mut commands: Commands,
    _lobby: ResMut<Lobby>,
    mut room: ResMut<Room>,
    mut socket: ResMut<Socket>,
    query: Query<Entity, With<RoomComponent>>,
    assets: Res<MyAssets>,
) {
    let binding = socket.receive_unreliable();
    let events = Vec::from_iter(
        binding.iter(), // .filter(|e| e.src != lobby.socket.id().unwrap()),
    );
    for AddressedEvent { src, event } in events {
        match event {
            Event::JoinRoom => {
                if room.player2.is_none() || room.player3.is_none() {
                    room.join(*src);
                    let s = socket.unreliable_id().unwrap();
                    socket.send_unreliable(
                        AddressedEvent {
                            src: s,
                            event: Event::JoinRoomSuccess(*room),
                        },
                        vec![*src],
                    );
                    if room.player2.is_some() {
                        // commands.spawn(TextBundle {
                        //     text: Text::from_section(
                        //         "player 2",
                        //         TextStyle {
                        //             font: assets.font.clone(),
                        //             font_size: 24.0,
                        //             color: Color::GOLD,
                        //             ..Default::default()
                        //         },
                        //     )
                        //     .with_alignment(TextAlignment::Center),
                        //     style: Style {
                        //         margin: UiRect::all(Val::Px(10.0)),
                        //         position_type: PositionType::Absolute,
                        //         top: Val::Px(70.),
                        //         left: Val::Px(70.),
                        //         ..Default::default()
                        //     },
                        //     ..default()
                        // });
                        // commands.spawn(ImageBundle {
                        //     image: assets.room_touxiang.clone().into(),
                        //     style: Style {
                        //         // width: Val::Px(70.),
                        //         // height: Val::Px(50.),
                        //         margin: UiRect::all(Val::Px(10.0)),
                        //         position_type: PositionType::Absolute,
                        //         top: Val::Px(50.),
                        //         left: Val::Px(50.),
                        //         ..Default::default()
                        //     },
                        //     ..Default::default()
                        // });
                    }
                    if room.player3.is_some() {
                        commands.spawn(TextBundle {
                            text: Text::from_section(
                                "player 3",
                                TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 24.0,
                                    color: Color::GOLD,
                                    ..Default::default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                position_type: PositionType::Absolute,
                                top: Val::Px(70.),
                                left: Val::Px(70.),
                                ..Default::default()
                            },
                            ..default()
                        });
                    }
                }
            }
            Event::Test(_) => todo!(),
            _ => {}
        }
    }
}
