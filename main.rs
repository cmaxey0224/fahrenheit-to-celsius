use std::io;




fn main() {

	let mut temp = String::new();

	println!("What is the temperature in Fahrenheit? ");

	io::stdin()
		.read_line(&mut temp)
		.expect("Failed to read line");

	let temp: f64 = temp.trim().parse().expect("Please enter a number!");

	let result = ((temp - 32f64) * 5f64) / 9f64;

	println!("The temperature is: {result} degrees Celsius");
}
