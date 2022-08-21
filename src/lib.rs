use bevy::{
    input::{Input, keyboard::KeyCode},
    prelude::*,
};
use wasm_bindgen::prelude::*;

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn app() -> App {

    console_log!("Bevy Game in WASM!");

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement);

    app
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Player {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(PlayerBundle {
            player: Player {},
            sprite: SpriteBundle {
                texture: asset_server.load("bevy.png"),
                transform: Transform::from_xyz(100., 0., 0.),
                ..default()
            },
        });
        // .insert(Direction::Up);
}

const SPEED: f32 = 200.;

fn movement(
    keys: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>,
) {
    // for (mut _player, mut transform) in query.iter_mut() {

    let (_player, mut transform) = query.single_mut();

        let up: f32 = if keys.pressed(KeyCode::W) { 1. } else { 0. };
        let down: f32 = if keys.pressed(KeyCode::S) { 1. } else { 0. };
        let left: f32 = if keys.pressed(KeyCode::A) { 1. } else { 0. };
        let right: f32 = if keys.pressed(KeyCode::D) { 1. } else { 0. };

        transform.translation.x += (right - left) * SPEED * time.delta_seconds();
        transform.translation.y += (up - down) * SPEED * time.delta_seconds();
    // }


    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::W) {}
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
