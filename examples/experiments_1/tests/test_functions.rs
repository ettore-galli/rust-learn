use experiments_1::utils::count_words;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_words(){
        let mut expected:HashMap<&str, u32> = HashMap::new();
        expected.insert("alfa", 1);
        expected.insert("beta", 1);
  
        assert_eq!(
            count_words("alfa beta"), expected
        );
    }
}