use std::{error, io::{self, Write}, num::ParseIntError, process::exit};
fn main() {
    start();

    
}

fn start() {
    loop {
        let mut line = String::new();
        println!("Enter command:");
        println!("1) add");
        println!("2) multiply");
        println!("3) subtract");
        println!("4) divide");
        println!("5) exit");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        let n: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                error("This is not a valid answer");
                continue;
            }
        };

        match n {
            5 => {
                exit(0);
            }
            1..=4 => {
                get_input(n);
            }
            _ => {
                error("Number out of range");
            }
        }
    }
}




fn get_input(n: i32) {
    let mut n1 = String::new();
    let mut n2 = String::new();

    print!("Type first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n1).unwrap();

    print!("Type second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n2).unwrap();

    let a: i32 = match n1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            error("First input is not a number");
            return;
        }
    };

    let b: i32 = match n2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            error("Second input is not a number");
            return;
        }
    };

    match n {
        1 => println!("{}",add(a, b)),
        2 => println!("{}",multiply(a, b)),
        3 => println!("{}",subtract(a, b)),
        4 => println!("{}",divide(a, b)),
        _ => error("This can't happen anyway"),
    }
}


fn error(message:&str) {
    println!("{}",message);
    start();
}
// 1001
// 0101
// 0100

fn add(a: i32, b: i32) -> i32 {
    let mut sum = a;
    let mut carry = b;

    while carry != 0 {
        let temp_sum = sum ^ carry;          
        carry = (sum & carry) << 1;          
        sum = temp_sum;                      
    }

    sum
}


fn multiply(a: i32, b: i32) -> i32 {
    let mut result = 0;
    let mut multiplier = b;
    let mut multiplicand = a;

    while multiplier != 0 {
        
        if multiplier & 1 != 0 {
            result = add(result, multiplicand);
        }
        
        multiplicand <<= 1;
        multiplier >>= 1;
    }

    result
}

fn subtract(a:i32,b:i32)  ->i32 {
    let mut subtraction = a ^ b;
    let mut carry = b;

    while carry != 0 {
        let sub = subtraction | carry;
        carry = (subtraction ^ sub) >> 1;
        subtraction = carry;
    }

    subtraction
}

fn divide(a:i32,b:i32) -> i32 {
    a
}


