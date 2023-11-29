use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{ggrs::DesyncDetection, prelude::*};
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{AppState, MyAssets};

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
struct CollabId(u16);
struct Peer {
    chalk: Entity,
    cursor: Entity,
}
struct Peers(HashMap<CollabId, Peer>);
#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Room {
    pub local_player: PeerId,
    pub player1: Option<PeerId>,
    pub player2: Option<PeerId>,
}

pub struct Rooms(Vec<Vec3>);

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
                ..Default::default()
            },
            // Room,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.table_bg_1.clone().into(),
                style: Style {
                    ..Default::default()
                },
                ..default()
            });
        });
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if socket.get_channel(0).is_err() {
        return;
    }
    socket.update_peers();
    let players = socket.players();
    let num_players = 2;
    if players.len() < num_players {
        // 等待更多玩家
        return;
    }
    let mut session_builder = SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_desync_detection_mode(DesyncDetection::On { interval: 1 });
    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }
    let socket = socket.take_channel(0).unwrap();
    let ggrs_session = session_builder
        .start_p2p_session(socket)
        .expect("failed to start session");
    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    println!("playing...");
    next_state.set(AppState::Playing);
}
