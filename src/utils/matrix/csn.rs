use serde::{Deserialize, Serialize};

use super::{
    cs::Cs,
    css::Css,
    suitesparse::{cs_din, csn_init},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Csn {
    pub l: Cs,
    pub u: Cs,
    pub pinv: Vec<i32>,
}

impl Csn {
    pub fn new(cs: &mut Cs, css: &mut Css) -> Option<Self> {
        let res;
        unsafe {
            let csn = csn_init(&cs.as_ffi(), &css.as_ffi());
            if csn.is_null() {
                res = None;
            } else {
                res = Some(Csn::from_ffi(csn, cs.n));
            }
        }
        res
    }

    unsafe fn from_ffi(ffi: *mut cs_din, n: usize) -> Csn {
        Csn {
            l: if !(*ffi).L.is_null() {
                Cs::from_ffi((*ffi).L)
            } else {
                panic!("Error");
            },
            u: if !(*ffi).U.is_null() {
                Cs::from_ffi((*ffi).U)
            } else {
                panic!("Error");
            },
            pinv: Vec::from_raw_parts((*ffi).pinv, n, n),
        }
    }
}
