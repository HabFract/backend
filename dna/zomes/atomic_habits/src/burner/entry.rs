use std::collections::BTreeMap;

use hdk::prelude::*;

#[hdk_entry(id = "burner")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Burner {
    pub name: String,
    pub metadata: BTreeMap<String, String>,
}
