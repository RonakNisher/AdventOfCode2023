// #[warn(dead_code)]
use std::{collections::{HashMap, HashSet}, cmp::max};

fn hike(x: i32, y: i32, end_pos: (i32, i32), forest: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>) -> i32 {

	if x < 0 || y < 0 || x >= forest.len() as i32 || y >= forest[0].len() as i32 {
		return -1;
	}

	if visited.contains(&(x, y)) {
		return -1;
	}
	
	visited.insert((x, y));

	if x == end_pos.0 && y == end_pos.1 {
		return 1;
	}
	
	let tile = forest[x as usize][y as usize];

	if tile == '#' {
		return -1;
	}

	let mut max_hike_length = -1;

	// tile is one of the slopes
	match tile {
		'.' => {
			[(-1, 0), (1, 0), (0, -1), (0, 1)].iter().for_each(|(dx, dy)| {
				let hike_length = hike(x + dx, y + dy, end_pos, forest, &mut visited.clone());
				if hike_length != -1 {
					max_hike_length = max(max_hike_length, hike_length + 1);
				}
			});
		},
		'>' => {
			let hike_length = hike(x, y + 1, end_pos, forest, &mut visited.clone());
			if hike_length != -1 {
				max_hike_length = max(max_hike_length, hike_length + 1);
			}
		},
		'<' => {
			let hike_length = hike(x, y - 1, end_pos, forest, &mut visited.clone());
			if hike_length != -1 {
				max_hike_length = max(max_hike_length, hike_length + 1);
			}
		},
		'v' => {
			let hike_length = hike(x + 1, y, end_pos, forest, &mut visited.clone());
			if hike_length != -1 {
				max_hike_length = max(max_hike_length, hike_length + 1);
			}
		},
		'^' => {
			let hike_length: i32 = hike(x - 1, y, end_pos, forest, &mut visited.clone());
			if hike_length != -1 {
				max_hike_length = max(max_hike_length, hike_length + 1);
			}
		},
		_ => { panic!("Invalid tile {}", tile);}
	}

	return max_hike_length;
}

fn part2_dfs(nodes: &HashMap::<(i32, i32),Vec<(i32, i32, i32)>>, visited: &mut HashMap<(i32, i32), bool>, current_pos: (i32, i32), end_row: i32) -> i32 {

	if current_pos.0 == end_row {
		return 0;
	}

	let mut max_dist = -1;

	for &(r, c, dist) in nodes.get(&current_pos).unwrap() {
		if !visited.contains_key(&(r, c)) {

			visited.insert((r, c), true);
	
			let new_dist = part2_dfs(nodes, visited, (r, c), end_row);
	
			visited.remove(&(r, c));
	
			if new_dist != -1 {
				max_dist = max(max_dist, dist + new_dist);
			}
		}
	}

	return max_dist;
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut forest: Vec<Vec<char>> = Vec::new();

	input.lines().for_each(|line| {
		// println!();
		// println!("line is {}", line);
		forest.push(line.chars().collect::<Vec<char>>());
	});

	let start_pos = forest[0].iter().position(|&x| x == '.').unwrap();
	let end_pos = forest[forest.len() - 1].iter().position(|&x| x == '.').unwrap();

	println!("start_pos is {}", start_pos);
	println!("end_pos is {}", end_pos);

	result = hike(0, start_pos as i32, (forest.len() as i32 - 1, end_pos as i32), &forest, &mut HashSet::new()) - 1;

	let mut graph = HashMap::<(i32, i32),Vec<(i32, i32, i32)>>::new(); // row,col -> neighbour row, col, distance

	// make graph with key as the pos and value as the non # neighbours
	for r in 0..forest.len() {
		for c in 0..forest[0].len() {
			if forest[r][c] == '#' {
				continue;
			}

			[(1, 0), (0, 1), (-1, 0), (0, -1)].iter().map(|(dx, dy)| {
				let mut new_tile = '#';
				let newx = r as i32 + dx;
				let newy = c as i32 + dy;
			
				if newx < 0 || newy < 0 || newx >= forest.len() as i32 || newy >= forest[0].len() as i32 {
					// nothing
				}
				else {
					new_tile = forest[newx as usize][newy as usize];
				}
				(new_tile, newx, newy)
			}).filter(|&x| x.0 != '#')
			.for_each(|(_tile, x, y)| {
				graph.entry((r as i32, c as i32)).or_insert(Vec::new()).push((x, y, 1));
			});
		};
	}

	// corroders are nodes with just 2 neighbours (single path)
	let corridors = graph.iter()
		.filter(|(_,n)| n.len() == 2)
		.map(|(&node,_)| node)
		.collect::<Vec<_>>();

	// remove corridors and connect the neighbours
	for (r,c) in corridors {
		// println!("Corridor: r is {}, c is {}", r, c);
		let neighbors = graph.remove(&(r,c)).unwrap();
		let (r1,c1,d1) = neighbors[0];
		let (r2,c2,d2) = neighbors[1];

		let n1 = graph.get_mut(&(r1,c1)).unwrap();
		if let Some(i) = n1.iter().position(|&(rr,cc,_)| (rr,cc) == (r,c)) {
		  n1[i] = (r2,c2,d1+d2);
		}

		let n2 = graph.get_mut(&(r2,c2)).unwrap();
		if let Some(i) = n2.iter().position(|&(rr,cc,_)| (rr,cc) == (r,c)) {
		  n2[i] = (r1,c1,d1+d2);
		}
	  }

	let mut visited = HashMap::<(i32, i32), bool>::new();
	visited.insert((0, 1), true);

	result_part2 = part2_dfs(&graph, &mut visited, (0, 1), forest.len() as i32 - 1);

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}