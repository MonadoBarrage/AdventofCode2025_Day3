use std::fs::File;
use std::io::read_to_string;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let file_string = read_to_string(file).unwrap();
    let mut sum = 0;
    for line in file_string.lines() {
        // println!("{}", line);
        sum += find_highest_number(&String::from(line));
    }
    println!("Sum: {}", sum);
}

fn find_highest_number(number_string: &String) -> u64{
    if number_string.len() < 2 {return 0;}

    let mut highest_number = 0;
    let mut counter = 1;

    let split_numbers = number_string.chars();
    for first_num in split_numbers {
        let split_numbers2 = number_string[counter..].chars();

        for second_num in split_numbers2 {
            let new_num = (first_num.to_string() + second_num.to_string().as_str()).parse::<u64>().unwrap();
            if new_num > highest_number {
                highest_number = new_num;
            }
        }
        counter += 1;

    }

    return highest_number;
}
