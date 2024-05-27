use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player {
    direction: Direction,
    action: Action,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: Direction::None,
            action: Action::Idle,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

enum Direction {
    None,
    Left,
    Right,
}

enum Action {
    Idle,
    Walking,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (animate_sprite, player_moviment, player_input));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
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
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        },
        Player::default(),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        RigidBody::Dynamic,
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
}

fn player_moviment(mut query: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, player) in query.iter_mut() {
        match player.direction {
            Direction::Left => transform.translation.x -= 6.0,
            Direction::None => (),
            Direction::Right => transform.translation.x += 6.0,
        };

        println!("player position: {:?}", transform.translation);
    }
}
