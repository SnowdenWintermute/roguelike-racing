use crate::components::common_components::molocules::text_submit::TextSubmit;
use crate::components::game::combatant::combatant_class_icon::CombatantClassIcon;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::combatants::combatant_classes::CombatantClass;
use common::combatants::combatant_classes::COMBATANT_CLASS_DESCRIPTIONS;
use common::packets::client_to_server::CharacterCreation;
use common::packets::client_to_server::PlayerInputs;
use std::ops::Deref;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(CharacterCreationMenu)]
pub fn character_creation_menu() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let combatant_class_selection_state = use_state(|| CombatantClass::Warrior);

    let cloned_combatant_class_selection_state = combatant_class_selection_state.clone();
    let create_character = move |character_name: AttrValue| {
        let selected_class = &*cloned_combatant_class_selection_state;
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateCharacter(CharacterCreation {
                character_name: character_name.deref().to_string(),
                combatant_class: selected_class.clone(),
            }),
        );
    };

    let class_selection_buttons = {
        let combatant_classes: Vec<CombatantClass> = CombatantClass::iter().collect();
        let mut to_return = Vec::new();
        for combatant_class in combatant_classes {
            to_return.push(html!(
                    <CombatantClassSelectionButton combatant_class={combatant_class} combatant_class_selection_state={combatant_class_selection_state.clone()} />
                    ))
        }
        to_return
    };

    let combatant_class = &*combatant_class_selection_state.clone();

    html!(
        <div class="mb-2">
            <ul class="flex mb-2" >
                {class_selection_buttons}
            </ul>
            <div class="mb-2 flex " >
                <span class="h-20 w-20 p-1 flex justify-center rotate-45 mr-4" >
                    <CombatantClassIcon combatant_class={combatant_class.clone()} />
                </span>
                <div>
                    <h5 class="font-bold mb-1" >
                        {format!("{}", *combatant_class_selection_state)}
                    </h5>
                    <p>
                        {format!("{}", COMBATANT_CLASS_DESCRIPTIONS.get(&*combatant_class_selection_state).unwrap_or_else(||&""))}

                    </p>
                </div>
            </div>
            <TextSubmit
                input_name={"character name"}
                input_placeholder={"Character name..."}
                button_title={"Create Character"}
                submit_disabled={false}
                submit_handler_callback={create_character}
            />
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    combatant_class: CombatantClass,
    combatant_class_selection_state: UseStateHandle<CombatantClass>,
}

#[function_component(CombatantClassSelectionButton)]
pub fn combatant_class_selection_button(props: &Props) -> Html {
    let selected_style = if &*props.combatant_class_selection_state == &props.combatant_class {
        "border-yellow-400"
    } else {
        "border-slate-400"
    };

    let cloned_combatant_class_selection_state = props.combatant_class_selection_state.clone();
    let cloned_combatant_class = props.combatant_class.clone();
    let handle_click = Callback::from(move |_| {
        cloned_combatant_class_selection_state.set(cloned_combatant_class.clone())
    });

    html!(
        <li class="mr-2 last:mr-0 ">
            <button class={format!("border h-10 p-2 {}", selected_style)} onclick={handle_click} >
                {format!("{}", &props.combatant_class)}
            </button>
        </li>
    )
}
