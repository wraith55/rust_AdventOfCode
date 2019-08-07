use std::collections::HashMap;
use std::fs;

pub fn day1_solve() {
    let file_contents = read_day1_file();
    let input_ints = file_to_array(file_contents);
    let input_sum = add_all_in_vec(&input_ints);
    println!("sum result: {}", input_sum);
    let double_freq = find_first_repeat_result(&input_ints);
    println!("first double freq: {}", double_freq);
}

fn read_day1_file() -> String {
    let file_name = "./data/day1.input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file!");
    return contents;
}

fn file_to_array(contents: String) -> Vec<i32> {
    let input_vec = contents.split_whitespace().collect::<Vec<&str>>();
    let mut input_ints = Vec::new();

    for input in input_vec {
        let input_int = input.parse::<i32>().unwrap();
        input_ints.push(input_int);
    }

    return input_ints;
}

fn add_all_in_vec(input_ints: &[i32]) -> i32 {
    let sum: i32 = input_ints.iter().sum();
    return sum;
}

fn find_first_repeat_result(input_ints: &[i32]) -> i32 {
    let mut result_map = HashMap::new();
    let mut acc = 0;
    result_map.insert(acc.clone(), 1);

    loop {
        for input in input_ints {
            acc += input;
            if !result_map.contains_key(&acc) {
                result_map.insert(acc.clone(), 1);
            } else {
                return acc;
            }
        }
    }
}
