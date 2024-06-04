use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::{
        component::Component, schedule::IntoSystemConfigs, system::{Commands, Query, Res, ResMut}
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    reflect::Reflect,
    sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    text::TextStyle,
    time::{Timer, TimerMode},
    transform::components::Transform,
    ui::node_bundles::TextBundle,
};
use bevy_rapier2d::{
    dynamics::{GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, Friction, Restitution},
};

use crate::animator::{animate_sprite, AnimationIndices, AnimationTimer};

#[derive(Component, Reflect)]
pub struct Player {
    direction: Direction,
    action: Action,
    jump_force: f32,
    life: u32,
    pub jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: Direction::None,
            action: Action::Idle,
            jump_force: 1000.0,
            life: 100,
            jumping: false,
        }
    }
}

#[derive(Reflect, PartialEq, Eq, Debug)]
#[reflect(PartialEq)]
enum Direction {
    None,
    Left,
    Right,
    Up,
}

#[derive(Reflect, PartialEq, Eq, Debug)]
#[reflect(PartialEq)]
enum Action {
    Idle,
    Walking,
    Jumping,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Player>()
            .add_systems(Startup, (setup_player, player_hud).chain())
            .add_systems(Update, (animate_sprite, player_moviment, player_input));
    }
}

fn setup_player(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 30.0), 4, 1, None, None);
    let texture_atlas_layout = texture_layout.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 3 };

    command.spawn((
        SpriteSheetBundle {
            texture: asset_server.load("sprites/idle.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3::new(-50.0, 0.0, 1.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Player::default(),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        Friction::coefficient(0.5),
        Velocity::zero(),
        GravityScale(5.0),
        Restitution::coefficient(0.1),
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(7.0, 9.0),
    ));
}

fn player_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
    let mut player = query.single_mut();

    if input.pressed(KeyCode::ArrowLeft) {
        player.direction = Direction::Left;
        player.action = Action::Walking;
    } else if input.pressed(KeyCode::ArrowRight) {
        player.direction = Direction::Right;
        player.action = Action::Walking;
    } else {
        player.direction = Direction::None;
        player.action = Action::Idle;
    }

    if input.just_pressed(KeyCode::Space) && !player.jumping {
        player.direction = Direction::Up;
        player.action = Action::Jumping;
        player.jumping = true;
    }
}

fn player_moviment(mut query: Query<(&mut Transform, &mut Velocity, &mut Player, &mut Sprite)>) {
    for (mut transform, mut velocity, mut player, mut sprite) in query.iter_mut() {
        match player.direction {
            Direction::Left => {
                transform.translation.x -= 6.0;
                sprite.flip_x = true;
                player.action = Action::Walking;
            }
            Direction::Right => {
                transform.translation.x += 6.0;
                sprite.flip_x = false;
                player.action = Action::Walking;
            }
            Direction::Up => {
                if player.action == Action::Jumping {
                    velocity.linvel.y = player.jump_force;
                }
            }
            Direction::None => {
                if player.action == Action::Jumping && velocity.linvel.y == 0.0 {
                    player.jumping = false;
                }
                player.action = Action::Idle;
            }
        }

        // Ensure the player can jump while moving
        if player.action == Action::Jumping {
            velocity.linvel.y = player.jump_force;
        }
    }
}

fn player_hud(mut command: Commands, player_query: Query<&Player>) {
    let player = player_query.single();

    command.spawn(TextBundle::from_section(
        format!("HP: {:?}", player.life),
        TextStyle {
            font_size: 100.0,
            ..Default::default()
        },
    ));
}
