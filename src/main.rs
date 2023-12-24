use rand::Rng; // Used for random number generation
use std::cmp::Ordering; // Used for checking if value is greater lesser or equal
use std::io; // Used for reading input

fn read_input(message: &str) -> String {

    println!("{}", message);

    let mut input = String::new(); // Creating a growable empty string

    // Reading line from the terminal and storing it's result in the above empty string
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    return input;
}

fn generate_random_number() -> i32 {
    return rand::thread_rng().gen_range(1..=100); // generating random number
}


fn print_box (message: &str) {
    let len = message.len();
    let sym = "-";
    let line  =sym.repeat(len + 4);

    println!("{}", line); 
    println!("| {} |", message);
    println!("{}", line); 

}

fn main() {
    print_box("Welcome to the guessing game");

    let secret_number: i32 = generate_random_number();

    loop { // Game loop
        let guess = read_input("Guess the number");


        if guess.trim() == "exit" {
            print_box("Thankyou for playing :)");
            break;
        }

        println!("You guessed {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse input to number");
                break;
            }
        };
            
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                print_box("Hurray You win!");
                break;
            }
            Ordering::Greater => print_box("Too high"),
            Ordering::Less => print_box("Too low"),
        }
    }

}
