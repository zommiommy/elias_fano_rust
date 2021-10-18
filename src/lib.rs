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
//!
//! # Optimal Quantum
//! Simple Select stores the position of every `q`-th one and zero in the bitvec.
//! This hyperparameter `q` it's a tunable for the trade-off between time and memory.
//! 
//! To optimally set this we benchmarked both EliasFano and SparseIndex with different quantums.
//! In this experiment we store 32_000_000 values between 0 and 202_500_000_000 (0.01% density) on an AMD Ryzed 3900x (12 cores, 24 threads @ 4.0Ghz) averaged over 1_000 trials.
//! ![](https://raw.githubusercontent.com/zommiommy/elias_fano_rust/master/img/quantum_tradeoff.svg)
//! The definition of optimal in this case is highly dependant on the goal task, but we can observe that for these particular experiment
//! the quantums values of 1024 (10) and 2048 (11) offers good trade-offs.
//!
//! **TLDR**: [Vigna uses 256 (8)](https://shonan.nii.ac.jp/archives/seminar/029/wp-content/uploads/sites/12/2013/07/Sebastiano_Shonan.pdf) but 
//! in our implementatione we use 1024 (10) as the default quantum (`INDEX_SHIFT`) because it provide a better time-memory tradeoff.
#![feature(core_intrinsics)]

pub mod utils;
pub mod constants;
pub mod compact_array;
pub mod sparse_index;
pub mod elias_fano;
pub mod codes;
pub mod traits;
pub use codes::BitStream;
//pub mod webgraph;


#[cfg(feature="fuzz")]
pub mod fuzz;