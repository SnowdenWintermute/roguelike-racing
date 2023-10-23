use common::character::combatant_properties::CombatantClass;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    combatant_class: CombatantClass,
}

#[function_component(CombatantClassDisplay)]
pub fn combatant_class_display(props: &Props) -> Html {
    html!(
            <div>
                {"Class: " }
                {format!("{}", props.combatant_class)}
            </div>
    )
}
