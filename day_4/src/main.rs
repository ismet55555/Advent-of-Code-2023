use std::fs;

const INPUT_FILEPATH: &str = "inputs/INPUT_2_sample.txt";

fn main() {
    // Read the file
    let contents = fs::read_to_string(INPUT_FILEPATH).expect("ERROR: Failed to read input file");

    // Divide the lines in file into a vector of strings
    let input_lines: Vec<&str> = contents.split("\n").collect();

    // Remove all empty lines
    let input_lines: Vec<&str> = input_lines
        .iter()
        .filter(|&x| x != &"")
        .map(|&x| x)
        .collect();

    ////////////////////////////////////////////////////////////////////////

    let mut total_points = 0;

    for card in input_lines {
        println!("------------------------");
        println!("{}", card);

        // let card_number = card.split(":").collect::<Vec<&str>>()[0]
        //     .split(" ")
        //     .collect::<Vec<&str>>()[1];

        // Split the card into two sets of numbers
        let card_numbers = card.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        // Get winning numbers array
        let winning_numbers = card_numbers[0]
            .split(" ")
            .map(|s| s.trim())
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        println!("Winning Numbers: {:?}", winning_numbers);

        // Get played numbers array
        let played_numbers = card_numbers[1]
            .split(" ")
            .map(|s| s.trim())
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        println!("Played Numbers: {:?}", played_numbers);

        let mut card_points = 0;

        // Check if played numbers are winning numbers
        for played_number in played_numbers {
            if winning_numbers.contains(&played_number) {
                println!("{} is a winning number", played_number);
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            } else {
                continue;
            }
        }
        println!("Card Points: {}", card_points);
        total_points += card_points;
    }
    println!("PART1: Total Points: {}", total_points)
}
