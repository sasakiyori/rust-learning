use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, pbr::CascadeShadowConfigBuilder, prelude::*,
    render::camera::Viewport, window::WindowResized,
};

fn main() {
    App::new()
        .init_resource::<CanMove>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(set_camera_viewports)
        .add_system(block_check_if_can_move)
        .add_system(block_move)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("Fox.glb#Scene0"),
        ..default()
    });

    commands.spawn((
        // cube
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Block,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 2,
            first_cascade_far_bound: 200.0,
            maximum_distance: 280.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Left Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 200.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        LeftCamera,
    ));

    // Right Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(100.0, 100., 150.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RightCamera,
    ));
}

#[derive(Component)]
struct LeftCamera;

#[derive(Component)]
struct RightCamera;

#[derive(Component)]
struct Block;

#[derive(Resource, Default)]
struct CanMove(bool);

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        let window = windows.get(resize_event.window).unwrap();
        let mut left_camera = left_camera.single_mut();
        left_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });

        let mut right_camera = right_camera.single_mut();
        right_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });
    }
}

fn block_check_if_can_move(keyboard_input: Res<Input<KeyCode>>, mut can_move: ResMut<CanMove>) {
    if keyboard_input.pressed(KeyCode::M) {
        can_move.0 = true;
    }
}

fn block_move(mut query: Query<&mut Transform, With<Block>>, can_move: Res<CanMove>) {
    if can_move.0 {
        for mut transform in query.iter_mut() {
            transform.translation.y += 0.1;
        }
    }
}
