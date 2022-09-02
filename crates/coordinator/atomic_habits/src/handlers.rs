use hc_zome_atomic_habits_coordinator_types::{Connection, Edge, Node, UpdateBurnerInput};
use hc_zome_atomic_habits_integrity_types::*;
use hdk::prelude::*;

pub fn create_burner(_burner: Burner) -> ExternResult<Edge<Node<Record>>> {
    unimplemented!()
}

pub fn update_burner(_input: UpdateBurnerInput) -> ExternResult<Edge<Node<Record>>> {
    unimplemented!()
}

pub fn delete_burner(_header_hash: String) -> ExternResult<Option<Edge<String>>> {
    unimplemented!()
}

pub fn get_burner(_entry_hash: String) -> ExternResult<Option<Edge<Node<Record>>>> {
    unimplemented!()
}

pub fn get_all_burners() -> ExternResult<Connection<Record>> {
    unimplemented!()
}
