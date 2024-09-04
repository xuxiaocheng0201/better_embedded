pub mod check;

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use crate::check::CheckStrategy;

pub fn embedded_file(data: &'static [u8], file: impl AsRef<Path>) -> Result<()> {
    embedded_file_check(data, file, CheckStrategy::default())
}

pub fn embedded_file_check(data: &'static [u8], file: impl AsRef<Path>, check: CheckStrategy) -> Result<()> {
    let mut file = File::options().read(true).write(true).create(true).open(file)?;
    if check.check_is_same(data, &file)? {
        Ok(())
    } else {
        file.write_all(data)
    }
}
