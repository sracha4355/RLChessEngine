use core::panic;
use crate::engine::utils;


#[derive(Debug, Clone, Copy)]
pub enum PieceType { 
    PAWN,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    ROOK
}  

#[derive(Debug, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}
pub trait PieceTrait {
    fn get_piece_type(&self) -> PieceType;
    fn get_color(&self) -> Color;
    fn get_bitboard(&self) -> u64;
    fn get_has_moved(&self) -> bool;

    fn piece_type(&mut self, piece_type: PieceType) -> ();
    fn color(&mut self, color: Color) -> ();
    fn bitboard(&mut self, bitboard: u64) -> ();
    fn has_moved(&mut self, has_moved: bool) -> ();
}
pub struct Piece {
    piece_type: PieceType, 
    color: Color,
    bitboard: u64,
    has_moved: bool
}

impl PieceTrait for Piece {
    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }
    fn get_color(&self) -> Color {
        self.color
    }
 
    fn get_bitboard(&self) -> u64 { 
        self.bitboard
    }
    fn get_has_moved(&self) -> bool {
        self.has_moved
    }
    fn piece_type(&mut self, piece_type: PieceType) -> () {
        self.piece_type = piece_type;
    }
    fn color(&mut self, color: Color) -> () {
        self.color = color;
    }

    fn bitboard(&mut self, bitboard: u64) -> () {
        self.bitboard = bitboard;
    }
    fn has_moved(&mut self, has_moved: bool) -> () {
        self.has_moved = has_moved;
    }
}


// ----------------------------------------//
// Functions to instantiate Piece Bitboards//
// ----------------------------------------//

pub fn pawns(color: &Color) -> Piece {
    let mut bitboard = 0xFFu64 << (8*6);
    if matches!(color, Color::WHITE){ bitboard = 0xFFu64 << 8} 
    Piece {
        piece_type: PieceType::PAWN,
        color: *color,       
        bitboard: bitboard,
        has_moved: false
    }
}

pub fn bishop(color: &Color) -> Piece { 
    let mut bitboard = 0u64;
    let mut rank = 0u64;
    let file_for_bishop1 = 'c';
    let file_for_bishop2 = 'f';
    if matches!(color , Color::WHITE) { rank = 1; } 
    else { rank = 8; }
    
    let mask = utils::bitmask_from_board_positions(
        &vec![(rank, file_for_bishop1), (rank, file_for_bishop2)]
    );
    if let Ok(bitmask) = mask { bitboard |= bitmask; } 
    else { panic!("Something went wrong during bishop bitboard creation!");}

    Piece {
        piece_type: PieceType::BISHOP,
        color: *color,       
        bitboard: bitboard,
        has_moved: false
    }
}

pub fn rook(color: &Color) -> Piece { 
    let mut bitboard = 0u64;
    let mut rank = 0;
    let file_for_rook1 = 'a';
    let file_for_rook2 = 'h';

    if matches!(color , Color::WHITE) { rank = 1; } 
    else { rank = 8; }

    let mask = utils::bitmask_from_board_positions(
        &vec![(rank, file_for_rook1), (rank, file_for_rook2)]
    );
    if let Ok(bitmask) = mask { bitboard |= bitmask; } 
    else { panic!("Something went wrong during rook bitboard creation!"); }

    Piece {
        piece_type: PieceType::ROOK,
        color: *color,
        bitboard: bitboard,
        has_moved: false
    }
}

pub fn knight(color: &Color) -> Piece { 
    let mut bitboard = 0u64;
    let mut rank = 0;
    let file_for_knight1 = 'b';
    let file_for_knight2 = 'g';

    if matches!(color , Color::WHITE) { rank = 1; } 
    else { rank = 8; }

    let mask = utils::bitmask_from_board_positions(
        &vec![(rank, file_for_knight1), (rank, file_for_knight2)]
    );
    if let Ok(bitmask) = mask { bitboard |= bitmask; } 
    else { panic!("Something went wrong during knight bitboard creation!"); }

    Piece {
        piece_type: PieceType::KNIGHT,
        color: *color,       
        bitboard: bitboard,
        has_moved: false
    }
}

pub fn queen(color: &Color) -> Piece { 
    let mut bitboard = 0u64;
    let mut rank = 0;
    let file = 'd';

    if matches!(color , Color::WHITE) { rank = 1; } 
    else { rank = 8; }

    let mask = utils::get_bit_from_position(rank, file)
        .and_then(|bit1| Ok(1u64 << bit1));

    if let Ok(bitmask) = mask {
        bitboard |= bitmask;
    }  
    Piece {
        piece_type: PieceType::QUEEN,
        color: *color,       
        bitboard: bitboard,
        has_moved: false
    }
}

pub fn king(color: &Color) -> Piece { 
    let mut bitboard = 0u64;
    let mut rank = 0;
    let file = 'e';

    if matches!(color , Color::WHITE) { rank = 1; } 
    else { rank = 8; }

    let mask = utils::get_bit_from_position(rank, file)
        .and_then(|bit1| Ok(1u64 << bit1));

    if let Ok(bitmask) = mask {
        bitboard |= bitmask;
    }  

    Piece {
        piece_type: PieceType::KING,
        color: *color,      
        bitboard: bitboard,
        has_moved: false
    }
}

