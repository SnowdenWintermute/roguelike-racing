pub mod combat_log_message;
use self::combat_log_message::CombatLogMessage;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CombatLog)]
pub fn combat_log() -> Html {
    let (game_state, _) = use_store::<GameStore>();

    html!(
        <div class="h-full flex flex-col">
            <h3 class="flex-grow-0 flex-shrink" >{"Combat log"}</h3>
            <div class="list-none overflow-y-auto
           flex flex-col-reverse flex-1" >
               <ul class="" >
               {game_state.combat_log.iter().map(|log_entry| html!(
                       <CombatLogMessageElement combat_log_message={log_entry.clone()} />))
                   .collect::<Html>()}
               </ul>
            </div>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    combat_log_message: CombatLogMessage,
}

#[function_component(CombatLogMessageElement)]
pub fn combat_log_message_element(props: &Props) -> Html {
    let Props { combat_log_message } = props;
    let color = match combat_log_message.style {
        combat_log_message::CombatLogMessageStyle::Basic => "",
        combat_log_message::CombatLogMessageStyle::PartyProgress => "text-yellow-400",
        combat_log_message::CombatLogMessageStyle::PartyWipe => "text-red-400",
        combat_log_message::CombatLogMessageStyle::PartyEscape => "text-green-600",
        combat_log_message::CombatLogMessageStyle::BattleVictory => "text-green-600",
    };

    html!(
        <li class={color}>{combat_log_message.message.clone()}</li>
    )
}
