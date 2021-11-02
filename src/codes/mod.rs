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
pub use delta::CodeDelta;

mod fixed_length;
pub use fixed_length::CodeFixedLength;

mod gamma;
pub use gamma::CodeGamma;

mod golomb;
pub use golomb::CodeGolomb;

// mod interpolative;
// pub use interpolative::CodeInterpolative;

mod minimal_binary;
pub use minimal_binary::CodeMinimalBinary;

mod minimal_binary_bv;
pub use minimal_binary_bv::CodeMinimalBinaryBV;

mod unary;
pub use unary::CodeUnary;

mod var_length;
pub use var_length::CodeVarLength;

mod zeta;
pub use zeta::CodeZeta;

pub use golomb::compute_optimal_golomb_block_size;

