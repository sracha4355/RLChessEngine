pub mod engine;
pub mod tests;
use crate::engine::piece::*;
use crate::engine::*;

fn main() {
    let pawns: Piece = white_pawns();
    // println!("bitboard: {:#066b}", pawns.get_bitboard());
    utils::print_bitboard(&pawns.get_bitboard());
    println!("{}", 0xFFu64);
    println!("{}", 0xFFu64 << 8);
    println!("{}", 0xFFu64 >> 8);
}