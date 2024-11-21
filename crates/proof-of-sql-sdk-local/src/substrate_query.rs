use crate::sxt_chain_runtime::api::runtime_types::{
    bounded_collections::bounded_vec::BoundedVec, sxt_core::tables::TableIdentifier,
};
use proof_of_sql_parser::{Identifier, ResourceId};

/// Convert PoSQL `Identifier` to SxT Core `BoundedVec<u8>`
fn identifier_to_byte_string(identifier: &Identifier) -> BoundedVec<u8> {
    BoundedVec::<u8>(identifier.as_str().to_uppercase().as_bytes().to_vec())
}

/// Convert PoSQL resource IDs to SxT Core table identifiers
pub fn resource_id_to_table_id(resource_id: &ResourceId) -> TableIdentifier {
    TableIdentifier {
        name: identifier_to_byte_string(&resource_id.object_name()),
        namespace: identifier_to_byte_string(&resource_id.schema()),
    }
}
