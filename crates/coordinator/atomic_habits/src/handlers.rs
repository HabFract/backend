use hc_zome_atomic_habits_coordinator_types::{Connection, Edge, Node, UpdateBurnerInput};
use hc_zome_atomic_habits_integrity::*;
use hc_zome_atomic_habits_integrity_types::*;
use hdk::{hash_path::path::TypedPath, prelude::*};

pub fn create_burner(burner: Burner) -> ExternResult<Node<Record>> {
    let agent_info = agent_info()?;

    let action_hash = create_entry(EntryTypes::Burner(burner.clone()))?;

    let path = prefix_path(burner.name.clone())?;

    path.ensure()?;

    let agent_address = agent_info.agent_initial_pubkey.clone();

    create_link(
        path.path_entry_hash()?,
        action_hash.clone(),
        LinkTypes::PathToBurner,
        burner.name.as_bytes().to_vec(),
    )?;
    create_link(
        agent_address,
        action_hash.clone(),
        LinkTypes::AgentToBurner,
        (),
    )?;

    let record = get(action_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Unreachable".into())))?;

    Ok(Node(record))
}

pub fn update_burner(_input: UpdateBurnerInput) -> ExternResult<Edge<Node<Record>>> {
    unimplemented!()
}

pub fn delete_burner(_header_hash: String) -> ExternResult<Option<Edge<String>>> {
    unimplemented!()
}

pub fn get_burner(_entry_hash: String) -> ExternResult<Option<Edge<Node<Record>>>> {
    unimplemented!()
}

pub fn get_all_burners() -> ExternResult<Connection<Record>> {
    unimplemented!()
}

/** Private helpers */

fn prefix_path(name: String) -> ExternResult<TypedPath> {
    // convert to lowercase for path for ease of search
    let lower_name = name.to_lowercase();
    let (prefix, _) = lower_name.as_str().split_at(3);

    Path::from(format!("all_burners.{}", prefix)).typed(LinkTypes::PrefixPath)
}
