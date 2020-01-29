use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::reverse;
use rustfst::fst_impls::VectorFst;
use rustfst::DrawingConfig;

use crate::fsts::fst_001;
use crate::utils::generate_image;

pub fn generate_reverse_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let fst = fst_001()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst, "reverse_in", &config)?;

    let fst_reversed: VectorFst<_> = reverse(&fst)?;

    // TODO: Add call to rm_epsilon to have a better looking fst

    generate_image(path_images, &fst_reversed, "reverse_out", &config)?;

    Ok(())
}
