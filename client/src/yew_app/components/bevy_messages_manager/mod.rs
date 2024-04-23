mod finished_processing_turn;
mod hp_change_message_handler;
mod started_processing_turn_result;
use self::finished_processing_turn::finished_processing_turn;
use self::hp_change_message_handler::hp_change_message_handler;
use self::started_processing_turn_result::started_processing_turn_result;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::YewTransmitter;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use gloo::console::log;
use std::ops::Deref;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties)]
pub struct Props {
    // pub shared: Arc<Mutex<SharedState>>,
    pub yew_transmitter: YewTransmitter,
    pub bevy_transmitter: BevyTransmitter,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[function_component(BevyMessagesManager)]
pub fn bevy_messages_manager(props: &Props) -> Html {
    let (_, dispatch) = use_store::<BevyCommunicationStore>();
    let (_, game_dispatch) = use_store::<GameStore>();
    let Props {
        bevy_transmitter,
        yew_transmitter,
    } = props;

    let cloned_transmitter = yew_transmitter.clone();
    let cloned_dispatch = dispatch.clone();
    use_effect_with((), move |_| {
        log!("setting yew transmitter");
        cloned_dispatch.reduce_mut(|store| store.transmitter_option = Some(cloned_transmitter))
    });

    // let name = shared.lock().unwrap().name.clone();
    let most_recent_message_from_bevy_state = use_state(|| Vec::new());
    let queued_bevy_messages_state: UseStateHandle<Vec<MessageFromBevy>> = use_state(|| Vec::new());

    // GET THE MOST RECENT MESSAGE
    let mut receiver = bevy_transmitter.subscribe();
    use_effect_with((), {
        let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
        move |()| {
            let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
            spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    let mut messages = Vec::from([message]);
                    while let Ok(subsequent_message) = receiver.try_recv() {
                        messages.push(subsequent_message)
                    }
                    // log!(format!("got messages from bevy: {:#?}", messages));
                    most_recent_message_from_bevy_state.set(messages);
                }
            });
        }
    });

    // READ THE MOST RECENT MESSAGE AND ADD TO QUEUE
    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
    use_effect_with(most_recent_message_from_bevy_state, move |_| {
        let mut message_to_enqueue = cloned_most_recent_message_from_bevy_state.deref().clone();

        // log!(format!("enqueuing message : {:#?}", message_to_enqueue));
        let mut current_messages = cloned_queued_bevy_messages_state.deref().clone();
        current_messages.append(&mut message_to_enqueue);
        cloned_queued_bevy_messages_state.set(current_messages);
        cloned_most_recent_message_from_bevy_state.set(Vec::new());
    });

    // DEQUEUE AND HANDLE MESSAGES
    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_dispatch = dispatch.clone();
    let cloned_game_dispatch = game_dispatch.clone();
    use_effect_with(
        cloned_queued_bevy_messages_state.clone(),
        move |cloned_queued_bevy_messages_state| {
            let messages = cloned_queued_bevy_messages_state.deref();
            for message in messages {
                // log!(format!("processing message {:?}", message));
                match message {
                    MessageFromBevy::AssetsLoaded => {
                        cloned_dispatch.reduce_mut(|store| store.bevy_assets_loaded = true)
                    }
                    MessageFromBevy::CameraPosition(camera_position) => cloned_dispatch
                        .reduce_mut(|store| store.camera_position = camera_position.clone()),
                    MessageFromBevy::HpChangeById(hp_change_message) => {
                        let _result = hp_change_message_handler(
                            cloned_game_dispatch.clone(),
                            hp_change_message.combatant_id,
                            hp_change_message.hp_change,
                        );
                    }
                    MessageFromBevy::StartedProcessingTurnResult(combatant_id) => {
                        let _result =
                            started_processing_turn_result(game_dispatch.clone(), *combatant_id);
                    }
                    MessageFromBevy::FinishedProcessingTurnResult(combatant_id) => {
                        let _result =
                            finished_processing_turn(game_dispatch.clone(), *combatant_id);
                    }
                    _ => (), // MessageFromBevy::PartNames(part_names) => cloned_dispatch
                             //     .reduce_mut(|store| store.parts_available = part_names.clone()),
                             // MessageFromBevy::AnimationsAvailable(animation_names) => cloned_dispatch
                             //     .reduce_mut(|store| store.animation_names = animation_names.clone()),
                             // MessageFromBevy::CombatantSpawned(combatant_id) => {
                             //     cloned_dispatch.reduce_mut(|store| store.character_ids.push(*combatant_id))
                             // }
                }
            }
            // @TODO PERF don't reallocate a vec every time, instead try to mutate it
            cloned_queued_bevy_messages_state.set(Vec::new());
        },
    );

    html!(
    <div>
        {queued_bevy_messages_state.deref().iter().map(|item| html!(<div>{format!("{:#?}", item)}</div>)).collect::<Html>()}
    </div>
    )
}
