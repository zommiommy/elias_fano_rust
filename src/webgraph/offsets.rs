use crate::elias_fano::EliasFano;
use std::path::Path;
use crate::Result;
use crate::Error;
use super::*;
use crate::prelude::BitArrayM2L;
use crate::prelude::MemoryMappedFileReadOnly;

/// Struct that wraps the Elias-Fano structure used for the node indices in
/// Webgraph. This wrapper is mainly used to abstract away its usage and
/// handle the reading of offsets files.
pub struct Offsets<const QUANTUM_LOG2: usize>(EliasFano<QUANTUM_LOG2>);

impl<const QUANTUM_LOG2: usize> Offsets<QUANTUM_LOG2> {
    pub fn from_offsets_file<P: AsRef<Path>>(path: P, properties: Properties) -> Result<Self> {
        let mmap = MemoryMappedFileReadOnly::open(path)?;
        let backend_reader =  BitArrayM2L::new(mmap);
        let mut ef = EliasFano::new(
            0,
            properties.nodes,
        ).unwrap();

        let mut reader = backend_reader.get_codes_reader(0);

        let mut cumulative_sum = 0;
        for _ in 0..properties.nodes.saturating_sub(1) {
            let code = reader.read_gamma()?;
            cumulative_sum += code;
            ef.push(cumulative_sum).unwrap();
        }

        Ok(Self(ef))
    }

    pub fn get(&self, node_id: usize) -> Result<usize> {
        self.0.select(node_id).map_err(|_| Error::IndexOutOfBound{
            index: node_id,
            len: self.0.len(),
        })
    }
}