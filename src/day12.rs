// #[warn(dead_code)]
use std::collections::HashMap;

use itertools::Itertools;

fn should_return_out_of_bounds(index: usize, length: usize, is_code_vec_empty: bool) -> (bool, u64) {
	if index == length {
		if is_code_vec_empty {
			return (true, 1);
		}
		else {
			return (true, 0);
		}
	}

	return (false, 0);
}

fn get_number_of_patterns(code_vec: &Vec<u32>, partial_string: &mut Vec<char>, valid_count_map: &mut HashMap<(Vec<char>, Vec<u32>), u64>) -> u64 {

	let (should_return, res) = should_return_out_of_bounds(0, partial_string.len(), code_vec.is_empty());
	if should_return {
		return res;
	}

	if valid_count_map.contains_key(&(partial_string.clone(), code_vec.clone())) {
		return valid_count_map.get(&(partial_string.clone(), code_vec.clone())).unwrap().clone();
	}

	let mut i = 0;
	while i < partial_string.len() && partial_string.iter().nth(i).unwrap() == &'.' {
		i += 1;
	}
	
	let (should_return, res) = should_return_out_of_bounds(i, partial_string.len(), code_vec.is_empty());
	if should_return {
		return res;
	}
	
	let mut current_char = partial_string.iter().nth(i).unwrap();

	if current_char == &'#' {
		let mut damaged_count = 0;

		while i < partial_string.len() && partial_string.iter().nth(i).unwrap() == &'#' {
			i += 1;
			damaged_count += 1;
		}

		if i == partial_string.len() {
			if code_vec.len() == 1 && damaged_count == code_vec[0] {
				return 1;
			}
			else {
				return 0;
			}
		}

		current_char = partial_string.iter().nth(i).unwrap();

		if current_char == &'.' {
			// end of a block, check if the length matches
			if code_vec.first() == Some(&damaged_count) {
				let mut partial_string_copy = partial_string.clone();
				partial_string_copy.drain(0..i);

				let res = get_number_of_patterns(&code_vec[1..].to_vec(), &mut partial_string_copy, valid_count_map);

				return res;
			}
			else {
				return 0;
			}
		}
	}

	if current_char == &'?' {
		let mut path1 = partial_string.clone();
		path1[i] = '.';

		let mut path2 = partial_string.clone();
		path2[i] = '#';
		
		let path1_res = get_number_of_patterns(code_vec, &mut path1, valid_count_map);
		valid_count_map.insert((path1, code_vec.clone()), path1_res);
		
		let path2_res = get_number_of_patterns(code_vec, &mut path2, valid_count_map);
		valid_count_map.insert((path2, code_vec.clone()), path2_res);
		
		valid_count_map.insert((partial_string.clone(), code_vec.clone()), path1_res + path2_res);
		
		return path1_res + path2_res;
	}

	return 0;
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut valid_count_map: HashMap<(Vec<char>, Vec<u32>), u64> = HashMap::new();
	valid_count_map.reserve(10_000_0);

	input.lines().for_each(|line| {
		let (pattern, code) = line.split_once(" ").unwrap();
		// println!("pattern is {}", pattern);
		// println!("code is {}", code);

		let code_vec = code.split(",").map(|x| x.parse::<u32>().unwrap()).collect_vec();
		result += get_number_of_patterns(&code_vec, &mut pattern.chars().collect_vec(), &mut valid_count_map);

		valid_count_map.clear();

		// part 2

		let repeat_times = 5; // 5
		let mut pattern_part2 = pattern.to_string();
		pattern_part2.push('?');
		pattern_part2 = pattern_part2.repeat(repeat_times);
		pattern_part2.pop();

		let mut code_part2 = code.to_string();
		code_part2.push(',');
		code_part2 = code_part2.repeat(repeat_times);
		code_part2.pop();

		let code_vec = code_part2.split(",").map(|x| x.parse::<u32>().unwrap()).collect_vec();
		result_part2 += get_number_of_patterns(&code_vec, &mut pattern_part2.chars().collect_vec(), &mut valid_count_map);

		valid_count_map.clear();
	});

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}