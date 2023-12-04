// #[warn(dead_code)]
use core::panic;
use std::collections::{HashMap, hash_map::Entry};

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut symbols_map: HashMap<(i32, usize), char> = HashMap::new(); //(x,y) -> symbol
	let mut symbols_map_part2: HashMap<(i32, usize), Vec<i32>> = HashMap::new(); //(x,y) -> gears
	let mut row = 0;
	
	input.lines().for_each(|line| {
		// println!("\n line is {}", line);

		let mut col = 0;
		line.chars().for_each(|x| {

			if !x.is_numeric() && x != '.' {
				symbols_map.insert((row, col), x);
			}

			col += 1;
		});

		row += 1;

	});

	let mut row: i32 = 0;

	input.lines().for_each(|line| {

		println!("\nline is {}", line);
		let chars: Vec<char> = line.chars().collect();
		let mut startPos = 0;

		while startPos < chars.len() {

			if !chars[startPos].is_numeric() {
				startPos += 1;
				continue;
			}
	
			let mut endPos = startPos;
	
			while (endPos < chars.len() && chars[endPos].is_numeric()) {
				endPos += 1;
			}

			let num = chars[startPos..endPos].iter().collect::<String>().parse::<i32>().unwrap();

			println!("Row: {} -> startPos is {}, endPos is {}", row, startPos, endPos);
			println!("num is {}", num);

			let mut found_adjacent_symbol = false;
			let mut matchingRow = 0;
			let mut matchingCol = 0;

			for col in startPos..endPos {
				// println!("x is {}", col);
				found_adjacent_symbol = symbols_map.get(&(row - 1, col)) != None || 
										symbols_map.get(&(row + 1, col)) != None;

				println!("upper/lower row is {}", found_adjacent_symbol);

				if found_adjacent_symbol {

					matchingCol = col;

					matchingRow = if symbols_map.get(&(row - 1, col)) != None {
						row - 1
					} else {
						row + 1
					};

					break;
				}
			}

			if !found_adjacent_symbol {
				// check corners

				let end = endPos - 1;

				println!("checking corners: {} -> {}, {}", row, startPos, end);

				if row > 0 {
					found_adjacent_symbol = symbols_map.get(&(row - 1, end + 1)) != None; // top right

					if found_adjacent_symbol {
						matchingRow = row - 1;
						matchingCol = end + 1;
					}

					println!("top right is {}", found_adjacent_symbol);
				}

				if !found_adjacent_symbol && row > 0 && startPos > 0 {
					found_adjacent_symbol = symbols_map.get(&(row - 1, startPos - 1)) != None; // top left

					if found_adjacent_symbol {
						matchingRow = row - 1;
						matchingCol = startPos - 1;
					}

					println!("top left is {}", found_adjacent_symbol);
				}
				
				if !found_adjacent_symbol && startPos > 0 {
					found_adjacent_symbol = symbols_map.get(&(row, startPos - 1)) != None; // left
												
					if found_adjacent_symbol {
						matchingRow = row;
						matchingCol = startPos - 1;
					}
					
					if !found_adjacent_symbol {
						found_adjacent_symbol = symbols_map.get(&(row + 1, startPos - 1)) != None; // bottom left

						if found_adjacent_symbol {
							matchingRow = row + 1;
							matchingCol = startPos - 1;
						}
					}
					
					println!("left is {}", found_adjacent_symbol);
				}
				
				if !found_adjacent_symbol {
					found_adjacent_symbol = symbols_map.get(&(row, end + 1)) != None;	// right

					if found_adjacent_symbol {
						matchingRow = row;
						matchingCol = end + 1;
					}
					else {
						found_adjacent_symbol = symbols_map.get(&(row + 1, end + 1)) != None;	// bottom right

						if (found_adjacent_symbol) {
							matchingRow = row + 1;
							matchingCol = end + 1;
						}
					}
											

					println!("right is {}", found_adjacent_symbol);
				}
			}

			if found_adjacent_symbol {
				result += num;
				println!("*******************");
				println!("found adjacent symbol for num {}, new result {}", num, result);
				println!("*******************");

				match symbols_map_part2.entry((matchingRow, matchingCol)) {
					Entry::Vacant(e) => { e.insert(vec![num]); },
					Entry::Occupied(mut e) => { println!("------------------  found entry for {}", num); e.get_mut().push(num); }
				}
			}
	
			startPos = endPos;
	
		}

		row += 1;

	 });


	println!("Part 2 symbols_map is {:?}", symbols_map_part2);

	// Part 2 calc

	result_part2 = symbols_map_part2.iter().filter(|entry| {
		return entry.1.len() == 2;
	}).map(|entry| {
		return entry.1[0] * entry.1[1];
	}).sum::<i32>();

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}