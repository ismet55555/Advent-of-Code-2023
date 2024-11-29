use std::fs;

// Game <ID>: <cubes pulled from bag>; <cubes pulled from bag>; etc
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

// PART 1: Which games played would have been possible with:
//    - 12 red cubes
//    - 13 green cubes
//    - 14 blue cubes
//
// PART 2: For each game, what is fewest number of cubes of each color
//         that could have been in the bag to make the game possible?
//  - power of game = minimum red * minimum green * minimum blue
//  - What is the sum of all the power of games?

const INPUT_FILEPATH: &str = "inputs/INPUT.txt";

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

    let red_possible = 12;
    let green_possible = 13;
    let blue_possible = 14;

    let mut game_possible_sum = 0;
    let mut game_power_sum = 0;

    for game in input_lines {
        // Get game ID as number only without ("Game")
        let game_id: i32 = game.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        // Get cubes pulled from bag
        let cubes_pulled_from_bag: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();

        let mut possible_game = true;

        let mut red_min = 0;
        let mut blue_min = 0;
        let mut green_min = 0;

        for cubes in cubes_pulled_from_bag {
            println!("----");
            let cubes_list = cubes.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for pull in cubes_list {
                let color = pull.split(" ").collect::<Vec<&str>>()[1];
                let number = pull.split(" ").collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap();

                if color == "red" {
                    if number > red_possible {
                        possible_game = false;
                    }
                } else if color == "green" {
                    if number > green_possible {
                        possible_game = false;
                    }
                } else if color == "blue" {
                    if number > blue_possible {
                        possible_game = false;
                    }
                }

                if color == "red" && (red_min == 0 || number > red_min) {
                    red_min = number;
                } else if color == "green" && (green_min == 0 || number > green_min) {
                    green_min = number;
                } else if color == "blue" && (blue_min == 0 || number > blue_min) {
                    blue_min = number;
                }
            }
        }
        if possible_game {
            game_possible_sum += game_id;
        }

        game_power_sum += red_min * green_min * blue_min;
    }

    println!("----------------------------");
    println!("PART 1: Sum of possible games: {}", game_possible_sum);
    println!("PART 2: Power sum:           : {}", game_power_sum);
}
