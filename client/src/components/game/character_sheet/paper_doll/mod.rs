mod paper_doll_slot;
use common::items::{equipment::EquipmentSlots, Item};
use std::collections::HashMap;
use yew::prelude::*;

use crate::components::game::character_sheet::paper_doll::paper_doll_slot::PaperDollSlot;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub equipment: HashMap<EquipmentSlots, Item>,
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
                        class="border border-slate-400 h-10 w-10" />
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::LeftRing)}
                        class="border border-slate-400 h-10 w-10" />
                </div>
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::RightHand)}
                    class="h-40 w-full border border-slate-400" />
            </div>
            <div class="w-1/3 mr-2 ml-2" >
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Head)}
                    class="h-24 w-full border border-slate-400 mb-2" />
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::Body)}
                    class="h-40 w-full border border-slate-400" />
            </div>
            <div class="w-1/3" >
                <div class="h-24 mb-2 flex justify-end items-end" >
                    <PaperDollSlot
                        item_option={equipment.remove(&EquipmentSlots::Amulet)}
                        class="border border-slate-400 h-10 w-10" />
                </div>
                <PaperDollSlot
                    item_option={equipment.remove(&EquipmentSlots::LeftHand)}
                    class="h-40 w-full border border-slate-400" />
            </div>
        </div>
    )
}
