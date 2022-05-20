
fn main(){
    let item_price: f32 = 7.97;
    let mut line = String::new();

    println!("The price of the item is ${}", item_price);
    println!("How much of this item would you like to purchase?");
    std::io::stdin()                        // using stdin from io
        .read_line(&mut line)               // read the line and store it in line
        .expect("Failed to read line");     // if it fails, write message
    
    let num_purchased: i32 = line
                                .trim()                     // get rid of white spaces
                                .parse()                    // parse this tring slice into another type
                                .expect("Not an integer");  // if it fails to parse, write this message
    line.clear();   // clear the input string once we are done ith it use user in other io operations
    
    let total: f32 = item_price * num_purchased as f32; // casting uses the "as" keyword. Here: I multiply item_price (f32) by num_purchased (i32) as a f32
    println!("Your total purchase price is ${:.2}", total);

    println!("How much money are you giving the casier?");
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let cash: f32 = line
                        .trim()
                        .parse()
                        .expect("Error at cash assignment");

    let change: f32 = cash - total;
    println!("Your change is ${:.2}. Have a nice day!", change);
}