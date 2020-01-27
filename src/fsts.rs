use std::rc::Rc;

use failure::Fallible;
use rustfst::prelude::*;
use rustfst::symt;
use rustfst::SymbolTable;

// Used for invert, project
pub fn fst_001() -> Fallible<VectorFst<TropicalWeight>> {
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt);

    fst.add_states(4);

    fst.emplace_arc(0, 1, 2, 1.2, 1)?;
    fst.emplace_arc(1, 3, 4, 1.2, 2)?;
    fst.emplace_arc(2, 5, 6, 1.2, 3)?;

    fst.emplace_arc(1, 7, 8, 1.2, 1)?;
    fst.emplace_arc(0, 9, 10, 1.2, 1)?;

    fst.set_start(0)?;
    fst.set_final(3, 0.2)?;
    Ok(fst)
}

// Used for closure
pub fn fst_002() -> Fallible<VectorFst<TropicalWeight>> {
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt);

    fst.add_states(2);

    fst.set_start(0)?;
    fst.set_final(1, 1.3)?;

    fst.emplace_arc(0, 1, 2, 2.3, 0)?;
    fst.emplace_arc(0, 3, 4, 1.1, 1)?;
    fst.emplace_arc(1, 5, 6, 3.4, 1)?;

    Ok(fst)
}

// Used for connect
pub fn fst_003() -> Fallible<VectorFst<TropicalWeight>> {
    let mut fst = VectorFst::<TropicalWeight>::new();
    let symt = symt!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];
    let symt = Rc::new(symt);

    fst.set_input_symbols(symt.clone());
    fst.set_output_symbols(symt);

    fst.add_states(6);

    fst.set_start(0)?;
    fst.set_final(1, 1.2)?;

    fst.emplace_arc(5, 1, 2, 2.1, 0)?;
    fst.emplace_arc(0, 3, 4, 1.3, 1)?;
    fst.emplace_arc(1, 5, 6, 2.6, 2)?;
    fst.emplace_arc(2, 7, 8, 0.3, 3)?;
    fst.emplace_arc(3, 9, 10, 0.7, 4)?;
    fst.emplace_arc(3, 11, 12, 1.8, 0)?;

    Ok(fst)
}
