pub fn get_input_filename() -> Result<String, String> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Err(String::from("Filename is mandatory")),
        _ => Ok(args[1].clone()),
    }
}

pub fn get_file_content(filename: &str) -> Result<String, String> {
    use std::env;
    use std::fs;

    println!("{p}", p = env::current_dir().unwrap().to_string_lossy());

    match fs::read_to_string(filename) {
        Ok(content) => Ok(content),
        Err(error) => Err(error.to_string()),
    }
}

use std::collections::HashMap;

pub fn count_words(content: &str) -> HashMap<&str, u32> {
    let mut word_map = HashMap::new();

    for word in content.split_whitespace() {
        let current = word_map.entry(word).or_insert(0);
        *current += 1;
    }

    word_map
}

pub fn most_common_words<'a>(word_map: &HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut vec: Vec<(&'a str, u32)> = word_map.iter().map(|(k, v)| (*k, *v)).collect();
    vec.sort_by_key(|(_, v)| *v);

    vec
}