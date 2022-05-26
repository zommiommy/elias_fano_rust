use super::*;

#[derive(Eq, PartialEq, Clone, Debug)]
/// Enum for constant dispatching of codes
pub enum Code {
    /// Elias Delta Code
    Delta,
    /// Elias Gamma Code
    Gamma,
    /// Golomb code
    Golomb(usize),
    /// TODO!:
    SkewedGolomb,
    /// Unary Code
    Unary,
    /// Zeta Code
    Zeta(usize),
    /// TODO!:
    Nibble,
}

#[derive(Debug, Clone)]
/// Code settings for WebGraph
pub struct CodesSettings {
    pub outdegree: Code,

    pub reference_offset: Code,

    pub block_count: Code,
    pub blocks: Code,

    pub interval_count: Code,
    pub interval_start: Code,
    pub interval_len: Code,

    pub first_residual: Code,
    pub residual: Code,
}

impl Default for CodesSettings {
    fn default() -> Self {
        CodesSettings {
            outdegree: Code::Gamma,

            reference_offset: Code::Unary,

            block_count: Code::Gamma,
            blocks: Code::Gamma,

            interval_count: Code::Gamma,
            interval_start: Code::Gamma,
            interval_len: Code::Gamma,

            first_residual: Code::Zeta(3),
            residual: Code::Zeta(3),
        }
    }
}

impl MemoryFootprint for CodesSettings {
    fn total_size(&self) -> usize {
        std::mem::size_of::<Code>() * 6
    }
}

/// WebGraph reader with runtime dynamic dispatching.
/// Once I figure out how to have the proper lifetimes we can make this a 
/// single static dispatching so that at runtime we pay only the indirection.
pub struct RuntimeWebGraphReader<Backend: CodesReader> {
    backend: Backend,
    settings: CodesSettings,
}

impl<Backend: CodesReader> RuntimeWebGraphReader<Backend> {
    #[inline]
    pub fn new(settings: CodesSettings, backend: Backend) -> Self {
        RuntimeWebGraphReader {
            backend,
            settings,
        }
    }

    #[inline]
    /// wrap a reader so that it dispatch the codes
    pub fn wrap<'a>(
        &self,
        reader: Backend::CodesReaderType<'a>,
    ) -> RuntimeWebGraphReaderBackend<Backend::CodesReaderType<'a>> {
        RuntimeWebGraphReaderBackend {
            reader,
            settings: self.settings.clone(),
        }
    }
}

impl<Backend: CodesReader> WebGraphReader for RuntimeWebGraphReader<Backend> {
    type WebGraphReaderType<'a> = RuntimeWebGraphReaderBackend<Backend::CodesReaderType<'a>>
    where
        Self: 'a;

    #[inline]
    fn get_reader(&self, offset: usize) -> Self::WebGraphReaderType<'_> {
        self.wrap((&self.backend).get_codes_reader(offset))
    }
}

/// Reader with dynamic dispatching
pub struct RuntimeWebGraphReaderBackend<READER: CodesRead> {
    reader: READER,
    settings: CodesSettings,
}

macro_rules! impl_method_reader {
    ($method_name:ident, $var_match:ident) => {
        #[inline]
        fn $method_name(&mut self) -> Result<usize> {
            match self.settings.$var_match {
                Code::Unary => self.reader.read_unary(),
                Code::Gamma => self.reader.read_gamma(),
                Code::Delta => self.reader.read_delta(),
                Code::Golomb(b) => self.reader.read_golomb_runtime(b),
                Code::Zeta(k) => self.reader.read_zeta_runtime(k),
                _ => unimplemented!("The wanted code is not implemented yet."),
            }
        }
    };
}

impl<READER> WebGraphReaderBackend for RuntimeWebGraphReaderBackend<READER>
where
    READER: CodesRead,
{
    impl_method_reader!(read_outdegree, outdegree);
    impl_method_reader!(read_reference_offset, reference_offset);
    impl_method_reader!(read_block_count, block_count);
    impl_method_reader!(read_blocks, blocks);
    impl_method_reader!(read_interval_count, interval_count);
    impl_method_reader!(read_interval_start, interval_start);
    impl_method_reader!(read_interval_len, interval_len);
    impl_method_reader!(read_first_residual, first_residual);
    impl_method_reader!(read_residual, residual);
}

impl<READER> crate::traits::ReadBit for RuntimeWebGraphReaderBackend<READER>
where
    READER: CodesRead,
{
    #[inline]
    fn read_bit(&mut self) -> Result<bool> {
        self.reader.read_bit()
    }
    
    #[inline]
    fn peek_byte(&mut self) -> Result<u8> {
        self.reader.peek_byte()
    }

    #[inline]
    fn seek_bits(&mut self, bit_offset: usize) -> Result<()> {
        self.reader.seek_bits(bit_offset)
    }

    #[inline]
    fn tell_bits(&self) -> Result<usize> {
        self.reader.tell_bits()
    }
}
