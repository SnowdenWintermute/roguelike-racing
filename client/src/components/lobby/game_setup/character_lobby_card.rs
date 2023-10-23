use common::character::Character;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub character: Character,
}

#[function_component(CharacterLobbyCard)]
pub fn character_lobby_card(props: &Props) -> Html {
    let character = &props.character;
    html!(
            <div>
                <div>
                    {&character.entity_properties.name}
                </div>
                <div>
                    {"Class: " }
                    {format!("{}", &character.combatant_properties.combatant_class)}
                </div>
            </div>
    )
}
