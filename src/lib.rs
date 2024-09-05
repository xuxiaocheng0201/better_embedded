#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod check;

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub use crate::check::CheckStrategy;

/// Write the embedded data to the file.
///
/// If the file already exists, it will be overwritten.
///
/// # Usage
/// ```rust,ignore
/// embedded_file(include_bytes!("data/file.txt"), "file.txt");
/// ```
pub fn embedded_file(data: &'static [u8], file: impl AsRef<Path>) -> Result<()> {
    embedded_file_check(data, file, CheckStrategy::Always)
}

/// Check if the file is the same as the embedded data, if not, write the embedded data to the file.
///
/// # Usage
/// ```rust,ignore
/// embedded_file_check(include_bytes!("data/file.txt"), "file.txt", CheckStrategy::lite());
/// ```
pub fn embedded_file_check(data: &'static [u8], file: impl AsRef<Path>, check: CheckStrategy) -> Result<()> {
    let mut file = File::options().read(true).write(true).create(true).open(file)?;
    if check.check_file(data, &file)? {
        Ok(())
    } else {
        file.write_all(data)
    }
}
