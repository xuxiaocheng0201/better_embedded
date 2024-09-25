//! Strategy enabled by random feature.

use std::fs::{File, Metadata};
use std::io::Result;

use rand::Rng;

use crate::strategies::CheckStrategy;

/// Overwrite file randomly.
pub struct RandomCheckStrategy(u32, u32);

impl CheckStrategy for RandomCheckStrategy {
    #[inline]
    fn compare_file(&self, _data: &'static [u8], _metadata: &Metadata, _file: &mut File) -> Result<bool> {
        Ok(rand::thread_rng().gen_ratio(self.0, self.1))
    }
}

impl RandomCheckStrategy {
    /// Create a new random check strategy.
    ///
    /// The probability of overwriting is `numerator / denominator`.
    pub fn new(numerator: u32, denominator: u32) -> Self {
        assert!(denominator != 0 && numerator <= denominator);
        Self(denominator - numerator, denominator)
    }
}

impl Default for RandomCheckStrategy {
    #[inline]
    fn default() -> Self {
        Self::new(1, 2)
    }
}
