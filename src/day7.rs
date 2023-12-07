// #[warn(dead_code)]

use std::{collections::HashMap, fmt};

use itertools::Itertools;

#[derive(Debug)]
#[derive(Clone)]
enum HandType {
	HighCard,
	Pair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfaKind,
}

#[derive(Debug)]
#[derive(Clone)]
struct Hand {
	cards: String,
	bet: i32,
	hand_type: HandType,
	hand_type_part2: HandType,
	rank: i32,
	rank_part2: i32,
}

impl fmt::Display for Hand {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Hand: {}, Bet: {}, Type: {:?}, Rank: {}, Type2: {:?}, Rank2: {}", self.cards, self.bet, self.hand_type, self.rank,self.hand_type_part2, self.rank_part2)
	}
}

impl Hand {
	fn get_numuric_value(c: char) -> i32 {
		match c {
			'A' => 14,
			'K' => 13,
			'Q' => 12,
			'J' => 11,
			'T' => 10,
			_ => c.to_digit(10).unwrap() as i32,
		}
	}

	fn compare_cards(&self, other: &Hand) -> std::cmp::Ordering {
		for (a, b) in self.cards.chars().zip(other.cards.chars()) {

			let a = Self::get_numuric_value(a);
			let b = Self::get_numuric_value(b);

			let cmp = a.cmp(&b);

			if cmp != std::cmp::Ordering::Equal {
				return cmp;
			}
		}

		std::cmp::Ordering::Equal
	}

	fn get_numuric_value_part2(c: char) -> i32 {
		match c {
			'A' => 14,
			'K' => 13,
			'Q' => 12,
			'J' => 1,
			'T' => 10,
			_ => c.to_digit(10).unwrap() as i32,
		}
	}

	fn compare_cards_part2(&self, other: &Hand) -> std::cmp::Ordering {
		for (a, b) in self.cards.chars().zip(other.cards.chars()) {

			let a = Self::get_numuric_value_part2(a);
			let b = Self::get_numuric_value_part2(b);

			let cmp = a.cmp(&b);

			if cmp != std::cmp::Ordering::Equal {
				return cmp;
			}
		}

		std::cmp::Ordering::Equal
	}
}

fn get_hand_type(cards: &str) -> HandType {
	let mut cards_map: HashMap<char, i32> = HashMap::new();

	cards.chars().for_each(|c| {
		let count = cards_map.get(&c).unwrap_or(&0);
		cards_map.insert(c, count + 1);
	});

	let mut sorted = cards_map.values().sorted().rev();

	let first = sorted.next().unwrap();

	match first {
		5 => return HandType::FiveOfaKind,
		4 => return HandType::FourOfAKind,
		3 =>  { 
				if sorted.next().unwrap() == &2 {
					return HandType::FullHouse;
				}
				else {
					return HandType::ThreeOfAKind; }
				},
		2 => {
				if sorted.next().unwrap() == &2 {
					return HandType::TwoPair;
				}
				else {
					return HandType::Pair; }
				},
		1 => return HandType::HighCard,
		_ => panic!("Invalid value"),
	}

}

fn get_hand_type_part2(cards: &str) -> HandType {
	let mut cards_map: HashMap<char, i32> = HashMap::new();
	let mut count_of_j = 0;

	cards.chars().for_each(|c| {
		if c == 'J' {
			count_of_j += 1;
		}
		else {
			let count = cards_map.get(&c).unwrap_or(&0);
			cards_map.insert(c, count + 1);
		}
	});

	let mut sorted = cards_map.values().sorted().rev();

	let first = sorted.next().unwrap_or(&0) + count_of_j;

	match first {
		5 => return HandType::FiveOfaKind,
		4 => return HandType::FourOfAKind,
		3 =>  { 
				if sorted.next().unwrap() == &2 {
					return HandType::FullHouse;
				}
				else {
					return HandType::ThreeOfAKind; }
				},
		2 => {
				if sorted.next().unwrap() == &2 {
					return HandType::TwoPair;
				}
				else {
					return HandType::Pair; }
				},
		1 => return HandType::HighCard,
		_ => panic!("Invalid value"),
	}

}

fn get_rank(hand_type: &HandType) -> i32 {
	match hand_type {
		HandType::FiveOfaKind => 7,
		HandType::FourOfAKind => 6,
		HandType::FullHouse => 5,
		HandType::ThreeOfAKind => 4,
		HandType::TwoPair => 3,
		HandType::Pair => 2,
		HandType::HighCard => 1,
	}
}

fn get_hand(cards: &str, bet: i32) -> Hand {

	let hand_type = get_hand_type(cards);
	let hand_type_part2 = get_hand_type_part2(cards);
	let hand = Hand {
		cards: String::from(cards),
		bet,
		rank: get_rank(&hand_type),
		hand_type: hand_type,
		rank_part2: get_rank(&hand_type_part2),
		hand_type_part2: hand_type_part2,
	};

	hand
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut hands: Vec<Hand> = Vec::new();

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		let (cards, bet) = line.split_once(" ").unwrap();

		hands.push(get_hand(cards, bet.parse::<i32>().unwrap()));

	});

	let mut hands_part2 = hands.clone();

	hands.sort_by(|a, b| {
		let first = a.rank.cmp(&b.rank);
		let second = a.compare_cards(&b);

		first.then(second)
	});

	for (i, value) in hands.iter().enumerate() {
		// println!("{}: {}", i, value);
		result += value.bet * (i as i32 + 1);
	}

	// println!("hands is {:?}", hands);

	// part 2

	println!("*******************");
	// hands_part2.iter().for_each(|h| {
	// 	println!("{}", h);
	// });

	hands_part2.sort_by(|a, b| {
		let first = a.rank_part2.cmp(&b.rank_part2);
		let second = a.compare_cards_part2(&b);

		first.then(second)
	});

	for (i, value) in hands_part2.iter().enumerate() {
		// println!("{}: {}", i, value);
		result_part2 += value.bet * (i as i32 + 1);
	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}