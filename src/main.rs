use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut values = HashMap::new();
    values.insert('I', 1);
    values.insert('V', 5);
    values.insert('X', 10);
    values.insert('L', 50);
    values.insert('C', 100);
    values.insert('D', 500);
    values.insert('M', 1000);

    let mut result = 0;

    let chars: Vec<char> = s.chars().collect();

    // loop through each character in the string
    for i in 0..chars.len() {
        // get the currenct and next value from HashMap
        let current_value = values[&chars[i]];

        let next_value = if i + 1 < chars.len() {
            values[&chars[i + 1]]
        } else {
            0
        };

        // add or subtract based on the next value
        if current_value < next_value {
            result -= current_value;
        } else {
            result += current_value;
        }
    }

    result
}

fn main() {
    // Test cases
    let roman1 = "III".to_string();
    let roman2 = "LVIII".to_string();
    let roman3 = "MCMXCIV".to_string();

    println!("{} -> {}", roman1, roman_to_int(roman1.clone())); // Output 3
    println!("{} -> {}", roman2, roman_to_int(roman2.clone())); // Output 58
    println!("{} -> {}", roman3, roman_to_int(roman3.clone())); // Output 1994
}
