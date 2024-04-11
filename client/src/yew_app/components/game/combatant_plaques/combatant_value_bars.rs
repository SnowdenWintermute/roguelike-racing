use crate::yew_app::components::game::combatant::value_bar::ValueBar;
use common::combatants::{combat_attributes::CombatAttributes, CombatantProperties};
use yew::prelude::*;

pub fn get_combatant_value_bars(combatant_properties: &CombatantProperties) -> (Html, Html, Html) {
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

    let experience_bar = {
        if let Some(required_exp_to_level) = combatant_properties
            .experience_points
            .required_for_next_level
        {
            html!(
                <ValueBar max={required_exp_to_level}
                          curr={combatant_properties.experience_points.current}
                          color={"ffxipink"}
                          hide_numbers={ true } />
            )
        } else {
            html!()
        }
    };

    (hp_bar, mp_bar, experience_bar)
}
