use std::collections::BTreeMap;

use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Burner {
    pub name: String,
    pub metadata: BTreeMap<String, String>,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Habit {
    pub name: String,
    pub timeframe: Timeframe,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeframe {
    pub start_time: String,
    pub end_time: String,
}
