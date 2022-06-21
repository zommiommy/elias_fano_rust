use std::collections::HashMap;
use std::io::BufRead;
use crate::codes::*;
use crate::errors::*;
use crate::traits::MemoryFootprint;
use core::convert::TryFrom;
use std::path::Path;
use std::stringify;

const DEFAULT_ZETA_K: usize = 3;
const DEFAULT_GOLOMB_B: usize = 3;

const OUTDEGREES_OFFSET: usize = 0;
const BLOCKS_OFFSET: usize = 4;
const RESIDUALS_OFFSET: usize = 8;
const REFERENCES_OFFSET: usize = 12;
const BLOCK_COUNT_OFFSET: usize = 16;
const OFFSETS_OFFSET: usize = 20;
#[allow(dead_code)]
const EXTRA_SPACE_OFFSET: usize = 24;

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

    pub offsets: Code,
}

impl CodesSettings {
    pub fn from_str<S: AsRef<str>>(value: S) -> Result<Self> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Ok(CodesSettings::default());
        }

        let parsed = value.parse::<usize>()
            .map_err(|_| 
                Error::PropertiyParsingError { 
                    key: "compressionflags".into(), 
                    _type: "usize".into(), 
                    value: value.into(),
                }
            )?;

        macro_rules! parse_code {
            ($value:expr, $offset:expr) => {{
                Code::try_from((($value >>  $offset) & 0xf) as u8)?
            }};
        }

        Ok(CodesSettings{
            outdegree: parse_code!(parsed, OUTDEGREES_OFFSET),

            reference_offset: parse_code!(parsed, REFERENCES_OFFSET),

            block_count: parse_code!(parsed, BLOCK_COUNT_OFFSET),
            blocks: parse_code!(parsed, BLOCKS_OFFSET),

            interval_count: Code::Gamma,
            interval_start: Code::Gamma,
            interval_len: Code::Gamma,

            first_residual: parse_code!(parsed, RESIDUALS_OFFSET),
            residual: parse_code!(parsed, RESIDUALS_OFFSET),

            offsets: parse_code!(parsed, OFFSETS_OFFSET),
        })
    }
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

            first_residual: Code::Zeta(DEFAULT_ZETA_K),
            residual: Code::Zeta(DEFAULT_ZETA_K),

            offsets: Code::Gamma,
        }
    }
}

impl MemoryFootprint for CodesSettings {
    fn total_size(&self) -> usize {
        std::mem::size_of::<Code>() * 6
    }
}


#[derive(Debug, Clone)]
pub struct Properties{
    pub version: usize,
    pub graph_class: String,
    pub nodes: usize,
    pub arcs: usize,

    pub compression_flags: CodesSettings,
    pub compression_ratio: f64,
    pub zeta_k: usize,
    pub window_size: usize,
    pub min_interval_len: usize,
    pub max_reference_count: usize,

    pub bits_for_blocks: usize,
    pub residual_arcs: usize,
    pub intervalised_arcs: usize,
    pub avgerage_ref: f64,
    pub copied_arcs: usize,

    pub bits_per_node: f64,
    pub bits_per_link: f64,
    pub bits_for_outdegrees: usize,
    pub bits_for_references: usize,
    pub bits_for_interval: usize,
    pub bits_for_residuals: usize,

    pub average_bits_for_blocks: f64,
    pub average_distance: f64,
    pub average_bits_for_intervals: f64,
    pub average_bits_for_residuals: f64,
    pub avgerage_bits_for_outdegrees: f64,
    pub residual_avgerage_gap: f64,
    pub residual_average_log_gap: f64,
    pub successor_average_log_gap: f64,

    pub residual_exp_stats: String,
    pub successor_exp_stats: String,
}


impl Properties {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = std::io::BufReader::new(
            std::fs::File::open(path).map_err(|_| {
                Error::CannotOpenFile{path: path.display().to_string()}
            })?
        );

        let mut map = file.lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.starts_with("#") && !line.is_empty())
            .map(|line| {
                let (key, value) = line.split_once("=").unwrap();
                (key.trim().to_string(), value.trim().to_string())
            })
            .collect::<HashMap<String, String>>();


        macro_rules! get_value {
            ($map:expr, $key:literal) => {{
                $map.remove($key)
                    .ok_or_else(|| Error::MissingPropertiyError{key: $key.into()})?
            }};
            ($map:expr, $key:literal, $type:ty) => {{
                let value = get_value!($map, $key);
                value.parse::<$type>().map_err(|_| {
                    Error::PropertiyParsingError{
                        key: $key.into(),
                        _type: stringify!($type).into(),
                        value: value,
                    }
                })?
            }};
        }
        

        Ok(Properties{
            version: get_value!(map, "version", usize),
            graph_class: get_value!(map, "graphclass"),
            nodes: get_value!(map, "nodes", usize),
            arcs:  get_value!(map, "arcs", usize),
        
            compression_flags: CodesSettings::from_str(get_value!(map, "compressionflags"))?,
            compression_ratio: get_value!(map, "compratio", f64),
            zeta_k: get_value!(map, "zetak", usize),
            window_size: get_value!(map, "windowsize", usize),
            min_interval_len: get_value!(map, "minintervallength", usize),
            max_reference_count: get_value!(map, "maxrefcount", usize),
        
            bits_for_blocks: get_value!(map, "bitsforblocks", usize),
            residual_arcs: get_value!(map, "residualarcs", usize),
            intervalised_arcs: get_value!(map, "intervalisedarcs", usize),
            copied_arcs: get_value!(map, "copiedarcs", usize),
        
            bits_per_node: get_value!(map, "bitspernode", f64),
            bits_per_link: get_value!(map, "bitsperlink", f64),
            bits_for_outdegrees: get_value!(map, "bitsforoutdegrees", usize),
            bits_for_references: get_value!(map, "bitsforreferences", usize),
            bits_for_interval: get_value!(map, "bitsforintervals", usize),
            bits_for_residuals: get_value!(map, "bitsforresiduals", usize),
        
            avgerage_ref: get_value!(map, "avgref", f64),
            average_bits_for_blocks: get_value!(map, "avgbitsforblocks", f64),
            average_distance: get_value!(map, "avgdist", f64),
            average_bits_for_intervals: get_value!(map, "avgbitsforintervals", f64),
            average_bits_for_residuals: get_value!(map, "avgbitsforresiduals", f64),
            avgerage_bits_for_outdegrees: get_value!(map, "avgbitsforoutdegrees", f64),
            residual_avgerage_gap: get_value!(map, "residualavggap", f64),
            residual_average_log_gap: get_value!(map, "residualavgloggap", f64),
            successor_average_log_gap: get_value!(map, "successoravgloggap", f64),
        
            residual_exp_stats: get_value!(map, "residualexpstats"),
            successor_exp_stats: get_value!(map, "successorexpstats"),
        })
    }
}