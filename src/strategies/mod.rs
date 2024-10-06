//! Check strategies.

pub mod common;
#[cfg(feature = "random")]
#[cfg_attr(docsrs, doc(cfg(feature = "random")))]
pub mod random;
#[cfg(feature = "md5")]
#[cfg_attr(docsrs, doc(cfg(feature = "md5")))]
pub mod md5;

use std::fs::{File, Metadata};
use std::io::Result;

/// The strategies to check if the data is the same as the file.
pub trait CheckStrategy {
    /// Compare the embedded data with existed file.
    ///
    /// If you implement this trait, and use `file` parameter,
    /// you should call `file.seek(SeekFrom::Start(0))?;` before use it.
    /// And **never** modify the content of file.
    ///
    /// Return false only if it needs overwrite.
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, file: &mut File) -> Result<bool>;
}

/// A helpful entrypoint to choose check strategy.
pub struct DefaultCheckStrategy;

impl DefaultCheckStrategy {
    /// The recommended strategy for configuration file. (not write if isn't empty)
    #[inline]
    pub fn config() -> ConfigCheckStrategy {
        ConfigCheckStrategy
    }

    /// The recommended strategy for lite supported file. (always different and overwrite)
    #[inline]
    pub fn lite() -> LiteCheckStrategy {
        LiteCheckStrategy
    }

    /// The recommended strategy for heavy supported file. (just do necessary check)
    #[inline]
    pub fn heavy() -> HeavyCheckStrategy {
        HeavyCheckStrategy
    }
}


/// The recommended strategy for configuration file. (not write if isn't empty)
pub struct ConfigCheckStrategy;
impl CheckStrategy for ConfigCheckStrategy {
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, _file: &mut File) -> Result<bool> {
        Ok(metadata.len() != 0 || data.len() == 0)
    }
}

/// The recommended strategy for lite supported file. (always different and overwrite)
pub struct LiteCheckStrategy;
impl CheckStrategy for LiteCheckStrategy {
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, file: &mut File) -> Result<bool> {
        common::AlwaysDifferent.compare_file(data, metadata, file)
    }
}

/// The recommended strategy for heavy supported file. (just do necessary check)
pub struct HeavyCheckStrategy;
impl CheckStrategy for HeavyCheckStrategy {
    fn compare_file(&self, data: &'static [u8], metadata: &Metadata, file: &mut File) -> Result<bool> {
        #[cfg(not(feature = "md5"))] {
            Ok(common::Size.compare_file(data, metadata, file)? && common::FirstNBytes::default().compare_file(data, metadata, file)?)
        }
        #[cfg(feature = "md5")] {
            Ok(common::Size.compare_file(data, metadata, file)? && md5::Md5CheckStrategy.compare_file(data, metadata, file)?)
        }
    }
}
