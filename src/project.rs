use std::path::Path;
use std::rc::Rc;

use failure::Fallible;
use rustfst::algorithms::{project, ProjectType};
use rustfst::Arc;
use rustfst::DrawingConfig;
use rustfst::fst_impls::VectorFst;
use rustfst::fst_traits::MutableFst;
use rustfst::semirings::{Semiring, TropicalWeight};
use rustfst::SymbolTable;
use rustfst::symt;

use crate::utils::generate_image;

pub fn generate_project_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt);

    fst.add_states(4);

    fst.add_arc(0, Arc::new(1, 2, TropicalWeight::new(1.2), 1))?;
    fst.add_arc(1, Arc::new(3, 4, TropicalWeight::new(1.2), 2))?;
    fst.add_arc(2, Arc::new(5, 6, TropicalWeight::new(1.2), 3))?;

    fst.add_arc(1, Arc::new(7, 8, TropicalWeight::new(1.2), 1))?;
    fst.add_arc(0, Arc::new(9, 10, TropicalWeight::new(1.2), 1))?;

    fst.set_start(0)?;
    fst.set_final(3, TropicalWeight::new(0.2))?;

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