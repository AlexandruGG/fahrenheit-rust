use std::io;

fn main() {
    let temp_f: i32 = read_input("Input Fahrenheit Temperature:")
        .trim()
        .parse()
        .unwrap();

    let temp_c = (temp_f - 32) * 5 / 9;
    println!("{}F is {}C", temp_f, temp_c);
}

fn read_input(message: &str) -> String {
    println!("{}", message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}
