use super::minimal_binary::CodeMinimalBinary;
use super::fixed_length::CodeFixedLength;
use crate::traits::*;
use crate::utils::fast_log2_ceil;

/// Binary Interpolative Coding.
/// 
/// The default implementation is based on the report:
/// "On Implementing the Binary Interpolative Coding Algorithm" by 
/// Giulio Ermano Pibiri
/// 
/// TODO!: Finish the implementation and figure out how to encode metadata
pub trait CodeInterpolative: CodeMinimalBinary + CodeFixedLength {
    #[inline]
    fn read_interpolative(&mut self) -> Result<Vec<usize>, Error> {

    }

    #[inline]
    fn write_interpolative(&mut self, values: Vec<usize>) 
        -> Result<(), Error> {
        debug_assert!(values.is_sorted());

        match values.len() {
            0 => Err(Error::InterpolativeCodeWithEmptyArray),
            1 => self.write_fixed_length(values[0], fast_log2_ceil(values[0])),
            length @ _ => {
                let min = *values.first().unwrap();
                let max = *values.last().unwrap();
                recursive_interpolative_write(
                    self, &values[..], 
                    length, min, max
                )
            }
        }
    }
}

// blanket implementation
impl<T> CodeInterpolative for T 
where  
    T: CodeMinimalBinary + CodeFixedLength
{}

/// recursive routine that encode the slices of values using the interpolative
/// encoding.
fn recursive_interpolative_write<BACKEND>(backend: BACKEND, values: &[usize], 
    index: usize, lower_bound: usize, higher_bound: usize)
    -> Result<(), Error> 
where
    BACKEND: CodeMinimalBinary
{
    debug_assert!(lower_bound <= higher_bound);
    let middle_index = index / 2;
    let element = values[middle_index];
    backend.write_minimal_binary(
        element - lower_bound - middle_index,
        higher_bound - lower_bound - index + 1 
    )?;
    recursive_interpolative_write(
        backend, values, 
        middle_index, lower_bound, element - 1 
    )?;
    recursive_interpolative_write(
        backend, &values[middle_index + 1..], 
        middle_index, lower_bound, element - 1 
    )?;
    Ok(())
}