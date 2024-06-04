use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    direction: Direction,
    action: Action,
    jump_force: f32,
    pub jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: Direction::None,
            action: Action::Idle,
            jump_force: 1000.0,
            jumping: false,
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
            .add_systems(Startup, setup_player)
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
            transform: Transform {
                translation: Vec3::new(-50.0, 0.0, 1.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Player::default(),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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

fn player_moviment(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
) {
    let mut sprite = sprite_query.single_mut();
    for (mut transform, mut velocity, mut player) in query.iter_mut() {
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
