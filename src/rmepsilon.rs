use std::path::Path;
use std::rc::Rc;

use failure::Fallible;
use rustfst::DrawingConfig;
use rustfst::fst_impls::VectorFst;
use rustfst::prelude::*;
use rustfst::semirings::TropicalWeight;
use rustfst::SymbolTable;
use rustfst::symt;

use crate::utils::generate_image;

pub fn generate_rmepsilon_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt.clone());

    fst.add_states(4);

    fst.set_start(0)?;
    fst.set_final(3, 2.3)?;

    fst.emplace_arc(0, 0, 0, 1.0, 1)?;
    fst.emplace_arc(1, 1, 0, 2.0, 2)?;
    fst.emplace_arc(1, 0, 2, 3.0, 2)?;
    fst.emplace_arc(1, 0, 0, 4.0, 2)?;
    fst.emplace_arc(2, 0, 0, 5.0, 2)?;
    fst.emplace_arc(2, 0, 0, 6.0, 3)?;

    let path_images = path_images.as_ref();

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;
    config.show_weight_one = false;

    generate_image(path_images, &fst, "rmepsilon_in", &config)?;

    rm_epsilon(&mut fst)?;

    generate_image(path_images, &fst, "rmepsilon_out", &config)?;

    Ok(())
}
