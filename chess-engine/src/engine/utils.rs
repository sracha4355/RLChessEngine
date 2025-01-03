pub fn print_bitboard(board: &u64) -> () {
    print!("   ");
    for file in 1..9 as u8 {
        print!(" {} ", ((file - 1 + 97) as char).to_ascii_lowercase());
    } 
    println!("");
    let mut rank = 8;
    for i in (0..64).rev() {
        if i % 8 == 7 {
            print!(" {} ", rank);
            rank -= 1;
        }
        let mask: u64 = 1 << i;
        if board & mask != 0 {
            print!(" 1 ");
        } else {
            print!(" 0 ");
        }
        if i % 8 == 0 {
            println!("");
        }
    }
}

pub fn get_bit(bit: u64, bitboard: u64) -> Result<u64, String> {
    if bit > 63 {
        return Err(format!("Invalid bit of {bit} requested"));
    }
    Ok((bitboard & (1 << bit)) >> bit)
}

pub fn get_bit_from_position(rank: u64, file: char) -> Result<u64, String> {
    let file = file as u64 + 1 - 97;
    // println!("file {}", file);
    let bit = (rank - 1) * 8  + (file - 1);
    if bit > 63 {
        return Err(format!("Invalid bit requested based on provided rank: {rank} and file: {file}"));
    }
    Ok(bit)
}

pub fn bitmask_from_board_positions(positions: &Vec<(u64, char)>) -> Result<u64, String> {
    let mut mask = 0u64;
    for (rank, file) in positions {
        let bit_number = get_bit_from_position(
            *rank,
            *file
        )?;
        mask |= 1 << bit_number;
    }
    return Ok(mask);
}


pub fn get_position(bit_index: u64) {
    
}


/*
    a  b  c  d  e  f  g  h 
 8  0  0  0  0  0  0  0  0 -> index 63
 7  0  0  0  0  0  0  0  0 -> index 55
 6  0  0  0  0  0  0  0  0 -> index 47
 5  0  0  0  0  0  0  0  0 -> index 39
 4  0  0  0  0  0  0  0  0 -> index 31
 3  0  0  0  0  0  0  0  0 -> index 23
 2  1  1  1  1  1  1  1  1 -> index 15
 1  0  0  0  0  0  0  0  0 -> index 7 
 *bits' index increase from left to right from the bottom
*/