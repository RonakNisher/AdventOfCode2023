// #[warn(dead_code)]

use std::{collections::{HashMap, HashSet}, cmp::{min, max}};

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

		let mut found_one_brick_below = false;
		let mut supporting_bricks_below: Vec<i32> = Vec::new();
		let mut z = 0;

		for j in 0..i {
			// println!("j is {}", j);
			let brick_below = &bricks[j];
			// println!("{:?}", brick_below);

			// if found_one_brick_below && z != brick_below.x1.2 {
			// 	break;
			// }

			// we need both x & y coords to intersect to be able to stack
			// starts before nelow ends & ends after below starts
			let does_x_intersect = current_brick.x1.0 <= brick_below.x2.0 && current_brick.x2.0 >= brick_below.x1.0; // top start <= below end & top end >= below start
			let does_y_intersect = current_brick.x1.1 <= brick_below.x2.1 && current_brick.x2.1 >= brick_below.x1.1;

			if does_x_intersect && does_y_intersect {
				// println!("brick {} can be stacked on top of brick {}", current_brick.id, brick_below.id);
				found_one_brick_below = true;
				supporting_bricks_below.push(brick_below.id);
				z = max(z, brick_below.x2.2);

				// if brick_below.x1.2 != brick_below.x2.2 {
				// 	println!("*******************");
				// 	println!("brick {} is not on same z axis, diff is {}", brick_below.id, brick_below.x1.2 - brick_below.x2.2);
				// 	println!("*******************");
				// }

				// drop the brick, if needed (update z value)
				// if current_brick.x1.2 != z + 1 {
				// 	println!("dropping brick {} to z = {}", current_brick.id, z + 1);
				// 	current_brick.x1.2 = z + 1;
				// 	current_brick.x2.2 = z + 1;

				// 	dropped_count += 1;
				// }
				// break;
			}
			else {
				// println!("brick {} cannot be stacked on top of brick {}", current_brick.id, brick_below.id);			
			}
		}

		// update z for brick
		let z_drop = current_brick.x1.2 - z - 1;
		if z_drop > 0 {
			println!("dropping brick {} by z_drop = {}", current_brick.id, z_drop);
			current_brick.x1.2 -= z_drop;
			current_brick.x2.2 -= z_drop;

			dropped_count += 1;
		}

		// if supporting_bricks_below.len() > 1 {
		// 	supporting_bricks_below.iter().for_each(|x| { bricks_disintegratable.insert(*x); } ); // indicates that any of the below bricks can be disintegrated
		// 	println!("brick {} is supported by bricks {:?}", current_brick.id, supporting_bricks_below);
		// }

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

	let mut bricks_disintegratable: HashSet<i32> = HashSet::new();

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);

		let (x1, x2) = line.split_once('~').unwrap();
		let x1: (i32,i32, i32) = x1.split(',').into_iter().map( |c| c.parse::<i32>().unwrap()).collect_tuple().unwrap();
		let x2: (i32, i32, i32) = x2.split(',').into_iter().map( |c| c.parse::<i32>().unwrap()).collect_tuple().unwrap();

		// let x2 = x2.split(',').into_iter().map( |c| c.parse::<i32>().unwrap()).collect_tuple().unwrap();

		let brick: Brick = Brick {
			id: id,
			x1: x1,
			x2: x2,
		};

		bricks.push(brick);

		id += 1;
	});

	// println!("Before sorting");

	// bricks.iter().for_each( |brick| {
	// 	println!("brick id: {}", brick.id);
	// 	println!("brick x1: {:?}", brick.x1);
	// 	println!("brick x2: {:?}", brick.x2);
	// });

	bricks.sort_by_key(|k| min(k.x1.2, k.x2.2));

	println!();

	drop_bricks(&mut bricks);

	bricks.iter().for_each( |brick| {
		println!("brick id: {}", brick.id);
		println!("brick x1: {:?}", brick.x1);
		println!("brick x2: {:?}", brick.x2);
	});

	println!("*******************");
	println!("Dropping bricks");
	println!("*******************");

	for i in 0..bricks.len() {
		println!();
		println!("i is {}", i);

		let mut temp_bricks = bricks.clone();
		temp_bricks.remove(i);

		let dropped_count = drop_bricks(&mut temp_bricks);

		if dropped_count == 0 {
			println!("*******************");
			println!("for brick {}, dropped_count is 0", bricks[i].id);
			println!("*******************");
			result += 1;
		}

		result_part2 += dropped_count;
	}

	// ***************
	// my part 1
	// ***************

	// for i in 1..bricks.len() {
	// 	// println!();
	// 	// println!("i is {}", i);
	// 	let mut current_brick = bricks[i].clone();

	// 	let mut found_one_brick_below = false;
	// 	let mut supporting_bricks_below: Vec<i32> = Vec::new();
	// 	let mut z = 0;

	// 	for j in (0..i).rev() {
	// 		// println!("j is {}", j);
	// 		let brick_below = &bricks[j];
	// 		// println!("{:?}", brick_below);

	// 		if found_one_brick_below && z != brick_below.x1.2 {
	// 			break;
	// 		}

	// 		// we need both x & y coords to intersect to be able to stack
	// 		// starts before nelow ends & ends after below starts
	// 		let does_x_intersect = current_brick.x1.0 <= brick_below.x2.0 && current_brick.x2.0 >= brick_below.x1.0; // top start <= below end & top end >= below start
	// 		let does_y_intersect = current_brick.x1.1 <= brick_below.x2.1 && current_brick.x2.1 >= brick_below.x1.1;

	// 		if does_x_intersect && does_y_intersect {
	// 			// println!("brick {} can be stacked on top of brick {}", current_brick.id, brick_below.id);
	// 			found_one_brick_below = true;
	// 			supporting_bricks_below.push(brick_below.id);
	// 			z = max(brick_below.x1.2, brick_below.x2.2);

	// 			if brick_below.x1.2 != brick_below.x2.2 {
	// 				println!("*******************");
	// 				println!("brick {} is not on same z axis, diff is {}", brick_below.id, brick_below.x1.2 - brick_below.x2.2);
	// 				println!("*******************");
	// 			}

	// 			// drop the brick, if needed (update z value)
	// 			if current_brick.x1.2 != z + 1 {
	// 				// println!("dropping brick {} to z = {}", current_brick.id, z + 1);
	// 				current_brick.x1.2 = z + 1;
	// 				current_brick.x2.2 = z + 1;
	// 			}
	// 			// break;
	// 		}
	// 		else {
	// 			// println!("brick {} cannot be stacked on top of brick {}", current_brick.id, brick_below.id);			
	// 		}
	// 	}

	// 	if supporting_bricks_below.len() > 1 {
	// 		supporting_bricks_below.iter().for_each(|x| { bricks_disintegratable.insert(*x); } ); // indicates that any of the below bricks can be disintegrated
	// 		println!("brick {} is supported by bricks {:?}", current_brick.id, supporting_bricks_below);
	// 	}

	// 	// update current brick now that we know where to drop it
	// 	bricks[i] = current_brick;

	// 	// let brick_below = &bricks[i-1];

	// 	// we need both x & y coords to intersect to be able to stack
	// 	// starts before nelow ends & ends after below starts
	// 	// let does_x_intersect = current_brick.x1.0 <= brick_below.x2.0 && current_brick.x2.0 >= brick_below.x1.0; // top start <= below end & top end >= below start
	// 	// let does_y_intersect = current_brick.x1.1 <= brick_below.x2.1 && current_brick.x2.1 >= brick_below.x1.1;

	// 	// if does_x_intersect && does_y_intersect {
	// 	// 	println!("brick {} can be stacked on top of brick {}", current_brick.id, brick_below.id);
	// 	// 	// result += 1;
	// 	// }
	// };

	// println!();
	// bricks.iter().for_each( |brick| {
	// 	println!("brick id: {}", brick.id);
	// 	println!("brick x1: {:?}", brick.x1);
	// 	println!("brick x2: {:?}", brick.x2);
	// });

	// println!("After sorting");

	// println!("bricks_disintegratable: {:?}", bricks_disintegratable);

	// result = bricks_disintegratable.len() as i32;

	// result += 1; // add the topmost brick

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}