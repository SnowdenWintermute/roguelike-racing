use common::items::{equipment::EquipmentSlots, Item};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub equipment: HashMap<EquipmentSlots, Item>,
}

#[function_component(PaperDoll)]
pub fn paper_doll(props: &Props) -> Html {
    let Props { equipment } = props;

    html!(
        <div id="paper-doll" class="flex w-1/2" >
            <div class="w-1/3" >
                <div class="h-24 mb-2 flex justify-between items-end" >
                    <div class="border border-slate-400 h-10 w-10" >
                        {"r1"}
                    </div>
                    <div class="border border-slate-400 h-10 w-10" >
                        {"r2"}
                    </div>
                </div>
                <div class="h-40 border border-slate-400" >
                    {"lh"}
                </div>
            </div>
            <div class="w-1/3 mr-2 ml-2" >
                <div class="h-24 w-full border border-slate-400 mb-2" >
                    {"head"}
                </div>
                <div class="h-40 w-full border border-slate-400" >
                    {"body"}
                </div>
            </div>
            <div class="w-1/3" >
                <div class="h-24 mb-2 flex justify-end items-end" >
                    <div class="border border-slate-400 h-10 w-10" >
                        {"am"}
                    </div>
                </div>
                <div class="h-40 border border-slate-400" >
                    {"rh"}
                </div>
            </div>
        </div>
    )
}
