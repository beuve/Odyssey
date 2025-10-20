mod cs;
mod csn;
mod css;
mod mapped_matrix;
mod mapped_vector;

#[allow(warnings, clippy::all)]
mod suitesparse {
    include!("./suitesparse.rs");
}

pub use self::mapped_matrix::{MappedMatrix, MappedMatrixBuilder};
pub use self::mapped_vector::MappedVector;
