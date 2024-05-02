use bevy::prelude::*;

pub struct PlanePlugin;
impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_plane);
    }
}

fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let color_vec4 = Vec4::new(0.263, 0.235, 0.208, 1.0);
    let color_vec4 = Vec4::new(0.203, 0.295, 0.208, 1.0);
    let color_from_vec4 = Color::rgba_from_array(color_vec4);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(color_from_vec4),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
