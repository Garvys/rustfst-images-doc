use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::concat;
use rustfst::DrawingConfig;

use crate::fsts::{fst_001, fst_002};
use crate::utils::generate_image;

pub fn generate_concat_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let mut fst_1 = fst_001()?;
    let fst_2 = fst_002()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst_1, "concat_in_1", &config)?;
    generate_image(path_images, &fst_2, "concat_in_2", &config)?;

    concat(&mut fst_1, &fst_2)?;

    generate_image(path_images, &fst_1, "concat_out", &config)?;

    Ok(())
}
