//! Strategy enabled by md5 feature.

use std::fs::{File, Metadata};
use std::io::{copy, Result, Seek, SeekFrom};

use md5::{Digest, Md5};

use crate::strategies::CheckStrategy;

/// Compare md5 hash.
pub struct Md5CheckStrategy;

impl CheckStrategy for Md5CheckStrategy {
    fn compare_file(&self, data: &'static [u8], _metadata: &Metadata, file: &mut File) -> Result<bool> {
        let mut hasher = Md5::default(); // todo: hash in compile time
        hasher.update(data);
        let hash = hasher.finalize();

        file.seek(SeekFrom::Start(0))?;
        let mut file_hasher = Md5::default();
        copy(file, &mut file_hasher)?;
        let file_hash = file_hasher.finalize();

        Ok(hash == file_hash)
    }
}
