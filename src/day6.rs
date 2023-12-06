// #[warn(dead_code)]

use itertools::Itertools;

fn get_combinations(time: u64, distance: u64) -> u64 {
	// let mut results: u64 = 0;

	// for i in 1..time {
	// 	let speed: u64 = i as u64;
	// 	let distance_covered: u64 = speed * (time - i);

	// 	if distance_covered > distance {
	// 		results += 1;
	// 	}
	// }

	// results

	return (1..time).map(|x| x * (time - x)).filter( |&x| x > distance).count() as u64;
}

pub fn solve(input: String) {
	let mut result: u64 = 0;
	let mut result_part2: u64 = 0;

	let input_lines = input.lines().collect_vec();

	let times: Vec<u64> = input_lines[0].split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect_vec();
	let distances: Vec<u64> = input_lines[1].split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect_vec();

	result = times.iter().zip(distances.iter()).map(|(&dist, &time)| get_combinations(dist, time)).product::<u64>();
	
	// part 2
	let times_part2 = Itertools::join(&mut times.clone().iter(), "").parse::<u64>().unwrap();
	let distances_part2 = Itertools::join(&mut distances.clone().iter(), "").parse::<u64>().unwrap();

	let combinations = get_combinations(times_part2, distances_part2);

	result_part2 = combinations;

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}