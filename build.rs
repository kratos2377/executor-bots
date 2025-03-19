
use std::{collections::HashMap, path::Path};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?.canonicalize()?;

    let idl_source_path = current_dir.join("idls/vortex_contracts.json");
    let idl_mod_path = current_dir.join("src/vortex_idl.rs");
   // generate_idl_types(&idl_source_path, idl_mod_path.as_path())?;


    Ok(())
}

// fn generate_idl_types(
//     idl_source_path: &Path,
//     idl_mod_path: &Path,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let idl_mod_rs = vortex_idl_gen::generate_rust_types(&idl_source_path)
//         .map_err(|err| format!("generating IDL failed: {err:?}"))?;

//     std::fs::write(&idl_mod_path, idl_mod_rs)?;
//     Ok(())
// }