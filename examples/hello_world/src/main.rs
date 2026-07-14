fn basics() {
    let x1: i8 = 4;
    println!("x1 = {}", x1);

    let mut x2: i8 = 127;
    println!("x2 = {}", x2);
    x2 -= 1;
    println!("x2 = {}", x2);

    let x3: f64 = 0.1;
    let x4: f64 = 0.2;
    println!("x3 {}, x4 {}, res = {:#10.6}", x3, x4, x3 + x4 / 3.0);
    println!("x3 {x3}, x4 {x4}, res = {res:#10.6}", res = x3 + x4 / 3.0);

    let b1: u8 = 0b10011001;
    let m1: u8 = 0b11110000;
    let m2: u8 = 0b00001111;
    println!("{b1}");
    println!("{b1:08b}");
    println!("{r1:08b}", r1 = b1 & m1);
    println!("{r2:08b}", r2 = b1 & m2);
    println!("{r2:08b}", r2 = b1 & 0b00001000);
    for n in 1..8 {
        println!("{r2:08b}", r2 = 0b0001 << n);
    }

    let coso = '\u{2987}';
    println!("{}", coso);

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let af: f32 = a as f32;

    let avg: f32 = (af + (b as f32) + c) / 3.0;
    let avg2: f64 = ((a as f64) + (b as f64) + (c as f64)) / 3.0;

    println!("avg({a}, {b}, {c}) == {avg}");
    println!("avg({a}, {b}, {c}) == {avg2}");

    const ARR_SIZE: usize = 5;

    let numbers: [f64; ARR_SIZE];
    numbers = [0.0; ARR_SIZE];
    println!("{numbers:#?}");

    const FLOORS: usize = 5;
    const ROWS: usize = 10;
    const PLACES: usize = 40;

    let mut parking: [[[bool; PLACES]; ROWS]; FLOORS];
    parking = [[[false; PLACES]; ROWS]; FLOORS];
    parking[2][4][7] = true;

    println!("{p:#?}", p = parking[2][4]);

    fn square(x: f64) -> f64 {
        x * x
    }
    fn powers(x: f64) -> (f64, f64, f64) {
        let q = x * x;
        let c = q * x;
        let f = c * x;
        return (q, c, f);
    }
    println!("square: {}={}", 63.0, square(63.0));
    println!("powers: {}={:#?}", 141.0, powers(141.0));

    fn c2f(celsius: f64) -> f64 {
        32.0 + celsius * 9.0 / 5.0
    }
    let cels = 23.0;
    let farh = c2f(cels);

    assert_eq!(farh, 73.4);

    println!("{cels} °C == {farh} °F");
}

fn control() {
    let run_basics: bool = false;
    if run_basics {
        basics();
    }
    let numbers: [u32; 5] = [1001, 1002, 1003, 1004, 1005];
    for (index, number) in numbers.iter().enumerate() {
        println!("{index}->{number}");
        if *number == 1003 {
            break;
        }
    }

    for n in 1..7 {
        println!("{n}");
    }

    let numbers: [i32; _] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

    fn maxmeanavg(data: &[i32]) -> (i32, i32, f64) {
        let mut min: i32 = data[0];
        let mut max: i32 = data[0];
        let mut avg: f64 = 0.0;
        for item in data {
            if *item < min {
                min = *item
            }
            if *item > max {
                max = *item;
            }
            avg += *item as f64;
        }
        return (min, max, avg / (data.len() as f64));
    }

    let min: i32;
    let max: i32;
    let avg: f64;

    (min, max, avg) = maxmeanavg(&numbers);
    println!("{numbers:#?} -> {min} {max} {avg}");
}

fn ownership() {
    fn saluta(nominativo: String) -> String {
        println!("Hello, {nominativo}");
        nominativo
    }

    fn completa(stringa: &mut String) -> () {
        stringa.insert_str(0, "<<<");
        stringa.push_str(">>>");
    }

    let mut nome: String = String::from("Ettore");

    completa(&mut nome);

    let nome = saluta(nome);

    println!("Fatto uso di [{nome}]");
}

fn main() {
    let do_basics = false;
    if do_basics {
        basics();
    }

    let do_control = false;
    if do_control {
        control();
    }

    let do_ownership = true;
    if do_ownership {
        ownership();
    }
}
