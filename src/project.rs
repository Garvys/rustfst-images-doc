use std::path::Path;

use failure::Fallible;
use rustfst::DrawingConfig;
use rustfst::prelude::*;

use crate::fsts::fst_1;
use crate::utils::generate_image;

pub fn generate_project_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let fst = fst_1()?;

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;

    generate_image(path_images, &fst, "project_in", &config)?;
    {
        let mut fst_input_project = fst.clone();
        project(&mut fst_input_project, ProjectType::ProjectInput);
        generate_image(
            path_images,
            &fst_input_project,
            "project_out_project-input",
            &config,
        )?;
    }
    {
        let mut fst_output_project = fst.clone();
        project(&mut fst_output_project, ProjectType::ProjectOutput);
        generate_image(
            path_images,
            &fst_output_project,
            "project_out_project-output",
            &config,
        )?;
    }
    Ok(())
}
