use crate::codes::*;
use crate::errors::*;
use crate::traits::MemoryFootprint;
use core::convert::TryFrom;

const DEFAULT_ZETA_K: usize = 3;
const DEFAULT_GOLOMB_B: usize = 3;

#[allow(dead_code)]
mod constants {
    pub const OUTDEGREES_OFFSET: usize = 0;
    pub const BLOCKS_OFFSET: usize = 4;
    pub const RESIDUALS_OFFSET: usize = 8;
    pub const REFERENCES_OFFSET: usize = 12;
    pub const BLOCK_COUNT_OFFSET: usize = 16;
    pub const OFFSETS_OFFSET: usize = 20;
    pub const EXTRA_SPACE_OFFSET: usize = 24;
}

impl TryFrom<u8> for Code {
    type Error = Error;
    fn try_from(value: u8) -> Result<Code> {
        match value {
            1 => Ok(Code::Delta),
            2 => Ok(Code::Gamma),
            3 => Ok(Code::Golomb(DEFAULT_GOLOMB_B)),
            4 => Ok(Code::SkewedGolomb),
            5 => Ok(Code::Unary),
            6 => Ok(Code::Zeta(DEFAULT_ZETA_K)),
            7 => Ok(Code::Nibble),
            x => Err(Error::InvalidCodeNibble(x)),
        }
    }
}

impl From<Code> for u8 {
    fn from(v: Code) -> Self {
        match v {
            Code::Delta => 1,
            Code::Gamma => 2,
            Code::Golomb(_) => 3,
            Code::SkewedGolomb => 4,
            Code::Unary => 5,
            Code::Zeta(_) => 6,
            Code::Nibble => 7,
        }
    }
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
