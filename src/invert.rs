use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::invert;
use rustfst::DrawingConfig;

use crate::fsts::fst_1;
use crate::utils::generate_image;

pub fn generate_invert_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let mut fst = fst_1()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst, "invert_in", &config)?;

    invert(&mut fst);

    generate_image(path_images, &fst, "invert_out", &config)?;

    Ok(())
}
