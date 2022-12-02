use std::fs;

pub fn part_01(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let strategy_guide: Vec<&str> = input
        .lines()
        .collect();
    
    let mut total = 0;

    for round in strategy_guide {
        let round_vec: Vec<&str> = round.split(' ').collect();
        let opponent = round_vec[0].chars().next().expect("Empty string") as u32 - ('A' as u32) + 1;
        let player = round_vec[1].chars().next().expect("Empty string") as u32 - ('X' as u32) + 1;

        if opponent == player {
            total += 3 // tie
        } else if (opponent == 3 && player == 1) || (opponent == player - 1) {
            total += 6 // win
        }

        total += player;
    }

    total
}

#[test]
fn part_01_with_example_data() {
    let result = part_01("inputs/day_02/example.txt");
    assert_eq!(result, 15);
}