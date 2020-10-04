use super::*;

impl EliasFano {
    /// Return the number of **bits** used by the structure

    pub fn debug(&self) {
        println!("------------ELIAS-FANO------------------");
        println!("\tuniverse: {}", self.universe);
        println!("\tnumber_of_elements: {}", self.number_of_elements);
        println!(
            "\tcurrent_number_of_elements: {}",
            self.current_number_of_elements
        );
        println!("\tlast_value: {}", self.last_value);
        println!("\tlow_bit_count: {}", self.low_bit_count);
        println!("\tlow_bit_mask: {}", self.low_bit_mask);
        if self.number_of_elements < 10 {
            println!("---------------low-bits-----------------");
            for i in 0..self.number_of_elements {
                print!("{}, ", self.read_lowbits(i));
            }
            println!("\n--------------high-bits-----------------");
            for i in 0..self.high_bits.len() {
                print!("{}", self.high_bits.get_bit(i as u64) as u64);
            }
            println!("\n--------------values--------------------");
            for v in self.iter() {
                print!("{}, ", v);
            }
        }
        println!("\n----------------END---------------------");
    }
}