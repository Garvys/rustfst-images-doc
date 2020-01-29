use std::path::Path;

use failure::Fallible;
use rustfst::DrawingConfig;

use crate::utils::generate_image;
use rustfst::algorithms::replace;
use rustfst::fst;
use rustfst::fst_impls::VectorFst;
use rustfst::fst_traits::MutableFst;
use rustfst::prelude::*;
use rustfst::semirings::TropicalWeight;
use rustfst::symt;
use rustfst::utils::acceptor;
use rustfst::SymbolTable;
use std::rc::Rc;

type Weight = TropicalWeight;

pub fn fsts_replace() -> Fallible<(
    VectorFst<Weight>,
    VectorFst<Weight>,
    VectorFst<Weight>,
    VectorFst<Weight>,
    VectorFst<Weight>,
)> {
    let symt = symt![
        "call",
        "#NAME",
        "#FIRST_NAME",
        "#LAST_NAME",
        "now",
        "david",
        "john",
        "bowie",
        "williams"
    ];
    let symt = Rc::new(symt);

    // call #NAME now
    let mut fst_1: VectorFst<Weight> = fst![1, 2, 5];
    fst_1.set_input_symbols(symt.clone());
    fst_1.set_output_symbols(symt.clone());

    // #FIRST_NAME #LAST_NAME
    let mut fst_2: VectorFst<Weight> = fst![3, 4];
    fst_2.set_input_symbols(symt.clone());
    fst_2.set_output_symbols(symt.clone());

    let mut fst_3 = VectorFst::<Weight>::new();
    fst_3.add_states(2);
    fst_3.set_start(0)?;
    fst_3.set_final(1, 0.0)?;
    fst_3.emplace_arc(0, 6, 6, 0.0, 1)?;
    fst_3.emplace_arc(0, 7, 7, 0.0, 1)?;
    fst_3.set_input_symbols(symt.clone());
    fst_3.set_output_symbols(symt.clone());

    let mut fst_4 = VectorFst::<Weight>::new();
    fst_4.add_states(2);
    fst_4.set_start(0)?;
    fst_4.set_final(1, 0.0)?;
    fst_4.emplace_arc(0, 8, 8, 0.0, 1)?;
    fst_4.emplace_arc(0, 9, 9, 0.0, 1)?;
    fst_4.set_input_symbols(symt.clone());
    fst_4.set_output_symbols(symt.clone());

    let fst_list = vec![
        (10, fst_1.clone()),
        (2, fst_2.clone()),
        (3, fst_3.clone()),
        (4, fst_4.clone()),
    ];
    let mut fst_5: VectorFst<Weight> = replace(fst_list, 10, false)?;

    // TODO: Shouldn't need that
    fst_5.set_input_symbols(symt.clone());
    fst_5.set_output_symbols(symt.clone());

    Ok((fst_1, fst_2, fst_3, fst_4, fst_5))
}

pub fn generate_replace_images<P: AsRef<Path>>(path_images: P) -> Fallible<()> {
    let path_images = path_images.as_ref();

    let mut config = DrawingConfig::default();
    config.portrait = true;
    config.vertical = false;
    config.show_weight_one = false;

    let (fst_1, fst_2, fst_3, fst_4, fst_5) = fsts_replace()?;

    generate_image(path_images, &fst_1, "replace_in_1", &config)?;
    generate_image(path_images, &fst_2, "replace_in_2", &config)?;
    generate_image(path_images, &fst_3, "replace_in_3", &config)?;
    generate_image(path_images, &fst_4, "replace_in_4", &config)?;

    generate_image(path_images, &fst_5, "replace_out", &config)?;

    Ok(())
}
