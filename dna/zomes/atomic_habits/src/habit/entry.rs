use std::collections::BTreeMap;

use hdk::prelude::*;

#[hdk_entry(id = "habit")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Habit {
    pub name: String,
    pub timeframe: Timeframe,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeframe {
    pub start_time: String,
    pub end_time: String,
}
