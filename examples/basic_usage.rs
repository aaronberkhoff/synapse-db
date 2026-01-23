use synapse_db::{query::QueryEngine, storage::Storage};

fn main() {
    println!("Synapse DB - Basic Usage Example");

    let storage = Storage::new();
    let engine = QueryEngine::new();

    println!(
        "Storage initialized: {:?}",
        std::any::type_name_of_val(&storage)
    );
    println!(
        "Query engine initialized: {:?}",
        std::any::type_name_of_val(&engine)
    );

    // Example query execution
    match engine.execute("SELECT * FROM users") {
        Ok(_) => println!("Query executed successfully"),
        Err(e) => println!("Query failed: {}", e),
    }
}
