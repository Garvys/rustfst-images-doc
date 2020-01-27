use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::connect;
use rustfst::DrawingConfig;

use crate::fsts::fst_003;
use crate::utils::generate_image;

pub fn generate_connect_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let mut fst = fst_003()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst, "connect_in", &config)?;

    connect(&mut fst)?;

    generate_image(path_images, &fst, "connect_out", &config)?;

    Ok(())
}
