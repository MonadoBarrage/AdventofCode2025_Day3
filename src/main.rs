use std::fs::File;
use std::io::read_to_string;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let file_string = read_to_string(file).unwrap();
    let mut sum = 0;
    for line in file_string.lines() {
        // println!("{}", line);
        sum += find_highest_number_part2(&String::from(line));
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

fn find_highest_number_part2(number_string: &String) -> u64{
    let length = number_string.len();
    if length < 12 {return 0; }
    if length == 12 { return number_string.parse::<u64>().unwrap(); }

    let mut highest_number : u64 = 0;


    let split_numbers = number_string.chars().collect::<Vec<char>>();


    let mut position_tracker = 12;

    let mut counter = 0;
    let mut current_highest_int :u64 = 0;

    while position_tracker > 0 {

        for i in counter..(length-position_tracker+1){
            let compared_num = split_numbers[i].to_digit(10).unwrap() as u64;
            if compared_num > current_highest_int{
                counter = i;
                current_highest_int = compared_num;
            }
        }
        highest_number += current_highest_int*(10_u64.pow(position_tracker as u32 -1));

        counter += 1;
        current_highest_int = 0;

        position_tracker -= 1;
    }
    println!("Highest number: {}", highest_number);

    highest_number
}
