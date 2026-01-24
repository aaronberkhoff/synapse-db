//! Binary for generating Python type stubs (.pyi files).
//!
//! Run with: `cargo run --bin stub_gen --features python`
//!
//! This generates the type stubs in `python/synapse_db/synapse_db.pyi`

use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // Get the stub info from the library
    let stub = synapse_db::stub_info()?;

    // Generate the stub file
    stub.generate()?;

    println!("Successfully generated Python type stubs!");
    Ok(())
}
