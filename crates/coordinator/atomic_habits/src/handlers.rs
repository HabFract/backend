use hc_zome_atomic_habits_coordinator_types::{Node, UpdateBurnerInput};
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

pub fn update_burner(input: UpdateBurnerInput) -> ExternResult<Record> {
    debug!("{:?}", input);
    let record = crate::get_burner(input.original_header_hash)?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner doesn't exist".into(),)
    ))?;
    debug!("{:?}", record);
    let action_hash = update_entry(record.action_address().clone(), &input.updated_burner)?;

    let path = prefix_path(input.updated_burner.name.clone())?;

    path.ensure()?;

    let record = get(action_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Unreachable".into())))?;

    Ok(record)
}

pub fn delete_burner(_header_hash: String) -> ExternResult<Option<String>> {
    unimplemented!()
}

pub fn get_burner(_entry_hash: String) -> ExternResult<Option<Record>> {
    unimplemented!()
}

pub fn get_all_burners() -> ExternResult<Vec<Record>> {
    let path = Path::from("all_burners").typed(LinkTypes::PrefixPath)?;

    let children = path.children_paths()?;

    let get_links_input: Vec<GetLinksInput> = children
        .into_iter()
        .map(|path| {
            Ok(GetLinksInput::new(
                path.path_entry_hash()?.into(),
                LinkTypes::PathToBurner.try_into_filter()?,
                None,
            ))
        })
        .collect::<ExternResult<Vec<GetLinksInput>>>()?;

    let links = HDK
        .with(|h| h.borrow().get_links(get_links_input))?
        .into_iter()
        .flatten()
        .collect::<Vec<Link>>();

    let burners = links
        .into_iter()
        .map(|link| get_latest(link.target.into()))
        .collect::<ExternResult<Vec<Record>>>()?;

    Ok(burners)
}

/** Private helpers */

fn prefix_path(name: String) -> ExternResult<TypedPath> {
    // convert to lowercase for path for ease of search
    let lower_name = name.to_lowercase();
    let (prefix, _) = lower_name.as_str().split_at(3);

    Path::from(format!("all_burners.{}", prefix)).typed(LinkTypes::PrefixPath)
}

fn get_latest(action_hash: ActionHash) -> ExternResult<Record> {
    let details = get_details(action_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner not found".into())
    ))?;

    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(element_details) => match element_details.updates.last() {
            Some(update) => get_latest(update.action_address().clone()),
            None => Ok(element_details.record),
        },
    }
}
