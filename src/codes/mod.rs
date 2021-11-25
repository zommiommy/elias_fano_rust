//!
//!
//! |       alpha |         Code |
//! |-------------|--------------|
//! | < 1.06      | Elia's Delta |
//! | [1.06,1.08] | zeta<6>      |
//! | [1.08,1.11] | zeta<5>      |
//! | [1.11,1.16] | zeta<4>      |
//! | [1.16,1.27] | zeta<3>      |
//! | [1.27,1.57] | zeta<2>      |
//! | [1.57,1.2]  | Elia's Gamma |
//!
mod delta;
pub use delta::*;

mod fixed_length;
pub use fixed_length::*;

mod gamma;
pub use gamma::*;

mod golomb;
pub use golomb::*;

mod golomb_runtime;
pub use golomb_runtime::*;

// mod interpolative;
// pub use interpolative::CodeInterpolative;

mod minimal_binary_little;
pub use minimal_binary_little::*;

mod minimal_binary_big;
pub use minimal_binary_big::*;

mod minimal_binary;
pub use minimal_binary::*;

mod unary;
pub use unary::*;

mod var_length;
pub use var_length::*;

mod zeta;
pub use zeta::*;

mod zeta_runtime;
pub use zeta_runtime::*;

pub use golomb::compute_optimal_golomb_block_size;
use crate::errors::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Code {
    Delta,
    Gamma,
    Golomb(usize),
    SkewedGolomb,
    Unary,
    Zeta(usize),
    Nibble,
}

trait CodesWrite:
    CodeWriteDelta + CodeWriteGamma + CodeWriteGolomb + CodeWriteUnary + CodeWriteZeta
{
    fn write_code<const CODE: Code>(inner: &Self, value: usize) -> Result<()> {
        match CODE {
            Code::Delta => inner.write_delta(value),
            Code::Gamma => inner.write_gamma(value),
            Code::Golomb(B) => inner.write_golomb::<{B}>(value),
            Code::Unary => inner.write_unary(value),
            Code::Zeta(B) => inner.write_zeta::<{B}>(value),
            _ => unimplemented!("The given code is not implemented yet."),
        }
    }
}
