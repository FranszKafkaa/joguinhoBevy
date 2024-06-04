#![allow(unused)] // <- vai ser removido depois

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3, Vec3A},
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    time::{self, Time, Timer, TimerMode},
    transform::components::Transform,
};
use bevy_rapier2d::{
    dynamics::{GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider},
};

use crate::animator::{animate_sprite, AnimationIndices, AnimationTimer};

#[derive(Component)]
pub struct Enemy {
    life: i32,
    dmg: i32,
}
#[derive(Component)]
struct EnemyMoviment(Timer);

impl Default for Enemy {
    fn default() -> Self {
        Self { life: 100, dmg: 20 }
    }
}

impl Enemy {
    pub fn get_dmg(&self) -> i32 {
        self.dmg
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_enemy)
            .add_systems(Update, (animate_sprite, enemy_moviment));
    }
}

fn setup_enemy(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 30.0), 4, 3, None, None);
    let texture_atlas_layout = texture_layout.add(layout);
    let animaion_indices = AnimationIndices { first: 1, last: 3 };

    command.spawn((
        SpriteSheetBundle {
            texture: asset_server.load("sprites/slime_green.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animaion_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(7.0, 9.0),
        ActiveEvents::COLLISION_EVENTS,
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        EnemyMoviment(Timer::from_seconds(0.1, TimerMode::Repeating)),
        GravityScale(5.0),
        Velocity::zero(),
        Enemy::default(),
        animaion_indices,
    ));
}

fn enemy_moviment(mut enemy_query: Query<(&mut Velocity, &mut EnemyMoviment)>, timer: Res<Time>) {
    for (mut velocity, mut moviment) in &mut enemy_query {
        moviment.0.tick(timer.delta());

        if moviment.0.just_finished() {
            velocity.linvel.x = -200.0;
        }
    }
}
