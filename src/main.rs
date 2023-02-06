use std::io::{stdin, stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
    
}

fn main() {
    println!("Welcome to Adi's calculator");

    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut operator = String::new();

    println!("What is the first number: ");
    read(&mut n1);
    println!("What is the second number: ");
    read(&mut n2);
    println!("What is the operation you want to perform? (+-*/): ");
    read(&mut operator);
    
    //change to float type as we need to carryout division operations
    let n1: f32 = n1.trim().parse().unwrap();
    let n2: f32 = n2.trim().parse().unwrap();
    let operator: char = operator.trim().parse().unwrap();

    // let operations = String::from("+-*/");

    // if !operations.contains(operator){
    //     println!("Invalid operator");
    //     return;
    // }

    let result = match operator {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' => n1 / n2,
        _ => {
            println!("unknown operation");
            return;
        },
    };

    println!("Result of the opration is {} {} {} = {}", n1, operator, n2, result);
}

// "mut": This keyword makes the binding mutable
//The "String::new()" function returns a new, empty string
//The empty string is being assigned to the binding "n1"
//stdout().flush() is used to make sure that the output written to the standard output stream is immediately displayed and not buffered. It's especially useful when debugging, where the immediate display of output can help in understanding the program's behavior.
// n1.trim(): This method call trims whitespace from the beginning and end of the string stored in n1.
// .parse(): This method call attempts to parse the trimmed string into a number.