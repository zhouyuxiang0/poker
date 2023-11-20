use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum AppState {
    #[default]
    StartMenu,
    Lobby,
    Playing,
    Paused,
    GameOver,
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

pub fn setup_game_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        game_bg: asset_server.load("sounds/bg.mp3"),
        duizi: asset_server.load("sounds/duizi.mp3"),
        fapai: asset_server.load("sounds/fapai.mp3"),
        login_bg: asset_server.load("sounds/login_bg.ogg"),
        man_san_dai_yi: asset_server.load("sounds/man_san_dai_yi_dui.ogg"),
        start: asset_server.load("sounds/start_a.ogg"),
        woman_bu_jiao: asset_server.load("sounds/woman_bu_jiao.ogg"),
        woman_jiao_di_zhu: asset_server.load("sounds/woman_jiao_di_zhu.ogg"),
    });
}

#[derive(Debug, Resource)]
pub struct GameTextureAtlasHandles {
    pub card: Handle<TextureAtlas>,
    pub you_qing: Handle<TextureAtlas>,
}

pub fn setup_game_texture_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(GameTextureAtlasHandles {
        card: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("image/card/card.png"),
            Vec2::new(2000.0, 4000.0),
            13,
            5,
            None,
            None,
        )),
        you_qing: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("image/youqingTip.png"),
            Vec2::new(402.0, 600.0),
            1,
            3,
            Some(Vec2::new(0.0, 0.0)),
            Some(Vec2::new(0.0, 0.0)),
        )),
    });
}

pub fn menu_button_press_system(
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Traveler => {
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
