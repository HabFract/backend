use hdk::prelude::{holo_hash::ActionHash, *};

mod handlers;

use hc_zome_atomic_habits_coordinator_types::UpdateBurnerInput;
use hc_zome_atomic_habits_integrity_types::*;

/// Creates a burner for the agent executing this call.
#[hdk_extern]
pub fn create_burner(burner: Burner) -> ExternResult<Record> {
    handlers::create_burner(burner)
}

/// Updates a specific burner for an agent, if it exists for them.
#[hdk_extern]
pub fn update_burner(input: UpdateBurnerInput) -> ExternResult<Record> {
    let updated_burner = handlers::update_burner(input)?;
    Ok(updated_burner)
}

/// Deletes a specific burner, returning the delete action hash.
#[hdk_extern]
pub fn delete_my_burner(original_hash: ActionHash) -> ExternResult<Option<ActionHash>> {
    let burner_deletion = handlers::delete_my_burner(original_hash)?;
    Ok(burner_deletion)
}

/// Returns a specific burner for an agent, if it exists for them.
#[hdk_extern]
pub fn get_my_burner(original_hash: ActionHash) -> ExternResult<Option<Record>> {
    let burner = handlers::get_my_burner(original_hash)?;
    Ok(burner) // Ok boomer...
}

/// Returns the burners for the given agent.
#[hdk_extern]
pub fn get_my_burners(_: ()) -> ExternResult<Vec<Record>> {
    let burners = handlers::get_my_burners()?;
    Ok(burners)
}
