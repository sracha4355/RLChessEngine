
pub mod board {
    use std::collections::HashMap;
    use crate::{engine::*, PieceTrait};
    struct Board {
        white_bitboards: HashMap<String, piece::Piece>,
        black_bitboards: HashMap<String, piece::Piece>
    }

    impl Board {
        fn white_bitboards(&self) -> &HashMap<String, piece::Piece> {
            &self.white_bitboards
        }
        fn black_bitboards(&self) -> &HashMap<String, piece::Piece>{
            &self.black_bitboards
        }
        fn white_bitboards_mut(&mut self) -> &mut HashMap<String, piece::Piece> {
            &mut self.white_bitboards
        }
        fn black_bitboards_mut(&mut self) -> &mut HashMap<String, piece::Piece>{
            &mut self.black_bitboards
        }

        fn print_board(&self) -> u64 {
            let board = self.white_bitboards.
                iter().
                fold(
                    0u64, 
                    |acc, item|  
                    acc | item.1.get_bitboard()
                ) 
            | 
            self.black_bitboards.
                iter().
                fold(
                    0u64, 
                    |acc, item|  
                    acc | item.1.get_bitboard()
                ) ;
            return board;
        }
        
        fn new(&self) -> Self{
            let mut white_bitboards: HashMap<String, piece::Piece> = HashMap::new();
            let mut black_bitboards: HashMap<String, piece::Piece> = HashMap::new();

            white_bitboards.insert(
                String::from("pawn"),
                piece::pawns(&piece::Color::WHITE)
            );

            white_bitboards.insert(
                String::from("bishop"),
                piece::bishop(&piece::Color::WHITE)
            );

            white_bitboards.insert(
                String::from("knight"),
                piece::knight(&piece::Color::WHITE)
            );

            white_bitboards.insert(
                String::from("rook"),
                piece::rook(&piece::Color::WHITE)
            );

            white_bitboards.insert(
                String::from("queen"),
                piece::queen(&piece::Color::WHITE)
            );

            white_bitboards.insert(
                String::from("king"),
                piece::king(&piece::Color::WHITE)
            );

            black_bitboards.insert(
                String::from("pawn"),
                piece::pawns(&piece::Color::BLACK)
            );

            black_bitboards.insert(
                String::from("bishop"),
                piece::bishop(&piece::Color::BLACK)
            );

            black_bitboards.insert(
                String::from("knight"),
                piece::knight(&piece::Color::BLACK)
            );

            black_bitboards.insert(
                String::from("rook"),
                piece::rook(&piece::Color::BLACK)
            );

            black_bitboards.insert(
                String::from("queen"),
                piece::queen(&piece::Color::BLACK)
            );

            black_bitboards.insert(
                String::from("king"),
                piece::king(&piece::Color::BLACK)
            );

            Self {
                white_bitboards: white_bitboards,
                black_bitboards: black_bitboards
            }
        }
    }


    // needs to be tested //
    pub fn position_to_index(position: &(u64, char)) -> u64 {
        match position {
            (rank, file ) => (rank - 1) + (file.to_ascii_lowercase() as u64 - 97)
        }
    }
}
