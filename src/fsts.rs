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

    fst.add_arc(0, Arc::new(1, 2, TropicalWeight::new(1.2), 1))?;
    fst.add_arc(1, Arc::new(3, 4, TropicalWeight::new(1.2), 2))?;
    fst.add_arc(2, Arc::new(5, 6, TropicalWeight::new(1.2), 3))?;

    fst.add_arc(1, Arc::new(7, 8, TropicalWeight::new(1.2), 1))?;
    fst.add_arc(0, Arc::new(9, 10, TropicalWeight::new(1.2), 1))?;

    fst.set_start(0)?;
    fst.set_final(3, TropicalWeight::new(0.2))?;
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
    fst.set_final(1, TropicalWeight::new(1.3))?;

    fst.add_arc(0, Arc::new(1, 2, TropicalWeight::new(2.3), 0))?;
    fst.add_arc(0, Arc::new(3, 4, TropicalWeight::new(1.1), 1))?;
    fst.add_arc(1, Arc::new(5, 6, TropicalWeight::new(3.4), 1))?;

    Ok(fst)
}
