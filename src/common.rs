use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::{matchbox_socket::WebRtcSocket, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{player::Player, room::Room};

pub enum CardIndex {
    方片9 = 0,
    方片8,
    方片7,
    方片6,
    小王 = 5,
    大王,
    黑桃K,
    黑桃Q,
    黑桃J,
    方片5,
    黑桃10,
    黑桃9,
    黑桃8,
    黑桃7,
    黑桃6,
    黑桃5,
    黑桃4,
    黑桃3,
    黑桃2,
    黑桃A,
    方片4,
    红桃K,
    红桃Q,
    红桃J,
    红桃10,
    红桃9,
    红桃8,
    红桃7,
    红桃6,
    红桃5,
    红桃4,
    方片3,
    红桃3,
    红桃2,
    红桃A,
    梅花K,
    梅花Q,
    梅花J,
    梅花10,
    梅花9,
    梅花8,
    梅花7,
    方片2,
    梅花6,
    梅花5,
    梅花4,
    梅花3,
    梅花2,
    梅花A,
    方片K,
    方片Q,
    方片J,
    方片10,
    方片A,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    StartMenu,
    Lobby,
    InRoom,
    DealCard,
    Playing,
    Paused,
    GameOver,
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "embedded://sounds/bg.mp3")]
    pub game_bg: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/duizi.mp3")]
    pub duizi: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/fapai.mp3")]
    pub fapai: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/login_bg.ogg")]
    pub login_bg: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/man_san_dai_yi_dui.ogg")]
    pub man_san_dai_yi: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/start_a.ogg")]
    pub start: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/woman_bu_jiao.ogg")]
    pub woman_bu_jiao: Handle<AudioSource>,
    #[asset(path = "embedded://sounds/woman_jiao_di_zhu.ogg")]
    pub woman_jiao_di_zhu: Handle<AudioSource>,

    #[asset(texture_atlas(
        tile_size_x = 116.,
        tile_size_y = 159.,
        columns = 13,
        rows = 5,
        padding_x = 2.,
        padding_y = 2.,
        offset_x = 0.,
        offset_y = 0.
    ))]
    #[asset(path = "embedded://image/card/card.png")]
    pub card: Handle<TextureAtlas>,

    #[asset(path = "embedded://image/youqing_girl.png")]
    pub you_qing_girl: Handle<Image>,
    #[asset(path = "embedded://image/youqing_boy.png")]
    pub you_qing_boy: Handle<Image>,
    #[asset(path = "embedded://image/tip.png")]
    pub tip: Handle<Image>,
    #[asset(path = "embedded://bg_login.jpg")]
    pub bg_login: Handle<Image>,
    #[asset(path = "embedded://loading_bg.png")]
    pub loading_bg: Handle<Image>,
    #[asset(path = "embedded://image/btn_enter_room.png")]
    pub btn_enter_room: Handle<Image>,
    #[asset(path = "embedded://image/button/btn_weixin.png")]
    pub btn_weixin: Handle<Image>,
    #[asset(path = "embedded://image/button/btn_ traveler.png")]
    pub btn_traveler: Handle<Image>,
    #[asset(path = "embedded://image/yonghuxieyi.png")]
    pub yonghuxieyi: Handle<Image>,
    #[asset(path = "embedded://image/check_mark.png")]
    pub check_mark: Handle<Image>,
    #[asset(path = "embedded://image/btn_create_room.png")]
    pub btn_create_room: Handle<Image>,
    #[asset(path = "embedded://table_bg_1.jpg")]
    pub table_bg_1: Handle<Image>,
    #[asset(path = "embedded://image/room_touxiang.png")]
    pub room_touxiang: Handle<Image>,
    #[asset(path = "embedded://font/FZKTJW.ttf")]
    pub font: Handle<Font>,
}

#[derive(Component)]
pub enum MenuButton {
    Traveler,
    Weixin,
}

#[derive(Debug, Resource)]
pub struct GameSounds {
    pub game_bg: Handle<AudioSource>,
    pub duizi: Handle<AudioSource>,
    pub fapai: Handle<AudioSource>,
    pub login_bg: Handle<AudioSource>,
    pub man_san_dai_yi: Handle<AudioSource>,
    pub start: Handle<AudioSource>,
    pub woman_bu_jiao: Handle<AudioSource>,
    pub woman_jiao_di_zhu: Handle<AudioSource>,
}

#[derive(Debug, Resource)]
pub struct GameTextureAtlasHandles {
    pub card: Handle<TextureAtlas>,
    pub you_qing: Handle<TextureAtlas>,
}

#[derive(Debug, Resource, Clone)]
pub struct PokerSocket(pub Arc<RwLock<WebRtcSocket>>);
impl ggrs::NonBlockingSocket<PeerId> for PokerSocket {
    fn send_to(&mut self, msg: &ggrs::Message, addr: &PeerId) {
        self.0
            .write()
            // if the lock is poisoned, we're already doomed, time to panic
            .expect("failed to lock socket for reading")
            .send_to(msg, addr);
    }

    fn receive_all_messages(&mut self) -> Vec<(PeerId, ggrs::Message)> {
        self.0
            .write()
            // if the lock is poisoned, we're already doomed, time to panic
            .expect("failed to lock socket for receiving")
            .receive_all_messages()
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum PokerMessage {
//     Chat { handle: GgrsHandle, message: String },
// }
pub fn menu_button_press_system(
    mut commands: Commands,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Traveler => {
                    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
                    // let (socket, message_loop) = WebRtcSocket::builder(room_url)
                    //     .add_unreliable_channel()
                    //     .add_reliable_channel()
                    //     .build();
                    info!("connecting to matchbox server: {room_url}");
                    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
                    state.set(AppState::Lobby);
                }
                MenuButton::Weixin => {
                    // println!("weixin");
                }
            }
        }
    }
}

#[derive(Debug, Resource)]
pub struct Cards {}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Event {
    SyncRoom(Room),
    JoinRoom,
    JoinRoomSuccess(Room),
    Test(i32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddressedEvent {
    pub(crate) src: Player,
    pub(crate) event: Event,
}

#[derive(Resource)]
pub struct Socket {
    unreliable: MatchboxSocket<SingleChannel>,
}

impl Socket {
    pub fn new(unreliable: MatchboxSocket<SingleChannel>) -> Self {
        Self { unreliable }
    }

    pub fn receive_unreliable(&mut self) -> Vec<AddressedEvent> {
        self.unreliable
            .receive()
            .iter()
            .map(|(_, payload)| payload)
            .filter_map(|payload| ciborium::de::from_reader(&payload[..]).ok())
            .collect()
    }

    pub fn send_unreliable(&mut self, event: AddressedEvent, peers: Vec<PeerId>) {
        let mut payload = Vec::new();
        ciborium::ser::into_writer(&event, &mut payload).unwrap();
        for peer in peers {
            self.unreliable.send(payload.clone().into(), peer);
        }
    }

    pub fn unreliable_id(&mut self) -> Option<PeerId> {
        self.unreliable.id()
    }

    pub fn unreliable_connected_peers(&mut self) -> impl std::iter::Iterator<Item = PeerId> + '_ {
        self.update_peers_unreliable();
        self.unreliable.connected_peers()
    }

    pub fn update_peers_unreliable(&mut self) -> Vec<(PeerId, PeerState)> {
        self.unreliable.update_peers()
    }
}
