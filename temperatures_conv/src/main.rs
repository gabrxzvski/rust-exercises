use std::io;
fn main() {
    loop {
        println!("Type in the fahrenheit value to be converted to celsius: ");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let converted = ((fahrenheit - 32.) * 5.) / 9.;

        println!(
            "{:.2} F° converted to celsius is {:.2} C°",
            fahrenheit, converted
        );
        break;
    }
}
