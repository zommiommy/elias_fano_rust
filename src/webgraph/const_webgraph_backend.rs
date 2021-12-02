use super::*;

/// Reader dispatcher, this is a zero-cost abstraction that will be used just
/// for homogeneity with the runtime dispatcher. During compilation all this
/// code will be basically removed. This exists only for user ease.
pub struct ConstWebGraphReader<
    Backend: CodesReader<CodesReaderType>,
    CodesReaderType: CodesRead,
    const OUTDEGREE_CODE: Code = {Code::Gamma},
    const REFERENCES_OFFSET_CODE: Code = {Code::Unary},
    const BLOCK_COUNT_CODE: Code = {Code::Gamma},
    const BLOCKS_CODE: Code = {Code::Gamma},
    const INVERVAL_COUNT_CODE: Code = {Code::Gamma},
    const INVERVAL_START_CODE: Code = {Code::Gamma},
    const INVERVAL_LEN_CODE: Code = {Code::Gamma},
    const FIRST_RESIDUAL_CODE: Code = {Code::Zeta(3)},
    const RESIDUALS_CODE: Code = {Code::Zeta(3)},
> {
    backend: Backend,
    _marker: core::marker::PhantomData<CodesReaderType>,
}

impl<
    Backend: CodesReader<CodesReaderType>,
    CodesReaderType: CodesRead,
    const OUTDEGREE_CODE: Code,
    const REFERENCES_OFFSET_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const BLOCKS_CODE: Code,
    const INVERVAL_COUNT_CODE: Code,
    const INVERVAL_START_CODE: Code,
    const INVERVAL_LEN_CODE: Code,
    const FIRST_RESIDUAL_CODE: Code,
    const RESIDUALS_CODE: Code,
    >
    ConstWebGraphReader<
        Backend,
        CodesReaderType,
        OUTDEGREE_CODE,
        REFERENCES_OFFSET_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    #[inline]
    pub fn new(backend: Backend) -> Self {
        ConstWebGraphReader {
            backend,
            _marker: core::marker::PhantomData::default(),
        }
    }
}

impl<
    Backend: CodesReader<CodesReaderType>,
    CodesReaderType: CodesRead,
    const OUTDEGREE_CODE: Code,
    const REFERENCES_OFFSET_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const BLOCKS_CODE: Code,
    const INVERVAL_COUNT_CODE: Code,
    const INVERVAL_START_CODE: Code,
    const INVERVAL_LEN_CODE: Code,
    const FIRST_RESIDUAL_CODE: Code,
    const RESIDUALS_CODE: Code,
    > WebGraphReader<ConstWebGraphReaderBackend<
        CodesReaderType,
        OUTDEGREE_CODE,
        REFERENCES_OFFSET_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >> 
    for ConstWebGraphReader<
        Backend,
        CodesReaderType,
        OUTDEGREE_CODE,
        REFERENCES_OFFSET_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    >
{
    #[inline]
    fn get_reader(&'a self, offset: usize) -> ConstWebGraphReaderBackend<
        CodesReaderType,
        OUTDEGREE_CODE,
        REFERENCES_OFFSET_CODE,
        BLOCK_COUNT_CODE,
        BLOCKS_CODE,
        INVERVAL_COUNT_CODE,
        INVERVAL_START_CODE,
        INVERVAL_LEN_CODE,
        FIRST_RESIDUAL_CODE,
        RESIDUALS_CODE,
    > {
        ConstWebGraphReaderBackend((&self.backend).get_codes_reader(offset))
    }
}

/// Wrapped reader, this is a zero-cost layer that just forces the readers
/// to do constant propagation of the choosen codes.
pub struct ConstWebGraphReaderBackend<
    Reader: CodesRead,
    const OUTDEGREE_CODE: Code,
    const REFERENCES_OFFSET_CODE: Code,
    const BLOCK_COUNT_CODE: Code,
    const BLOCKS_CODE: Code,
    const INVERVAL_COUNT_CODE: Code,
    const INVERVAL_START_CODE: Code,
    const INVERVAL_LEN_CODE: Code,
    const FIRST_RESIDUAL_CODE: Code,
    const RESIDUALS_CODE: Code,
>(Reader);

impl<
        READER: CodesRead,
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
    #[inline]
    fn read_outdegree(&mut self) -> Result<usize> {
        <READER>::read_code::<OUTDEGREE_CODE>(&mut self.0)
    }

    #[inline]
    fn read_reference_offset(&mut self) -> Result<usize> {
        <READER>::read_code::<REFERENCES_CODE>(&mut self.0)
    }

    #[inline]
    fn read_block_count(&mut self) -> Result<usize> {
        <READER>::read_code::<BLOCK_COUNT_CODE>(&mut self.0)
    }

    #[inline]
    fn read_blocks(&mut self) -> Result<usize> {
        <READER>::read_code::<BLOCKS_CODE>(&mut self.0)
    }

    #[inline]
    fn read_interval_count(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_COUNT_CODE>(&mut self.0)
    }

    #[inline]
    fn read_interval_start(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_START_CODE>(&mut self.0)
    }

    #[inline]
    fn read_interval_len(&mut self) -> Result<usize> {
        <READER>::read_code::<INVERVAL_LEN_CODE>(&mut self.0)
    }

    #[inline]
    fn read_first_residual(&mut self) -> Result<usize> {
        <READER>::read_code::<FIRST_RESIDUAL_CODE>(&mut self.0)
    }

    #[inline]
    fn read_residual(&mut self) -> Result<usize> {
        <READER>::read_code::<RESIDUALS_CODE>(&mut self.0)
    }
}


impl<
        READER: CodesRead,
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
    #[inline]
    fn read_bit(&mut self) -> Result<bool> {
        self.0.read_bit()
    }

    #[inline]
    fn seek_bits(&mut self, bit_offset: usize) -> Result<()> {
        self.0.seek_bits(bit_offset)
    }

    #[inline]
    fn tell_bits(&self) -> Result<usize> {
        self.0.tell_bits()
    }
}