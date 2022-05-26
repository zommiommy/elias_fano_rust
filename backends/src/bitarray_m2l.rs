use codes::*;
use crate::constants::*;
use crate::traits::*;
use crate::Result;
use core::intrinsics::likely;

/// A general BitArrayM2L wrapper over some word reader and writers.
/// This assumes that the words of memory are read and write from the MSB to the
/// LSB;
///
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
///
/// let mut ba = BitArrayM2L::new();
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
/// // rewind completely the BitArrayM2L
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
/// // rewind completely the BitArrayM2L
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
/// // rewind completely the BitArrayM2L
/// ba.clear();
/// for i in 0..513 {
///     ba.write_unary(i).unwrap();
/// }  
/// ba.seek_bits(0);
/// for i in 0..513 {
///     assert_eq!(ba.read_unary().unwrap(), i);
/// }  
/// ```
pub struct BitArrayM2L<BACKEND: MemorySlice>(BACKEND);

impl<BACKEND: MemorySlice> CodesReader for BitArrayM2L<BACKEND> {
    type CodesReaderType<'a> = BitArrayM2LReader<'a, BACKEND>
    where
        Self: 'a;

    fn get_codes_reader(&self, offset: usize) -> Self::CodesReaderType<'_> {
        BitArrayM2LReader::new(&self.0, offset)
    }
}

impl<BACKEND: MemorySlice> BitArrayM2L<BACKEND> {
    pub fn new(data: BACKEND) -> BitArrayM2L<BACKEND> {
        // TODO!: maybe we have to push 0 at the end
        BitArrayM2L(data)
    }

    #[inline]
    /// Destroy the BitArrayM2L wrapper and return the inner backend
    pub fn into_inner(self) -> BACKEND {
        self.0
    }
}


/// Reader on the Big-endian Bit Array
pub struct BitArrayM2LReader<'a, BACKEND: MemorySlice + 'a> {
    /// Reference to the data
    pub data: &'a BACKEND,

    /// Index that keeps track of which bit are we at
    pub offset: usize,
}

impl<'a, BACKEND: MemorySlice + 'a> BitArrayM2LReader<'a, BACKEND> {
    fn new(data: &'a BACKEND, offset: usize) -> BitArrayM2LReader<'a, BACKEND> {
        BitArrayM2LReader {
            data,
            offset,
        }
    }

    #[inline(always)]
    /// Get the word index of the current offset
    fn get_word_index(&self) -> usize {
        self.offset >> WORD_SHIFT
    }

    #[inline(always)]
    /// Get the index of the bit in the given word
    fn get_bit_index(&self) -> usize {
        self.offset & WORD_BIT_SIZE_MASK
    }
}

impl<'a, BACKEND: MemorySlice + 'a> ReadBit for BitArrayM2LReader<'a, BACKEND> {
    #[inline]
    /// Read a single bit
    fn read_bit(&mut self) -> Result<bool> {
        let code = self.data[self.get_word_index()].to_be();
        let res = (code << self.get_bit_index()) & WORD_HIGHEST_BIT_MASK;
        self.skip_bits(1)?;
        Ok(res != 0)
    }

    #[inline]
    fn peek_byte(&mut self) -> Result<u8> {
        // this is horrible, TODO: find a safe way to to the same thing
        Ok(unsafe{
            let ptr =  (self.data.as_ptr() as *const u8)
                .add(self.offset >> 3)
                as * const u16;

            let mut value = (*ptr).to_be();
            value <<= self.offset & 7;
            value >>= 8;
            value as u8
        })
    }

    #[inline]
    /// Seek to the given bit_index
    fn seek_bits(&mut self, bit_index: usize) -> Result<()> {
        self.offset = bit_index;
        Ok(())
    }

    #[inline]
    /// Return the current position (bit index) in the bit array
    fn tell_bits(&self) -> Result<usize> {
        Ok(self.offset)
    }
}

/// Use the msb to lsb version
impl<'a, BACKEND: MemorySlice + 'a> CodeReadMinimalBinary 
    for BitArrayM2LReader<'a, BACKEND> {
    #[inline]
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.read_minimal_binary_m2l(max)
    }
}

/// Optimal for gemetric distribution of ratio 1/2
impl<'a, BACKEND: MemorySlice + 'a> CodeReadUnary 
    for BitArrayM2LReader<'a, BACKEND> {
    #[inline]
    fn read_unary(&mut self) -> Result<usize> {
        #[cfg(feature = "code_tables")]
        {
            // check if the value is in one of the tables
            let (res, len) = UNARY_TABLE[self.peek_byte()? as usize];
            // if the value was in the table, return it and offset
            if len != 0 {
                self.skip_bits(len as usize)?;
                return Ok(res as usize)
            }
        }
        
        // fallback to the implementation
        let mut word_index = self.get_word_index();
        let bit_index = self.get_bit_index();

        let mut word = self.data[word_index].to_be() << bit_index;
        let bits_in_word = WORD_BIT_SIZE - bit_index;
        let mut n_of_zeros = word.leading_zeros() as usize;

        // Unary codes are meant to be distrubted geometrically, so most codes
        // will fit in a singol word, thus we should make this the fast case
        if likely(n_of_zeros < bits_in_word) {
            // skip the 1
            self.skip_bits(n_of_zeros + 1)?;
            return Ok(n_of_zeros);
        }

        self.skip_bits(bits_in_word)?;
        let mut acc = bits_in_word;
        word_index += 1;

        // otherwise continue scanning the next words
        loop {
            // parse the current word
            word = self.data[word_index].to_be();
            n_of_zeros = word.leading_zeros() as usize;
            self.skip_bits(n_of_zeros)?;

            // check if the code is finished (fast path because more probable)
            if likely(n_of_zeros < WORD_BIT_SIZE) {
                // skip the 1
                self.skip_bits(1)?;
                return Ok(n_of_zeros + acc);
            }

            // go to the next word
            word_index += 1;
            acc += WORD_BIT_SIZE;
        }
    }
}

/// Optimized implementation that exploit the fact that all the data is already
/// in memory
impl<'a, BACKEND: MemorySlice + 'a> CodeReadFixedLength 
    for BitArrayM2LReader<'a, BACKEND> {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize> {
        // unaligned accesses are about 0.5% slower than aligned ones, so 
        // we can just avoid doing math and directly read an unaligned value
        // fixing only the alignement in a byte. If we accept to potentially 
        // lose 7 bits we can just don't fix the code for performance sake.
        // If we **actually** need more than 57 bits (improbable due to the 
        // useage of these codes), we could read the extra byte and merge it
        // at the cost of a bit of overhead.
        debug_assert!(number_of_bits <= WORD_BIT_SIZE - 7);

        // TODO!: verify if this fast path actually speed up
        if number_of_bits == 0 {
            return Ok(0);
        }

        // get the offset byte-aligned of the values we want.
        let byte_addr = self.offset >> 3;
        
        let mut word =  unsafe{
            *((self.data.as_ptr() as *const u8).add(byte_addr)
            as *const usize)
        }.to_be();
        
        // remove the bits before the start
        word <<= self.offset & 7;
        // move the bits from the MSB to the LSB
        word >>= WORD_BIT_SIZE - number_of_bits;

        // Update the pointers to where we read
        self.skip_bits(number_of_bits as usize)?;

        Ok(word)
    }
}