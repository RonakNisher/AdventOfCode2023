// #[warn(dead_code)]
use core::panic;

const PART1_RED_MAX: i32 = 12;
const PART1_GREEN_MAX: i32 = 13;
const PART1_BLUE_MAX: i32 = 14;

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	input.lines().for_each(|line| {
		// println!("\n line is {}", line);

		let line_vec = line.split(": ").collect::<Vec<&str>>();
		let game_number = line_vec[0].split(" ").last().unwrap().parse::<u32>().unwrap();
		// println!("gameNumber is {}", game_number);

		let mut invalid_part1 = false;

		let mut part2_red_max = 0;
		let mut part2_green_max = 0;
		let mut part2_blue_max = 0;

		let re = regex::Regex::new(r"(\d+) (blue|green|red)").unwrap(); // "number color"
		line_vec[1].trim().split(";").for_each(|x| {
			// println!("turn is {}", x);
			x.split(",").for_each(|balls| {

				let Some((_, [number, color])) = re.captures(balls).map(|caps| caps.extract()) else { return };

				let number = number.parse::<i32>().unwrap();
				match color {
					"blue" => {
						if number > PART1_BLUE_MAX {
							invalid_part1 = true;
						};

						part2_blue_max = core::cmp::max(part2_blue_max, number);
					},
					"green" => {
						if number > PART1_GREEN_MAX {
							invalid_part1 = true;
						};

						part2_green_max = core::cmp::max(part2_green_max, number);
					},
					"red" => {
						if number > PART1_RED_MAX {
							invalid_part1 = true;
						};

						part2_red_max = core::cmp::max(part2_red_max, number);
					},
					_ => {
						panic!("Invalid color");
					}
				}
			});
		});

		if !invalid_part1 {
			result += game_number;
		}

		//// Part2
		result_part2 += part2_red_max * part2_green_max * part2_blue_max;

	});

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}