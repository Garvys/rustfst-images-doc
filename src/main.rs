use std::path::PathBuf;

use failure::Fallible;

pub mod closure;
pub mod fsts;
pub mod invert;
pub mod project;
pub mod utils;

fn main() -> Fallible<()> {
    let path_crate = PathBuf::from(file!())
        .canonicalize()?
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .canonicalize()?;
    let path_images = path_crate.join("images");

    project::generate_project_images(&path_images)?;
    invert::generate_invert_images(&path_images)?;
    closure::generate_closure_images(&path_images)?;

    Ok(())
}
