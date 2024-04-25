pub mod action_menu;
mod character_autofocus_manager;
pub mod character_sheet;
pub mod combat_log;
pub mod combatant;
mod combatant_plaques;
pub mod context_dependant_information_display;
pub mod debug;
mod dungeon_room;
mod item_details_viewers;
mod items_on_ground;
mod ready_up_display;
mod tailwind_class_loader;
mod top_info_bar;
use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::utils::set_bevy_canvas_visibility;
use crate::yew_app::components::game::action_menu::ActionMenu;
use crate::yew_app::components::game::character_autofocus_manager::CharacterAutofocusManager;
use crate::yew_app::components::game::character_sheet::item_details_viewer::ItemDetailsViewer;
use crate::yew_app::components::game::character_sheet::CharacterSheet;
use crate::yew_app::components::game::combat_log::CombatLog;
use crate::yew_app::components::game::combatant_plaques::combatant_plaque_group::CombatantPlaqueGroup;
use crate::yew_app::components::game::item_details_viewers::ItemDetailsAndComparison;
use crate::yew_app::components::game::items_on_ground::ItemsOnGround;
use crate::yew_app::components::game::ready_up_display::ReadyUpDisplay;
use crate::yew_app::components::game::tailwind_class_loader::TailwindClassLoader;
use crate::yew_app::components::game::top_info_bar::TopInfoBar;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use common::game::getters::get_ally_ids_and_opponent_ids_option;
use gloo::console::log;
use gloo::events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::html::SendAsMessage;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();
    let (bevy_communication_state, _) = use_store::<BevyCommunicationStore>();
    if !bevy_communication_state.bevy_assets_loaded {
        return html!({ "loading assets" });
    };
    let cloned_bevy_communication_state = bevy_communication_state.clone();
    let transmitter_option = bevy_communication_state.transmitter_option.clone();
    use_effect_with(transmitter_option, move |_| {
        if let Some(transmitter) = cloned_bevy_communication_state.transmitter_option.clone() {
            let _result = transmitter.send(MessageFromYew::SetBevyRendering(true));
            log!("set bevy to start rendering")
        }
    });

    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");
    let player = game
        .players
        .get(&lobby_state.username)
        .expect("a player should exist by the username stored on the client")
        .clone();

    let party_id = game_state.current_party_id.expect("must have party id");

    let party = game
        .adventuring_parties
        .get(&party_id)
        .expect("must have a party id")
        .clone();

    use_effect_with((), move |_| set_bevy_canvas_visibility(true));

    let cloned_dispatch = game_dispatch.clone();
    let keyup_listener_state = use_state(|| None::<EventListener>);
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            if event.key() == "Escape" {
                cloned_dispatch.reduce_mut(|store| store.detailed_entity = None);
            }
        });
        keyup_listener_state.set(Some(listener));
    });

    let cloned_dispatch = game_dispatch.clone();
    use_effect_with((), move |_| {
        cloned_dispatch.reduce_mut(|game_state| {
            if let Some(ids) = &player.character_ids {
                let mut character_ids_vec = Vec::from_iter(ids);
                character_ids_vec.sort();
                game_state.focused_character_id = *character_ids_vec[0];
            }
        })
    });

    let focused_character = party.characters.get(&game_state.focused_character_id);

    let focused_character_has_selected_combat_action = match focused_character {
        Some(character) => character
            .combatant_properties
            .selected_combat_action
            .is_some(),
        None => false,
    };

    let show_character_sheet = if let Some(character) = focused_character {
        (game_state.viewing_inventory || game_state.viewing_attribute_point_assignment_menu)
            && !focused_character_has_selected_combat_action
    } else {
        false
    };

    let conditional_styles = if show_character_sheet {
        "items-center justify-end"
    } else {
        ""
    };

    let (ally_character_plaques, monster_plaques) = {
        let mut to_return = (
            html!( <CombatantPlaqueGroup combatant_ids={party.character_positions.clone()} show_experience={true} /> ),
            html!( <div/> ),
        );
        if let Some(battle_id) = party.battle_id {
            if let Ok((_, opponent_ids_option)) = get_ally_ids_and_opponent_ids_option(
                &party.character_positions,
                game.battles.get(&battle_id),
                game_state.focused_character_id,
            ) {
                if let Some(opponent_ids) = opponent_ids_option {
                    to_return.1 = html!(<CombatantPlaqueGroup combatant_ids={opponent_ids} show_experience={false} />)
                }
            };
        }
        to_return
    };

    let viewing_character_sheet = game_state.viewing_inventory
        || game_state.viewing_equipped_items
        || game_state.viewing_attribute_point_assignment_menu;

    let action_menu_and_character_sheet_container_conditional_classes = if viewing_character_sheet {
        ""
    } else {
        "w-full"
    };

    html!(
        <main class="h-screen w-screen flex justify-center relative">
            <TailwindClassLoader />
            <CharacterAutofocusManager />
            // <GameDebug />
            <div class="w-full h-full max-h-[calc(0.5625 * 100vw)] text-zinc-300 flex flex-col" >
                <TopInfoBar />
                <div class="p-4 flex-grow flex flex-col justify-between">
                    <ReadyUpDisplay />
                    <div class="flex justify-end">
                        <div class="w-fit">
                            {monster_plaques}
                        </div>
                    </div>
                    <div class="flex flex-wrap justify-between">
                        <div class="h-[14rem] min-w-[23rem] max-w-[26rem] w-full border border-slate-400 bg-slate-700 p-2 pointer-events-auto">
                            <CombatLog />
                        </div>
                        <div class="flex flex-grow justify-end mt-3.5">
                            <div class="w-fit flex items-end">
                                {ally_character_plaques}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            // Action Menu and Inventory/Equipment/Character sheet container
            <div class={ format!( "absolute z-31 top-1/2 -translate-y-1/2 w-full p-4 text-zinc-300 flex flex-row {}", conditional_styles)}>
                <div class={ format!("flex flex-col {} max-w-full", action_menu_and_character_sheet_container_conditional_classes)}>
                    <div class="flex">
                        <div class="flex flex-col flex-grow justify-end max-w-full">
                            <div class="flex justify-between overflow-auto">
                                <ActionMenu />
                                if !viewing_character_sheet {
                                    <div class="flex overflow-auto">
                                        <div class="max-h-[13.375rem] h-fit flex flex-grow justify-end">
                                            <div class="mr-2 w-[50rem]">
                                                <ItemDetailsAndComparison />
                                            </div>
                                            <div class="max-w-[25rem] w-[25rem]" >
                                                <ItemsOnGround max_height={25.0} />
                                            </div>
                                        </div>
                                    </div>
                                }
                            </div>
                        </div>
                        <CharacterSheet />
                    </div>
                    if !focused_character_has_selected_combat_action {
                        <ItemDetailsViewer />
                    }
                </div>
            </div>
        </main>
    )
}
