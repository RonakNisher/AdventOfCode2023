// #[warn(dead_code)]

use std::{collections::HashSet, cmp::{min, max}};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Brick {
	id: i32,
	x1: (i32, i32, i32), //x, y, z
	x2: (i32, i32, i32), //x, y, z
}

fn drop_bricks(bricks: &mut Vec<Brick>) -> u32 {

	let mut dropped_count = 0;

	for i in 1..bricks.len() {
		// println!();
		// println!("i is {}", i);
		let mut current_brick = bricks[i].clone();

		let mut z = 0;

		for j in 0..i {
			let brick_below = &bricks[j];

			// we need both x & y coords to intersect to be able to stack
			// starts before below ends & ends after below starts
			let does_x_intersect = current_brick.x1.0 <= brick_below.x2.0 && current_brick.x2.0 >= brick_below.x1.0;
			let does_y_intersect = current_brick.x1.1 <= brick_below.x2.1 && current_brick.x2.1 >= brick_below.x1.1;

			if does_x_intersect && does_y_intersect {
				z = max(z, brick_below.x2.2);
			}
		}

		// update z for brick
		let z_drop = current_brick.x1.2 - z - 1;
		if z_drop > 0 {
			current_brick.x1.2 -= z_drop;
			current_brick.x2.2 -= z_drop;

			dropped_count += 1;
		}

		// update current brick now that we know where to drop it
		bricks[i] = current_brick;
	};

	return dropped_count;
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;
	let mut id = 1;

	let mut bricks: Vec<Brick> = Vec::new();
	bricks.reserve(input.lines().count());

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		let (x1, x2) = line.split_once('~').unwrap();
		let x1: (i32,i32, i32) = x1.split(',').into_iter().map( |c| c.parse::<i32>().unwrap()).collect_tuple().unwrap();
		let x2: (i32, i32, i32) = x2.split(',').into_iter().map( |c| c.parse::<i32>().unwrap()).collect_tuple().unwrap();

		let brick: Brick = Brick {
			id: id,
			x1: x1,
			x2: x2,
		};

		bricks.push(brick);

		id += 1;
	});

	bricks.sort_by_key(|k| min(k.x1.2, k.x2.2));

	println!();

	drop_bricks(&mut bricks);

	for i in 0..bricks.len() {
		let mut temp_bricks = bricks.clone();
		temp_bricks.remove(i);

		let dropped_count = drop_bricks(&mut temp_bricks);

		if dropped_count == 0 {
			result += 1;
		}

		result_part2 += dropped_count;
	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}