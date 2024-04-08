use crate::yew_app::components::game::combatant_plaques::CombatantPlaque;
use common::packets::CharacterId;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub combatant_ids: Vec<CharacterId>,
    pub show_experience: bool,
}

#[function_component(CombatantPlaqueGroup)]
pub fn combatant_plaque_group(props: &Props) -> Html {
    let plaques = props
        .combatant_ids
        .iter()
        .map(|id| {
            html!(
                <li class="mr-4 last:mr-0">
                    <CombatantPlaque combatant_id={id} show_experience={props.show_experience} />
                </li>
            )
        })
        .collect::<Html>();

    html!(
        <ul class="w-full flex justify-end items-end list-none">
            {plaques}
        </ul>
    )
}
