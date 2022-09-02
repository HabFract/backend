use std::collections::BTreeMap;

use super::Burner;
use crate::utils;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

enum BurnerLinkType {
    PathToBurner = 0,
    // AgentToBurner = 1,
}

impl From<BurnerLinkType> for LinkType {
    fn from(hdk_link_type: BurnerLinkType) -> Self {
        Self(hdk_link_type as u8)
    }
}

pub fn get_hash_from_link_target(target_hash: EntryHash) -> ExternResult<HeaderHashB64> {
    let get_input = GetInput::new(target_hash.into(), GetOptions::default());
    let get_output = HDK.with(|h| h.borrow().get_details(vec![get_input]))?;

    match get_output[0].clone() {
        Some(Details::Element(output)) => {
            return Ok(HeaderHashB64::new(output.element.signed_header.hashed.hash))
        }
        _ => {}
    }

    let header = create_entry(Burner {
        name: String::from("ok"),
        metadata: BTreeMap::new(),
    })?;
    Ok(HeaderHashB64::new(header))
}

#[hdk_extern]
pub fn create_burner(burner: Burner) -> ExternResult<GraphQLCreatePayload> {
    let header_hash = create_entry(&burner.clone())?;
    let entry_hash = hash_entry(&burner)?;

    // let path = prefix_path(burner.name.clone());
    let path = burners_path();
    path.ensure()?;
    // let agent_address = agent_info()?.agent_initial_pubkey.clone();

    create_link(
        path.path_entry_hash()?,
        entry_hash.clone(),
        BurnerLinkType::PathToBurner,
        (),
    )?;
    // create_link(
    //     agent_address,
    //     entry_hash.clone(),
    //     BurnerLinkType::AgentToBurner,
    //     ()
    // )?;

    let output = GraphQLCreatePayload {
        payload: NewBurnerOutput {
            header_hash: HeaderHashB64::from(header_hash),
            entry_hash: EntryHashB64::from(entry_hash),
        },
    };

    Ok(output)
}

#[hdk_extern]
pub fn get_burner(entry_hash: EntryHashB64) -> ExternResult<Option<Burner>> {
    let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

    match maybe_element {
        None => Ok(None),
        Some(element) => {
            let burner: Burner = element.entry().to_app_option()?.ok_or(WasmError::Guest(
                "Could not deserialize element to Burner.".into(),
            ))?;

            Ok(Some(burner))
        }
    }
}

#[hdk_extern]
pub fn get_all_burners(_: ()) -> ExternResult<Connection> {
    let path = burners_path();
    let children = get_links(path.path_entry_hash()?, None)?;

    let burners: Vec<Burner> = children
        .clone()
        .into_iter()
        .map(|link| get_burner_from_link_target(link.target.into()))
        .collect::<ExternResult<Vec<Burner>>>()?
        .into_iter()
        .collect();
    let ids: Vec<HeaderHashB64> = children
        .into_iter()
        .map(|link| get_hash_from_link_target(link.target.into()))
        .collect::<ExternResult<Vec<HeaderHashB64>>>()?
        .into_iter()
        .collect();
    debug!("{:?}", ids);
    let burner_connection = Connection::new(burners, ids);

    Ok(burner_connection)
}

#[hdk_extern]
pub fn update_burner(input: UpdateBurnerInput) -> ExternResult<NewBurnerOutput> {
    let header_hash = update_entry(
        HeaderHash::from(input.original_header_hash),
        &input.updated_burner,
    )?;

    let entry_hash = hash_entry(&input.updated_burner)?;

    let output = NewBurnerOutput {
        header_hash: HeaderHashB64::from(header_hash),
        entry_hash: EntryHashB64::from(entry_hash),
    };

    Ok(output)
}

#[hdk_extern]
pub fn delete_burner(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
    delete_entry(HeaderHash::from(header_hash))
}

/** Private helpers */

// fn prefix_path(name: String) -> Path {
//     // conver to lowercase for path for ease of search
//     let lower_name = name.to_lowercase();
//     let (prefix, _) = lower_name.as_str().split_at(3);

//     Path::from(format!("all_burners.{}", prefix))
// }

fn burners_path() -> Path {
    Path::from(format!("all_burners"))
}

fn get_burner_from_element(element: Element) -> ExternResult<Burner> {
    let burner: Burner = utils::try_from_element(element)?;
    Ok(burner)
}

fn get_burner_from_link_target(target_hash: EntryHash) -> ExternResult<Burner> {
    // let links = get_links(target_hash, None)?;
    // debug!("links: {:?}", links);

    // let get_input = links
    //     .into_iter()
    //     .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
    //     .collect();

    let get_input = GetInput::new(target_hash.into(), GetOptions::default());

    let get_output = HDK.with(|h| h.borrow().get(vec![get_input]))?;

    let burner = get_output
        .into_iter()
        .filter_map(|maybe_option| maybe_option)
        .map(get_burner_from_element)
        .next()
        .unwrap();
    return burner;
}

/** GraphQL Structs */
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLCreatePayload {
    payload: NewBurnerOutput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnerNode {
    pub id: HeaderHashB64,
    pub name: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewBurnerOutput {
    header_hash: HeaderHashB64,
    entry_hash: EntryHashB64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBurnerInput {
    original_header_hash: HeaderHashB64,
    updated_burner: Burner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    edges: Vec<Edge>,
    page_info: String,
}

impl Connection {
    pub fn new(burners: Vec<Burner>, ids: Vec<HeaderHashB64>) -> Connection {
        return Connection {
            edges: burners
                .into_iter()
                .enumerate()
                .map(|(i, b)| {
                    Edge::new(BurnerNode {
                        id: ids[i].clone(),
                        name: b.name.clone(),
                        metadata: b.metadata.clone(),
                    })
                })
                .collect(),
            page_info: String::from("NOT IMPLEMENTED"),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    cursor: String,
    node: BurnerNode,
}

impl Edge {
    pub fn new(burner: BurnerNode) -> Edge {
        return Edge {
            cursor: String::from("NOT IMPLEMENTED"),
            node: burner,
        };
    }
}
