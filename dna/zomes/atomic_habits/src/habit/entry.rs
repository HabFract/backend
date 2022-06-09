use hdk::prelude::*;





#[hdk_entry(id = "habit")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Habit {
  pub name: String,
  pub timeframe: String,
  pub habit_metadata: String,
}