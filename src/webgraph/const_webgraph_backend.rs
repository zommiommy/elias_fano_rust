use crate::traits::*;
use super::*;

pub struct ConstWebGraphReaderBackend<
    BACKEND: ReadBit,
    const OUTDEGREE_CODE: Code,
    const BLOCKS_CODE: Code,
    const RESIDUALS_CODE: Code,
    const REFERENCES_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const OFFSETS_CODE: Code,
>(BACKEND);

impl<
        BACKEND: ReadBit,
        const OUTDEGREE_CODE: Code,
        const BLOCKS_CODE: Code,
        const RESIDUALS_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const OFFSETS_CODE: Code,
    > WebGraphReaderCodesBackend
    for ConstWebGraphReaderBackend<
        BACKEND,
        OUTDEGREE_CODE,
        BLOCKS_CODE,
        RESIDUALS_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        OFFSETS_CODE,
    >
{
    fn read_outdegrees(&mut self) -> usize {
        self.parse::<OUTDEGREE_CODE>()
    }

    fn read_blocks(&mut self) -> usize {
        self.parse::<BLOCKS_CODE>()
    }

    fn read_residuals(&mut self) -> usize {
        self.parse::<RESIDUALS_CODE>()
    }

    fn read_references(&mut self) -> usize {
        self.parse::<REFERENCES_CODE>()
    }

    fn read_block_count(&mut self) -> usize {
        self.parse::<BLOCK_COUNT_CODE>()
    }

    fn read_offsets(&mut self) -> usize {
        self.parse::<OFFSETS_CODE>()
    }
}
