mod file;
mod string;

pub use file::{generate_unique_filename, is_valid_file};
#[allow(unused_imports)]
pub use string::str_option_to_slice;
