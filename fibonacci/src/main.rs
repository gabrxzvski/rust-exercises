use std::io;
fn main() {
    loop {
        println!("Enter the number: ");

        let mut num_input = String::new();

        io::stdin()
            .read_line(&mut num_input)
            .expect("Failed to read line");

        let num_input: u32 = match num_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}th fibonacci number is {}", num_input, fib(num_input));
        break;
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
