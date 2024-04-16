pub mod comm_channel_bevy_plugin;
use crate::bevy_app::modular_character_plugin::CombatantId;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::frontend_common::AttackCommand;
use crate::frontend_common::CharacterPartSelection;
use crate::frontend_common::CombatantSpecies;
use crate::frontend_common::PartsByName;
use bevy::prelude::*;
use broadcast::Receiver;
use broadcast::Sender;
use common::combat::CombatTurnResult;
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
        HashMap<EquipmentSlots, Item>,
    ),
    DespawnCombatantModel(CombatantId),
    ExecuteAttackSequence(AttackCommand),
    NewTurnResults(VecDeque<CombatTurnResult>),
}
#[derive(Clone, Debug, Event)]
pub struct CharacterPartSelectionEvent(pub CharacterPartSelection);

#[derive(Clone, Debug, Event)]
pub struct CharacterSpawnEvent(
    pub CombatantId,
    pub HomeLocation,
    pub CombatantSpecies,
    pub HashMap<EquipmentSlots, Item>,
);

#[derive(Clone, Debug, Event)]
pub struct DespawnCombatantModelEvent(pub CombatantId);

#[derive(Clone, Debug, Event)]
pub struct StartAttackSequenceEvent(pub AttackCommand);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CameraPosition {
    pub focus: Vec3,
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub radius: Option<f32>,
}

// BEVY MESSAGES
#[derive(Debug, Clone, PartialEq)]
pub enum MessageFromBevy {
    PartNames(PartsByName),
    AnimationsAvailable(HashSet<String>),
    CombatantSpawned(CombatantId),
    AssetsLoaded,
    CameraPosition(CameraPosition),
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
