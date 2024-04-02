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
    let color_vec4 = Vec4::new(0.3, 0.7, 0.5, 1.0);
    let color_from_vec4 = Color::rgba_from_array(color_vec4);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(color_from_vec4),
        ..Default::default()
    });
}
