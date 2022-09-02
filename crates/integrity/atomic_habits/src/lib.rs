//! ## hc_zome_atomic_habits_integrity
//!
//! Atomic habits tracking zome.
//!
//! If you need to manage atomic habits (burners, habits, daily progess etc.)
//! you can directly include this zome in your DNA.
//!

use hc_zome_atomic_habits_integrity_types::*;
use hdi::prelude::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Burner(Burner),
    Habit(Habit),
}

#[hdk_link_types]
pub enum LinkTypes {
    PrefixPath,
    PathToBurner,
    PathToHabit,
    AgentToBurner,
    AgentToHabit,
}
