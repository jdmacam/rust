// use std::io::Read;
use std::char;

fn main() {
    let mut input = String::new();
    let mut choice: String;
    loop {
        println!("What would you like to do?");
        println!("  1. Encode a message.");
        println!("  2. Decode a message.");
        println!("  q. Quit.");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error at user input.");
        let precheck = input.trim();
        if precheck.is_empty() {
            println!("*** Input is empty ***");
            continue;
        }
        choice = precheck
            .parse()
            .expect("Error converting user input to char");
        input.clear();

        // Encoding
        if choice == "1" {
            let shift = get_int_io();
            let output = encode(shift);
            println!("Encoded message: {}", output);
        }
        // Decoding
        else if choice == "2" {
            println!("Enter message to decode:");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading user input");
            let msg = input.trim().to_string();
            input.clear();
            let shift = get_int_io();
            let output = decode(msg, shift);
            println!("Decoded message: {}", output);
        } else if choice == "q" {
            println!("kthxbai!");
            break;
        } else {
            println!("*** Invalid input. ***")
        }
    }
}

fn get_int_io() -> u8{
    let mut input = String::new();
    loop {
        println!("Enter shift value: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading user input");
        let int_check = input.trim().parse::<i8>().is_ok();
        if int_check {
            break;
        }
        println!("*** Entered a non-numeric number ***");
        input.clear();
    }
    let shift: u8 = input
        .trim()
        .parse()
        .expect("Error converting user input to integer.");
    input.clear();
    return shift;
}

fn encode(shift: u8) -> String {
    let mut input = String::new();
    let mut encoded = String::new();
    println!("Please enter the message you'd like to encode:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error at user input.");
    let msg = input.trim();
    for c in msg.chars() {
        if c.is_whitespace() {
            encoded.push(c);
            continue;
        }
        let c_i = (c as u8) + shift;
        encoded.push(c_i as char);
    }
    input.clear();
    return encoded;
}

fn decode(msg: String, shift: u8) -> String{
    let mut decoded = String::new();
    for c in msg.chars(){
        if c.is_whitespace(){
            decoded.push(c);
        }
        let c_i = (c as u8) - shift;
        decoded.push(c_i as char);
    }
    return decoded;
}
