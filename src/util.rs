pub mod parse_matrix;
pub mod parse;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub mod matrix;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub mod matrix_vec;
