use rand::Rng;


fn main() {
    let min = 0;
    let max = 100;
    let mut answer = rand::thread_rng().gen_range(min..max+1);
    println!("Welcome to the guessin game!");

    let mut input = String::new();
    let mut play: char;
    let mut guess: i32;
    loop{
        println!("Please guess the number from 1 to 100: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading user input");
        guess = input
            .trim()
            .parse()
            .expect("Error converting user input to integer");
        input.clear();


        if guess < answer{println!("Higher");}
        else if guess > answer{println!("Lower");}
        else{ 
            println!("Correct!");
            println!("Would you like to play again? y/n");
            // Get user input
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading user input");
            // Convert to desired type
            play = input
                .trim()
                .parse()
                .expect("Error converting user input to char");
            input.clear();
            if play == 'y'{
                // generate new number
                answer = rand::thread_rng().gen_range(min..max+1);
            } else{ 
                break; 
            }
        }
    }
}
