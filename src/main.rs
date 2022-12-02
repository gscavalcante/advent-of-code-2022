use std::io::{self, Write};

mod day_01;
mod day_02;

fn main() {
    print!("\tAdvent of Code 2022\n");
    print!("Choose a day (1-25): ");
    io::stdout().flush().unwrap();

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line!");

    let x = option
        .trim()
        .parse::<i32>()
        .expect("Number");

    match x {
        1 => {
            println!("\tDay 01 - Calorie Couting");
            println!("Find the Elf carrying the most Calories. How many total Calories is that Elf carrying? {}", day_01::part_01("inputs/day_01/input.txt"));
            println!("Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total? {}", day_01::part_02("inputs/day_01/input.txt"));
        },
        2 => {
            println!("\tDay 02 - Rock Paper Scissors");
            println!("What would your total score be if everything goes exactly according to your strategy guide? {}", day_02::part_01("inputs/day_02/input.txt"));
            println!("Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide? {}", day_02::part_02("inputs/day_02/input.txt"));
        } 
        _ => println!("Invalid option")
    }
}
