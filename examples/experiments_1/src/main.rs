mod utils;

fn word_count_main() {
    let filename = utils::get_input_filename();

    match filename {
        Ok(filename) => {
            let read_result = utils::get_file_content(&filename);
            match read_result {
                Err(err) => println!("ERRORE: {}", err),
                Ok(content) => {
                    let word_map = utils::count_words(&content);
                    let most_common = utils::most_common_words(&word_map);
                    let slice = most_common.as_slice();
                    let lasts = &slice[slice.len().saturating_sub(5)..];
                    println!("{:?}", lasts);
                }
            }
        }
        Err(err) => println!("ERROR: {}", err),
    }
}

fn result_enum_main() {
    //utils::utils::get_input_filename().and_then(|filename|=>)
    //let filename = "data/text.txt";

    fn display_in_lines(content: &str) -> Result<String, String> {
        for item in content.split_whitespace() {
            println!("<<{item}>>");
        }
        return Ok(content.to_string());
    }

    fn process_file(filename: &str) -> Result<String, String> {
        utils::get_file_content(filename).and_then(|content| display_in_lines(content.as_str()))
    }

    println!("{:?}", process_file("data/text.txt"));
}

fn main() {
    result_enum_main();
}
