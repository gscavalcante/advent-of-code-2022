use std::fs;

pub fn part_01(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path)
        .expect("Should be able to read the file");
    let runsacks: Vec<&str>  = input
        .lines()
        .collect();
    
    let sum: u32 = 0;
    for runsack in runsacks {
        let mut count: usize = 0;

        while count < runsack.len() / 2 {
            let position = runsack.chars().nth(count).unwrap();
            let mut found = false;
            for x in (runsack.len() / 2)..runsack.len() {
                let x_char = runsack.chars().nth(x).unwrap();
                if position.eq(&x_char) {
                    found = true;
                    break;
                }
            }

            if found {
                println!("Equal char is {position}");
                break;
            }

            count += 1
        }
    }

    sum
}

#[test]
fn part_01_with_example_data() {
    let result = part_01("inputs/day_03/example.txt");
    assert_eq!(result, 157);
}
