// Writing a own bitboard function to keep clean code
#[derive(Debug, Copy, Clone)]
pub struct Bitboard {
    pub data: u64,
}

impl Bitboard {
    // Constructor
    pub fn new(data: u64) -> Bitboard {
        Bitboard { data }
    }
    pub fn default() -> Bitboard {
        Bitboard { data: 0 }
    }
    // Simple Bit Operations

    // Method to set a specific bit at a given position to 1
    pub fn set_bit(&mut self, position: u8) {
        self.data |= 1u64 << position;
    }

    // Method to check if a specific bit at a given position is set
    pub fn is_bit_set(&self, position: u8) -> bool {
        (self.data & (1u64 << position)) != 0
    }

    // Method to clear a specific bit at a given position
    pub fn clear_bit(&mut self, position: u8) {
        self.data &= !(1u64 << position);
    }

    // Method to print the binary representation of the bitboard
    pub fn print_binary_representation(&self) {
        for row in (0..8).rev() {
            for col in (0..8).rev() {
                let position = row * 8 + col;
                let bit_value = if self.is_bit_set(position as u8) {
                    '1'
                } else {
                    '0'
                };
                print!("{} ", bit_value);
            }
            println!();
        }
        println!();
    }

    // Simple Set Theory Operations
    pub fn bitwise_and(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data & other.data,
        }
    }

    pub fn bitwise_or(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data | other.data,
        }
    }

    pub fn bitwise_xor(&self, other: &Bitboard) -> Bitboard {
        Bitboard {
            data: self.data ^ other.data,
        }
    }

    pub fn bitwise_eq(&self, other: &Bitboard) -> bool {
        self.data == other.data
    }

    // subfunction to calculate the leasst sigificant one
    pub fn twos_complement_negation(&self) -> Bitboard {
        Bitboard {
            data: !self.data + 1,
        }
    }

    // caclulates the least sigificant one as bitboard
    pub fn least_sigificant_one(&self) -> Bitboard {
        return self.bitwise_and(&self.twos_complement_negation());
    }

    // calculates simply -1 interpreting the bitboard as int
    pub fn minus_one(&self) -> Bitboard {
        Bitboard {
            data: self.data - 1,
        }
    }

    // counts all ones
    pub fn count_bits(mut data: u64) -> u8 {
        let mut counter = 0u8;
        while data != 0u64 {
            counter += 1;

            data &= data - 1;
        }
        return counter;
    }

    // Bitscan
    pub fn get_bit_position(mut data: u64) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        while data != 0u64 {
            // least significant one
            let temp_data = (data & !data + 1) - 1;
            output.push(Self::count_bits(temp_data) + 1);
            for &elemnt in output.iter() {
                println!("{}", elemnt);
            }

            data ^= temp_data;
        }
        return output;
    }
}

pub struct StaticBitboards;

impl StaticBitboards {
    // Associated constants for file bitboards
    pub const FILE_A: Bitboard = Bitboard {
        data: 0b1000000010000000100000001000000010000000100000001000000010000000,
    };
    pub const FILE_B: Bitboard = Bitboard {
        data: 0b0100000001000000010000000100000001000000010000000100000001000000,
    };
    pub const FILE_C: Bitboard = Bitboard {
        data: 0b0010000000100000001000000010000000100000001000000010000000100000,
    };
    pub const FILE_D: Bitboard = Bitboard {
        data: 0b0001000000010000000100000001000000010000000100000001000000010000,
    };
    pub const FILE_E: Bitboard = Bitboard {
        data: 0b0000100000001000000010000000100000001000000010000000100000001000,
    };
    pub const FILE_F: Bitboard = Bitboard {
        data: 0b0000010000000100000001000000010000000100000001000000010000000100,
    };
    pub const FILE_G: Bitboard = Bitboard {
        data: 0b0000001000000010000000100000001000000010000000100000001000000010,
    };
    pub const FILE_H: Bitboard = Bitboard {
        data: 0b0000000100000001000000010000000100000001000000010000000100000001,
    };
    // Associatet constatns for file^T bitboards
    pub const RANK_1: Bitboard = Bitboard {
        data: 0b0000000000000000000000000000000000000000000000000000000011111111,
    };
    pub const RANK_2: Bitboard = Bitboard {
        data: 0b0000000000000000000000000000000000000000000000001111111100000000,
    };
    pub const RANK_3: Bitboard = Bitboard {
        data: 0b0000000000000000000000000000000000000000111111110000000000000000,
    };
    pub const RANK_4: Bitboard = Bitboard {
        data: 0b0000000000000000000000000000000011111111000000000000000000000000,
    };
    pub const RANK_5: Bitboard = Bitboard {
        data: 0b0000000000000000000000001111111100000000000000000000000000000000,
    };
    pub const RANK_6: Bitboard = Bitboard {
        data: 0b0000000000000000111111110000000000000000000000000000000000000000,
    };
    pub const RANK_7: Bitboard = Bitboard {
        data: 0b0000000011111111000000000000000000000000000000000000000000000000,
    };
    pub const RANK_8: Bitboard = Bitboard {
        data: 0b1111111100000000000000000000000000000000000000000000000000000000,
    };
    // Associatet constatns for Edge and Corner
    pub const EDGE: Bitboard = Bitboard {
        data: 0b1111111110000001100000011000000110000001100000011000000111111111,
    };
    pub const CORNER: Bitboard = Bitboard {
        data: 0b1000000010000000000000000000000000000000000000000000000010000001,
    };
    pub const EMPTY: Bitboard = Bitboard {
        data: 0b0000000000000000000000000000000000000000000000000000000000000000,
    };
}
