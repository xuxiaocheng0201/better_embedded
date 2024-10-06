#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
pub fn release_file_with_check(data: &'static [u8], path: impl AsRef<Path>, check: impl CheckStrategy) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    #[cfg(feature = "log")]
    log::debug!("Checking to release file: {}", path.display());
    let mut file = File::options().read(true).write(true).create(true).open(path)?;
    let metadata = file.metadata()?;
    if check.compare_file(data, &metadata, &mut file)? {
        Ok(())
    } else {
        #[cfg(feature = "log")]
        log::info!("Releasing file: {}, data length: {}", path.display(), data.len());
        file.write_all(data)
    }
}
