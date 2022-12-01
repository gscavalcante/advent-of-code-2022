use std::{fs, ops::Add};

pub fn calorie_counting_part_01(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .add("\n ");
    let input_vec: Vec<&str> = input
        .lines()
        .collect();

    let mut higher = 0;

    let mut sum = 0;
    for calories in input_vec {
        if calories.trim().is_empty() {
            if sum > higher {
                higher = sum;
            }

            sum = 0;
            continue;
        }
        
        sum = sum + calories.parse::<i32>()
            .expect("Should be a number");
    }

    higher
}

#[test]
fn calorie_counting_part_01_with_example_data() {
    let result = calorie_counting_part_01("inputs/day_01/example.txt");
    assert_eq!(result, 24000);
}

pub fn calorie_counting_part_02(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .add("\n ");
    let input_vec: Vec<&str> = input
        .lines()
        .collect();

    let mut top_three: [i32; 3] = [0; 3];

    let mut sum = 0;
    for calories in input_vec {
        if calories.trim().is_empty() {
            let mut higher = 0;
            for index in 0..top_three.len() {
                if sum > top_three[index] {
                    higher += 1;
                }
            }

            if higher > 0 {
                top_three[0] = sum;
                top_three.sort();
            }

            sum = 0;
            continue;
        }
        
        sum = sum + calories.parse::<i32>()
            .expect("Should be a number");
    }

    top_three.iter().sum()
}

#[test]
fn calorie_counting_part_02_with_example_data() {
    let result = calorie_counting_part_02("inputs/day_01/example.txt");
    assert_eq!(result, 45000);
}
