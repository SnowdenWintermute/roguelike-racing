pub mod comm_channel_bevy_plugin;
pub mod messages_from_bevy;
pub mod messages_from_yew;
use self::messages_from_bevy::MessageFromBevy;
use self::messages_from_yew::MessageFromYew;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::frontend_common::CharacterPartSelection;
use bevy::prelude::*;
use broadcast::Receiver;
use broadcast::Sender;
use common::combatants::combatant_species::CombatantSpecies;
use common::combatants::CombatantProperties;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use common::primatives::EntityId;
use common::primatives::EntityProperties;
use tokio::sync::broadcast;

#[derive(Clone, Debug, Event)]
pub struct CharacterPartSelectionEvent(pub CharacterPartSelection);

#[derive(Clone, Debug, Event)]
pub struct CharacterSpawnEvent(
    pub EntityId,
    pub HomeLocation,
    pub CombatantSpecies,
    pub CombatantProperties,
    pub EntityProperties,
);

#[derive(Clone, Debug, Event)]
pub struct DespawnCombatantModelEvent(pub EntityId);

pub type IdOfCombatantTurnJustFinished = u32;
#[derive(Clone, Debug, Event)]
pub struct ProcessNextTurnResultEvent(pub Option<IdOfCombatantTurnJustFinished>);

#[derive(Clone, Debug)]
pub enum CombatantItemEvents {
    PickedUp(Item),
    Dropped(EntityId),
    DroppedEquipped(EquipmentSlots),
    Equipped(EntityId, bool),
    Unequipped(EquipmentSlots),
}

#[derive(Clone, Debug, Event)]
pub struct CombatantItemEvent {
    pub combatant_id: EntityId,
    pub event_type: CombatantItemEvents,
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
