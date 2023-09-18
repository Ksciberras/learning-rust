use std::io;

fn main() {
    println!("Please enter a position in the fibonacci sequence :");

    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to enter nth number");
    let mut previous_value: u32;
    let mut temp_value = 0;
    let mut current_value: u32 = 1;
    for index in 0..position.trim().parse().unwrap() {
        if index == 0 {
            continue;
        }
        previous_value = temp_value;
        temp_value = current_value;
        current_value = current_value + previous_value;
    }
    println!("The {position}th value in the fibonacci is {current_value}");
}
