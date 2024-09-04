use std::fs::File;
use std::io::Result;

use bitflags::bitflags;

bitflags! {
    pub struct CheckStrategy: u8 {
        /// Always different.
        const Always = 1 << 0;
        /// Compare by file size.
        const Size = 1 << 1;
    }
}

impl Default for CheckStrategy {
    fn default() -> Self {
        CheckStrategy::Size
    }
}

impl CheckStrategy {
    pub fn check_is_same(&self, data: &'static [u8], file: &File) -> Result<bool> {
        if self.contains(CheckStrategy::Always) {
            return Ok(false);
        }
        if self.contains(CheckStrategy::Size) && data.len() as u64 != file.metadata()?.len() {
            return Ok(false);
        }
        Ok(true)
    }
}
