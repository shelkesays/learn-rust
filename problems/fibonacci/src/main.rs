use std::io;

fn fibonacci(n: i64) -> i64 {
    let mut prev = 0;
    let mut current = 1;
    for _ in 1..n {
        current = prev + current;
        prev = current - prev;
    }

    current
}

fn main() {
    println!("Enter the number N for which to get the Fibinacci value: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read the Nth number.");

    let number: i64 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    let fib_num = fibonacci(number);
    println!("The {}th fibonacci number is: {}", number, fib_num);
}
