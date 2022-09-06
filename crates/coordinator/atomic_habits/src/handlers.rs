use hc_zome_atomic_habits_coordinator_types::UpdateBurnerInput;
use hc_zome_atomic_habits_integrity::*;
use hc_zome_atomic_habits_integrity_types::*;
use hdk::{
    hash_path::path::TypedPath,
    prelude::{holo_hash::ActionHash, *},
};

pub fn create_burner(burner: Burner) -> ExternResult<Record> {
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

    debug!("created record: {:?}", record);
    Ok(record)
}

pub fn update_burner(input: UpdateBurnerInput) -> ExternResult<Record> {
    let record = crate::get_my_burner(input.original_action_hash)?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner doesn't exist or isn't owned by you".into(),)
    ))?;
    let action_hash = update_entry(record.action_address().clone(), &input.updated_burner)?;

    let path = prefix_path(input.updated_burner.name.clone())?;

    path.ensure()?;

    let record = get(action_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Unreachable".into())))?;

    debug!("updated record: {:?}", record);
    Ok(record)
}

pub fn delete_burner(_original_action_hash: ActionHash) -> ExternResult<Option<ActionHash>> {
    unimplemented!()
}

pub fn get_my_burner(original_action_hash: ActionHash) -> ExternResult<Option<Record>> {
    let agent_address = agent_info()?.agent_initial_pubkey.clone();
    let links = get_links(agent_address, LinkTypes::AgentToBurner, None)?;
    // debug!("AgentToBurner links: {:?}", links);

    if links.len() == 0 {
        return Ok(None);
    }

    let filtered_links = links
        .into_iter()
        .filter(|link| {
            link.target.to_owned() == AnyLinkableHash::from(original_action_hash.to_owned())
        })
        .map(|link| get_latest(link.target.into()))
        .collect::<ExternResult<Vec<Record>>>()?;
    // debug!("Filtered links: {:?}", filtered_links);
    if filtered_links.len() == 0 {
        return Ok(None);
    }

    debug!("fetched record: {:?}", filtered_links[0].clone());
    Ok(Some(filtered_links[0].clone()))
}

pub fn get_my_burners() -> ExternResult<Vec<Record>> {
    let agent_address = agent_info()?.agent_initial_pubkey.clone();

    let get_links_input: GetLinksInput = GetLinksInput::new(
        AnyLinkableHash::from(agent_address.clone()),
        LinkTypes::AgentToBurner.try_into_filter()?,
        None,
    );

    let links = HDK
        .with(|h| h.borrow().get_links(vec![get_links_input]))?
        .into_iter()
        .flatten()
        .collect::<Vec<Link>>();

    let burners = links
        .into_iter()
        .map(|link| get_latest(link.target.into()))
        .collect::<ExternResult<Vec<Record>>>()?;

    Ok(burners)
}

pub fn _get_burners() -> ExternResult<Vec<Record>> {
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
