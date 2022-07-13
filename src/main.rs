use bevy::{prelude::*, render::camera::Camera3d};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Nice cock".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 10. })),
        material: materials.add(Color::rgb(1., 0., 0.).into()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2300.,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(14., 15., 150.),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        camera: Camera::default(),
        transform: Transform::from_xyz(120., 120., 10.),
        ..Default::default()
    });
}
