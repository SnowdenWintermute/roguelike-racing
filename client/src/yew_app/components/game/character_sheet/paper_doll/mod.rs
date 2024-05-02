mod paper_doll_slot;
use crate::yew_app::components::game::character_sheet::paper_doll::paper_doll_slot::PaperDollSlot;
use common::combatants::combat_attributes::CombatAttributes;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub equipment: HashMap<EquipmentSlots, Item>,
    pub attributes: HashMap<CombatAttributes, u16>,
}

#[function_component(PaperDoll)]
pub fn paper_doll(props: &Props) -> Html {
    let mut equipment = props.equipment.clone();

    html!(
        <div id="paper-doll" class="flex w-[23.75rem] mr-5" >
            <div class="w-[7.5rem] mr-2.5" >
                <div class="h-[6.25rem] mb-2.5 flex justify-between items-end" >
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::RightRing)}
                        character_attributes={props.attributes.clone()}
                        slot={EquipmentSlots::RightRing}
                        class=" h-10 max-h-10 w-10 max-w-10" />
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::LeftRing)}
                        character_attributes={props.attributes.clone()}
                        slot={EquipmentSlots::LeftRing}
                        class=" h-10 max-h-10 w-10 max-w-10" />
                </div>
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::MainHand)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::MainHand}
                    class="h-[12.125rem] max-h-[12.125rem] w-full" />
            </div>
            <div class="w-[7.5rem] mr-2.5" >
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Head)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::Head}
                    class="h-[6.25rem] w-full mb-2.5" />
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Body)}
                    character_attributes={props.attributes.clone()}
                    slot={EquipmentSlots::Body}
                    class="h-[12.125rem] max-h-[12.125rem] w-full" />
            </div>
            <div class="w-[7.5rem]" >
                <div class="h-[6.25rem] mb-2.5 flex justify-end items-end" >
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
                    class="h-[12.125rem] w-full" />
            </div>
        </div>
    )
}
