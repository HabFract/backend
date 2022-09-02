use hdk::prelude::*;

mod burner;
mod habit;
mod utils;

use burner::Burner;
use habit::Habit;

entry_defs![
    Habit::entry_def(),
    Burner::entry_def(),
    PathEntry::entry_def()
];

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
