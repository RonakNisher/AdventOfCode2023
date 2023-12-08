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
	fn get_numuric_value(c: char, is_part_two: bool) -> i32 {
		match c {
			'A' => 14,
			'K' => 13,
			'Q' => 12,
			'J' => if is_part_two { 1 } else { 11 },
			'T' => 10,
			_ => c.to_digit(10).unwrap() as i32,
		}
	}

	fn compare_cards(&self, other: &Hand, is_part_two: bool) -> std::cmp::Ordering {
		for (a, b) in self.cards.chars().zip(other.cards.chars()) {

			let a = Self::get_numuric_value(a, is_part_two);
			let b = Self::get_numuric_value(b, is_part_two);

			let cmp = a.cmp(&b);

			if cmp != std::cmp::Ordering::Equal {
				return cmp;
			}
		}

		std::cmp::Ordering::Equal
	}
}

fn match_hand_type(max_common_cards_count: &i32, next_common_cards_count: &i32) -> HandType {
	match (max_common_cards_count, next_common_cards_count) {
		(5, _) => HandType::FiveOfaKind,
		(4, _) => HandType::FourOfAKind,
		(3, 2) => HandType::FullHouse,
		(3, _) => HandType::ThreeOfAKind,
		(2, 2) => HandType::TwoPair,
		(2, _) => HandType::Pair,
		(1, _) => HandType::HighCard,
		_ => panic!("Invalid value"),
	}
}

fn get_hand_type(cards: &str) -> HandType {
	let mut cards_map: HashMap<char, i32> = HashMap::new();

	cards.chars().for_each(|c| {
		let count = cards_map.get(&c).unwrap_or(&0);
		cards_map.insert(c, count + 1);
	});

	let mut sorted = cards_map.values().sorted().rev();

	let (first, second) = (sorted.next().unwrap(), sorted.next().unwrap_or(&0));

	return match_hand_type(first, second);
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

	return match_hand_type(&first, sorted.next().unwrap_or(&0));

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
		let (cards, bet) = line.split_once(" ").unwrap();

		hands.push(get_hand(cards, bet.parse::<i32>().unwrap()));

	});

	let mut hands_part2 = hands.clone();

	hands.sort_by(|a, b| {
		let first = a.rank.cmp(&b.rank);
		let second = a.compare_cards(&b, false /*is_part_two*/);

		first.then(second)
	});

	for (i, value) in hands.iter().enumerate() {
		result += value.bet * (i as i32 + 1);
	}

	//////////////////////
	// Part 2
	//////////////////////

	hands_part2.sort_by(|a, b| {
		let first = a.rank_part2.cmp(&b.rank_part2);
		let second = a.compare_cards(&b, true /*is_part_two*/);

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