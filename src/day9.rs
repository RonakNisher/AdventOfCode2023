// #[warn(dead_code)]

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		let mut input: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
		let mut last_number: Vec<i64> = Vec::new();
		let mut first_number: Vec<i64> = Vec::new();

		while !input.iter().all(|x| x == &0) {
			last_number.push(input.last().unwrap().clone());
			first_number.push(input.first().unwrap().clone());

			let mut next_input: Vec<i64> = Vec::new();

			for i in 0..input.len() - 1 {
				next_input.push(input[i + 1] - input[i]);
			}

			input = next_input;
		}

		result += last_number.iter().sum::<i64>();

		// part 2
		first_number.reverse();
		let mut diff = 0;
		first_number.iter().for_each(|x| { diff = x - diff; });

		result_part2 += diff;
	});

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}