use common::combatants::combat_attributes::CombatAttributes;
use common::combatants::CombatantProperties;
use std::collections::HashMap;
use yew::html;
use yew::Html;

pub fn hp_and_mp(
    combatant_properties: &CombatantProperties,
    total_attributes: &HashMap<CombatAttributes, u16>,
) -> Html {
    let max_hp_option = total_attributes.get(&CombatAttributes::Hp);
    let max_mp_option = total_attributes.get(&CombatAttributes::Mp);

    html!(
        <div class="flex" >
            <div class="w-1/2 flex justify-between mr-1" >
                <span>
                    {"HP"}
                </span>
                <span>
                {
                    if let Some(max_hp) = max_hp_option{
                        format!("{}/{}", combatant_properties.hit_points, max_hp)
                    } else {
                        {"Immortal Object".to_string()}
                    }
                }
                </span>
            </div>
            <div class="w-1/2 flex justify-between ml-1" >
                <span>
                    {"Mana"}
                </span>
                <span>
                {
                    if let Some(max_mp) = max_mp_option{
                        format!("{}/{}", combatant_properties.mana, max_mp)
                    } else {
                        {"Infinite Mana".to_string()}
                    }
                }
                </span>
            </div>
        </div>
    )
}
