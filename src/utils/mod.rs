pub mod constants;
pub mod matrix;
mod read_only;
pub mod search;

pub type RO<T> = read_only::RO<T>;
