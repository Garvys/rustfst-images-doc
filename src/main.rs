use std::path::PathBuf;

use failure::Fallible;

pub mod project;
pub mod utils;

fn main() -> Fallible<()> {
    let path_crate = PathBuf::from(file!())
        .canonicalize()?
        .parent()
        .unwrap().parent().unwrap()
        .canonicalize()?;
    let path_images = path_crate.join("images");

    project::generate_project_images(&path_images)?;
    Ok(())
}
