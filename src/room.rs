use bevy::{audio::PlaybackMode, prelude::*, transform};
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    common::{despawn_screen, AddressedEvent, AppState, Event, MyAssets, Socket},
    lobby::{Lobby, LobbyComponent},
    player::{self, Player},
};

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

const PLAYER_POSITION: [[f32; 2]; 3] = [[85., 5.], [20., 5.], [20., 85.]];

// 客户端房间资源
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    // pub id: PeerId,
    pub players: [Option<Player>; 3],
    pub owner: Player,
}

#[derive(Component)]
pub struct RoomUIComponent;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct RoomComponent {
    pub id: PeerId,
    pub players: Vec<Player>,
}

#[derive(Component)]
pub struct PlayerComponent {
    pub peer: PeerId,
}

impl Room {
    pub fn new(player: Player) -> Self {
        Self {
            players: [Some(player), None, None],
            owner: player,
        }
    }

    pub fn join(&mut self, player: Player) -> bool {
        if let Some(index) = self.players.iter().position(|v| v.is_none()) {
            self.players[index] = Some(player);
            true
        } else {
            false
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
    }
}

impl Eq for Room {}

#[derive(Resource)]
pub struct Rooms(Vec<Room>);

// const PLAYER_LOCATION: Vec = [];

impl Plugin for RoomUIComponent {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InRoom), setup_room)
            .add_systems(
                Update,
                (setup_player, publish_room, receive_events).run_if(in_state(AppState::InRoom)),
            )
            .add_systems(OnEnter(AppState::Lobby), init_card);
        // .add_systems(OnExit(AppState::Playing), despawn_screen::<RoomUIComponent>);
    }
}

fn init_card(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut q: Query<&Transform, With<LobbyComponent>>,
) {
    println!("------------");
    for transform in &mut q {
        println!("{:?}", transform);
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 51,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(0.65, 0.65, 0.65),
                ..Default::default()
            },
            texture_atlas: assets.card.clone(),
            ..Default::default()
        });
    }
}

pub fn setup_room(mut commands: Commands, assets: Res<MyAssets>) {
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
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            },
            RoomUIComponent,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.table_bg_1.clone().into(),
                style: Style {
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            });
        });
}

fn setup_player(
    mut commands: Commands,
    local: Res<Player>,
    assets: Res<MyAssets>,
    room: ResMut<Room>,
    mut state: ResMut<NextState<AppState>>,
) {
    let local_index = room
        .players
        .iter()
        .position(|&p| p.is_some() && p.unwrap() == *local);
    if let Some(index) = local_index {
        let sort_players = match index {
            0 => [room.players[0], room.players[1], room.players[2]],
            1 => [room.players[1], room.players[2], room.players[0]],
            2 => [room.players[2], room.players[0], room.players[1]],
            _ => unreachable!(),
        };
        for index in 0..sort_players.len() {
            if let Some(player) = sort_players[index] {
                commands
                    .spawn(TextBundle {
                        text: Text::from_section(
                            player.id.to_string().get(0..7).unwrap_or("player"),
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 24.0,
                                color: Color::GOLD,
                                ..Default::default()
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(PLAYER_POSITION[index][0]),
                            left: Val::Percent(PLAYER_POSITION[index][1]),
                            ..Default::default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(ImageBundle {
                            image: assets.room_touxiang.clone().into(),
                            style: Style {
                                position_type: PositionType::Relative,
                                top: Val::Px(-85.),
                                left: Val::Px(-5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
        }
        if room.players.iter().filter(|p| p.is_some()).count() >= 3 {
            println!("playing................");
            state.set(AppState::Playing);
        }
    }
}

pub fn publish_room(
    _lobby: ResMut<Lobby>,
    room: ResMut<Room>,
    mut socket: ResMut<Socket>,
    local: Res<Player>,
) {
    let peers = socket
        .unreliable_connected_peers()
        .collect::<Vec<PeerId>>()
        .to_owned();
    socket.send_unreliable(
        AddressedEvent {
            src: *local,
            event: Event::SyncRoom(room.clone()),
        },
        peers,
    );
}

pub fn receive_events(
    _lobby: ResMut<Lobby>,
    mut room: ResMut<Room>,
    mut socket: ResMut<Socket>,
    local: Res<Player>,
) {
    let binding = socket.receive_unreliable();
    let events = Vec::from_iter(
        binding.iter(), // .filter(|e| e.src != lobby.socket.id().unwrap()),
    );
    for AddressedEvent { src, event } in events {
        match event {
            Event::JoinRoom => {
                if room.join(*src) {
                    println!("{:?}", room.players);
                    let mut peers = room
                        .players
                        .iter()
                        .filter(|p| p.is_some() && p.unwrap().id != local.id)
                        .map(|p| p.unwrap().id)
                        .collect::<Vec<PeerId>>();
                    peers.push(src.id);
                    socket.send_unreliable(
                        AddressedEvent {
                            src: *local,
                            event: Event::JoinRoomSuccess(room.clone()),
                        },
                        peers,
                    );
                }
            }
            Event::Test(_) => todo!(),
            _ => {}
        }
    }
}
