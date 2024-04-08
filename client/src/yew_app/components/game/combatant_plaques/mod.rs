pub mod combatant_plaque_group;
use common::{combatants::combat_attributes::CombatAttributes, packets::CharacterId};
use yew::prelude::*;
use yewdux::use_store;

use crate::yew_app::{
    components::game::combatant::value_bar::ValueBar, store::game_store::GameStore,
};

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub combatant_id: CharacterId,
    pub show_experience: bool,
}

#[function_component(CombatantPlaque)]
pub fn combatant_plaque(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();

    let game = game_state.game.as_ref().expect("to be in a game");

    let (entity_properties, combatant_properties) = game
        .get_combatant_by_id(&props.combatant_id)
        .expect("to have a reference to a valid combatant");

    let total_attributes = combatant_properties.get_total_attributes();
    let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
    let max_mp_option = total_attributes.get(&CombatAttributes::Mp);

    let hp_bar = if let Some(max_hp) = max_hp_option {
        html!(<ValueBar max={max_hp} curr={combatant_properties.hit_points} color={"green-700"} />)
    } else {
        html!({ "Immortal Object" })
    };

    let mp_bar = if let Some(max_mp) = max_mp_option {
        if *max_mp == 0 {
            html!()
        } else {
            html!(<ValueBar max={max_mp} curr={combatant_properties.mana} color={"blue-700"} />)
        }
    } else {
        html!({ "Infinite Mana" })
    };

    html!(
    <div class="w-96 h-full border border-slate-400 bg-slate-700 p-2 flex pointer-events-auto">
        <div class="h-full aspect-square mr-2 border border-slate-400 bg-slate-600 rounded-full relative">
            <div class="absolute -bottom-1 left-1/2 -translate-x-1/2 h-5 border border-slate-400 bg-slate-700 pr-2 pl-2 text-sm flex items-center justify-center">
                {combatant_properties.level}
            </div>
        </div>
        <div class="flex-grow">
            <div class="mb-1.5 flex justify-between text-lg">
                <span>
                    {entity_properties.name.clone()}
                </span>
                <button>
                    {"ⓘ "}
                </button>
            </div>
            <div class="h-5 mb-1">
                {hp_bar}
            </div>
            <div class="h-5">
                {mp_bar}
            </div>
        </div>
    </div>
    )
}
