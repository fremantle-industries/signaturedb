use datafusion::arrow::datatypes::Schema;

pub fn create(table_name: &str, schema: &Schema) {
    println!("create table table_name={}, schema={}", table_name, schema);
}
