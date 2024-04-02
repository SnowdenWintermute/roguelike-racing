use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    RegisteringAnimations,
    Done,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoaderState>().add_loading_state(
            LoadingState::new(AssetLoaderState::Loading)
                .continue_to_state(AssetLoaderState::RegisteringAnimations)
                .load_collection::<MyAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(
        paths("scifi_head.glb", "witch_head.glb", "space_head.glb"),
        collection(typed, mapped)
    )]
    pub heads: HashMap<String, Handle<Gltf>>,
    #[asset(
        paths(
            "main_skeleton.glb",
            "wasp_main_skeleton.glb",
            "frog_main_skeleton.glb"
        ),
        collection(typed, mapped)
    )]
    pub main_skeletons_with_animations: HashMap<String, Handle<Gltf>>,
    #[asset(paths("wasp_full.glb", "frog_full.glb"), collection(typed, mapped))]
    pub non_humanoids: HashMap<String, Handle<Gltf>>,
    #[asset(paths("scifi_torso.glb", "witch_torso.glb"), collection(typed, mapped))]
    pub torsos: HashMap<String, Handle<Gltf>>,
    #[asset(paths("scifi_legs.glb", "witch_legs.glb"), collection(typed, mapped))]
    pub legs: HashMap<String, Handle<Gltf>>,
    #[asset(paths("sword.glb", "spear.glb"), collection(typed, mapped))]
    pub weapons: HashMap<String, Handle<Gltf>>,
    #[asset(paths("FiraSans-Regular.ttf"), collection(typed, mapped))]
    pub font_files: HashMap<String, Handle<Font>>,
}
