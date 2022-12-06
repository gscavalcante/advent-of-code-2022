use std::io::{self, Write};

mod day_01;
mod day_02;
mod day_03;

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
            println!("\tDay 01: Calorie Couting");
            println!("How many total Calories is that Elf carrying? {}", day_01::part_01("inputs/day_01/input.txt"));
            println!("How many Calories are those Elves carrying in total? {}", day_01::part_02("inputs/day_01/input.txt"));
        },
        2 => {
            println!("\tDay 02: Rock Paper Scissors");
            println!("What would your total score be if everything goes exactly according to your strategy guide? {}", day_02::part_01("inputs/day_02/input.txt"));
            println!("Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide? {}", day_02::part_02("inputs/day_02/input.txt"));
        },
        3 => {
            println!("\tDay 03: Rucksack Reorganization");
            println!("What is the sum of the priorities of those item types? {}", day_03::part_01("inputs/day_03/input.txt"));
        } 
        _ => println!("Invalid option")
    }
}
