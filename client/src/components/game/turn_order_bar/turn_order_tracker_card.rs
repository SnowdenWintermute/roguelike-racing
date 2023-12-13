use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[function_component(TurnOrderTrackerCard)]
pub fn turn_order_tracker_card(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_result = game_state.get_current_party();
    let entity_option = {
        if let Ok(party) = party_result {
            if let Some(character) = party.characters.get(&props.id) {
                Some(&character.entity_properties)
            } else if let Some(monsters) = &party.current_room.monsters {
                let mut to_return = None;
                for monster in monsters {
                    if monster.entity_properties.id == props.id {
                        to_return = Some(&monster.entity_properties);
                        break;
                    }
                }
                to_return
            } else {
                None
            }
        } else {
            None
        }
    };

    let button_content = match entity_option {
        Some(entity) => html!({ &entity.name }),
        None => {
            html!({ "Error - no entity found" })
        }
    };

    html!(
    <button class="border-r border-slate-400 p-2 last:border-r-0 w-20 whitespace-nowrap overflow-hidden text-ellipsis" >
        {button_content}
    </button>
    )
}
