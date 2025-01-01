pub mod board {
    pub fn position_to_index(position: &(u64, char)) -> u64 {
        match position {
            (rank, file ) => (rank - 1) + (file.to_ascii_lowercase() as u64 - 97)
        }
    }
}
