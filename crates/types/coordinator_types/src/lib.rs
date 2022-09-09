use hdk::prelude::{holo_hash::ActionHash, *};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInput<T> {
    pub original_action_hash: ActionHash,
    pub updated: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    pub delete_action_hash: ActionHash,
    pub original_action_hash: ActionHash,
}
