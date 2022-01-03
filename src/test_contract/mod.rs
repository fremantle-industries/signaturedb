use std::sync::Arc;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use crate::storage;

pub fn init() {
    let balance_schema = Arc::new(Schema::new(vec![
        // TODO: How to make this like Ethereum address type u256?
        Field::new("address", DataType::Int64, false),
        Field::new("value", DataType::Decimal(10, 2), false),
    ]));
    storage::create("balances", &balance_schema);
}
