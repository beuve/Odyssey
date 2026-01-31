use serde::{Deserialize, Serialize};

use super::{
    cs::Cs,
    css::Css,
    suitesparse::{cs_din, csn_init},
};

/// Numeric factorization of a sparse matrix.
///
/// This struct represents the **numeric phase** of a sparse
/// LU factorization as performed by SuiteSparse / CSparse.
///
/// The factor matrices `l` and `u` are stored as sparse matrices
/// in CSC format.
///
/// Corresponds to `csn` in CSparse.
#[derive(Serialize, Deserialize, Debug)]
pub struct Csn {
    /// Lower-triangular factor `L`.
    ///
    /// Stored in CSC format. The unit diagonal is typically
    /// implicit, depending on the factorization routine used.
    pub l: Cs,

    /// Upper-triangular factor `U`.
    ///
    /// Stored in CSC format and contains the numerical values
    /// produced by the factorization.
    pub u: Cs,

    /// Inverse row permutation used during factorization.
    ///
    /// This maps permuted rows back to their original positions:
    /// `pinv[p[k]] = k`.
    pub pinv: Vec<i32>,
}

impl Csn {
    /// Creates a new sparse matrix in CSC format.
    /// Only basics checks are performed. The caller is responsible
    /// for providing a valid CSC representation.
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

    /// Takes ownership of a `csn_di` allocated by SuiteSparse / CSparse
    /// and converts it into a safe Rust [Csn].
    ///
    /// This function assumes that the memory referenced by `ffi`
    /// was allocated using the C allocator and is exclusively owned
    /// by the caller.
    ///
    /// # Safety
    ///
    /// - `ffi` must be a valid, non-null pointer
    /// - `ffi->L` and `ffi->U` must point to valid allocations
    /// - The memory must not be freed elsewhere after this call
    /// - After calling this function, the Rust [Csn] takes ownership
    ///   of the underlying buffers
    ///
    /// Violating these conditions results in undefined behavior.
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
