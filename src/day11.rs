// #[warn(dead_code)]
use std::{collections::HashSet, cmp::{min, max}};

use itertools::Itertools;
use num::abs;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Node {
	id: usize,
	row: usize,
	col: usize,
}

pub fn solve(input: String) {
	let mut result: u64 = 0;
	let mut result_part2 = 0;

	let mut grid: Vec<Vec<char>> = Vec::new();
	let mut empty_cols: HashSet<usize> = HashSet::new();
	let mut empty_rows: HashSet<usize> = HashSet::new();

	let mut nodes: Vec<Node> = Vec::new();
	let mut next_node_index = 1;

	let expansion_factor: u64 = 1; // 2 - 1
	let expansion_factor_part2: u64 = 1000000 - 1;// -1 since we are always counting the original row/column once in the manhattan dist;

	for (i, line) in input.lines().enumerate() {
		// println!();
		// println!("line is {}", line);

		let chars = line.chars().collect_vec();

		for (j, c) in chars.iter().enumerate() {
			if *c == '#' {
				let node = Node {
					id: next_node_index,
					row: i,
					col: j,
				};

				nodes.push(node);

				next_node_index += 1;
			}
		}

		grid.push(chars);

		// expand rows
		if line.chars().all(|c| c == '.') {
			empty_rows.insert(i);
		}
	};

	// expand columns
	for i in 0..grid[0].len() {
		let mut all_zeros = true;

		for j in 0..grid.len() {
			if grid[j][i] != '.' {
				all_zeros = false;
				break;
			}
		}

		if all_zeros {
			empty_cols.insert(i);
		}
	}

	// println!("empty_rows is {:?}", empty_rows);
	// println!("empty_cols is {:?}", empty_cols);

	// grid.iter().for_each(|row| {
	// 	println!("{:?}", row.iter().join(""));
	// });

	for v in 1..=nodes.len() {
		for v2 in v+1..=nodes.len() {
			let n1 = nodes.get(v - 1).unwrap();
			let n2 = nodes.get(v2 - 1).unwrap();

			let dist = abs(n1.row as i64 - n2.row as i64) + abs(n1.col as i64 - n2.col as i64);

			let min_row = min(n1.row, n2.row);
			let max_row = max(n1.row, n2.row);
			let min_col = min(n1.col, n2.col);
			let max_col = max(n1.col, n2.col);

			let row_expansion_count = (min_row..=max_row).filter(|x| empty_rows.contains(x)).count();
			let col_expansion_count = (min_col..=max_col).filter(|x| empty_cols.contains(x)).count();

			result += dist as u64 + (row_expansion_count as u64 + col_expansion_count as u64) * expansion_factor;
			result_part2 += dist as u64 + (row_expansion_count as u64 + col_expansion_count as u64) * expansion_factor_part2;
		}
	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}