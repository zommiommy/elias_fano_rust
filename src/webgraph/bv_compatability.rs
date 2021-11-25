use core::convert::TryFrom;
use crate::codes::*;
use crate::errors::*;

const DEFAULT_ZETA_K: usize = 3;
const DEFAULT_GOLOMB_B: usize = 3;

const OUTDEGREES_OFFSET: usize = 0;
const BLOCKS_OFFSET: usize = 4;
const RESIDUALS_OFFSET: usize = 8;
const REFERENCES_OFFSET: usize = 12;
const BLOCK_COUNT_OFFSET: usize = 16;
const OFFSETS_OFFSET: usize = 20;
const EXTRA_SPACE_OFFSET: usize = 24;

impl TryFrom<u8> for Code {
    fn try_from(value: u8) -> Result<Code, Error> {
        match value {
            1 => Ok(Code::Delta),
            2 => Ok(Code::Gamma),
            3 => Ok(Code::Golomb(DEFAULT_GOLOMB_B)),
            4 => Ok(Code::SkewedGolomb),
            5 => Ok(Code::Unary),
            6 => Ok(Code::Zeta(DEFAULT_ZETA_K)),
            7 => Ok(Code::Nibble),
            _ => Err(Error::InvalidCodeNibble),
        }
    }
}

impl Into<u8> for Code {
    fn into(self) -> u8 {
        match self {
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
    /// Which code to use to encode the outdegrees
    outdegrees: Code,
    /// Which code to use to encode the copy-block list
    blocks: Code,
    /// Which code to use to encode residuals
    residuals: Code,
    /// Which code to use to encode references
    references: Code,
    /// Which code to use to encode the number of blocks
    block_count: Code,
    /// Which code to use to encode offsets
    offsets: Code,
}

impl Default for CodesSettings {
    fn default() -> Self {
        CodesSettings {
            outdegrees: Code::Gamma,
            blocks: Code::Gamma,
            residuals: Code::Zeta(DEFAULT_ZETA_K),
            references: Code::Unary,
            block_count: Code::Gamma,
            offsets: Code::Gamma,
        }
    }
}

impl TryFrom<u32> for CodesSettings {
    fn try_from(value: u32) -> Result<CodesSettings> {
        Ok(CodesSettings {
            outdegrees: Code::try_from(0xF & (value >> OUTDEGREES_OFFSET) as u8)?,
            blocks: Code::try_from(0xF & (value >> BLOCKS_OFFSET) as u8)?,
            residuals: Code::try_from(0xF & (value >> RESIDUALS_OFFSET) as u8)?,
            references: Code::try_from(0xF & (value >> REFERENCES_OFFSET) as u8)?,
            block_count: Code::try_from(0xF & (value >> BLOCK_COUNT_OFFSET) as u8)?,
            offsets: Code::try_from(0xF & (value >> OFFSETS_OFFSET) as u8)?,
        })
    }
}

impl Into<u32> for CodesSettings {
    fn into(self) -> u32 {
        let mut result = 0;
        result |= self.outdegrees.into() << OUTDEGREES_OFFSET;
        result |= self.blocks.into() << BLOCKS_OFFSET;
        result |= self.residuals.into() << RESIDUALS_OFFSET;
        result |= self.references.into() << REFERENCES_OFFSET;
        result |= self.block_count.into() << BLOCK_COUNT_OFFSET;
        result |= self.offsets.into() << OFFSETS_OFFSET;
    }
}
