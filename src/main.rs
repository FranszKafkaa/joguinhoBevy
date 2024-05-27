mod player;

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowTheme},
};

use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;

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
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -900.8),
            force_update_from_transform_changes: true,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 0,
            timestep_mode: TimestepMode::Variable {
                max_dt: 0.2,
                time_scale: 1.0,
                substeps: 1,
            },
        })
        .add_systems(Startup, (spawn_camera, world_gravity))
        .run();
}

fn spawn_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}

//isso será removido no futuro
fn world_gravity(mut commands: Commands) {
    // Ground
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(500.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(250.0, 10.0));
}
