// Writing a own bitboard function to keep clean code
struct Bitboard {
    data: u64,
}

impl Bitboard {
    // Constructor
    fn new(data: u64) -> Bitboard {
        Bitboard {data}
    }
    // Simple Bit Operations

    // Method to set a specific bit at a given position to 1
    fn set_bit(&mut self, position: u8) {
        self.data |= 1u64 << position;
    }

    // Method to check if a specific bit at a given position is set
    fn is_bit_set(&self, position: u8) -> bool {
        (self.data & (1u64 << position)) != 0
    }

    // Method to clear a specific bit at a given position
    fn clear_bit(&mut self, position: u8) {
        self.data &= !(1u64 << position);
    }

    // Method to print the binary representation of the bitboard
    fn print_binary_representation(&self) {
        for row in (0..8).rev() {
            for col in (0..8).rev() {
                let position = row * 8 + col;
                let bit_value = if self.is_bit_set(position as u8) { '1' } else { '0' };
                print!("{} ", bit_value);
            }
            println!();
        }
        println!();
    }

    // Simple Set Theory Operations
    fn bitwise_and(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data & other.data,
        }
    }

    fn bitwise_or(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data | other.data,
        }
    }

    fn bitwise_xor(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data ^ other.data,
        }
    }
    
}

fn main() {
    let mut my_bitboard = Bitboard::new(0);
    let mut my_bitboard2 = Bitboard::new(0);
    my_bitboard2.set_bit(7);
    my_bitboard.set_bit(3);
    my_bitboard.set_bit(9);
    my_bitboard.print_binary_representation();
    println!("{}", my_bitboard.is_bit_set(4));
    my_bitboard.clear_bit(3);
    my_bitboard.print_binary_representation();
    let both = my_bitboard.bitwise_xor(&my_bitboard2);
    both.print_binary_representation()
}