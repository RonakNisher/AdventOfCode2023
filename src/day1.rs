// #[warn(dead_code)]

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	input.lines().for_each(|line| {
		// println!("{}", line);

		let nums = line.chars().filter(|x| x.is_ascii_digit()).collect::<Vec<char>>();
		let res = format!("{}{}",&nums[0], &nums[nums.len() - 1]).parse::<u32>().unwrap();
		
		result += res;

		////////////////////////////
		// part 2
		////////////////////////////

		let line_part2 = line.replace("one", "one1one")
									.replace("two", "two2two")
									.replace("three", "three3three")
									.replace("four", "four4four")
									.replace("five", "five5five")
									.replace("six", "six6six")
									.replace("seven", "seven7seven")
									.replace("eight", "eight8eight")
									.replace("nine", "nine9nine");
		
		let nums = line_part2.chars().filter(|x| x.is_ascii_digit()).collect::<Vec<char>>();
		let res = format!("{}{}",&nums[0], &nums[nums.len() - 1]).parse::<u32>().unwrap();
		
		// println!("{} -> {}", line_part2, res);
		result_part2 += res;
	});

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}