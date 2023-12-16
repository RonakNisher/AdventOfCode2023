// #[warn(dead_code)]

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

fn get_result(tiles: &Vec<Vec<char>>) -> usize {
		return tiles.iter().filter_map(|x| Some(x.iter().filter(|&x| *x == '#').count())).sum();
}

fn get_next_position(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
	match direction {
		Direction::Up => {
			return (x - 1, y);
		},
		Direction::Down => {
			return (x + 1, y);
		},
		Direction::Left => {
			return (x, y - 1);
		},
		Direction::Right => {
			return (x, y + 1);
		},
	}
}

fn move_light(grid: &Vec<Vec<char>>, tiles: &mut Vec<Vec<char>>, x: i32, y: i32, direction: &Direction, visited: &mut HashSet<(i32, i32, Direction)>) {

	if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid.len() as i32 {
		return;
	}

	if visited.contains(&(x, y, *direction)) {
		return;
	}

	tiles[x as usize][y as usize] = '#';

	visited.insert((x, y, direction.clone()));

	let tile = grid[x as usize][y as usize];
	match (tile, direction) {
		// passthrough
		('.', _) => {
			let (newx, newy) = get_next_position(x, y, direction);
			move_light(grid, tiles, newx, newy, direction, visited);
		},
		('|', Direction::Up) => {
			let (newx, newy) = get_next_position(x, y, direction);
			move_light(grid, tiles, newx, newy, direction, visited);
		},
		('|', Direction::Down) => {
			let (newx, newy) = get_next_position(x, y, direction);
			move_light(grid, tiles, newx, newy, direction, visited);
		},
		('-', Direction::Left) => {
			let (newx, newy) = get_next_position(x, y, direction);
			move_light(grid, tiles, newx, newy, direction, visited);
		},
		('-', Direction::Right) => {
			let (newx, newy) = get_next_position(x, y, direction);
			move_light(grid, tiles, newx, newy, direction, visited);
		},
		// Reflect on /
		('/', Direction::Right) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Up);
			move_light(grid, tiles, newx, newy, &Direction::Up, visited);
		},
		('/', Direction::Left) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Down);
			move_light(grid, tiles, newx, newy, &Direction::Down, visited);
		},
		('/', Direction::Up) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Right);
			move_light(grid, tiles, newx, newy, &Direction::Right, visited);
		},
		('/', Direction::Down) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Left);
			move_light(grid, tiles, newx, newy, &Direction::Left, visited);
		},
		// Reflect on \
		('\\', Direction::Right) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Down);
			move_light(grid, tiles, newx, newy, &Direction::Down, visited);
		},
		('\\', Direction::Left) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Up);
			move_light(grid, tiles, newx, newy, &Direction::Up, visited);
		},
		('\\', Direction::Up) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Left);
			move_light(grid, tiles, newx, newy, &Direction::Left, visited);
		},
		('\\', Direction::Down) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Right);
			move_light(grid, tiles, newx, newy, &Direction::Right, visited);
		},
		// Split at |
		('|', Direction::Left) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Up);
			move_light(grid, tiles, newx, newy, &Direction::Up, visited);

			let (newx, newy) = get_next_position(x, y, &Direction::Down);
			move_light(grid, tiles, newx, newy, &Direction::Down, visited);
		},
		('|', Direction::Right) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Up);
			move_light(grid, tiles, newx, newy, &Direction::Up, visited);

			let (newx, newy) = get_next_position(x, y, &Direction::Down);
			move_light(grid, tiles, newx, newy, &Direction::Down, visited);
		},
		// Split at |
		('-', Direction::Up) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Left);
			move_light(grid, tiles, newx, newy, &Direction::Left, visited);

			let (newx, newy) = get_next_position(x, y, &Direction::Right);
			move_light(grid, tiles, newx, newy, &Direction::Right, visited);
		},
		('-', Direction::Down) => {
			let (newx, newy) = get_next_position(x, y, &Direction::Left);
			move_light(grid, tiles, newx, newy, &Direction::Left, visited);

			let (newx, newy) = get_next_position(x, y, &Direction::Right);
			move_light(grid, tiles, newx, newy, &Direction::Right, visited);
		},
		_ => {
			panic!("Error: unknown tile {}", tile);
		}
	}

}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut grid: Vec<Vec<char>> = Vec::new();
	let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);
		grid.push(line.chars().collect());
	});

	let mut tiles = grid.clone();

	move_light(&grid, &mut tiles, 0, 0, &Direction::Right, &mut visited);
	result = get_result(&tiles);

	// part 2
	// going right from col 0 in each row
	for i in 0..grid.len() {
		visited.clear();
		tiles = grid.clone();

		move_light(&grid, &mut tiles, i as i32, 0, &Direction::Right, &mut visited);

		let res = get_result(&tiles);
		if res > result_part2 {
			result_part2 = res;
		}
	}

	// going left from last col in each row
	for i in 0..grid.len() {
		visited.clear();
		tiles = grid.clone();

		move_light(&grid, &mut tiles, i as i32, grid[0].len() as i32 - 1, &Direction::Left, &mut visited);

		let res = get_result(&tiles);
		if res > result_part2 {
			result_part2 = res;
		}
	}

	// going down from row 0 in each col
	for i in 0..grid[0].len() {
		visited.clear();
		tiles = grid.clone();

		move_light(&grid, &mut tiles, 0, i as i32, &Direction::Down, &mut visited);

		let res = get_result(&tiles);
		if res > result_part2 {
			result_part2 = res;
		}
	}

	// going up from last row in each col
	for i in 0..grid[0].len() {
		visited.clear();
		tiles = grid.clone();

		move_light(&grid, &mut tiles, grid.len() as i32 - 1, i as i32, &Direction::Up, &mut visited);

		let res = get_result(&tiles);
		if res > result_part2 {
			result_part2 = res;
		}
	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}