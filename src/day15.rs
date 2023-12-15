// #[warn(dead_code)]
use std::collections::HashMap;

struct Lens {
	label: String,
	focal_length: i32,
}

fn get_hash(code: &str) -> i32 {
	let mut current_value = 0;

	code.chars().for_each(|c| {
		current_value += c as i32;
		current_value = current_value.wrapping_mul(17).wrapping_rem(256);
	});

	return current_value;
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut codes: Vec<&str> = Vec::new();

	let mut boxes: HashMap<i32, Vec<Lens>> = HashMap::new();

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		codes = line.split(",").collect();

		codes.iter().for_each(|&code| {
			result += get_hash(code);

			// part 2
			if code.contains("=") {
				// insert into hashmap
				let (label, focal_length) = code.split_once("=").unwrap();
				let box_number = get_hash(label);
				let entry = boxes.entry(box_number).or_insert(Vec::new());
				let pos = entry.iter_mut().position(| lens | lens.label == label);

				if pos.is_some() {
					entry[pos.unwrap()].focal_length = focal_length.parse::<i32>().unwrap();
				}
				else {
					entry.push(Lens {
						label: label.to_string(),
						focal_length: focal_length.parse::<i32>().unwrap(),
					});
				}
			} else {
				// remove from hashmap
				let label = &code[0..code.len() - 1];
				let box_number = get_hash(label);
				let entry = boxes.entry(box_number).or_default();

				if entry.len() > 0 {
					let pos = entry.iter_mut().position(| lens | lens.label == label);
					
					if pos.is_some() {
						entry.remove(pos.unwrap());
					}
				}
			}
		});


		boxes.iter().for_each(|(box_number, lenses)| {
			for (index, lens) in lenses.iter().enumerate() {
				result_part2 += (box_number + 1) * lens.focal_length * (index as i32 + 1);
			};
		});

	});
	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}