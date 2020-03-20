// Hash Map for LHS to RHS

use cliftinstbuilder::{self, CtonInst, CtonInstKind, CtonOpcode, CtonOperand, CtonValueDef};
use std::collections::HashMap;

pub fn map_lhs_to_rhs(
    id: usize,
    rhs: Vec<CtonInst>,
    mut table: HashMap<usize, Vec<CtonInst>>,
) -> HashMap<usize, Vec<CtonInst>> {
    // Does hashmap already have the key
    if table.contains_key(&id) {
        // TODO: Do we want to compare the existing RHS entry with incoming argument rhs?
        // Ideally, for one hash_id, we should only have one unique RHS
        // for now, don't insert anything and return the table as it is
        table
    } else {
        table.insert(id, rhs);
        table
    }
}
