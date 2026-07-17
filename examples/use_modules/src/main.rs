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

fn main() {
    if false {
        stdinput();
    }
    if false {
        random();
    }
    if true {
        guess_game();
    }
}
