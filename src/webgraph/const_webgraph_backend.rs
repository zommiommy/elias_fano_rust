use super::*;

/// Reader dispatcher, this is a zero-cost abstraction that will be used just
/// for homogeneity with the runtime dispatcher. During compilation all this
/// code will be basically removed. This exists only for user ease.
pub struct ConstWebGraphReader<
    'a,
    BACKEND: CodesReader<'a>,
    const OUTDEGREE_CODE: Code,
    const REFERENCES_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const BLOCKS_CODE: Code,
    const INVERVAL_COUNT_CODE: Code,
    const INVERVAL_START_CODE: Code,
    const INVERVAL_LEN_CODE: Code,
    const FIRST_RESIDUAL_CODE: Code,
    const RESIDUALS_CODE: Code,
> {
    backend: BACKEND,
    phantom: std::marker::PhantomData<&'a [()]>,
}

impl<
        'a,
        BACKEND: CodesReader<'a>,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    >
    ConstWebGraphReader<
        'a,
        BACKEND,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    pub fn new(backend: BACKEND) -> Self {
        ConstWebGraphReader {
            backend,
            phantom: std::marker::PhantomData::default(),
        }
    }
}

impl<
        'a,
        BACKEND: CodesReader<'a> + MemoryFootprint,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    > WebGraphReader<'a>
    for ConstWebGraphReader<
        'a,
        BACKEND,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    type ReaderType = ConstWebGraphReaderBackend<
        BACKEND::CodesReaderType,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >;
    fn get_reader(&'a self, offset: usize) -> Self::ReaderType {
        ConstWebGraphReaderBackend(self.backend.get_codes_reader(offset))
    }
}

/// Wrapped reader, this is a zero-cost layer that just forces the readers
/// to do constant propagation of the choosen codes.
pub struct ConstWebGraphReaderBackend<
    Reader: CodesRead,
    const OUTDEGREE_CODE: Code,
    const REFERENCES_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const BLOCKS_CODE: Code,
    const INVERVAL_COUNT_CODE: Code,
    const INVERVAL_START_CODE: Code,
    const INVERVAL_LEN_CODE: Code,
    const FIRST_RESIDUAL_CODE: Code,
    const RESIDUALS_CODE: Code,
>(Reader);

impl<
        READER: CodesRead + MemoryFootprint,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    > WebGraphReaderBackend
    for ConstWebGraphReaderBackend<
        READER,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    fn read_outdegree(&mut self) -> Result<usize> {
        <READER>::read_code::<OUTDEGREE_CODE>(&mut self.0)
    }

    fn read_reference_offset(&mut self) -> Result<usize> {
        <READER>::read_code::<REFERENCES_CODE>(&mut self.0)
    }

    fn read_block_count(&mut self) -> Result<usize> {
        <READER>::read_code::<BLOCK_COUNT_CODE>(&mut self.0)
    }

    fn read_blocks(&mut self) -> Result<usize> {
        <READER>::read_code::<BLOCKS_CODE>(&mut self.0)
    }

    fn read_interval_count(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_COUNT_CODE>(&mut self.0)
    }

    fn read_interval_start(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_START_CODE>(&mut self.0)
    }

    fn read_interval_len(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_LEN_CODE>(&mut self.0)
    }

    fn read_first_residual(&mut self) -> Result<usize> {
        <READER>::read_code::<FIRST_RESIDUAL_CODE>(&mut self.0)
    }

    fn read_residual(&mut self) -> Result<usize> {
        <READER>::read_code::<RESIDUALS_CODE>(&mut self.0)
    }
}

impl<
        'a,
        BACKEND: CodesReader<'a> + MemoryFootprint,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    > crate::traits::MemoryFootprint
    for ConstWebGraphReader<
        'a,
        BACKEND,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    fn total_size(&self) -> usize {
        self.backend.total_size()
    }
}

impl<
        READER: CodesRead + MemoryFootprint,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    > crate::traits::ReadBit
    for ConstWebGraphReaderBackend<
        READER,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    fn read_bit(&mut self) -> Result<bool> {
        self.0.read_bit()
    }

    fn seek_bits(&mut self, bit_offset: usize) -> Result<()> {
        self.0.seek_bits(bit_offset)
    }

    fn tell_bits(&self) -> Result<usize> {
        self.0.tell_bits()
    }
}

impl<
        READER: CodesRead + MemoryFootprint,
        const OUTDEGREE_CODE: Code,
        const REFERENCES_CODE: Code,
        const BLOCK_COUNT_CODE: Code,
        const BLOCKS_CODE: Code,
        const INVERVAL_COUNT_CODE: Code,
        const INVERVAL_START_CODE: Code,
        const INVERVAL_LEN_CODE: Code,
        const FIRST_RESIDUAL_CODE: Code,
        const RESIDUALS_CODE: Code,
    > crate::traits::MemoryFootprint
    for ConstWebGraphReaderBackend<
        READER,
        OUTDEGREE_CODE,
        REFERENCES_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    fn total_size(&self) -> usize {
        self.0.total_size()
    }
}
