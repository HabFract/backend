use hc_zome_atomic_habits_coordinator_types::{DeleteResponse, UpdateInput};
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

    debug!("_+_+_+_+_+_+_+_+_+_ Created record: {:#?}", record);
    Ok(record)
}

pub fn update_burner(input: UpdateInput<Burner>) -> ExternResult<Record> {
    let record = crate::get_my_burner(input.original_action_hash.clone())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner doesn't exist or isn't owned by you".into(),)
    ))?;
    let action_hash = update_entry(record.action_address().clone(), &input.updated)?;

    let path = prefix_path(input.updated.name.clone())?;
    path.ensure()?;

    // Delete agent link to stale header
    let existing_links = agent_to_burner_links()?.unwrap();
    let link_to_delete = existing_links
        .into_iter()
        .filter(|link| {
            link.target.clone() == AnyLinkableHash::from(input.original_action_hash.to_owned())
        })
        .map(|link| link.create_link_hash)
        .collect::<Vec<ActionHash>>();
    delete_link(link_to_delete[0].clone())?;

    // Create agent link to updated header
    let agent_address = agent_info()?.agent_initial_pubkey.clone();
    create_link(
        agent_address,
        action_hash.clone(),
        LinkTypes::AgentToBurner,
        (),
    )?;

    // Create anchor link to updated header
    create_link(
        path.path_entry_hash()?,
        action_hash.clone(),
        LinkTypes::PathToBurner,
        input.updated.name.as_bytes().to_vec(),
    )?;

    let record = get(action_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Unreachable".into())))?;

    debug!("_+_+_+_+_+_+_+_+_+_ Updated record: {:#?}", record);
    Ok(record)
}

pub fn delete_my_burner(original_action_hash: ActionHash) -> ExternResult<Option<DeleteResponse>> {
    crate::get_my_burner(original_action_hash.clone())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner doesn't exist or isn't owned by you".into(),)
    ))?;

    let input = DeleteInput {
        deletes_action_hash: original_action_hash.clone(),
        chain_top_ordering: ChainTopOrdering::Strict,
    };

    let action_hash = delete_entry(input)?;

    let response = DeleteResponse {
        delete_action_hash: action_hash,
        original_action_hash,
    };
    debug!("_+_+_+_+_+_+_+_+_+_  Delete response: {:#?}", response);
    Ok(Some(response))
}

pub fn get_my_live_burner_from_entry(
    original_entry_hash: AnyLinkableHash,
) -> ExternResult<Option<Entry>> {
    let details = get_details(original_entry_hash, GetOptions::latest())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Burner not found".into())
    ))?;

    let my_latest_burner_entry: Option<Entry> = match details {
        Details::Record(_) => {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "Malformed details".into()
            )))
        }
        Details::Entry(entry_details) => match entry_details.updates.last() {
            // TODO implement recursion
            Some(_update) => Some(entry_details.entry),
            None => Some(entry_details.entry),
        },
    };
    Ok(my_latest_burner_entry)
}

pub fn get_my_live_burners() -> ExternResult<Vec<Record>> {
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

    let maybe_entries = links
        .clone()
        .into_iter()
        .map(|link| get_latest(link.target.into()))
        .map(|latest_record| {
            let entry = latest_record
                .map(|record| {
                    let entry_hash = record
                        .signed_action
                        .hashed
                        .content
                        .entry_hash()
                        .map(|eh| eh.to_owned());
                    entry_hash
                })
                .expect("Only valid records mapped ");

            // Return an option of a live entry
            get(
                entry.unwrap(),
                GetOptions {
                    strategy: GetStrategy::Latest,
                },
            )
            .unwrap()
        })
        .collect::<Vec<Option<Record>>>();

    // debug!(
    //     "_+_+_+_+_+_+_+_+_+_ Fetched maybes: {:#?}",
    //     maybe_entries.clone()
    // );
    let entries = maybe_entries
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<Record>>();
    // debug!(
    //     "_+_+_+_+_+_+_+_+_+_ Fetched records: {:#?}",
    //     entries.clone()
    // );

    // TODO: fix edge case where an entry is recreated with the same hash. (Avoid recreating, just find the delete and remove?)
    match entries.len() {
        0 => return Ok(Vec::<Record>::new()),
        _ => return Ok(entries),
    }
}

pub fn get_my_burner(original_action_hash: ActionHash) -> ExternResult<Option<Record>> {
    let burner_entry_type: EntryType = UnitEntryTypes::Burner.try_into()?;

    // Filter source chain for entry type
    let filter = ChainQueryFilter::new()
        .entry_type(burner_entry_type)
        .include_entries(true);

    // Filter source chain for given action hash
    let burners = query(filter)?
        .into_iter()
        .filter(|record| {
            let action_hash = record.signed_action().hashed.hash.to_owned();
            action_hash == original_action_hash.to_owned()
        })
        .collect::<Vec<Record>>();

    debug!(
        "_+_+_+_+_+_+_+_+_+_ Fetched record: {:#?}",
        burners.last().clone()
    );
    match burners.last() {
        Some(record) => Ok(Some(record.to_owned())),
        None => Ok(None),
    }
}

pub fn _get_all_my_burners() -> ExternResult<Vec<Record>> {
    // TODO refactor this so it just queries the source chain of the current agent
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

    debug!(
        "_+_+_+_+_+_+_+_+_+_ My Latest burners record: {:#?}",
        burners.clone()
    );
    Ok(burners)
}

pub fn _get_burners() -> ExternResult<Vec<Record>> {
    let path = Path::from("all_burners").typed(LinkTypes::PrefixPath)?;

    let children = path.children_paths()?;
    // Get all paths that are linked from the anchor e.g. all_burners.abc

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

    // Get a vector of all links from each path
    let links = HDK
        .with(|h| h.borrow().get_links(get_links_input))?
        .into_iter()
        .flatten()
        .collect::<Vec<Link>>();

    // Get from the DHT the latest burner at the target of each link
    let burners = links
        .into_iter()
        .map(|link| get_latest(link.target.into()))
        .collect::<ExternResult<Vec<Record>>>()?;

    Ok(burners)
}

/** Private helpers */

fn agent_to_burner_links() -> ExternResult<Option<Vec<Link>>> {
    let agent_address = agent_info()?.agent_initial_pubkey.clone();
    let links = get_links(agent_address, LinkTypes::AgentToBurner, None)?;
    debug!("---- LINKS ---- {:#?}", links);
    if links.len() == 0 {
        return Ok(None);
    }
    Ok(Some(links))
}

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
