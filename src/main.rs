use std::fs;
use std::env;

mod day1;
mod day2;

fn main() {
    // println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let day = &args[1];

    println!("Day: {}", day);

	let filepath = format!("/Users/ronaknisher/advent_of_code_2023/src/day{}_input.txt", day);

	println!("Filepath: {}", filepath);
    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

	// contents.lines().for_each(|line| {
	// 	println!("{}", line);
	// });

	match day.as_str() {
		"1" => day1::solve(contents),
		"2" => day2::solve(contents),
		_ => println!("No solution for day {}", day),
	}

	// day1::solve();
}
