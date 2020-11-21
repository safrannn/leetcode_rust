// use std::io;
// use std::io::BufRead;
// pub fn main() {}

// fn main_1() {
//     let stdin = io::stdin();
//     let mut lines = stdin.lock().lines();
//     let line_count = lines.next().unwrap().unwrap();
//     let count = line_count.parse::<usize>().unwrap();
//     let ratings: Vec<i32> = lines
//         .take(count)
//         .map(|value| value.unwrap().parse::<i32>().unwrap())
//         .collect();
//     // let change_count = candies(&ratings);
//     // println!("{}", change_count);
// }

// pub fn main_2() {
//     // variable declaration
//     let mut _num_str_1 = String::new();
//     let mut _num_str_2 = String::new();

//     // read variables
//     io::stdin()
//         .read_line(&mut _num_str_1)
//         .ok()
//         .expect("read error");
//     io::stdin()
//         .read_line(&mut _num_str_2)
//         .ok()
//         .expect("read error");

//     // parse integers
//     let mut _num_1: i32 = _num_str_1.trim().parse().ok().expect("parse error");
//     let mut _num_2: i32 = _num_str_2.trim().parse().ok().expect("parse error");

//     // print the sum
//     // Hint: Type println!("{}", _num_1 + _num_2); below
//     println!("{}", _num_1 + _num_2);
// }

// pub fn read_line() -> String {
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Could not read stdin!");
//     return input;
// }
