#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod strategies;

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::strategies::{CheckStrategy, DefaultCheckStrategy};

/// Write the embedded data to the file.
///
/// If the file already exists, it will be overwritten.
///
/// # Usage
/// ```rust,ignore
/// release_file(include_bytes!("data/file.txt"), "file.txt");
/// ```
pub fn release_file(data: &'static [u8], file: impl AsRef<Path>) -> Result<()> {
    release_file_with_check(data, file, DefaultCheckStrategy::lite())
}

/// Check if the file is the same as the embedded data, if not, write the embedded data to the file.
///
/// See [DefaultCheckStrategy] to switch a satisfied check strategy.
///
/// # Usage
/// ```rust,ignore
/// release_file_with_check(include_bytes!("data/file.txt"), "file.txt", DefaultCheckStrategy::lite());
/// ```
pub fn release_file_with_check(data: &'static [u8], file: impl AsRef<Path>, check: impl CheckStrategy) -> Result<()> {
    let mut file = File::options().read(true).write(true).create(true).open(file)?;
    let metadata = file.metadata()?;
    if check.compare_file(data, &metadata, &mut file)? {
        Ok(())
    } else {
        file.write_all(data)
    }
}
