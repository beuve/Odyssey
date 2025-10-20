use serde::{Deserialize, Serialize};

use super::suitesparse::{cs_dis, css_init};

use super::cs::Cs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Css {
    pub q: Vec<i32>,
    pub lnz: f64,
    pub unz: f64,
}

impl Css {
    pub fn new(cs: &mut Cs) -> Option<Self> {
        let res;
        unsafe {
            let css = css_init(&cs.as_ffi());
            if css.is_null() {
                res = None;
            } else {
                res = Some(Css::from_ffi(css, cs.n));
            }
        }
        res
    }

    pub fn as_ffi(&mut self) -> cs_dis {
        cs_dis {
            pinv: std::ptr::null_mut(),
            q: self.q.as_mut_ptr(),
            parent: std::ptr::null_mut(),
            cp: std::ptr::null_mut(),
            leftmost: std::ptr::null_mut(),
            m2: 0i32,
            lnz: self.lnz,
            unz: self.unz,
        }
    }

    unsafe fn from_ffi(ffi: *mut cs_dis, n: usize) -> Self {
        Self {
            q: Vec::from_raw_parts((*ffi).q, n, n),
            lnz: (*ffi).lnz,
            unz: (*ffi).unz,
        }
    }
}
