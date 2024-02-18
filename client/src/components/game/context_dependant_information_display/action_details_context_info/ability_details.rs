use crate::components::client_consts::UNMET_REQUIREMENT_TEXT_COLOR;
use crate::components::game::character_sheet::character_attributes::weapon_damage::CharacterSheetWeaponDamage;
use common::combat::combat_actions::CombatActionProperties;
use common::combatants::abilities::CombatantAbility;
use common::combatants::abilities::CombatantAbilityNames;
use common::combatants::CombatantProperties;
use common::game::RoguelikeRacerGame;
use yew::html;
use yew::Html;

pub fn ability_details(
    game: &RoguelikeRacerGame,
    ability: &CombatantAbility,
    combat_action_properties: &CombatActionProperties,
    user_combatant_properties: &CombatantProperties,
    combatant_id: u32,
) -> Html {
    let ability_attributes = ability.ability_name.get_attributes();
    let mp_cost = ability_attributes.mana_cost
        * ability_attributes.mana_cost_level_multiplier
        * user_combatant_properties.level;

    let mp_cost_style = if mp_cost as u16 > user_combatant_properties.mana {
        UNMET_REQUIREMENT_TEXT_COLOR
    } else {
        ""
    };

    let attack_damage_display = match ability.ability_name {
        CombatantAbilityNames::Attack => {
            html!(<CharacterSheetWeaponDamage combatant_id={combatant_id} />)
        }
        _ => html!(),
    };

    let mp_cost_display = if mp_cost > 0 {
        html!(<div class={format!("{}", mp_cost_style)}>{"MP Cost: "}{mp_cost}</div>)
    } else {
        html!()
    };

    let value_range_option = match &combat_action_properties.hp_change_properties {
        Some(hp_change_properties) => Some(
            game.calculate_combat_action_hp_change_range(
                user_combatant_properties,
                hp_change_properties,
                &Some((
                    ability.level,
                    ability_attributes.base_hp_change_values_level_multiplier,
                )),
            )
            .expect("to have valid data"),
        ),
        None => None,
    };

    let value_range_display = match value_range_option {
        Some(range) => html!(
          <div>{"Value range: "}{range.0}{" - "}{range.1}</div>
        ),
        None => html!(),
    };

    html!(<div class="flex flex-col justify-between" >
              {mp_cost_display}
              {value_range_display}
              {attack_damage_display}
          </div>)
}
