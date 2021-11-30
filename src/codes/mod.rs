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
use crate::traits::*;

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

mod minimal_binary_l2m;
pub use minimal_binary_l2m::*;

mod minimal_binary_m2l;
pub use minimal_binary_m2l::*;

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

use crate::errors::*;
pub use golomb::compute_optimal_golomb_block_size;

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

macro_rules! impl_code_trait_parametric {
    ($($c:literal),*) => {
        /// Generic trait for data tat can write codes
        pub trait CodesWrite:
            CodeWriteDelta + CodeWriteGamma + CodeWriteGolomb + CodeWriteUnary + CodeWriteZeta
            + CodeWriteUnary + CodeWriteFixedLength + WriteBit
        {
            /// Write a code using constant dispatcing
            fn write_code<const CODE: Code>(inner: &mut Self, value: usize) -> Result<()> {
                match CODE {
                    Code::Delta => inner.write_delta(value),
                    Code::Gamma => inner.write_gamma(value),
                    Code::Unary => inner.write_unary(value),
                    $(
                        Code::Golomb($c) => inner.write_golomb::<{ $c }>(value),
                        Code::Zeta($c) => inner.write_zeta::<{ $c }>(value),
                    )*
                    _ => unimplemented!("The given code is not implemented yet."),
                }
            }
        }

        /// Generic trait for data tat can read codes
        pub trait CodesRead:
            CodeReadDelta + CodeReadGamma + CodeReadGolomb + CodeReadUnary + CodeReadZeta
            + CodeReadUnary + CodeReadFixedLength + ReadBit
        {
            /// Read a code using constant dispatcing
            fn read_code<const CODE: Code>(inner: &mut Self) -> Result<usize> {
                match CODE {
                    Code::Delta => inner.read_delta(),
                    Code::Gamma => inner.read_gamma(),
                    Code::Unary => inner.read_unary(),
                    $(
                        Code::Golomb($c) => inner.read_golomb::<{ $c }>(),
                        Code::Zeta($c) => inner.read_zeta::<{ $c }>(),
                    )*
                    _ => unimplemented!("The given code is not implemented yet."),
                }
            }
        }
    };
}

impl_code_trait_parametric!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

/// blanket implementation
impl<T> CodesRead for T where
    T: CodeReadDelta
        + CodeReadGamma
        + CodeReadGolomb
        + CodeReadUnary
        + CodeReadZeta
        + CodeReadUnary
        + CodeReadFixedLength
        + ReadBit
{
}

/// blanket implementation
impl<T> CodesWrite for T where
    T: CodeWriteDelta
        + CodeWriteGamma
        + CodeWriteGolomb
        + CodeWriteUnary
        + CodeWriteZeta
        + CodeWriteUnary
        + CodeWriteFixedLength
        + WriteBit
{
}
