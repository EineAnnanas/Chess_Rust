mod bitboard;
mod chessboard;
mod legalmoves;
use crate::bitboard::Bitboard;
use crate::legalmoves::get_pawn_attacks;
use chessboard::{ChessBoard, Color, PieceType};

fn main() {
    let my_test_bitboard = Bitboard::new(123516345u64);
    my_test_bitboard.print_binary_representation();
    let my_test_vec = Bitboard::get_bit_position(my_test_bitboard.data);
}
