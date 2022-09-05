use hdk::prelude::*;

mod handlers;

use hc_zome_atomic_habits_coordinator_types::{Node, UpdateBurnerInput};
use hc_zome_atomic_habits_integrity_types::*;

/// Creates a burner for the agent executing this call.
#[hdk_extern]
pub fn create_burner(burner: Burner) -> ExternResult<Node<Record>> {
    handlers::create_burner(burner)
}

/// Updates a specific burner.
#[hdk_extern]
pub fn update_burner(input: UpdateBurnerInput) -> ExternResult<Record> {
    handlers::update_burner(input)
}

/// Deletes a specific burner, returning.
#[hdk_extern]
pub fn delete_burner(header_hash: String) -> ExternResult<Option<String>> {
    let burner_deletion = handlers::delete_burner(header_hash)?;
    Ok(burner_deletion)
}

/// Returns a specific burner, if it exists.
#[hdk_extern]
pub fn get_burner(entry_hash: String) -> ExternResult<Option<Record>> {
    let burner = handlers::get_burner(entry_hash)?;
    Ok(burner) // Ok boomer...
}

/// Returns the burners for the given agent.
#[hdk_extern]
pub fn get_all_burners(_: ()) -> ExternResult<Vec<Record>> {
    let burners = handlers::get_all_burners()?;
    Ok(burners)
}
