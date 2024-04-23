use crate::comm_channels::messages_from_bevy::CameraPosition;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        // .add_systems(
        //     OnEnter(AssetLoaderState::Done),
        //     set_up_camera_position_text_nodes,
        // )
        // .add_systems(Update, update_camera_position_text);
    }
}

fn spawn_camera(mut commands: Commands) {
    let entity_commands = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            camera: Camera {
                is_active: true,
                ..Default::default()
            },
            ..Default::default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.66, 0.43, 0.07),
            radius: Some(9.44),
            alpha: Some(2.86),
            beta: Some(0.52),
            ..Default::default()
        },
    ));
}

// #[derive(Component)]
// pub struct ChangeableText;

// pub fn set_up_camera_position_text_nodes(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let font = asset_server.load("FiraSans-Regular.ttf");
//     commands.spawn(Camera2dBundle {
//         camera: Camera {
//             order: 1,
//             ..Default::default()
//         },
//         ..Default::default()
//     });

//     let root_uinode = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 justify_content: JustifyContent::SpaceBetween,

//                 ..default()
//             },
//             ..default()
//         })
//         .id();

//     let right_column = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::SpaceBetween,
//                 align_items: AlignItems::End,
//                 flex_grow: 1.,
//                 margin: UiRect::axes(Val::Px(15.), Val::Percent(10.)),
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|builder| {
//             builder.spawn((
//                 TextBundle::from_sections([TextSection::new(
//                     "This text changes in the bottom right",
//                     TextStyle {
//                         font: font.clone(),
//                         font_size: 25.0,
//                         color: Color::WHITE,
//                     },
//                 )]),
//                 ChangeableText,
//             ));
//         })
//         .id();

//     commands.entity(root_uinode).push_children(&[right_column]);
// }

pub fn update_camera_position_text(
    pan_orbit_camera_query: Query<&PanOrbitCamera>,
    bevy_transmitter: ResMut<BevyTransmitter>,
    // mut query: Query<&mut Text, With<ChangeableText>>,
) {
    // for mut text in &mut query {
    if let Ok(camera) = pan_orbit_camera_query.get_single() {
        // let changed_text = format!(
        //     "focus:{:?} alpha: {:?} beta: {:?} radius: {:?}",
        //     camera.focus, camera.alpha, camera.beta, camera.radius,
        // );
        // text.sections[0].value = format!("{changed_text}");
        let _ = bevy_transmitter
            .0
            .send(MessageFromBevy::CameraPosition(CameraPosition {
                focus: camera.focus,
                alpha: camera.alpha,
                beta: camera.beta,
                radius: camera.radius,
            }));
    };
    // }
}
