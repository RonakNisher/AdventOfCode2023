// #[warn(dead_code)]

use std::collections::HashMap;

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut map_card_id_to_count: HashMap<String, i32> = HashMap::new();

	for i in 1..input.lines().count()+1 {
		map_card_id_to_count.insert(i.to_string(), 1);
	}

	input.lines().for_each(|line| {
		println!();
		println!("line is {}", line);

		let (card, numbers) = line.split_once(": ").unwrap();

		let (_, mut card_id) = card.split_once(" ").unwrap();
		card_id = card_id.trim();

		let (winning_numbers, our_numbers) = numbers.split_once(" | ").unwrap();

		let winning_numbers = winning_numbers.split_whitespace().collect::<Vec<_>>();
		let our_numbers = our_numbers.split_whitespace().collect::<Vec<_>>();
		let matching_numbers = our_numbers.iter().filter(|x| winning_numbers.contains(x)).collect::<Vec<_>>();

		println!("no of matches are {}", matching_numbers.len());

		if matching_numbers.len() != 0 {
			let base:i32 = 2;
			let pow: u32 = matching_numbers.len() as u32;
			result += base.pow(pow-1);
			println!("result is {}", result);

			// part2
			let current_no_ofcards = map_card_id_to_count.get(card_id.parse::<usize>().unwrap().to_string().as_str()).unwrap().clone();
			println!("No of cards in {} is {:?}", card_id, current_no_ofcards);

			let start_pos: usize = card_id.parse::<usize>().unwrap() + 1;

			for i in 0..matching_numbers.len() {
				let key = i + start_pos;

				if let Some(x) = map_card_id_to_count.get_mut(key.to_string().as_str()) {
					*x += current_no_ofcards * 1;
				}
			}

		}
	});

	result_part2 = map_card_id_to_count.iter().map(|x| x.1).sum();

	// println!("map_card_id_to_count is {:?}", map_card_id_to_count);

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}