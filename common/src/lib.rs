//! # Elias Fano
//! In this crate we provide two main datastructure:
//! - EliasFano: Sebastiano Vigna's Quasi-Succint data-structure (good for sparse sets)
//! - SparseIndex: Simple bitmap (good for dense sets)
//!
//! # Elias Fano
//! This is a data-structure used to store a **sorted set of `n` positive integers with upper bound `u`** in space close to the minimum theoretical bound,
//! and allows for constant operations (on average).
//!
//! ![](https://raw.githubusercontent.com/zommiommy/elias_fano_rust/master/img/elias_fano.png)
//!
//! ```rust
//! use elias_fano_rust::elias_fano::EliasFano;
//!
//!
//! ```
//!
//! # SparseIndex
//!
//! # Performance over a sparse set
//! These are benchmarks of storing 32_000_000 values between 0 and 202_500_000_000 (0.01% density) on an AMD Ryzed 3900x (12 cores, 24 threads @ 4.0Ghz) averaged over 1_000 trials.
//!
//! # Time
//!
//! | Data-Structure | Rank Time (ns)    | Select Time (ns)   | Memory Usage (MiB) | Redundant size (MiB) | Bits per value |
//! |----------------|-------------------|--------------------|--------------------|----------------------|----------------|
//! | EliasFano      | 66.189 +/- 0.255  | 148.410 +/-  3.405 | 55.9616            | 0.6069               | 14.67          |
//! | SparseIndex   | 131.986 +/- 3.129 | 109.128 +/- 11.826 | 271.2588           | 15.25                | 71.10          |
//! | Sorted Vec     | 69.085 +/- 1.658  | 16.865  +/-  0.989 | 244.1406           | /                    | 64             |
//! | HashMap        | /  | 78.803  +/-  6.984 | 1478.4           | /                   |  387.5 |
//!
//! In this contest, Redunant size is how much memory is used for indices to speed-up the **rank** and **select** operations.
//! The Redundant size is **included** in the Memory usage.
//!
//! Comparisons against other succint datastructures: (TODO: figure out memory measurements)
//!
//! | Data-Structure | Rank Time (ns)    | Select Time (ns)   | Memory Usage (MiB) |
//! |----------------|-------------------|--------------------|--------------------|
//! | [bio](https://docs.rs/bio/0.38.0/bio/)                         | 17.488 +/- 0.526 | 151.620 +/- 6.668 | ? |
//! | [fid](https://docs.rs/fid/0.1.7/fid/)                          | 35.865 +/- 1.213 | 143.085 +/- 4.942 | ? |
//! | [rsdict](https://docs.rs/fid/0.1.7/fid/)                       | 21.915 +/- 0.623 | 45.674 +/- 0.317  | ? |
//! | [succint::jacobson](https://docs.rs/succinct/0.5.2/succinct/)  | 17.966 +/- 0.223 | 509.892 +/- 6.706 | ? |
//! | [succint::rank9](https://docs.rs/succinct/0.5.2/succinct/)     | 9.068 +/- 0.245  | 324.839 +/-1.784  | ? |
#![feature(core_intrinsics)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

//#![deny(missing_docs)]
//#![deny(clippy::missing_docs_in_private_items)]
//#![deny(clippy::missing_safety_doc)]
//#![warn(rustdoc::missing_doc_code_examples)]
//#![warn(clippy::todo)]

// No std so that all these structures can be used in
// bare metal environments (not making any assumption about architecture width),
// but we require to have an allocator
//#![no_std]
#[macro_use]
extern crate alloc;

mod errors;
pub use errors::*;
pub mod codes;
pub mod compact_array;
pub(crate) mod constants;
pub mod elias_fano;
pub mod sparse_index;
pub mod traits;
pub use codes::*;
pub mod backends;
pub mod webgraph;
pub mod utils;

/// Simple prelude module that import everything
pub mod prelude {
    pub use super::backends::*;
    pub use super::codes::*;
    pub use super::compact_array::CompactArray;
    pub use super::elias_fano::{ConcurrentEliasFanoBuilder, EliasFano};
    pub use super::sparse_index::SparseIndex;
    pub use super::traits::*;
    pub use super::utils;
    pub use super::webgraph::*;
}

#[cfg(feature = "fuzz")]
pub mod fuzz;
