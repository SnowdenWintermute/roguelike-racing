pub mod comm_channel_bevy_plugin;
use crate::bevy_app::modular_character_plugin::CombatantId;
use crate::bevy_app::modular_character_plugin::HitPoints;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::frontend_common::AttackCommand;
use crate::frontend_common::CharacterPartSelection;
use crate::frontend_common::CombatantSpecies;
use crate::frontend_common::PartsByName;
use crate::yew_app::components::mesh_manager::HpChange;
use bevy::prelude::*;
use broadcast::Receiver;
use broadcast::Sender;
use common::combat::CombatTurnResult;
use common::combatants::CombatantProperties;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use tokio::sync::broadcast;

// YEW MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromYew {
    SelectCharacterPart(CharacterPartSelection),
    SpawnCharacterWithHomeLocation(
        CombatantId,
        HomeLocation,
        CombatantSpecies,
        CombatantProperties,
    ),
    DespawnCombatantModel(CombatantId),
    NewTurnResults(VecDeque<CombatTurnResult>),
    SetBevyRendering(bool),
}
#[derive(Clone, Debug, Event)]
pub struct CharacterPartSelectionEvent(pub CharacterPartSelection);

#[derive(Clone, Debug, Event)]
pub struct CharacterSpawnEvent(
    pub CombatantId,
    pub HomeLocation,
    pub CombatantSpecies,
    pub CombatantProperties,
);

#[derive(Clone, Debug, Event)]
pub struct DespawnCombatantModelEvent(pub CombatantId);

#[derive(Clone, Debug, Event)]
pub struct ProcessNextTurnResultEvent;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CameraPosition {
    pub focus: Vec3,
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HpChangeMessageFromBevy {
    pub combatant_id: CombatantId,
    pub hp_change: i16,
}

// BEVY MESSAGES
#[derive(Debug, Clone, PartialEq)]
pub enum MessageFromBevy {
    PartNames(PartsByName),
    AnimationsAvailable(HashSet<String>),
    CombatantSpawned(CombatantId),
    AssetsLoaded,
    CameraPosition(CameraPosition),
    HpChangeById(HpChangeMessageFromBevy),
    CombatantEvadedAttack(CombatantId),
}
// CHANNELS
#[derive(Clone, Resource, Deref)]
pub struct YewTransmitter(pub Sender<MessageFromYew>);
#[derive(Resource, Deref, DerefMut)]
pub struct YewReceiver(pub Receiver<MessageFromBevy>);

// required so it can be passed as yew Props
impl PartialEq for YewTransmitter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct BevyTransmitter(pub Sender<MessageFromBevy>);
#[derive(Resource, Deref, DerefMut)]
pub struct BevyReceiver(pub Receiver<MessageFromYew>);

pub fn create_comm_channels() -> (
    (YewTransmitter, YewReceiver),
    (BevyTransmitter, BevyReceiver),
) {
    let (yew_transmitter, bevy_receiver) = broadcast::channel(50);
    let (bevy_transmitter, yew_receiver) = broadcast::channel(50);

    (
        (YewTransmitter(yew_transmitter), YewReceiver(yew_receiver)),
        (
            BevyTransmitter(bevy_transmitter),
            BevyReceiver(bevy_receiver),
        ),
    )
}
