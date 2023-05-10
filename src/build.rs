use cornucopia::{CodegenSettings, Error};

// This script will generate a new cornucopia file every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), Error> {
    // Queries contains the accessors that give us our functions.
    // Schema describes our database.
    let queries_path = "queries";
    let schema_file = "schema.sql"; // Do we want to move this into src/datastructures?
    let destination = "src/queries.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        &[schema_file],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}