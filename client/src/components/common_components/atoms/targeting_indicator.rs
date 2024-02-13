use common::combat::combat_actions::CombatAction;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combat_action: CombatAction,
}

#[function_component(TargetingIndicator)]
pub fn targeting_indicator(props: &Props) -> Html {
    let color = match props.combat_action {
        CombatAction::AbilityUsed(_) => "yellow-700",
        CombatAction::ConsumableUsed(_) => "green-600",
    };
    html!(
        <div class={format!("w-0 h-0 border-t-[1.5rem] border-t-{color}
                            border-r-[1.5rem] border-r-transparent border-l-[1.5rem] border-l-transparent
        ")} />
    )
}
