//! # Elias Fano
//! In this crate we provide two main datastructure:
//! - EliasFano: Sebastiano Vigna's Quasi-Succint data-structure (good for sparse sets)
//! - SimpleSelect: Simple bitmap (good for dense sets)
//!
//! # Elias Fano
//! This is a data-structure used to store a **sorted set of positive integers** in space close to the minimum theoretical bound,
//! and allows for constant operations (on average).
//!
//! ```rust
//! use elias_fano_rust::EliasFano;
//!
//! 
//! ```
//!
//! # SimpleSelect
//!
//! # Performance over a sparse set
//! These are benchmarks of storing 32_000_000 values between 0 and 202_500_000_000 (0.01% density) on an AMD Ryzed 3900x (12 cores, 24 threads @ 4.0Ghz) averaged over 1_000 trials.
//!
//! # Time
//!
//! | Data-Structure | Rank Time (ns)    | Select Time (ns)   | Memory Usage (MiB) | Redundant size (MiB) |
//! |----------------|-------------------|--------------------|--------------------|----------------------|
//! | EliasFano      | 66.189 +/- 0.255  | 148.410 +/-  3.405 | 55.9616            | 0.6069               |
//! | SimpleSelect   | 131.986 +/- 3.129 | 109.128 +/- 11.826 | 271.2588           | 15.25                |
//! | Sorted Vec     | 69.085 +/- 1.658  | 16.865  +/-  0.989 | 244.1406           | /                    |
//! | HashMap        | ?  | 78.803  +/-  6.984 | ?           | ?                   | / |
//! | BTreeMap       | ?  | 78.803  +/-  6.984 | ?           | ?                   | / |
//!
//! In this contest, Redunant size is how much memory is used for indices to speed-up the **rank** and **select** operations.
//! The Redundant size is **included** in the Memory usage.
//! 
//! Comparisons against other succint datastructures: 
//!
//! | Data-Structure | Rank Time (ns)    | Select Time (ns)   | Memory Usage (MiB) |
//! |----------------|-------------------|--------------------|--------------------|
//! | [bio](https://docs.rs/bio/0.38.0/bio/)                         | 17.488 +/- 0.526 | 151.620 +/- 6.668 | ? |
//! | [fid](https://docs.rs/fid/0.1.7/fid/)                          | 35.865 +/- 1.213 | 143.085 +/- 4.942 | ? |
//! | [rsdict](https://docs.rs/fid/0.1.7/fid/)                       | 21.915 +/- 0.623 | 45.674 +/- 0.317  | ? |
//! | [succint::jacobson](https://docs.rs/succinct/0.5.2/succinct/)  | 17.966 +/- 0.223 | 509.892 +/- 6.706 | ? |
//! | [succint::rank9](https://docs.rs/succinct/0.5.2/succinct/)     | 9.068 +/- 0.245  | 324.839 +/-1.784  | ? |
//!
//! # Quantum
//! Simple Select stores the position of every q-th one and zero in the bitvec.
//! This hyperparameter `q` it's a tunable for the trade-off between time and memory.
//!
//! ![](https://raw.githubusercontent.com/zommiommy/elias_fano_rust/master/img/quantum_tradeoff.svg)
#![feature(core_intrinsics)]

mod low_bits_primitives;
#[cfg(feature="fuzz")]
pub use low_bits_primitives::*;
#[cfg(not(feature="fuzz"))]
pub(crate) use low_bits_primitives::*;

mod constants;
#[cfg(feature="fuzz")]
pub use constants::*;
#[cfg(not(feature="fuzz"))]
pub(crate) use constants::*;

mod elias_fano;
mod builders;
mod utils_methods;
pub use elias_fano::*;

mod simple_select;
pub use simple_select::*;

mod hash;
mod getters;

mod iter;
mod par_iter;

mod concurrent_builder;
pub use concurrent_builder::*;

#[cfg(feature="fuzz")]
mod fuzz_harnesses;
#[cfg(feature="fuzz")]
pub use fuzz_harnesses::*;