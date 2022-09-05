use hdk::prelude::*;
use std::collections::BTreeMap;

use hc_zome_atomic_habits_integrity_types::*;

// GraphQL Types
#[derive(Debug, Serialize, Deserialize)]
pub struct Connection<T> {
    edges: Vec<Edge<T>>,
    page_info: String,
}

impl<T> Connection<T> {
    pub fn new(items: Vec<T>) -> Connection<Node<T>> {
        return Connection {
            edges: items
                .into_iter()
                .enumerate()
                .map(|(_, i)| Edge::new(Node(i)))
                .collect(),
            page_info: String::from("NOT IMPLEMENTED"),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge<T> {
    cursor: String,
    node: T,
}

impl<T> Edge<T> {
    pub fn new(node: T) -> Edge<T> {
        return Edge {
            cursor: String::from("NOT IMPLEMENTED"),
            node,
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<T>(pub T);

//legacy
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLCreatePayload {
    payload: NewBurnerOutput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BurnerNode {
    pub name: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewBurnerOutput {
    header_hash: String,
    entry_hash: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBurnerInput {
    pub original_header_hash: String,
    pub updated_burner: Burner,
}
