use super::Habit;
use crate::utils;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

enum HabitLinkType {
    PathToHabit = 0,
    // AgentToHabit = 1,
}

impl From<HabitLinkType> for LinkType {
    fn from(hdk_link_type: HabitLinkType) -> Self {
        Self(hdk_link_type as u8)
    }
}

#[hdk_extern]
pub fn create_habit(habit: Habit) -> ExternResult<GraphQLCreatePayload> {
    let header_hash = create_entry(&habit.clone())?;
    let entry_hash = hash_entry(&habit)?;

    // let path = prefix_path(habit.name.clone());
    let path = habits_path();
    path.ensure()?;
    // let agent_address = agent_info()?.agent_initial_pubkey.clone();

    create_link(
        path.path_entry_hash()?,
        entry_hash.clone(),
        HabitLinkType::PathToHabit,
        (),
    )?;
    // create_link(
    //     agent_address,
    //     entry_hash.clone(),
    //     HabitLinkType::AgentToHabit,
    //     ()
    // )?;

    let output = GraphQLCreatePayload {
        payload: NewHabitOutput {
            header_hash: HeaderHashB64::from(header_hash),
            entry_hash: EntryHashB64::from(entry_hash),
        },
    };

    Ok(output)
}

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

#[hdk_extern]
pub fn get_all_habits(_: ()) -> ExternResult<Connection> {
    let path = habits_path();
    let children = get_links(path.path_entry_hash()?, None)?;

    let habits: Vec<Habit> = children
        .into_iter()
        .map(|link| get_habit_from_link_target(link.target.into()))
        .collect::<ExternResult<Vec<Habit>>>()?
        .into_iter()
        .collect();
    let habit_connection = Connection::new(habits);

    Ok(habit_connection)
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

/** Private helpers */

// fn prefix_path(name: String) -> Path {
//     // conver to lowercase for path for ease of search
//     let lower_name = name.to_lowercase();
//     let (prefix, _) = lower_name.as_str().split_at(3);

//     Path::from(format!("all_habits.{}", prefix))
// }

fn habits_path() -> Path {
    Path::from(format!("all_habits"))
}

fn get_habit_from_element(element: Element) -> ExternResult<Habit> {
    let habit: Habit = utils::try_from_element(element)?;
    Ok(habit)
}

fn get_habit_from_link_target(target_hash: EntryHash) -> ExternResult<Habit> {
    // let links = get_links(target_hash, None)?;
    // debug!("links: {:?}", links);

    // let get_input = links
    //     .into_iter()
    //     .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
    //     .collect();

    let get_input = GetInput::new(target_hash.into(), GetOptions::default());

    let get_output = HDK.with(|h| h.borrow().get(vec![get_input]))?;

    let habit = get_output
        .into_iter()
        .filter_map(|maybe_option| maybe_option)
        .map(get_habit_from_element)
        .next()
        .unwrap();
    return habit;
}

/** GraphQL Structs */
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLCreatePayload {
    payload: NewHabitOutput,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewHabitOutput {
    header_hash: HeaderHashB64,
    entry_hash: EntryHashB64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitInput {
    original_header_hash: HeaderHashB64,
    updated_habit: Habit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    edges: Vec<Edge>,
    page_info: String,
}

impl Connection {
    pub fn new(habits: Vec<Habit>) -> Connection {
        return Connection {
            edges: habits.into_iter().map(|h| Edge::new(h)).collect(),
            page_info: String::from("NOT IMPLEMENTED"),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    cursor: String,
    node: Habit,
}

impl Edge {
    pub fn new(habit: Habit) -> Edge {
        return Edge {
            cursor: String::from("NOT IMPLEMENTED"),
            node: habit,
        };
    }
}
