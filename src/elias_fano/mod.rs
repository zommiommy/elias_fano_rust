//! # Elias-Fano
//! Implementaiton of the Elias-Fano by Sebastiano Vigna.
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
use super::{
    compact_array::CompactArray,
    sparse_index::SparseIndex,
};

mod elias_fano;
pub use elias_fano::*;

mod builders;
mod hash;
mod getters;

mod iter;

#[cfg(feature="par_iter")]
mod par_iter;

mod concurrent_builder;
pub use concurrent_builder::*;