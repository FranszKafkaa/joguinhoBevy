mod player;

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowTheme},
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "game".to_string(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: EnabledButtons {
                            close: true,
                            minimize: false,
                            maximize: false,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (spawn_camera, world_gravity))
        .add_systems(Update, collision_event)
        .run();
}

fn spawn_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}

//isso será removido no futuro
fn world_gravity(mut commands: Commands) {
    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.3, 0.3, 1.3),
                custom_size: Some(Vec2::new(500.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(250.0, 10.0),
        Ground,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 70.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(20.0, -50.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(25.0, 35.0),
    ));
}

fn collision_event(
    mut collision_event: EventReader<CollisionEvent>,
    mut player_query: Query<&mut player::Player>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let ground_entity = ground_query.single();

    for event in collision_event.read() {
        match event {
            CollisionEvent::Started(col1, col2, _) => {
                if *col1 == ground_entity {
                    if let Ok(mut player) = player_query.get_mut(*col2) {
                        player.jumping = false;
                    }
                } else if *col2 == ground_entity {
                    if let Ok(mut player) = player_query.get_mut(*col1) {
                        player.jumping = false;
                    }
                }
            }
            CollisionEvent::Stopped(_col1, _col2, _) => {}
        }
    }
}
