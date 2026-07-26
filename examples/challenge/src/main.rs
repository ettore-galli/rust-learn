fn get_input_filename() -> Result<String, String> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Err(String::from("Filename is mandatory")),
        _ => Ok(args[1].clone()),
    }
}

fn get_file_content(filename: &str) -> Result<String, String> {
    use std::env;
    use std::fs;

    println!("{p}", p = env::current_dir().unwrap().to_string_lossy());

    match fs::read_to_string(filename) {
        Ok(content) => Ok(content),
        Err(error) => Err(error.to_string()),
    }
}

use std::collections::HashMap;

fn count_words(content: &str) -> HashMap<&str, u32> {
    let mut word_map = HashMap::new();

    for word in content.split_whitespace() {
        let current = word_map.entry(word).or_insert(0);
        *current += 1;
    }

    word_map
}

fn most_common_words<'a>(word_map: &HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut vec: Vec<(&'a str, u32)>  = word_map.iter().map(|(k, v)| (*k, *v)).collect();
    vec.sort_by_key(|(_, v)| *v);

    vec
}

fn main() {
    let filename = get_input_filename();

    match filename {
        Ok(filename) => {
            let read_result = get_file_content(&filename);
            match read_result {
                Err(err) => println!("ERRORE: {}", err),
                Ok(content) => {
                    let word_map = count_words(&content);
                    let most_common = most_common_words(&word_map);
                    let slice = most_common.as_slice();
                    let lasts = &slice[slice.len().saturating_sub(5)..];
                    println!("{:?}", lasts);
                }
            }
        }
        Err(err) => println!("ERROR: {}", err),
    }
}
