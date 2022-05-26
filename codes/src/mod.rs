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

mod tables;
pub(crate) use tables::*;

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

/// A collection of zero-sized types that can be used to select / dispatch 
/// the code to use at compile time. The alternative is to use a const enum
/// but it's nor stable nor complete yet. 
/// Specifically it depends on:
/// `#![feature(adt_const_params)]` and `#![feature(generic_const_exprs)]`.
pub mod selectors {
    pub(crate) trait CodeSelector {}

    pub struct Delta();
    pub struct Gamma();
    pub struct SkewedGolomb();
    pub struct Unary();
    pub struct Nibble();
    pub struct Zeta<const K: usize>(core::marker::PhantomData<[(); K]>);
    pub struct Golomb<const B: usize>(core::marker::PhantomData<[(); B]>);

    impl CodeSelector for Delta {}
    impl CodeSelector for Gamma {}
    impl CodeSelector for SkewedGolomb {}
    impl CodeSelector for Unary {}
    impl CodeSelector for Nibble {}
    impl<const K: usize> CodeSelector for Zeta<K> {}
    impl<const B: usize> CodeSelector for Golomb<B> {}
}


// CodeReadDelta + CodeReadGamma + CodeReadGolomb + CodeReadUnary 
// + CodeReadZeta + CodeReadUnary + CodeReadFixedLength + ReadBit


// CodeWriteDelta + CodeWriteGamma + CodeWriteGolomb + CodeWriteUnary 
// + CodeWriteZeta + CodeWriteUnary + CodeWriteFixedLength + WriteBit

pub trait CodeReader<Code: selectors::CodeSelector> {
    fn read_code(&mut self) -> Result<usize>;
}

pub trait CodeWriter<Code: selectors::CodeSelector> {
    fn write_code(&mut self, value: usize) -> Result<()>;
}

/// Generic trait for data tat can write codes
pub trait WriteCode {
    /// Write a code using constant dispatcing
    fn write_code<Code: selectors::CodeSelector>(&mut self, value: usize) -> Result<()>;
}

/// Generic trait for data tat can read codes
pub trait ReadCode {
    /// Read a code using constant dispatcing
    fn read_code<Code: selectors::CodeSelector>(&mut self) -> Result<usize>;
}

/// blanket implementation
impl<T> ReadCode for T
{
    #[inline]
    fn read_code<Code: selectors::CodeSelector>(&mut self) -> Result<usize> {
        <Self as CodeReader<Code>>::read_code(self)
    }
}

/// blanket implementation
impl<T> WriteCode for T
{
    #[inline]
    fn write_code<Code: selectors::CodeSelector>(&mut self, value: usize) -> Result<()> {
        <Self as CodeWriter<Code>>::write_code(self, value)
    }
}

/// A trait for a data-structure that can instantiate multiple writers
/// (but only one at time can work)
pub trait GetCodesWriter {
    /// The writer returend
    type CodesWriterType: WriteCode;
    /// Get a newly instantiated writer, there can only be one active at time.
    /// The writer will be initialized to the end of the stream (append mode)
    ///
    /// Here self is borrowed immutabily because the implementer must guarantee
    /// the thread safety of the implementation. (Tipically we could use a
    /// RWLock).
    fn get_codes_writer(&self) -> Self::CodesWriterType;
}

/// A trait for a datastructure that can instantiate multiple readers
pub trait GetCodesReader {
    /// The reader returend
    type CodesReaderType<'a>: ReadCode
    where
        Self: 'a;
    /// Get a new reader at the given offset (in bytes) of the stream
    fn get_codes_reader(&'_ self, offset: usize) -> Self::CodesReaderType<'_>;
}
