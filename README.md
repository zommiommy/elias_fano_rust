# elias_fano_rust 
[![Build Status](https://travis-ci.com/zommiommy/elias_fano_rust.svg?branch=master)](https://travis-ci.com/zommiommy/elias_fano_rust) ![](https://img.shields.io/badge/rust-nightly-orange) ![](https://img.shields.io/badge/platform-linux--64%20%7C%20osx--64%20%7C%20win--64-lightgrey)

Rust implementation of [Sebastiano Vigna's elias fano](http://vigna.di.unimi.it/ftp/papers/QuasiSuccinctIndices.pdf).

Our goal is not to achieve best compression possible, instead we seek a good tradeoff between speed and memory.

We differ from the paper's implementation because we don't use a bitvector with skipping quantums for the high-bits but **currently** we use a Fully Indexable Dictionary.
This seems a reasonable idea because in SUX implementation of Elias Fano (SUX is one of Vigna's projects) he use simple select to store the high-bits.
Currently **we only need `select_0` and `select_1`** so better structures, to support select on the high-bits, might be explored in the futures.
For this reason one of the next steps is to implement `simple_select` as presented by Vigna in [Broadword Implementation of Rank/Select Queries](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).

# Rank and Select performances
We benchmark our library against all the data structures we found that supports rank and select.

The benchmark is done on a sorted vector of 1_000_000 values between 0 and 2_000_000.
For each run the select / rank is repeated 1_000 times with random inputs.
These are the results on my Ryzen 9 3900x 4Ghz 12c 24t.

```
test bio::rank                ... bench:      17,488 ns/iter (+/- 526)
test bio::select              ... bench:     151,620 ns/iter (+/- 6,668)
test ef::rank                 ... bench:      81,228 ns/iter (+/- 3,542)
test ef::select               ... bench:      49,509 ns/iter (+/- 1,761)
test fid::rank                ... bench:      35,865 ns/iter (+/- 1,213)
test fid::select              ... bench:     143,085 ns/iter (+/- 4,942)
test hashmap::select          ... bench:      78,803 ns/iter (+/- 6,984)
test indexed_bitvec::rank     ... bench:      52,563 ns/iter (+/- 1,369)
test indexed_bitvec::select   ... bench:     116,231 ns/iter (+/- 3,741)
test rsdict::rank             ... bench:      21,915 ns/iter (+/- 623)
test rsdict::select           ... bench:      45,674 ns/iter (+/- 317)
test succint::jacobson_rank   ... bench:      17,966 ns/iter (+/- 223)
test succint::jacobson_select ... bench:     509,892 ns/iter (+/- 6,706)
test succint::rank9_rank      ... bench:       9,068 ns/iter (+/- 245)
test succint::rank9_select    ... bench:     324,839 ns/iter (+/- 1,784)
test vec::rank                ... bench:     123,351 ns/iter (+/- 760)
test vec::select              ... bench:       3,880 ns/iter (+/- 61)
```

# Memory performances
TODO!
