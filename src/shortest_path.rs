use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::shortest_path;
use rustfst::DrawingConfig;

use crate::fsts::fst_004;
use crate::utils::generate_image;
use rustfst::fst_impls::VectorFst;

pub fn generate_shortestpath_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let fst = fst_004()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;
    config.acceptor = true;

    generate_image(path_images, &fst, "shortestpath_in", &config)?;

    let fst_n_1: VectorFst<_> = shortest_path(&fst, 1, true)?;

    generate_image(path_images, &fst_n_1, "shortestpath_out_n_1", &config)?;

    let fst_n_2: VectorFst<_> = shortest_path(&fst, 2, true)?;

    generate_image(path_images, &fst_n_2, "shortestpath_out_n_2", &config)?;

    Ok(())
}
