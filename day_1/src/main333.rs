use std::fs;
use std::collections::HashMap;

const INPUT_FILEPATH: &str = "inputs/INPUT.txt";

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    env_logger::init();

    // Read the file
    let contents = fs::read_to_string(INPUT_FILEPATH).expect("ERROR: Failed to read input file");

    // Divide the lines in file into a vector of strings
    let input_lines: Vec<&str> = contents.split("\n").collect();

    // Remove all empty lines
    let input_lines: Vec<&str> = input_lines.iter().filter(|&x| x != &"").map(|&x| x).collect();

    ////////////////////////////////////////////////////////////////////////

    let mut total_sum: u32 = 0;

    for line_raw in input_lines {
        println!("---------");
        println!("{}", line_raw);
        let mut line = line_raw.to_string();

        // Define string vector with all numbers
        let numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        // Put all numbers in hashmap as keys, and a empty vector as value
        let mut numbers_priority: HashMap<&str, Vec<u32>> = HashMap::new();
        for num in numbers {
            numbers_priority.insert(num, vec![]);
        }

        // Make map with keys as number strings, and values as actual digits matching
        let mut numbers_map: HashMap<&str, u32> = HashMap::new();
        numbers_map.insert("one", 1);
        numbers_map.insert("two", 2);
        numbers_map.insert("three", 3);
        numbers_map.insert("four", 4);
        numbers_map.insert("five", 5);
        numbers_map.insert("six", 6);
        numbers_map.insert("seven", 7);
        numbers_map.insert("eight", 8);
        numbers_map.insert("nine", 9);

        // Find the number substitution priority
        for (key, _value) in numbers_priority.iter_mut() {
            if let Some(found_index) = line.find(key) {
                // println!("Substring '{}' found at index {}.", key, found_index);
                _value.push(found_index as u32);
                // println!("{:?}, {:?}", key, _value);
            }
        }
        // println!("{:?}", numbers_priority);

        // let line = line.replace("nine", "9");

        // Sort the keys based on the lowest index to highest index
        let mut sorted_keys: Vec<&str> = numbers_priority.keys().cloned().collect();
        sorted_keys.sort_by_key(|key| {
            numbers_priority[key]
                .iter()
                .cloned()
                .min()
                .unwrap_or(u32::MAX) // Use a large value for keys with empty indices
        });
        // println!("{:?}", sorted_keys);

        for key in sorted_keys {
            // permanantly Replace the number strings in the input string with actual numbers
            let num = numbers_map[key];
            let num_str = num.to_string();
            line = line.replace(key, &num_str);
        }

        println!("{}", line);

        // Create vector with potential numbers
        let mut temp: Vec<u32> = vec![];

        for c in line.chars() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                temp.push(num);
            }
        }
        println!("{:?}", temp);

        let l = temp.len();
        let mut sum: u32 = 0;
        if l == 0 {
            // No numbers found
        } else if l == 1 {
            // Only one number found
            let first = temp[0].to_string();
            sum = format!("{}{}", first, first).parse::<u32>().unwrap();
        } else {
            // Multiple numbers found
            let first = temp[0].to_string();
            let last = temp[l - 1].to_string();
            let new_num = first + &last;
            sum = new_num.parse::<u32>().unwrap();

        }
        println!("{}", sum);
        total_sum += sum;
    }
    println!("----------------------");
    println!("{}", total_sum);
}
