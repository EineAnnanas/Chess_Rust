// all information stored to accses the board fast
use crate::bitboard::Bitboard;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BoardSide {
    Kingside,
    Queenside,
}

pub struct ChessBoard {
    // Indivudual Pieces
    pub piece_bitboards: HashMap<(Color, PieceType), Bitboard>,
    // Further Board Information
    pub en_passant_vulnerable_bitboard: Bitboard,
    pub castling_rights: HashMap<(Color, BoardSide), bool>,
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut piece_bitboards = HashMap::new();
        let mut en_passant_vulnerable_bitboard = Bitboard::new(0);
        let mut castling_rights = HashMap::new();
        // Initialize bitboards with appropriate data
        piece_bitboards.insert(
            (Color::White, PieceType::Pawn),
            Bitboard::new(0b0000000000000000000000000000000000000000000000001111111100000000),
        );
        piece_bitboards.insert(
            (Color::White, PieceType::Knight),
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000001000010),
        );
        piece_bitboards.insert(
            (Color::White, PieceType::Bishop),
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000000100100),
        );
        piece_bitboards.insert(
            (Color::White, PieceType::Rook),
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000000100001),
        );
        piece_bitboards.insert(
            (Color::White, PieceType::Queen),
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000000010000),
        );
        piece_bitboards.insert(
            (Color::White, PieceType::King),
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000000001000),
        );

        piece_bitboards.insert(
            (Color::Black, PieceType::Pawn),
            Bitboard::new(0b0000000011111111000000000000000000000000000000000000000000000000),
        );
        piece_bitboards.insert(
            (Color::Black, PieceType::Knight),
            Bitboard::new(0b0100001000000000000000000000000000000000000000000000000000000000),
        );
        piece_bitboards.insert(
            (Color::Black, PieceType::Bishop),
            Bitboard::new(0b0001001000000000000000000000000000000000000000000000000000000000),
        );
        piece_bitboards.insert(
            (Color::Black, PieceType::Rook),
            Bitboard::new(0b1000000100000000000000000000000000000000000000000000000000000000),
        );
        piece_bitboards.insert(
            (Color::Black, PieceType::Queen),
            Bitboard::new(0b0001000000000000000000000000000000000000000000000000000000000000),
        );
        piece_bitboards.insert(
            (Color::Black, PieceType::King),
            Bitboard::new(0b0000100000000000000000000000000000000000000000000000000000000000),
        );
        castling_rights.insert((Color::White, BoardSide::Kingside), true);
        castling_rights.insert((Color::White, BoardSide::Queenside), true);
        castling_rights.insert((Color::Black, BoardSide::Kingside), true);
        castling_rights.insert((Color::Black, BoardSide::Queenside), true);
        ChessBoard {
            piece_bitboards,
            en_passant_vulnerable_bitboard,
            castling_rights,
        }
    }
}
