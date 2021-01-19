use std::io;

fn input() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}

fn print_header(){
    println!("----------------------------------");
    println!("Prime Numbers Counters");
    println!("----------------------------------");
}

fn to_int(num: String) -> i32{
    let intnum: i32 = num
        .parse()
        .expect("Not a number!");
    return intnum;
}

fn get_valid_answer() -> i32 {
    let option: i32;
    option = loop {
        let inp_str = input();
        let inp = to_int(inp_str);

        if inp == 1 || inp == 2{
            break inp;
        }
    };

    return option;
}

fn main() {
    let valid_ans: i32;
    print_header();
    println!("1. Print prime numbers till a number");
    println!("2. Print number of prime numbers");
    valid_ans = get_valid_answer();
    println!("ok: {}", valid_ans);
}
