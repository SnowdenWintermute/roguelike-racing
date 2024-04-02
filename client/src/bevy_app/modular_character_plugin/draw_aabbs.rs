use bevy::prelude::*;

pub fn draw_aabbs(mut config_store: ResMut<GizmoConfigStore>) {
    config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
}
