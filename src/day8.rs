// #[warn(dead_code)]

use std::collections::HashMap;
use num::integer::lcm;

use itertools::Itertools;

#[derive(Debug)]
struct Node {
	left: String,
	right: String,
}

fn get_steps(start: &str, ends_with: &str, map_nodes: &HashMap<String, Node>, steps: &Vec<char>) -> u64 {

	let mut result: u64 = 0;
	let mut current_index = 0;
	let mut current_node = start;

	while !current_node.ends_with(ends_with) {
		let node = map_nodes.get(current_node).unwrap();
		// println!("node is {:?}", node);

		match steps[current_index] {
			'L' => {
				current_node = &node.left;
			},
			'R' => {
				current_node = &node.right;
			},
			_ => {
				panic!("Invalid step")
			}
		}

		result += 1;
		current_index+=1;
		current_index = current_index % steps.len();
	
	}

	return result;

}

pub fn solve(input: String) {
	let result;
	let mut result_part2: u64 = 0;

	let mut map_nodes: HashMap<String, Node> = HashMap::new();
	let input_vec = input.lines().collect::<Vec<_>>();
	let steps = input_vec[0].chars().collect_vec();

	let re = regex::Regex::new(r"(\w*) = \((\w*), (\w*)\)").unwrap();

	for i in (2..input_vec.len()) {
		let line = input_vec[i];
		let (name, left, right) = match re.captures(line) {
			Some(captures) => (
				captures.get(1).unwrap().as_str(),
				captures.get(2).unwrap().as_str(),
				captures.get(3).unwrap().as_str(),
			),
			None => continue,
		};

		let node = Node {
			left: left.to_string(),
			right: right.to_string(),
		};

		map_nodes.insert(String::from(name), node);
	}

	// part 1
	result = get_steps("AAA", "ZZZ", &map_nodes, &steps);

	// part 2
	result_part2 = 1;

	for node in map_nodes.keys().filter(|&x| x.ends_with("A")) {
		result_part2 = lcm(result_part2, get_steps(node, "Z", &map_nodes, &steps));
	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}