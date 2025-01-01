pub fn visualize_bitboard(board: &u64) -> () {
    print!(" ");
    for file in 1..9 as u8 {
        print!(" {} ", ((file - 1 + 97) as char).to_ascii_lowercase());
    }
    println!("");
    
    let ranks: [u64; 8] = [1,2,3,4,5,6,7,8];
    let mut mask: u64 = 0xFF00000000000000;
    for rank in ranks.iter().rev() {
        print!("{} ", rank);
        let bits = board | mask;
        
        println!("");
        mask = mask >> 8;
    }
}