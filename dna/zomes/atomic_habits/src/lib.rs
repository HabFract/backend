use hdk::prelude::*;

mod habit;

use habit::Habit;

entry_defs![Habit::entry_def(), PathEntry::entry_def()];

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
