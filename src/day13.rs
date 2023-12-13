// #[warn(dead_code)]
fn get_reflection_result(grid: &Vec<Vec<char>>, number_of_smudges_allowed: i32) -> u64 {
	// check horizontal reflection
	for r in 1..grid.len() {
		// check rows for reflection
		let mut found = true;
		let mut allowed_differences = number_of_smudges_allowed;
		for j in 0..r{

			if r + j >= grid.len() {
				break;
			}

			let top_row = &grid[r-j-1];
			let bottom_row = &grid[r+j];

			for c in 0..top_row.len() {
				if top_row[c] != bottom_row[c] {

					if allowed_differences > 0 {
						allowed_differences -= 1;
						continue;
					}

					found = false;
					break;
				}
			}

			if !found {
				break;
			}
		}

		if found && allowed_differences == 0 {
			return r as u64 * 100;
		}
	}

	// check vertical reflection
	for c in 1..grid[0].len() {
		// check cols for reflection
		let mut found = true;
		let mut allowed_differences = number_of_smudges_allowed;
		for j in 0..c{

			if c + j >= grid[0].len() {
				break;
			}

			for r in 0..grid.len() {
				if grid[r][c-j-1] != grid[r][c+j] {
					if allowed_differences > 0 {
						allowed_differences -= 1;
						continue;
					}

					found = false;
					break;
				}
			}

			if !found {
				break;
			}
		}

		if found && allowed_differences == 0{
			return c as u64;
		}
	}

	return 0;
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;
	let mut grid: Vec<Vec<char>> = Vec::new();

	input.lines().for_each(|line| {
		if line.is_empty() {
			result += get_reflection_result(&grid, 0);
			result_part2 += get_reflection_result(&grid, 1);
			
			grid.clear();
		}
		else {
			grid.push(line.chars().collect());
		}
	});

	result += get_reflection_result(&grid, 0);
	result_part2 += get_reflection_result(&grid, 1);

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}