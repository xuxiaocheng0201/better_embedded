use std::fs::File;
use std::io::Result;

use bitflags::bitflags;

bitflags! {
    /// The strategy to check if the data is the same as the file.
    pub struct CheckStrategy: u8 {
        /// Always different.
        const Always = 1 << 0;
        /// Compare by file size.
        const Size = 1 << 1;
        // const First64Bytes = 1 << 2;
    }
}

impl CheckStrategy {
    /// The recommended strategy for configuration file. (always same and don't write)
    #[inline]
    pub fn config() -> Self {
        Self::empty()
    }

    /// The recommended strategy for lite supported file. (always different and overwrite)
    #[inline]
    pub fn lite() -> Self {
        Self::Always
    }

    /// The recommended strategy for heavy supported file. (just do necessary check)
    #[inline]
    pub fn heavy() -> Self {
        // FIXME: replace if other strategies once available.
        Self::Size
    }
}

impl Default for CheckStrategy {
    #[inline]
    fn default() -> Self {
        Self::lite()
    }
}

impl CheckStrategy {
    /// Return false only if it needs overwrite
    pub(crate) fn check_file(&self, data: &'static [u8], file: &File) -> Result<bool> {
        if self.contains(CheckStrategy::Always) {
            return Ok(false);
        }
        let metadata = file.metadata()?;
        if self.contains(CheckStrategy::Size) && data.len() as u64 != metadata.len() {
            return Ok(false);
        }
        Ok(data.len() > 0 && metadata.len() > 0) // Check the file is not empty.
    }
}
