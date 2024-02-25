pub mod character_attributes;
mod paper_doll;
use crate::components::game::character_sheet::character_attributes::CharacterAttributes;
use crate::components::game::character_sheet::paper_doll::PaperDoll;
use crate::store::game_store::GameStore;
use common::character::Character;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub character: Character,
}

#[function_component(CharacterSheet)]
pub fn character_sheet(props: &Props) -> Html {
    let (_, game_dispatch) = use_store::<GameStore>();
    let Props { character } = props;

    use_effect_with((), move |_| {
        move || {
            game_dispatch.reduce_mut(|store| {
                store.hovered_entity = None;
            });
        }
    });

    html!(
        <section class="p-2 flex-grow border border-slate-400 bg-slate-700 overflow-y-auto flex">
            <PaperDoll equipment={character.combatant_properties.equipment.clone()} attributes={character.combatant_properties.get_total_attributes()} />
            <CharacterAttributes
                entity_properties={character.entity_properties.clone()}
                combatant_properties={character.combatant_properties.clone()}
                show_attribute_assignment_buttons={true}
            />
        </section>
    )
}
