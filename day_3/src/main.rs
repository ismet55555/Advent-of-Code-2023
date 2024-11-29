use std::fs;

// Add up all numbers adjacent to a symbol, next to horizontally, vertically, or diagonally
// Find sum of all the numbers in the grid.

const INPUT_FILEPATH: &str = "inputs/INPUT_1_sample.txt";

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

    let symbols = vec![
        '+', '-', '*', '/', '#', '@', '!', '$', '%', '^', '&', '*', '(', ')', '_', '=', '+', '~',
        '`', '[', ']', '{', '}', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '?', '/',
    ];

    let h_len = input_lines[0].len();
    let v_len = input_lines.len();
    println!("horizontal grid length {}", h_len);
    println!("vertical grid length   {}", v_len);

    for (v, line) in input_lines.iter().enumerate() {
        println!("==============================");
        println!("{} - {}", v, line);

        for (h, char) in line.chars().enumerate() {
            println!("    {} - {}", h, char);
            if symbols.contains(&char) {
                println!("        FOUND: {} at ({}, {})", char, v, h);

                // Look around the symbol to find a digit
                for k in 0..3 {
                    for l in 0..3 {
                        let x: i32 = (h as i32) + ((l as i32) - 1);
                        let y: i32 = (v as i32) + ((k as i32) - 1);

                        // Skip the symbol itself
                        if y == (v as i32) && x == (h as i32) {
                            continue;
                        }

                        // Out of bounds
                        if x < 0 || x > (h_len as i32) - 1 || y < 0 || y > (v_len as i32) - 1 {
                            continue
                        }
                        println!("        ({}, {})", x, y);

                        // Check if item around the symbol is a digit
                        let item = input_lines[y as usize].chars().nth(x as usize).unwrap();
                        if item.is_digit(10) {
                            println!("            DIGIT FOUND: {}", item);
                            
                            // Search horizontally left and right to find the whole number and save it
                            let mut number = String::new();


                            // Search left
                            let mut i = x - 1;
                            while i >= 0 {
                                let item = input_lines[y as usize].chars().nth(i as usize).unwrap();
                                if item.is_digit(10) {
                                    number.insert(0, item);
                                    i -= 1;
                                } else {
                                    break;
                                }
                            }

                            // Search right
                            let mut i = x + 1;
                            while i < h_len as i32 {
                                let item = input_lines[y as usize].chars().nth(i as usize).unwrap();
                                if item.is_digit(10) {
                                    number.push(item);
                                    i += 1;
                                } else {
                                    break;
                                }
                            }


                            println!("            NUMBER FOUND: {}", number);

                        }


                    }
                }

            }
        }
    }
}
