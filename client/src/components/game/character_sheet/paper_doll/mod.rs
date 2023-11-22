mod paper_doll_slot;
use common::{
    combatants::CombatAttributes,
    items::{equipment::EquipmentSlots, Item},
};
use std::collections::HashMap;
use yew::prelude::*;

use crate::components::game::character_sheet::paper_doll::paper_doll_slot::PaperDollSlot;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub attributes: HashMap<CombatAttributes, u16>,
}

#[function_component(PaperDoll)]
pub fn paper_doll(props: &Props) -> Html {
    let mut equipment = props.equipment.clone();

    html!(
        <div id="paper-doll" class="flex w-1/2" >
            <div class="w-1/3" >
                <div class="h-24 mb-2 flex justify-between items-end" >
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::RightRing)}
                        character_attributes={props.attributes.clone()}
                        slot={EquipmentSlots::RightRing}
                        class=" h-10 w-10" />
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::LeftRing)}
                        character_attributes={props.attributes.clone()}
                        slot={EquipmentSlots::LeftRing}
                        class=" h-10 w-10" />
                </div>
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::MainHand)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::MainHand}
                    class="h-40 w-full " />
            </div>
            <div class="w-1/3 mr-2 ml-2" >
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Head)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::Head}
                    class="h-24 w-full  mb-2" />
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Body)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::Body}
                    class="h-40 w-full " />
            </div>
            <div class="w-1/3" >
                <div class="h-24 mb-2 flex justify-end items-end" >
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::Amulet)}
                        character_attributes={props.attributes.clone()}
                        slot={EquipmentSlots::Amulet}
                        class=" h-10 w-10" />
                </div>
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::OffHand)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::OffHand}
                    class="h-40 w-full " />
            </div>
        </div>
    )
}
