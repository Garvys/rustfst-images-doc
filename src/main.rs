use std::path::PathBuf;

use failure::Fallible;

pub mod closure;
pub mod concat;
pub mod connect;
pub mod fsts;
pub mod invert;
pub mod project;
pub mod replace;
pub mod reverse;
pub mod shortest_path;
pub mod topsort;
pub mod union;
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
    connect::generate_connect_images(&path_images)?;
    union::generate_union_images(&path_images)?;
    concat::generate_concat_images(&path_images)?;
    replace::generate_replace_images(&path_images)?;
    reverse::generate_reverse_images(&path_images)?;
    shortest_path::generate_shortestpath_images(&path_images)?;
    topsort::generate_topsort_images(&path_images)?;

    Ok(())
}
