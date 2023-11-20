use bevy::prelude::*;

mod common;
mod lobby;
mod player;
mod ui;

use lobby::setup_lobby;
use ui::{despawn_screen, setup_start_menu, OnStartMenuScreen};

use common::{menu_button_press_system, setup_game_sounds, setup_game_texture_atlas, AppState};
use player::PlayerLives;

const BACKGROUND_COLOR: Color = Color::BLACK;
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(PlayerLives {
            play1: 3,
            play2: 3,
            play3: 3,
        })
        .add_systems(
            Startup,
            (setup_camera, setup_game_sounds, setup_game_texture_atlas),
        )
        .add_systems(OnEnter(AppState::StartMenu), (setup_start_menu))
        .add_systems(
            Update,
            (menu_button_press_system).run_if(in_state(AppState::StartMenu)),
        )
        .add_systems(
            OnExit(AppState::StartMenu),
            (despawn_screen::<OnStartMenuScreen>,),
        )
        .add_systems(OnEnter(AppState::Lobby), setup_lobby)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
