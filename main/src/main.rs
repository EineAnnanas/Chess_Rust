mod bitboard;
mod chessboard;
mod legalmoves;
use crate::bitboard::Bitboard;
use crate::legalmoves::get_pawn_attacks;
use chessboard::{ChessBoard, Color, PieceType};

fn main() {
    // Create a new chessboard
    let my_chessboard = ChessBoard::new();
    let test_value = my_chessboard.piece_bitboards;
    let board = test_value.get(&(Color::White, PieceType::Pawn));
    board.expect("REASON").print_binary_representation();
    // Access and print other bitboards
    let test = legalmoves::get_pawn_attacks(4, Color::White);
    let bit_test = Bitboard::new(test[0] as u64);
    bit_test.print_binary_representation();
    let mut bit = 1u64 << 2;
    bit |= 1u64 << 5;
    let test_v = Bitboard::new(bit);
    let negtest = test_v.twos_complement_negation();
    negtest.print_binary_representation();
    let both = negtest.bitwise_and(&test_v);
    both.print_binary_representation();
    let least_test = test_v.least_sigificant_one();
    least_test.print_binary_representation();
    let last_min_one = least_test.minus_one();
    last_min_one.print_binary_representation();
    let test_two = Bitboard::count_bits(last_min_one.data);
    println!("{}", test_two);
    last_min_one.print_binary_representation();
}
