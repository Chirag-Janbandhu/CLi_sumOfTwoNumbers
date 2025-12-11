use std::io;

// Do cargo run to execute the program
fn main(){
    println!("Hello,folks!\n This is a Cli program to calculate the sum of two numbers.");
    println!("Enter first number:");
    let mut  input1 =String::new(); 
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");
    let number1: i32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Input was not a valid integer.");
            return; // Exit or handle the error
        }
    };
    println!("Enter second number:");
    
    let mut  input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");
    let number2: i32 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Input was not a valid integer.");
            return; // Exit or handle the error
        }
    };


    let sum=number1 + number2;
    println!("The sum of {} and {} is {}", number1, number2, sum);
}