#![feature(clamp)]

use bevy::prelude::*;

struct Position {
    x: i32,
    y: i32,
}

struct Player {}

fn spawn_sprite(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    position: Position,
) -> &mut Commands {
    commands
        .spawn(SpriteComponents {
            material,
            ..Default::default()
        })
        .with(position)
}

fn sprite_update_system(position: &Position, mut transform: Mut<Transform>) {
    *transform = Transform::from_translation(Vec3::new(
        (position.x as f32 - 4.5) * 64.0,
        (position.y as f32 - 4.0) * 64.0,
        0.0,
    ));
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let block = materials.add(asset_server.load("block.png").unwrap().into());
    let player = materials.add(asset_server.load("a2_wide.png").unwrap().into());

    commands.spawn(Camera2dComponents::default());

    for i in 0..10 {
        spawn_sprite(&mut commands, block, Position { x: i, y: 0 });
        spawn_sprite(&mut commands, block, Position { x: i, y: 8 });
    }

    for i in 0..7 {
        spawn_sprite(&mut commands, block, Position { x: 0, y: 1 + i });
        spawn_sprite(&mut commands, block, Position { x: 9, y: 1 + i });
    }

    spawn_sprite(&mut commands, player, Position { x: 1, y: 1 }).with(Player {});
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    _player: &Player,
    mut position: Mut<Position>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        position.y += 1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        position.x -= 1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        position.y -= 1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        position.x += 1;
    }

    position.x = position.x.clamp(1, 10 - 2);
    position.y = position.y.clamp(1, 9 - 2);
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: (64 * 10) + (2 * 64),
            height: (64 * 9) + (2 * 64),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.792, 0.863, 0.624)))
        .add_startup_system(setup.system())
        .add_system(sprite_update_system.system())
        .add_system(player_movement_system.system())
        .run();
}
