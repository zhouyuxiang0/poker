use crate::{
    card::{get_sprite_index, new_deck},
    common::{AddressedEvent, AppState, Event, MyAssets, Socket},
    lobby::Lobby,
    player::Player,
};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

const PLAYER_POSITION: [[f32; 2]; 3] = [[85., 5.], [20., 5.], [20., 85.]];
const BOTTOM_CARD_POSITION: [[f32; 2]; 1] = [[20., 20.]];
const LEFT_CARD_POSITION: [[f32; 2]; 1] = [[0., 0.]];
const RIGHT_CARD_POSITION: [[f32; 2]; 1] = [[0., 0.]];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomPlayer {
    pub player: Player,
    pub room_position: i8,
}

// 客户端房间资源
#[derive(Resource, Serialize, Deserialize, Clone, Debug, Component)]
pub struct Room {
    pub players: [Option<RoomPlayer>; 3],
    pub owner: RoomPlayer,
    // 房间数据是否有变化
    pub changed: bool,
}

#[derive(Component)]
pub struct RoomUIComponent;

// #[derive(Component)]
// pub struct DealCardTimer(pub Timer);

impl Room {
    pub fn new(player: Player) -> Self {
        let rome_player = RoomPlayer {
            player,
            room_position: 0,
        };
        Self {
            players: [Some(rome_player.clone()), None, None],
            owner: rome_player,
            changed: true,
        }
    }

    pub fn join(&mut self, player: Player) -> bool {
        // 找到空位 并分配空位
        if let Some(index) = self.players.iter().position(|v| v.is_none()) {
            self.players[index] = Some(RoomPlayer {
                player,
                room_position: index as i8,
            });
            true
        } else {
            false
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.owner.player == other.owner.player
    }
}

impl Eq for Room {}

// const PLAYER_LOCATION: Vec = [];

impl Plugin for RoomUIComponent {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InRoom), setup)
            .add_systems(
                Update,
                (update, receive_events).run_if(in_state(AppState::InRoom)),
            )
            .add_systems(OnEnter(AppState::InRoom), init_card)
            .add_systems(Update, deal_card.run_if(in_state(AppState::DealCard)));
        // .add_systems(OnExit(AppState::Playing), despawn_screen::<RoomUIComponent>);
    }
}

fn init_card(
    mut commands: Commands,
    assets: Res<MyAssets>,
    room: Res<Room>,
    mut state: ResMut<NextState<AppState>>,
) {
    let deck = new_deck();
    for card in deck {
        let sprite_index = get_sprite_index(&card);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: assets.card.to_owned(),
                transform: Transform::from_xyz(0., 0.0, 0.0),
                sprite: TextureAtlasSprite::new(sprite_index),
                ..Default::default()
            })
            .insert(card);
    }
}

fn deal_card(
    mut commands: Commands,
    // mut deck: Vec<Card>,
    // card_textures: Res<CardTextureAtlas>,
    // mut players: ResMut<Vec<Player>>,
) {
    // println!("{:?}", time.delta_seconds());
    // *card_deal_timer += time.delta_seconds();
    // let mut index = 1.;
    // for (mut transfrom, card) in &mut q_card {
    //     if *card_deal_timer > 0.1 {
    //         transfrom.translation.x = transfrom.translation.x + 80. * time.delta_seconds();
    //         transfrom.translation.y = transfrom.translation.y - 80. * time.delta_seconds();
    //         index = index + 1.;
    //         // println!("{:?}", card);
    //         *card_deal_timer = 0.0;
    //     }
    // }
}

pub fn setup(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.table_bg_1.clone(),
            transform: Transform {
                scale: Vec3::new(1.2, 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        RoomUIComponent,
    ));
}

fn update(
    mut commands: Commands,
    local: Res<Player>,
    assets: Res<MyAssets>,
    mut room: ResMut<Room>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
) {
    // 实时发布房间信息
    if room.changed {
        let peers = socket
            .unreliable_connected_peers()
            .collect::<Vec<PeerId>>()
            .to_owned();
        socket.send_unreliable(
            AddressedEvent {
                src: local.clone(),
                event: Event::SyncRoom(room.clone()),
            },
            peers,
        );
    }
    // 更新房间房主 默认数组第一个
    if let Some(first) = &room.players[0] {
        room.owner = first.to_owned();
    }
    // 更新房间玩家 根据room_position 计算出用户的位置
    if let Some(local_room_player) = room.players.iter().find(|x| {
        if let Some(p) = x.to_owned() {
            p.player == *local
        } else {
            false
        }
    }) {
        // local_room_player.
    }
    // let local_index = room
    //     .players
    //     .iter()
    //     .position(|p| p.is_some() && p.clone().unwrap() == *local);
    // if let Some(index) = local_index {
    //     let sort_players = match index {
    //         0 => [
    //             room.players[0].clone(),
    //             room.players[1].clone(),
    //             room.players[2].clone(),
    //         ],
    //         1 => [
    //             room.players[1].clone(),
    //             room.players[2].clone(),
    //             room.players[0].clone(),
    //         ],
    //         2 => [
    //             room.players[2].clone(),
    //             room.players[0].clone(),
    //             room.players[1].clone(),
    //         ],
    //         _ => unreachable!(),
    //     };
    //     for index in 0..sort_players.len() {
    //         if let Some(player) = sort_players[index].clone() {
    //             commands
    //                 .spawn(TextBundle {
    //                     text: Text::from_section(
    //                         player.id.to_string().get(0..7).unwrap_or("player"),
    //                         TextStyle {
    //                             font: assets.font.clone(),
    //                             font_size: 24.0,
    //                             color: Color::GOLD,
    //                             ..Default::default()
    //                         },
    //                     )
    //                     .with_alignment(TextAlignment::Center),
    //                     style: Style {
    //                         position_type: PositionType::Absolute,
    //                         top: Val::Percent(PLAYER_POSITION[index][0]),
    //                         left: Val::Percent(PLAYER_POSITION[index][1]),
    //                         ..Default::default()
    //                     },
    //                     ..default()
    //                 })
    //                 .with_children(|builder| {
    //                     builder.spawn(ImageBundle {
    //                         image: assets.room_touxiang.clone().into(),
    //                         style: Style {
    //                             position_type: PositionType::Relative,
    //                             top: Val::Px(-85.),
    //                             left: Val::Px(-5.),
    //                             ..Default::default()
    //                         },
    //                         ..Default::default()
    //                     });
    //                 });
    //         }
    //     }
    //     if room.players.iter().filter(|p| p.is_some()).count() >= 3 {
    //         println!("playing................");
    //         state.set(AppState::Playing);
    //     }
    // }
    room.changed = false;
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
                if room.join(src.clone()) {
                    println!("{:?}", room.players);
                    let mut peers = room
                        .players
                        .iter()
                        .filter(|p| {
                            p.is_some()
                                && <std::option::Option<RoomPlayer> as Clone>::clone(&p)
                                    .unwrap()
                                    .player
                                    .id
                                    != local.id
                        })
                        .map(|p| p.clone().unwrap().player.id)
                        .collect::<Vec<PeerId>>();
                    peers.push(src.id);
                    socket.send_unreliable(
                        AddressedEvent {
                            src: local.clone(),
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
