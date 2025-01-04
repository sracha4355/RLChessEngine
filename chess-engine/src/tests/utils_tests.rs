#[cfg(test)]
mod tests {
    use crate::engine::utils::*;
    
    #[test]
    fn test_get_bit_from_position() {
        let rank = 1u64;
        let char = 'a';
        let bit = get_bit_from_position(rank, char).unwrap();
        assert_eq!(bit, 0 );

        let rank = 1u64;
        let char = 'c';
        let bit = get_bit_from_position(rank, char).unwrap();
        assert_eq!(bit, 2);

        let rank = 2u64;
        let char = 'h';
        let bit = get_bit_from_position(rank, char).unwrap();
        assert_eq!(bit, 15);

        let rank = 8u64;
        let char = 'h';
        let bit = get_bit_from_position(rank, char).unwrap();
        assert_eq!(bit, 63);

        let rank = 5u64;
        let char = 'e';
        let bit = get_bit_from_position(rank, char).unwrap();
        assert_eq!(bit, 36);

        let rank = 9u64;
        let char = 'h';
        match get_bit_from_position(rank, char) {
            Ok(_) => panic!(""),
            Err(_) => assert_eq!(true, true)
        }
    }
}