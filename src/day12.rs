// #[warn(dead_code)]
use std::{collections::{HashMap, HashSet}, result};

use itertools::{Itertools};

fn get_valid_combinations(combinations: &Vec<Vec<char>>, code_vec: &Vec<u32>) -> u64 {
	let mut result = 0;

	println!("code_vec is {:?}", code_vec);
	println!("count of combinations {}", combinations.len());

	let mut invalid_starts: HashSet<Vec<char>> = HashSet::new();

	combinations.iter().for_each(|combination| {
		let mut code_index = 0;
		let mut damaged_block_size = 0;
		let mut valid = true;

		println!();
		println!("validating combination: {}", combination.iter().join(""));

		if invalid_starts.iter().any(|x| combination.starts_with(x)) {
			valid = false;
			println!("Skipping due to prev invalid start");
		}

		let mut i = 0;
		while valid && i < combination.len() {
			if combination[i] == '#' {
				// let mut count = 0;
				while i < combination.len() && combination[i] == '#' {
					damaged_block_size += 1;
					i += 1;
				}
			}
			else {
				if damaged_block_size != 0 {
					if code_index >= code_vec.len() || damaged_block_size != code_vec[code_index] {
						valid = false;

						invalid_starts.insert(combination[0..i].to_vec());
						break;
					}
					else {
						code_index += 1;
						damaged_block_size = 0;
					}
				}
				i += 1;
			}
		}

		if valid && combination[combination.len() - 1] == '#' {
			// code_index += 1;

			// println!("combination: {}", combination.iter().join(""));
			// println!("code_index: {}", code_index);
			// println!("damaged_block_size: {}", damaged_block_size);

			if code_index >= code_vec.len() || damaged_block_size != code_vec[code_index] {
				// println!("invalid block size");
				valid = false;
			}

			code_index += 1;
		}

		if code_index != code_vec.len() {
			valid = false;
		}

		// println!("isValid: {}", valid);
		
		if valid {
			// println!("valid combination: {}", combination.iter().join(""));
			// println!();
			result += 1;
		}
	});

	println!("------------------------------- valid count is {}", result);

	return result;
}

fn get_number_of_patterns(pattern: &str, code: &str) -> u64 {

	let code_vec = code.split(",").map(|x| x.parse::<u32>().unwrap()).collect_vec();
	// println!("code_vec is {:?}", code_vec);
	// println!("*******************");

	// let mut result = 0;

	let mut combinations: Vec::<Vec<char>> = Vec::new();
	let mut pattern_copy = pattern.chars().collect_vec();

	let mut i = 0;

	while i < pattern.len() {
		// let mut code_index = 0;
		let mut count_unknown = 0;

		println!();
		println!("pattern_copy[i] is {}, index {}", pattern_copy[i], i);
		println!("combinations count is {}", combinations.len());

		if pattern_copy[i] == '?' {
			let mut j = i;
			while j < pattern_copy.len() && pattern_copy[j] == '?' {
				count_unknown += 1;
				j += 1;
			}

			i += count_unknown;

			// println!("count_unknown is {}", count_unknown);

			let sets = (0..count_unknown).powerset().collect::<Vec<_>>();

			// println!("sets is {:?}", sets);

			let seed: Vec<char> = vec!('.'; count_unknown);
			// println!("seed is {:?}", seed);

			let mut permutations: Vec<Vec<char>> = Vec::new();

			sets.iter().for_each(|x| {
				let mut seed_copy = seed.clone();
				x.iter().for_each(|y| {
					seed_copy[*y] = '#';
				});

				permutations.push(seed_copy);
			});

			if combinations.is_empty() {
				combinations = permutations.clone();
			}
			else {
				let mut combinations_copy: Vec<Vec<char>> = Vec::new();
				permutations.iter_mut().for_each(|perm| {
					let perm_copy = perm.clone();
					let _ = combinations.iter_mut().for_each(|x| {
						let mut entry: Vec::<char> = Vec::new();
						entry.append(x.clone().as_mut());
						entry.append(perm_copy.clone().as_mut());

						combinations_copy.push(entry);
						// x.append(perm_copy.as_mut());
						});
				});

				combinations = combinations_copy.clone();

				// println!("combinations is {:?}", combinations);
			}
			println!("unknown");
			// combinations.iter().for_each( |x | {
			// 	println!("{}", x.iter().join(""));
			// });
		}
		else {
			if combinations.is_empty() {
				combinations.push(vec![pattern_copy[i]]);
			}
			else {
				combinations.iter_mut().for_each(|x| x.push(pattern_copy[i]));
			}

			println!("not unknown");
			// combinations.iter().for_each( |x | {
			// 	println!("{}", x.iter().join(""));
			// });

			i += 1;

		}
	}

	// println!("final combinations --------------------------------");
	// combinations.iter().for_each( |x | {
	// 	println!("{}", x.iter().join(""));
	// });


	return  get_valid_combinations(&combinations, &code_vec);
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	input.lines().for_each(|line| {
		println!();
		// println!("line is {}", line);

		let (pattern, code) = line.split_once(" ").unwrap();
		println!("pattern is {}", pattern);
		println!("code is {}", code);

		// result += get_number_of_patterns(pattern, code);

		// part 2

		let repeat_times = 5; // 5
		let mut pattern_part2 = pattern.to_string();
		pattern_part2.push('?');
		pattern_part2 = pattern_part2.repeat(repeat_times);
		pattern_part2.pop();

		let mut code_part2 = code.to_string();
		code_part2.push(',');
		// let mut code_part2 = code.chars().collect_vec().clone();
		// code_part2.push(',');
		code_part2 = code_part2.repeat(repeat_times);//.iter().join("").pop().unwrap().to_string();
		code_part2.pop();
		// code_part2 = code_part2[0..code_part2.len() - 1];

		println!("pattern_part2 is {}", pattern_part2);
		println!("code_part2 is {}", code_part2);

		result_part2 += get_number_of_patterns(&pattern_part2, &code_part2);
	});

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}