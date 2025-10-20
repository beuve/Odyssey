use serde::{Deserialize, Serialize};

use super::suitesparse::cs_di;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cs {
    pub nzmax: usize,
    pub m: usize,
    pub n: usize,
    pub p: Vec<i32>,
    pub i: Vec<i32>,
    pub x: Vec<f64>,
}

impl Cs {
    pub fn new(m: usize, n: usize, p: Vec<i32>, i: Vec<i32>, x: Vec<f64>) -> Self {
        Cs {
            nzmax: x.len(),
            m,
            n,
            p,
            i,
            x,
        }
    }

    pub fn as_ffi(&mut self) -> cs_di {
        cs_di {
            m: self.m as i32,
            n: self.n as i32,
            nz: -1i32,
            p: self.p.as_mut_ptr(),
            i: self.i.as_mut_ptr(),
            x: self.x.as_mut_ptr(),
            nzmax: self.nzmax as i32,
        }
    }

    pub unsafe fn from_ffi(ffi: *mut cs_di) -> Self {
        Self {
            m: (*ffi).m as usize,
            n: (*ffi).n as usize,
            p: Vec::from_raw_parts((*ffi).p, (*ffi).n as usize + 1, (*ffi).nzmax as usize),
            i: Vec::from_raw_parts((*ffi).i, (*ffi).nzmax as usize, (*ffi).nzmax as usize),
            x: Vec::from_raw_parts((*ffi).x, (*ffi).nzmax as usize, (*ffi).nzmax as usize),
            nzmax: (*ffi).nzmax as usize,
        }
    }
}
