use crate::components::game::dungeon_room::combatant::Combatant;
use common::dungeon_rooms::DungeonRoom;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room: DungeonRoom,
}

#[function_component(MonsterLair)]
pub fn monster_lair(props: &Props) -> Html {
    // let (game_state, _) = use_store::<GameStore>();
    let empty_vec = vec![];
    let monsters = props.room.monsters.as_ref().unwrap_or(&empty_vec);

    html!(
        <div class="flex flex-col items-end whitespace-nowrap" >
            {monsters.iter().map(|monster| {
                html!(
                    <Combatant
                        entity_properties={monster.entity_properties.clone()}
                        combatant_properties={monster.combatant_properties.clone()}
                    />
                    )
               }).collect::<Html>()}
        </div>
    )
}