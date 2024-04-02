use common::combatants::combatant_classes::CombatantClass;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_class: CombatantClass,
}

#[function_component(CombatantClassIcon)]
pub fn combatant_class_icon(props: &Props) -> Html {
    match &props.combatant_class {
        CombatantClass::Warrior => html!(
                <img src="public/img/combatant-class-icons/warrior.svg" class="h-full" />
        ),
        CombatantClass::Mage => html!(
                <img src="public/img/combatant-class-icons/staff.svg" class="h-full" />
        ),
        CombatantClass::Rogue => html!(
                <img src="public/img/combatant-class-icons/sword.svg" class="h-full" />
        ),
    }
}
