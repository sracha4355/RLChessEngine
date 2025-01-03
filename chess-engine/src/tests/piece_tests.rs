#[cfg(test)]
mod tests {
    use crate::{engine::*, PieceTrait};
    #[test]
    fn verify_pawns_are_placed_correctly() {
        // Verify that white pawns are placed correctly 
        let pawns = piece::pawns(&piece::Color::WHITE);
        for  i in 0..8 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 0);
        }
        for  i in 8..16 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 1);
        }
        for  i in 16..64 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 0);
        }

        // Verify that black pawns are placed correctly
        let pawns = piece::pawns(&piece::Color::BLACK);
        for  i in 0..48 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 0);
        }
        for  i in 48..56 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 1);
        }
        for  i in 56..64 {
            let bit = utils::get_bit(i, pawns.get_bitboard()).unwrap();
            assert_eq!(bit, 0);
        }
    }
    #[test]
    fn verify_bishops_are_placed_correctly() { 
        let white_bishops = piece::bishop(&piece::Color::WHITE);
        let black_bishops = piece::bishop(&piece::Color::BLACK);
        let board = white_bishops.get_bitboard() | black_bishops.get_bitboard();
        
        for rank in 1..9 {
            for j in (1..9).map(|i| i as u8){
                let file = ((j - 1 + 97) as char).to_ascii_lowercase();
                if (rank == 1 || rank == 8) && (file == 'c' || file == 'f'){
                    assert_eq!(
                        1,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                } else {
                    assert_eq!(
                        0,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn verify_rooks_are_placed_correctly() { 
        let white_rooks = piece::rook(&piece::Color::WHITE);
        let black_rooks = piece::rook(&piece::Color::BLACK);
        let board = white_rooks.get_bitboard() | black_rooks.get_bitboard();
        
        for rank in 1..9 {
            for j in (1..9).map(|i| i as u8){
                let file = ((j - 1 + 97) as char).to_ascii_lowercase();
                if (rank == 1 || rank == 8) && (file == 'a' || file == 'h'){
                    assert_eq!(
                        1,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                } else {
                    assert_eq!(
                        0,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn verify_knights_are_placed_correctly() { 
        let white_knights = piece::knight(&piece::Color::WHITE);
        let black_knights = piece::knight(&piece::Color::BLACK);
        let board = white_knights.get_bitboard() | black_knights.get_bitboard();        
        for rank in 1..9 {
            for j in (1..9).map(|i| i as u8){
                let file = ((j - 1 + 97) as char).to_ascii_lowercase();
                if (rank == 1 || rank == 8) && (file == 'b' || file == 'g'){
                    assert_eq!(
                        1,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                } else {
                    assert_eq!(
                        0,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn verify_queens_are_placed_correctly() { 
        let white_queen = piece::queen(&piece::Color::WHITE);
        let black_queen = piece::queen(&piece::Color::BLACK);
        let board = white_queen.get_bitboard() | black_queen.get_bitboard();        
        for rank in 1..9 {
            for j in (1..9).map(|i| i as u8){
                let file = ((j - 1 + 97) as char).to_ascii_lowercase();
                if (rank == 1 || rank == 8) && (file == 'd'){
                    assert_eq!(
                        1,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                } else {
                    assert_eq!(
                        0,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn verify_kings_are_placed_correctly() { 
        let white_king = piece::king(&piece::Color::WHITE);
        let black_king = piece::king(&piece::Color::BLACK);
        let board = white_king.get_bitboard() | black_king.get_bitboard();        
        for rank in 1..9 {
            for j in (1..9).map(|i| i as u8){
                let file = ((j - 1 + 97) as char).to_ascii_lowercase();
                if (rank == 1 || rank == 8) && (file == 'e'){
                    assert_eq!(
                        1,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                } else {
                    assert_eq!(
                        0,
                        utils::get_bit(
                            utils::get_bit_from_position(rank, file).unwrap(),
                            board
                        ).unwrap()
                    );
                }
            }
        }
    }

}


