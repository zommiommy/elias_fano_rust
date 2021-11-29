use crate::codes::*;
use crate::constants::*;
use crate::traits::*;
use crate::utils::*;
use crate::Result;
use alloc::vec::Vec;
use core::intrinsics::unlikely;
use core::mem::size_of;

/// A general BitArrayBig wrapper over some word reader and writers.
/// This assumes that the words of memory are read and write in little-endian;
///
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
///
/// let mut ba = BitArrayBig::new();
///
/// // write a pattern of single bits to the array
/// for _ in 0..513 {
///     ba.write_bit(true);
/// }
/// for _ in 0..513 {
///     ba.write_bit(false);
/// }
/// for i in 0..513 {
///     ba.write_bit(i % 2 == 0);
/// }
///
/// // rewind completely the BitArrayBig
/// ba.seek_bits(0);
///
/// // Ensure that we read back exacly the same pattern
/// for _ in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), true);
/// }
/// for _ in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), false);
/// }
/// for i in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), i % 2 == 0);
/// }  
///
/// // rewind completely the BitArrayBig
/// ba.clear();
/// let max = 9;
/// for i in (0..1 << max).rev() {
///     ba.write_fixed_length(max, i).unwrap();
/// }  
/// ba.seek_bits(0);
/// for i in (0..1 << max).rev() {
///     assert_eq!(ba.read_fixed_length(max).unwrap(), i);
/// }
///
/// // rewind completely the BitArrayBig
/// ba.clear();
/// for i in 0..513 {
///     ba.write_unary(i).unwrap();
/// }  
/// ba.seek_bits(0);
/// for i in 0..513 {
///     assert_eq!(ba.read_unary().unwrap(), i);
/// }  
/// ```
pub struct BitArrayBig(Vec<usize>);

impl MemoryFootprint for BitArrayBig {
    fn total_size(&self) -> usize {
        self.0.total_size() + size_of::<Self>()
    }
}

impl<'a> CodesReader<'a> for BitArrayBig {
    type CodesReaderType = BitArrayBigReader<'a>;

    fn get_codes_reader(&'a self, offset: usize) -> BitArrayBigReader<'a> {
        BitArrayBigReader::new(&self.0, offset)
    }
}

/// Reader on the Big-endian Bit Array
pub struct BitArrayBigReader<'a> {
    /// Reference to the data
    pub data: &'a [usize],
    /// Index that keeps track in which word we currently are
    pub word_index: usize,
    /// Index that keeps track in which bit we are in the current word
    pub bit_index: usize,
}

impl<'a> MemoryFootprint for BitArrayBigReader<'a> {
    fn total_size(&self) -> usize {
        size_of::<Self>()
    }
}

impl<'a> BitArrayBigReader<'a> {
    fn new(data: &'a [usize], offset: usize) -> BitArrayBigReader<'a> {
        BitArrayBigReader {
            data,
            word_index: offset >> WORD_SHIFT,
            bit_index: offset & WORD_BIT_SIZE_MASK,
        }
    }
}

impl BitArrayBig {
    /// Create a new empty bitarray
    pub fn new() -> BitArrayBig {
        BitArrayBig(vec![0])
    }

    /// Create a new empty bitarray that will be able to write `capacity` bits
    /// without having to allocate memory.
    pub fn with_capacity(capacity: usize) -> BitArrayBig {
        let mut data = Vec::with_capacity(capacity / (8 * size_of::<usize>()));
        data.push(0);

        BitArrayBig(data)
    }

    #[inline]
    /// Destroy the BitArrayBig wrapper and return the inner backend
    pub fn into_inner(self) -> Vec<usize> {
        self.0
    }

    /// Reset the vector keeping the memory allocation
    pub fn clear(&mut self) {
        self.0.clear();
        self.0.push(0);
    }
}

impl<'a> ReadBit for BitArrayBigReader<'a> {
    #[inline]
    /// Read a single bit
    fn read_bit(&mut self) -> Result<bool> {
        // TODO!; optimize?
        let code = self.data[self.word_index];
        let res = (code << self.bit_index) & WORD_HIGHEST_BIT_MASK;
        self.skip_bits(1)?;
        Ok(res != 0)
    }

    #[inline]
    /// Seek to the given bit_index
    fn seek_bits(&mut self, bit_index: usize) -> Result<()> {
        self.word_index = bit_index >> WORD_SHIFT;
        self.bit_index = bit_index & WORD_BIT_SIZE_MASK;
        Ok(())
    }

    #[inline]
    /// Return the current position (bit index) in the bit array
    fn tell_bits(&self) -> Result<usize> {
        Ok((self.word_index << WORD_SHIFT) | self.bit_index)
    }

    #[inline]
    /// Overriding optimized version
    fn skip_bits(&mut self, bit_offset: usize) -> Result<()> {
        // TODO!: is this faster than a tell + seek?
        self.bit_index += bit_offset;
        self.word_index += self.bit_index >> WORD_SHIFT;
        self.bit_index &= WORD_BIT_SIZE_MASK;
        Ok(())
    }
}

/// Optimal for gemetric distribution of ratio 1/2
impl<'a> CodeReadUnary for BitArrayBigReader<'a> {
    #[inline]
    fn read_unary(&mut self) -> Result<usize> {
        let mut res = 0;
        loop {
            let word = self.data[self.word_index] << self.bit_index;
            let x = word.leading_zeros() as usize;
            // if the code is not finished, continue to the next word
            let bound = WORD_BIT_SIZE - self.bit_index;
            if unlikely(x >= bound) {
                self.word_index += 1;
                self.bit_index = 0;
                res += bound;
                continue;
            }

            // the code finish here
            self.skip_bits(1 + x)?;
            return Ok(x + res);
        }
    }
}

/// Optimized implementation that exploit the fact that all the data is already
/// in memory
impl<'a> CodeReadFixedLength for BitArrayBigReader<'a> {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize> {
        // Compute how many bits we are going to write to each word
        let space_left = WORD_BIT_SIZE - self.bit_index;
        let first_word_number_of_bits = number_of_bits.min(space_left as usize);
        let second_word_number_of_bits = number_of_bits - first_word_number_of_bits;

        // read the data from the current word
        let mut first_word_bits = self.data[self.word_index]
            >> WORD_BIT_SIZE.saturating_sub(self.bit_index + number_of_bits);
        first_word_bits &= power_of_two_to_mask(first_word_number_of_bits);
        // read the next word, this implies that we will always have one
        // extra word in the data stream
        let mut second_word_bits = self.data[self.word_index + 1]
            .checked_shr((WORD_BIT_SIZE - second_word_number_of_bits) as _)
            .unwrap_or(0);
        second_word_bits &= power_of_two_to_mask(second_word_number_of_bits);

        // concatenate the data from the two wordsnext
        let aligned_data = (first_word_bits << second_word_number_of_bits) | second_word_bits;

        // clear off the excess bits.
        // we shall keep only the lower `number_of_bits` bits.
        let result = aligned_data & power_of_two_to_mask(number_of_bits as _);

        // Update the pointers to where we read
        self.skip_bits(number_of_bits as usize)?;

        Ok(result)
    }
}

/// Use the big-endian version
impl<'a> CodeReadMinimalBinary for BitArrayBigReader<'a> {
    #[inline]
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.read_minimal_binary_big(max)
    }
}
