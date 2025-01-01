type Moveset = Vec<Move>;

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
#[derive(Debug, Clone, Copy)]
enum Shift { 
    LEFT, 
    RIGHT
}
#[derive(Debug, Clone, Copy)]
pub struct Move {
    shift: Shift,
    amount: u64
}    

pub trait PieceTrait {
    fn get_piece_type(&self) -> PieceType;
    fn get_color(&self) -> Color;
    fn get_movesets(&self) -> &Moveset;
    fn get_bitboard(&self) -> u64;
    fn get_has_moved(&self) -> bool;

    fn piece_type(&mut self, piece_type: PieceType) -> ();
    fn color(&mut self, color: Color) -> ();
    fn movesets(&mut self, movesets: &Moveset) -> ();
    fn bitboard(&mut self, bitboard: u64) -> ();
    fn has_moved(&mut self, has_moved: bool) -> ();
}
pub struct Piece {
    piece_type: PieceType, 
    color: Color,
    movesets: Moveset,
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
    fn get_movesets(&self) -> &Moveset { 
        &self.movesets
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
    fn movesets(&mut self, movesets: &Moveset) -> () {
        self.movesets = movesets
            .iter()
            .copied()
            .collect();
    }
    fn bitboard(&mut self, bitboard: u64) -> () {
        self.bitboard = bitboard;
    }
    fn has_moved(&mut self, has_moved: bool) -> () {
        self.has_moved = has_moved;
    }
}

pub fn white_pawns() -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::PAWN,
        color: Color::WHITE,       
        movesets: vec![
            Move {shift: Shift::LEFT, amount: 7}, 
            Move {shift: Shift::LEFT, amount: 8}, 
            Move {shift: Shift::LEFT, amount: 9}, 
            Move {shift: Shift::LEFT, amount: 16}
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

pub fn black_pawns() -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::PAWN,
        color: Color::BLACK,       
        movesets: vec![
            Move {shift: Shift::RIGHT, amount: 7}, 
            Move {shift: Shift::RIGHT, amount: 8}, 
            Move {shift: Shift::RIGHT, amount: 9}, 
            Move {shift: Shift::RIGHT, amount: 16}
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

// need to get the write bitboards and movesets for the pieces below then test
pub fn bishop(color: &Color) -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::BISHOP,
        color: *color,       
        movesets: vec![

        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

pub fn rook(color: &Color) -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::ROOK,
        color: *color,       
        movesets: vec![
            
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

pub fn knight(color: &Color) -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::KNIGHT,
        color: *color,       
        movesets: vec![
            
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

pub fn queen(color: &Color) -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::QUEEN,
        color: *color,       
        movesets: vec![
            
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

pub fn king(color: &Color) -> Piece { 
    /*
        * @movesets: 7, 8, 9, 16
        * values correspond to shifts
        * 7: attacking move to left
        * 8: move up one
        * 9: attacking move to right
        * 16: move up two
    */
    Piece {
        piece_type: PieceType::KING,
        color: *color,       
        movesets: vec![
            
        ], 
        bitboard: 0xFFu64 << 8,
        has_moved: false
    }
}

