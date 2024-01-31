use common::combatants::abilities::CombatantAbilityNames;
use common::items::consumables::ConsumableProperties;
use common::items::consumables::ConsumableTypes;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ability_name_option: Option<CombatantAbilityNames>,
    pub consumable_option: Option<ConsumableProperties>,
}

#[function_component(TargetingIndicator)]
pub fn targeting_indicator(props: &Props) -> Html {
    let mut color = "yellow-700";

    if let Some(consumable) = &props.consumable_option {
        match consumable.consumable_type {
            ConsumableTypes::HpAutoinjector => color = "green-600",
            ConsumableTypes::Grenade => (),
            ConsumableTypes::SmokeBomb => color = "gray-700",
        }
    }
    html!(
        <div class={format!("w-0 h-0 border-t-[1.5rem] border-t-{color}
                            border-r-[1.5rem] border-r-transparent border-l-[1.5rem] border-l-transparent
        ")} />
    )
}
