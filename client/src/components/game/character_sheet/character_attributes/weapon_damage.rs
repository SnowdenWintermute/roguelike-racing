use crate::store::game_store::GameStore;
use common::app_consts::OFF_HAND_ACCURACY_MODIFIER;
use common::app_consts::OFF_HAND_DAMAGE_MODIFIER;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;
use common::combatants::combat_attributes::CombatAttributes;
use common::items::equipment::EquipmentSlots;
use common::items::equipment::EquipmentTypes;
use common::primatives::Range;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub combatant_id: u32,
}

#[function_component(CharacterSheetWeaponDamage)]
pub fn character_sheet_weapon_damage(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let combatant_id = props.combatant_id;
    let game = game_state.get_current_game().expect("to be in a game");
    let (_, combatant_properties) = game
        .get_combatant_by_id(&combatant_id)
        .expect("to have a valid combatant");
    let combatant_level = combatant_properties.level;
    let combat_attributes = combatant_properties.get_total_attributes();
    let combatant_accuracy = combat_attributes
        .get(&CombatAttributes::Accuracy)
        .unwrap_or_else(|| &0);
    let mh_weapon_option = combatant_properties.get_weapon_in_slot(&EquipmentSlots::MainHand);
    let mh_is_two_handed = match mh_weapon_option {
        Some(equipment_properties) => equipment_properties.is_two_handed(),
        None => false,
    };
    let mh_ability_name = match mh_weapon_option {
        Some(equipment_properties) => match equipment_properties.equipment_type {
            EquipmentTypes::TwoHandedRangedWeapon(_, _) => {
                CombatantAbilityNames::AttackRangedMainhand
            }
            _ => CombatantAbilityNames::AttackMeleeMainhand,
        },
        None => CombatantAbilityNames::AttackMeleeMainhand,
    };
    let mh_attack_action = CombatAction::AbilityUsed(mh_ability_name.clone());

    let mh_attack_action_properties = mh_attack_action
        .get_properties_if_owned(game, combatant_id)
        .expect("all combatants to have this ability");
    let mh_ability_attributes = mh_ability_name.get_attributes();

    let (mh_min, mh_max) = game
        .calculate_combat_action_hp_change_range(
            &combatant_properties,
            &mh_attack_action_properties
                .hp_change_properties
                .expect("attack action to have hp change properties"),
            &Some((
                combatant_level,
                mh_ability_attributes.base_hp_change_values_level_multiplier,
            )),
        )
        .expect("to get valid response");

    let oh_equipment_option = combatant_properties.get_equipped_item(&EquipmentSlots::OffHand);
    let oh_ability_name_option = if mh_is_two_handed {
        None
    } else {
        match oh_equipment_option {
            Some(equipment_properties) => match equipment_properties.equipment_type {
                EquipmentTypes::Shield(_, _) => None,
                _ => Some(CombatantAbilityNames::AttackMeleeOffhand),
            },
            None => Some(CombatantAbilityNames::AttackMeleeOffhand),
        }
    };
    let oh_attack_action_option = match &oh_ability_name_option {
        Some(ability_name) => Some(CombatAction::AbilityUsed(ability_name.clone())),
        None => None,
    };

    let oh_attack_action_properties_option = match oh_attack_action_option {
        Some(oh_attack_action) => Some(
            oh_attack_action
                .get_properties_if_owned(game, combatant_id)
                .expect("all combatants to have this ability"),
        ),
        None => None,
    };
    let oh_damage_range_option = match oh_attack_action_properties_option {
        Some(action_properties) => {
            let oh_ability_attributes = oh_ability_name_option.expect("").get_attributes();
            Some(
                game.calculate_combat_action_hp_change_range(
                    &combatant_properties,
                    &action_properties
                        .hp_change_properties
                        .expect("attack action to have hp change properties"),
                    &Some((
                        combatant_level,
                        oh_ability_attributes.base_hp_change_values_level_multiplier,
                    )),
                )
                .expect("to get valid response"),
            )
        }
        None => None,
    };

    let mh_damage_and_acc_option = Some((
        Range::new(mh_min as u16, mh_max as u16),
        *combatant_accuracy,
    ));

    let oh_damage_and_acc_option = match oh_damage_range_option {
        Some(range) => Some((
            Range::new(
                (range.0 * OFF_HAND_DAMAGE_MODIFIER as f32 / 100.0) as u16,
                (range.1 * OFF_HAND_DAMAGE_MODIFIER as f32 / 100.0) as u16,
            ),
            (*combatant_accuracy as f32 * (OFF_HAND_ACCURACY_MODIFIER as f32 / 100.0)) as u16,
        )),
        None => None,
    };

    html!(
        <div class="flex" >
            {weapon_damage_entry(mh_damage_and_acc_option, &"Main Hand", &"mr-1")}
            {weapon_damage_entry(oh_damage_and_acc_option, &"Off Hand", &"ml-1")}
        </div>
    )
}

fn weapon_damage_entry(
    damage_and_accuracy_option: Option<(Range<u16>, u16)>,
    label: &str,
    padding_class: &str,
) -> Html {
    if let Some(damage_and_accuracy) = damage_and_accuracy_option {
        let damage = damage_and_accuracy.0;
        let accuracy = damage_and_accuracy.1;

        html!(
        <div class={format!("w-1/2 {}", padding_class )}>
            <div class="w-full flex justify-between">
                <span>
                    {label}
                </span>
                <span>
                    {format!("{}-{}",damage.min,damage.max)}
                </span>
            </div>
            <div class="w-full flex justify-between">
                <span>
                    {"Accuracy"}
                </span>
                <span>
                    {accuracy}
                </span>
            </div>
        </div>
        )
    } else {
        html!(<div class={format!("w-1/2 mr-1{}", padding_class)  }/>)
    }
}
