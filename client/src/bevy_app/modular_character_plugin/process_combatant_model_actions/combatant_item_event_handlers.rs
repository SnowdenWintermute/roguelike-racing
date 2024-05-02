use crate::comm_channels::CombatantItemEvent;
use crate::comm_channels::CombatantItemEvents;
use bevy::ecs::event::EventReader;

use super::process_active_model_actions::ModelActionSystemParams;

pub fn handle_combatant_item_events(
    mut combatant_item_events_reader: EventReader<CombatantItemEvent>,
    mut model_action_params: ModelActionSystemParams,
) {
    for event in combatant_item_events_reader.read() {
        let combatant_entity = model_action_params
            .combatants_by_id
            .0
            .get(&event.combatant_id)
            .expect("to have registered the combatant");
        let mut combatant = model_action_params
            .combatants_query
            .get_mut(*combatant_entity)
            .expect("to have the combatant entity");

        match &event.event_type {
            CombatantItemEvents::PickedUp(item) => {
                let _result = combatant
                    .combatant_properties_component
                    .0
                    .inventory
                    .items
                    .push(item.clone());
            }
            CombatantItemEvents::Dropped(item_id) => {
                let _result = combatant
                    .combatant_properties_component
                    .0
                    .inventory
                    .remove_item(*item_id);
            }
            CombatantItemEvents::Equipped(item_id, equip_to_alt_slot) => {
                let _result = combatant
                    .combatant_properties_component
                    .0
                    .equip_item(*item_id, *equip_to_alt_slot);
            }
            CombatantItemEvents::DroppedEquipped(equipment_slot) => {
                let _result = combatant
                    .combatant_properties_component
                    .0
                    .equipment
                    .remove(equipment_slot);
            }
            CombatantItemEvents::Unequipped(equipment_slot) => {
                let _result = combatant
                    .combatant_properties_component
                    .0
                    .unequip_slots(&Vec::from([equipment_slot.clone()]), false);
            }
        }
    }
}
