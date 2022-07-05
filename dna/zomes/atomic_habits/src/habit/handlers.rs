use super::Habit;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_habit(entry_hash: EntryHashB64) -> ExternResult<Option<Habit>> {
    let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

    match maybe_element {
        None => Ok(None),
        Some(element) => {
            let habit: Habit = element.entry().to_app_option()?.ok_or(WasmError::Guest(
                "Could not deserialize element to Habit.".into(),
            ))?;

            Ok(Some(habit))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLCreatePayload {
    payload: NewHabitOutput,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewHabitOutput {
    header_hash: HeaderHashB64,
    entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_habit(habit: Habit) -> ExternResult<GraphQLCreatePayload> {
    let header_hash = create_entry(&habit.clone())?;
    let entry_hash = hash_entry(&habit)?;

    let output = GraphQLCreatePayload {
        payload: NewHabitOutput {
            header_hash: HeaderHashB64::from(header_hash),
            entry_hash: EntryHashB64::from(entry_hash),
        },
    };

    Ok(output)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitInput {
    original_header_hash: HeaderHashB64,
    updated_habit: Habit,
}

#[hdk_extern]
pub fn update_habit(input: UpdateHabitInput) -> ExternResult<NewHabitOutput> {
    let header_hash = update_entry(
        HeaderHash::from(input.original_header_hash),
        &input.updated_habit,
    )?;

    let entry_hash = hash_entry(&input.updated_habit)?;

    let output = NewHabitOutput {
        header_hash: HeaderHashB64::from(header_hash),
        entry_hash: EntryHashB64::from(entry_hash),
    };

    Ok(output)
}

#[hdk_extern]
pub fn delete_habit(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
    delete_entry(HeaderHash::from(header_hash))
}
