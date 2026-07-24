fn stdinput() {
    use std::io;

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let number: i32 = buffer.trim().parse().unwrap();

    println!("{buffer:#?} {next}", next = number + 1);
}

fn random() {
    let rn = rand::random::<f64>();
    println!("{rn}")
}

fn guess_game() {
    use rand::random_range;
    use std::io;
    use std::num::ParseIntError;

    fn output_message(message: &str) -> () {
        println!("{message}");
    }

    type NumberFormat = u8;
    type ParseResult = Result<NumberFormat, ParseIntError>;

    fn get_user_number() -> ParseResult {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        return buffer.trim().parse::<NumberFormat>();
    }

    fn game_main() -> () {
        output_message("Indovina il numero");

        let numero: u8 = random_range(1..101);

        loop {
            let numero_utente = get_user_number();
            if numero_utente.is_err() {
                output_message(numero_utente.unwrap_err().to_string().as_str());
            } else {
                let tentativo: u8 = numero_utente.unwrap();
                if tentativo < numero {
                    output_message("più alto");
                } else if tentativo > numero {
                    output_message("più basso");
                } else {
                    if tentativo == numero {
                        output_message("Indovinato!!!");
                        break;
                    }
                }
            }
        }
    }
    game_main();
}

fn args() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("un po' pochino");
        }
        2 => {
            eprintln!("un po' pochino");
        }
        3 => {
            eprintln!("un po' pochino");
        }
        4 => {
            let a = &args[1];
            let b = &args[2];
            let c = &args[3];
            println!("-----");
            println!("[{a}] - [{b}] - [{c}]");
            println!("-----");
        }
        _ => {}
    }
    // for arg in env::args() {
    //     println!("* {arg}");
    // }
}

fn files() {
    use std::env;
    use std::fs;
    use std::io::Write;

    const FILENAME: &str = "data/afile.txt";

    println!("{p}", p = env::current_dir().unwrap().to_string_lossy());

    fn display_file(fqn: &str) -> () {
        match fs::read_to_string(fqn) {
            Ok(content) => {
                for line in content.lines() {
                    println!(". {line}");
                }
            }
            Err(error) => {
                println!("{error}")
            }
        }
    }

    fn append_to_file(fqn: &str, line: &str) -> () {
        match fs::OpenOptions::new().append(true).open(fqn) {
            Ok(mut file_pointer) => {
                let _ = file_pointer.write(line.as_bytes());
            }
            Err(error) => {
                println!("{error}");
            }
        }
    }
    display_file(FILENAME);
    append_to_file(FILENAME, "hello");
    display_file(FILENAME)
}

fn check_file() -> () {
    fn get_inputs() -> (String, String) {
        use std::env;
        let mut args = env::args().skip(1);

        match (args.next(), args.next()) {
            (Some(file), Some(search)) => {
                return (file, search);
            }
            _ => {
                eprintln!("E' richiesto esattamente 2 parametri");
                return (String::new(), String::new());
            }
        }
    }
    fn check_in_file(file: &str, search: &str) -> Result<usize, String> {
        use std::fs;

        match fs::read_to_string(file) {
            Ok(content) => {
                for (index, line) in content.lines().enumerate() {
                    if line.contains(search) {
                        return Ok(index);
                    }
                }
                return Err(String::from("Non trovato"));
            }
            Err(error) => {
                return Err(error.to_string());
            }
        }
    }

    fn mainline() {
        let (file, search) = get_inputs();
        let result = check_in_file(&file, &search);
        println!("file={file} search={search}, result={result:?}");
    }

    mainline();
}

fn structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }
    impl Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }

        fn scale(&mut self, factor: f64) -> () {
            self.width = self.width * factor;
            self.height = self.height * factor;
        }

        fn new(w: f64, h: f64) -> Rectangle {
            Rectangle {
                width: w,
                height: h,
            }
        }
    }

    let mut r1 = Rectangle::new(3.0, 4.0);
    assert_eq!(r1.get_area(), 12.0);
    r1.scale(2.0);
    assert_eq!(r1.get_area(), 48.0);
    println!("{r1:?}");

    use core::ops::Add;
    use core::ops::Div;
    use std::fmt::Debug;
    use std::str::FromStr;

    fn media<T>(a: T, b: T) -> T
    where
        T: Add<Output = T> + Div<Output = T> + FromStr + Debug,
        <T as FromStr>::Err: Debug,
    {
        let two: T = String::from("2").parse().unwrap();
        (a + b) / two
    }
    println!("{m}", m = media::<u8>(3, 4));
    println!("{m}", m = media::<f64>(3.0, 4.0));
}

fn sum_boxes_challenge() {
    use core::ops::Add;

    fn sum_boxes<T: Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
        let result = *a + *b;
        return Box::new(result);
    }

    println!("{s}", s = sum_boxes::<u8>(Box::new(3), Box::new(4)));
    println!(
        "{s}",
        s = sum_boxes::<f64>(Box::new(3.1415), Box::new(2.7182))
    );
}

fn main() {
    if false {
        stdinput();
    }
    if false {
        random();
    }
    if false {
        guess_game();
    }
    if false {
        args();
    }
    if false {
        files();
    }

    if false {
        check_file();
    }

    if false {
        structs()
    }

    if true {
        sum_boxes_challenge()
    }
}
