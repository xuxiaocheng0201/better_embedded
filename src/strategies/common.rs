//! Some common strategies.

use std::cmp::min;
use std::fs::{File, Metadata};
use std::io::{Read, Seek, SeekFrom};

use crate::CheckStrategy;

/// Always same.
pub struct AlwaysSame;
impl CheckStrategy for AlwaysSame {
    #[inline]
    fn compare_file(&self, _data: &'static [u8], _metadata: &Metadata, _file: &mut File) -> std::io::Result<bool> {
        Ok(true)
    }
}

/// Always different.
pub struct AlwaysDifferent;
impl CheckStrategy for AlwaysDifferent {
    #[inline]
    fn compare_file(&self, _data: &'static [u8], _metadata: &Metadata, _file: &mut File) -> std::io::Result<bool> {
        Ok(false)
    }
}

/// Compare file size.
pub struct Size;
impl CheckStrategy for Size {
    #[inline]
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, _file: &mut File) -> std::io::Result<bool> {
        Ok(data.len() as u64 == metadata.len())
    }
}

/// Compare the first n bytes.
pub struct FirstNBytes(pub u16);
impl CheckStrategy for FirstNBytes {
    #[inline]
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, file: &mut File) -> std::io::Result<bool> {
        let len = min(min(data.len(), self.0 as usize) as u64, metadata.len()) as usize;
        file.seek(SeekFrom::Start(0))?;
        let mut f = vec![0; len];
        file.read_exact(&mut f)?;
        if f != data[..len] {
            return Ok(false);
        }
        Ok(true)
    }
}
impl Default for FirstNBytes {
    #[inline]
    fn default() -> Self {
        Self(64)
    }
}
