use std::path::Path;

use failure::Fallible;
use rustfst::algorithms::top_sort;
use rustfst::DrawingConfig;

use crate::utils::generate_image;
use rustfst::fst_impls::VectorFst;
use rustfst::prelude::*;
use rustfst::semirings::TropicalWeight;
use rustfst::symt;
use rustfst::SymbolTable;
use std::rc::Rc;

pub fn generate_topsort_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt.clone());

    fst.add_states(7);

    fst.set_start(0)?;
    fst.set_final(1, 0.0)?;

    fst.emplace_arc(0, 0, 0, 0.0, 2)?;
    fst.emplace_arc(2, 1, 1, 0.0, 5)?;
    fst.emplace_arc(5, 2, 2, 0.0, 3)?;
    fst.emplace_arc(0, 3, 3, 0.0, 3)?;
    fst.emplace_arc(3, 4, 4, 0.0, 4)?;
    fst.emplace_arc(4, 5, 5, 0.0, 1)?;
    fst.emplace_arc(4, 6, 6, 0.0, 6)?;
    fst.emplace_arc(6, 7, 7, 0.0, 1)?;

    let path_images = path_images.as_ref();

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;
    config.show_weight_one = false;
    config.acceptor = true;

    generate_image(path_images, &fst, "topsort_in", &config)?;

    top_sort(&mut fst)?;

    generate_image(path_images, &fst, "topsort_out", &config)?;

    Ok(())
}
