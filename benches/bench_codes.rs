#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate rand;
use elias_fano_rust::*;

extern crate test;
use test::{Bencher, black_box};

const MAX: u64 = 10_000;
const MAX_LOG_2: u64 = utils::fast_log2_ceil(MAX);
const CAPACITY: u64 = 1 << 20;

mod unary {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_unary(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_unary());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_unary(i);
            }
        })
    }
}

mod gamma {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_gamma(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_gamma());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_gamma(i);
            }
        })
    }
}

mod delta {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_delta(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_delta());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_delta(i);
            }
        })
    }
}

mod golomb_2 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_golomb::<2>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_golomb::<2>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_golomb::<2>(i);
            }
        })
    }
}

mod golomb_3 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_golomb::<3>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_golomb::<3>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_golomb::<3>(i);
            }
        })
    }
}

mod golomb_4 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_golomb::<4>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_golomb::<4>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_golomb::<4>(i);
            }
        })
    }
}

mod var_length_2 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_var_length::<2>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_var_length::<2>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_var_length::<2>(i);
            }
        })
    }
}

mod var_length_3 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_var_length::<3>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_var_length::<3>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_var_length::<3>(i);
            }
        })
    }
}

mod var_length_4 {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_var_length::<4>(i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_var_length::<4>());
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_var_length::<4>(i);
            }
        })
    }
}

mod min_bin {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_minimal_binary(i, MAX);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_minimal_binary(MAX));
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_minimal_binary(i, MAX);
            }
        })
    }
}

mod min_bin_bv {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_minimal_binary_bv(i, MAX);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_minimal_binary_bv(MAX));
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_minimal_binary_bv(i, MAX);
            }
        })
    }
}

mod fixed_length {
    use super::*;

    #[bench]
    fn read(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        for i in 0..MAX {
            bs.write_bits(MAX_LOG_2, i);
        }
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                assert_eq!(i, bs.read_bits(MAX_LOG_2));
            }
        })
    }

    #[bench]
    fn write(b: &mut Bencher) {
        let mut bs = BitStream::with_capacity(CAPACITY as _);
        b.iter(|| {
            bs.seek(0);
            for i in 0..MAX {
                bs.write_bits(MAX_LOG_2, i);
            }
        })
    }
}