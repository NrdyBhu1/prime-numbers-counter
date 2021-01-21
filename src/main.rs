use std::io;

fn input() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess.trim().to_string();
}

fn print_header(){
    println!("----------------------------------");
    println!("Prime Numbers Counters");
    println!("----------------------------------");
}

fn to_int(num: String) -> i32{
    let intnum: i32 = num.parse::<i32>().unwrap();//.expect("Not a number");
        // .expect("Not a number!");
    return intnum;
}

fn get_valid_answer() -> i32 {
    let option: i32;
    option = loop {
        let inp_str = input();
        let inp = to_int(inp_str.to_string());

        if inp == 1 || inp == 2{
            break inp;
        }
        println!("That is not a valid option");
    };

    return option;
}

fn check_divisibility(num_one: i32, num_two: i32) -> bool {
    if num_one % num_two == 0{
        return true;
    }else {
        return false;
    }
}

fn check_if_prime(num: i32) -> bool {
    let mut is_prime: bool = true;
    for j in 1..9 {
        if j != 1 && j != num {
            if check_divisibility(num, j){
                is_prime = false;
                break;
            }else {
                is_prime = true;
            }
        }
    }

    return is_prime;
}

fn get_no_of(number: i32){
    let mut cur_num: i32 = 0;
    let mut cur_prime_num: i32 = 0;
    loop {
        if cur_num == number {
            break;
        }
        if check_if_prime(cur_prime_num){
            println!("{}", cur_prime_num);
            cur_num += 1;
        }
        cur_prime_num += 1;
    }
}

fn get_till_a_number(number: i32){
    for i in 1..number {
        if check_if_prime(i) {
            println!("{}", i);
        } else {
            continue;
        }
    }
}

fn main() {
    print_header();
    println!("1. Print prime numbers till a number");
    println!("2. Print number of prime numbers");
    let valid_ans: i32 = get_valid_answer();
    if valid_ans == 1 {
        println!("----------------------------------");
        println!("1. Print prime numbers till a number");
        println!("Give a number: ");
        let till_number: i32 = to_int(input());
        println!("----------------------------------");
        get_till_a_number(till_number);
    }else if valid_ans == 2 {
        println!("----------------------------------");
        println!("2. Print number of prime numbers");
        println!("Give a number: ");
        let till_number: i32 = to_int(input());
        println!("----------------------------------");
        get_no_of(till_number);
    }
}
