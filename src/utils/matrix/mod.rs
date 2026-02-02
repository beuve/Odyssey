mod cs;
mod mapped_matrix;
mod mapped_vector;

#[allow(warnings, clippy::all)]
mod suitesparse {
   include!(concat!(
        env!("OUT_DIR"),
        "/suitesparse_bindings.rs"
    ));
}

pub use self::mapped_matrix::{MappedMatrix, MappedMatrixBuilder};
pub use self::mapped_vector::MappedVector;
