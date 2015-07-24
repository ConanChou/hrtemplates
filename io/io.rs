#![feature(io)]
use std::old_io as io;

fn main() {
    let mut stdin = io::stdin();
    let line1 = stdin.read_line().unwrap();
    let line2 = stdin.read_line().unwrap();
    // Removes the new lines /n from the end
    let num_str1 = line1.trim();
    let num_str2 = line2.trim();
    // Convert to an int32
    let num1 = num_str1.parse::<i32>().unwrap();
    let num2 = num_str2.parse::<i32>().unwrap();
    // Print the sum
    println!("{}", num1 + num2 );
}
