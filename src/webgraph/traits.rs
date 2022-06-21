use crate::errors::*;
use crate::traits::*;

pub trait WebGraphReaderBackend: ReadBit {
    fn read_outdegree(&mut self) -> Result<usize>;

    // node reference
    fn read_reference_offset(&mut self) -> Result<usize>;

    // run length reference copy
    fn read_block_count(&mut self) -> Result<usize>;
    fn read_blocks(&mut self) -> Result<usize>;

    // intervallizzation
    fn read_interval_count(&mut self) -> Result<usize>;
    fn read_interval_start(&mut self) -> Result<usize>;
    fn read_interval_len(&mut self) -> Result<usize>;

    // extra nodes
    fn read_first_residual(&mut self) -> Result<usize>;
    fn read_residual(&mut self) -> Result<usize>;
}

pub trait WebGraphWriterBackend: WriteBit {
    fn write_outdegree(&mut self) -> Result<usize>;

    // node reference
    fn write_reference_offset(&mut self) -> Result<usize>;

    // run length reference copy
    fn write_block_count(&mut self) -> Result<usize>;
    fn write_blocks(&mut self) -> Result<usize>;

    // intervallizzation
    fn write_interval_count(&mut self) -> Result<usize>;
    fn write_interval_start(&mut self) -> Result<usize>;
    fn write_interval_len(&mut self) -> Result<usize>;

    // extra nodes
    fn write_first_residual(&mut self) -> Result<usize>;
    fn write_residual(&mut self) -> Result<usize>;
}

pub trait WebGraphReader {
    type WebGraphReaderType<'a>: WebGraphReaderBackend + 'a where Self: 'a;
    
    fn get_reader(&self, offset: usize) -> Self::WebGraphReaderType<'_>;
}
