# elias_fano_rust
Rust implementation of elias fano 

## Rank Benchmarks
```
test rank_100_000_000_dense  ... bench:   1,036,533 ns/iter (+/- 93,730)
test rank_100_000_000_ef     ... bench:     273,980 ns/iter (+/- 30,074)
test rank_100_000_000_normal ... bench:     889,776 ns/iter (+/- 22,159)
test rank_100_000_000_sparse ... bench:     903,448 ns/iter (+/- 91,389)
test rank_100_000_000_vec    ... bench:   4,244,086 ns/iter (+/- 161,835)
test rank_1_000_000_dense    ... bench:     967,104 ns/iter (+/- 84,169)
test rank_1_000_000_ef       ... bench:     322,767 ns/iter (+/- 31,899)
test rank_1_000_000_normal   ... bench:     425,776 ns/iter (+/- 42,014)
test rank_1_000_000_sparse   ... bench:     832,967 ns/iter (+/- 84,200)
test rank_1_000_000_vec      ... bench:   1,337,075 ns/iter (+/- 100,844)
```

## Select Benchmarks
```
test select_100_000_000_dense  ... bench:   1,110,009 ns/iter (+/- 92,790)
test select_100_000_000_ef     ... bench:   4,152,309 ns/iter (+/- 195,444)
test select_100_000_000_normal ... bench:   2,508,712 ns/iter (+/- 66,709)
test select_100_000_000_sparse ... bench:   2,534,860 ns/iter (+/- 132,341)
test select_100_000_000_vec    ... bench:     216,489 ns/iter (+/- 41,323)
test select_1_000_000_dense    ... bench:     875,700 ns/iter (+/- 84,727)
test select_1_000_000_ef       ... bench:   2,224,447 ns/iter (+/- 385,916)
test select_1_000_000_normal   ... bench:   1,786,589 ns/iter (+/- 106,175)
test select_1_000_000_sparse   ... bench:   2,333,858 ns/iter (+/- 137,544)
test select_1_000_000_vec      ... bench:      78,264 ns/iter (+/- 3,390)
```

## Memory

```
Vector:     400000
BitVector:  15816
Elias Fano: 42824
```