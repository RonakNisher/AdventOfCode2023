// #[warn(dead_code)]
use std::collections::HashMap;

// fn print_grid(grid: &Vec<Vec<char>>) {
// 	for r in 0..grid.len() {
// 		println!("{}", grid[r].iter().collect::<String>());
// 	}
// }

fn get_result_value(grid: &mut Vec<Vec<char>>) -> i32 {
	let mut result = 0;

	for (r, row) in grid.iter().enumerate() {
		let count = row.iter().filter(|&c| *c == 'O').count();
		result += (count * (grid.len() - r)) as i32;
	}

	return result;
}

fn get_tilted_value(grid: &mut Vec<Vec<char>>) {
	let mut result = 0;

	for c in 0..grid[0].len() {
		let mut r = 0;
		let mut count_zeros = 0;
		let mut start_zero_index = 0;
		while r < grid.len() {

			while r < grid.len() && grid[r][c] != '#' {
				if grid[r][c] == 'O' {
					count_zeros += 1;
				}
				r += 1;
			}

			if count_zeros > 0 {
				// tilt the zeros to the top
				let mut i = start_zero_index;
				while count_zeros > 0 {
					grid[i][c] = 'O';
					i += 1;
					count_zeros -= 1;
				}
	
				// fill the rest with .
				while i < grid.len() && i <= r - 1 {
					// println!(" i is {}", i);
					grid[i][c] = '.';
					i += 1;
				}
	
				// reset count_zeros & start_zero_index
				count_zeros = 0;
			}
			
			start_zero_index = r + 1;
			r += 1;
		}
	}
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut grid: Vec<Vec<char>> = Vec::new();
	let mut new_grid: Vec<Vec<char>> = Vec::new();

	let mut seen_map: HashMap<String, i32> = HashMap::new(); // key -> (cycle, result)
	let mut results_map: HashMap<i32,i32> = HashMap::new(); // cycle -> result
	let mut loop_length = 0;
	let mut loop_start_index = 0;

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		grid.push(line.chars().collect());
	});

	let cycles = 1000000000;

	for i in 0..cycles {
		///////////////////////////////
		// tilt north
		///////////////////////////////
		
		get_tilted_value(&mut grid);

		if i == 0 {
			result = get_result_value(&mut grid);
		}

		///////////////////////////////
		// transpose to West & Tilt
		///////////////////////////////
		
		new_grid.clear();

		for c in 0..grid[0].len() {
			let row = grid.iter().map(|row| row[c]).rev().collect::<Vec<char>>();
			new_grid.push(row);
		}
	
		get_tilted_value(&mut new_grid); // tilt west
	
		// transpose it back
	
		grid.clear();
	
		for c in 0..new_grid[0].len() {
			let row = new_grid.iter().map(|row| row[new_grid[0].len() - 1 - c]).collect::<Vec<char>>();
			grid.push(row);
		}
	
		new_grid.clear();
	
		///////////////////////////////
		// transpose to South & tilt
		///////////////////////////////
	
		for r in 0..grid.len() {
			new_grid.push(grid[grid.len() - 1 - r].clone());
		}
	
		get_tilted_value(&mut new_grid);
	
		// transpose it back
	
		grid.clear();
	
		for r in 0..new_grid.len() {
			grid.push(new_grid[new_grid.len() - 1 - r].clone());
		}
	
		///////////////////////////////
		// transpose to East & tilt
		///////////////////////////////

		new_grid.clear();

		for c in 0..grid[0].len() {
			let row = grid.iter().map(|row| row[grid[0].len() - 1 - c]).collect::<Vec<char>>();
			new_grid.push(row);
		}
		
		get_tilted_value(&mut new_grid);
		
		// transpose it back
		
		grid.clear();

		for c in 0..new_grid[0].len() {
			let row = new_grid.iter().map(|row| row[c]).rev().collect::<Vec<char>>();
			grid.push(row);
		}

		let res = get_result_value(&mut grid);


		// calculate key for finding loop 
		let mut res_string: String = String::new();
		grid.iter().for_each(|row| {
			res_string.push_str(row.iter().collect::<String>().as_str());
			res_string.push_str(":");
		});

		if seen_map.contains_key(&res_string) {
			loop_start_index = seen_map.get(&res_string).unwrap().clone();
			loop_length = i - loop_start_index;
			break;
		}

		seen_map.insert(res_string, i);
		results_map.insert(i, res);
	}

	// offset by loop_start_index since the loop does not start at 0
	let rem = (cycles - loop_start_index) % loop_length;

	// println!("rem: {}", rem);
	// println!("result: {}", results_map.get(&(rem - 1 + loop_start_index)).unwrap());
	result_part2 = results_map.get(&(rem - 1 + &loop_start_index)).unwrap().clone() as i32;


	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}