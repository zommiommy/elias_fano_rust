use crate::elias_fano::EliasFano;
use std::path::Path;

/// Struct that wraps the Elias-Fano structure used for the node indices in
/// Webgraph. This wrapper is mainly used to abstract away its usage and
/// handle the reading of offsets files.
pub struct Offsets<const QUANTUM_LOG2: usize>(EliasFano<QUANTUM_LOG2>);

impl<const QUANTUM_LOG2: usize> Offsets<QUANTUM_LOG2> {
    pub fn from_offsets_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        
    }
}