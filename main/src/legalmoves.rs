use crate::bitboard::Bitboard;
use crate::bitboard::StaticBitboards;
use crate::chessboard::{ChessBoard, Color, PieceType};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    NORTH = 8,
    WEST = 1,
    SOUTH = -8,
    EAST = -1,
    NORTHWEST = 9,
    NORTHEAST = 7,
    SOUTHWEST = -7,
    SOUTHEAST = -9,
}

pub fn get_pawn_attacks(position: usize, color: Color) -> Vec<usize> {
    let single_pawn = Bitboard::new(1u64 << position);
    let mut capture_squares = Vec::<usize>::new();

    if single_pawn
        .bitwise_and(&StaticBitboards::FILE_A)
        .bitwise_eq(&StaticBitboards::EMPTY)
    {
        if color == Color::White {
            capture_squares.push(position + Direction::SOUTHWEST as usize);
        } else {
            capture_squares.push(position + Direction::NORTHWEST as usize);
        }
    }
    if single_pawn
        .bitwise_and(&StaticBitboards::FILE_H)
        .bitwise_eq(&StaticBitboards::EMPTY)
    {
        if color == Color::White {
            capture_squares.push(position + Direction::NORTHWEST as usize);
        } else {
            capture_squares.push(position + Direction::SOUTHWEST as usize);
        }
    }
    return capture_squares;
}

pub fn test() {
    println!("test")
}
