use std::fs;

pub fn calorie_counting(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let input_vec: Vec<&str> = input
        .lines()
        .collect();

    let mut higher = 0;

    let mut sum = 0;
    for calories in input_vec {
        if calories.is_empty() {
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
fn part_01_with_example_data() {
    let result = calorie_counting("inputs/day_01/example.txt");
    assert_eq!(result, 24000);
}