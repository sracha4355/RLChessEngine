pub mod engine;
use crate::engine::piece::*;
use crate::engine::*;

fn main() {
    let pawns: Piece = white_pawns();
    // println!("bitboard: {:#066b}", pawns.get_bitboard());
    utils::visualize_bitboard(&pawns.get_bitboard());
    let a: u64 = 0xFF00000000000000;
    let b: u64 = 0xFF;
    assert_eq!(0x00FF000000000000, a >> 8);
    
    /*
    println!("{:#064b}", a);
    // println!("{:#064b}", b);
    println!("{:#064b}", a >> 8);
    */

}

/*
fn main() {
    let number: u64 = 255;
    let binary_string = format!("{:b}", number);
    println!("Binary representation: {}", binary_string);
} */