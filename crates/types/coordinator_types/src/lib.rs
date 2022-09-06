use hc_zome_atomic_habits_integrity_types::*;
use hdk::prelude::{holo_hash::ActionHash, *};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBurnerInput {
    pub original_action_hash: ActionHash,
    pub updated_burner: Burner,
}
