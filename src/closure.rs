use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::{closure, ClosureType};
use rustfst::DrawingConfig;

use crate::fsts::fst_002;
use crate::utils::generate_image;

pub fn generate_closure_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let fst = fst_002()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst, "closure_in", &config)?;

    {
        let mut fst_copy = fst.clone();
        closure(&mut fst_copy, ClosureType::ClosureStar);

        generate_image(path_images, &fst_copy, "closure_out_closure_star", &config)?;
    }

    {
        let mut fst_copy = fst.clone();
        closure(&mut fst_copy, ClosureType::ClosurePlus);

        generate_image(path_images, &fst_copy, "closure_out_closure_plus", &config)?;
    }

    Ok(())
}
