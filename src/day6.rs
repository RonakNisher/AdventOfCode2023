// #[warn(dead_code)]

use itertools::Itertools;

fn get_combinations(time: u64, distance: u64) -> Vec<u64> {
	let mut results: Vec<u64> = Vec::new();

	for i in 0..time {
		let speed: u64 = i as u64;
		let distance_covered: u64 = speed * (time - i);

		if distance_covered > distance {
			results.push(distance_covered);
		}
	}

	results
}

pub fn solve(input: String) {
	let mut result: u64 = 0;
	let mut result_part2: u64 = 0;

	let input_lines = input.lines().collect::<Vec<_>>();

	let times: Vec<u64> = input_lines[0].split_whitespace().skip(1).collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
	let distances: Vec<u64> = input_lines[1].split_whitespace().skip(1).collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
	let mut results: Vec<u64> = Vec::new();

	println!("times are {:?}", times);
	println!("distances are {:?}", distances);

	let times_part2 = Itertools::join(&mut times.clone().iter(), "").parse::<u64>().unwrap();
	let distances_part2 = Itertools::join(&mut distances.clone().iter(), "").parse::<u64>().unwrap();

	// let times_part2 = times.clone().join("").parse::<u32>().unwrap();
	println!("times_part2 is {}", times_part2);
	println!("distances_part2 is {}", distances_part2);

	for i in 0..times.len() {
		let time = times[i];
		let distance = distances[i];

		// let speed = distance / time;

		println!("time is {}, dist is {}", time, distance);

		let combinations = get_combinations(time, distance);

		println!("combinations are {:?}", combinations);

		results.push(combinations.len() as u64);
	}

	result = results.iter().product();

	// part 2
	// println!("time is {}, dist is {}", time, distance);

	let combinations = get_combinations(times_part2, distances_part2);

	// println!("combinations are {:?}", combinations);

	result_part2 = combinations.len() as u64;

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}