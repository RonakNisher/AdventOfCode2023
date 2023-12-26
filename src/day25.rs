// #[warn(dead_code)]

use std::collections::{HashMap, HashSet};
use rand::Rng;

use itertools::Itertools;

#[derive(Debug)]
struct Component {
	neighbors: HashSet<String>,
}

fn get_shortest_paths(start: &String, end: &String, components: &HashMap<String, Component>) -> Vec<(String, String)> {
	let mut res: Vec<(String, String)> = Vec::new();

	let mut parents: HashMap<String, String> = HashMap::new();
	let mut distances: HashMap<String, i32> = HashMap::new();
	let mut queue: Vec<String> = Vec::new();

	distances.insert(start.to_string(), 0);
	queue.push(start.to_string());

	while queue.len() > 0 {
		let node_str = queue.remove(0);
		let node = components.get(&node_str).unwrap();

		node.neighbors.iter().for_each(|neighbor| {
			let dist = 1 + distances.get(&node_str).unwrap();
			if &dist < distances.get(neighbor).unwrap_or(&i32::MAX) {
				distances.insert(neighbor.to_string(), dist);
				parents.insert(neighbor.to_string(), node_str.to_string());
				queue.push(neighbor.to_string());
			}
		});
	}

	let mut current_node = end.to_string();
	while &current_node != start {
		let parent = parents.get(&current_node).unwrap();
		res.push((parent.to_string(), current_node.to_string()));
		current_node = parent.to_string();
	}

	return res;
}

fn get_most_common_edge(components: &HashMap<String, Component>) -> Vec<String> {

	let mut res: Vec<String> = Vec::new();
	let mut edges_count: HashMap<Vec<String>, i32> = HashMap::new();

	let nodes = components.keys().collect_vec();
	let mut rng = rand::thread_rng();

	(0..500).for_each(|_| {
		let number1 = rng.gen_range(0..nodes.len());
		let number2 = rng.gen_range(0..nodes.len());

		let edges = get_shortest_paths(nodes.get(number1).unwrap(), nodes.get(number2).unwrap(), &components);

		edges.iter().for_each(|(node1, node2)| {
			let mut edge = Vec::from([node1.to_string(), node2.to_string()]);
			edge.sort();
			edges_count.entry(edge).and_modify(|e| *e += 1).or_insert(1);
		});
	});


	let (max_key, _max_value) = edges_count.iter().max_by_key(|&(_k, v)| v).unwrap();

	res.push(max_key.get(0).unwrap().to_string());
	res.push(max_key.get(1).unwrap().to_string());

	return res
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut components: HashMap<String, Component> = HashMap::new();

	input.lines().for_each(|line| {
		let (name, neighbors) = line.split_once(": ").unwrap();
		let neighbors: Vec<String> = neighbors.split_whitespace().map(|s| s.to_string()).collect();

		for neighbour in &neighbors {
			// add the reverse edge to the hashmap if it doesn't exist
			if components.contains_key(neighbour) {
				let component = components.get_mut(neighbour).unwrap();
				component.neighbors.insert(name.to_string());
			} else {
				let component = Component {
					neighbors: HashSet::from([name.to_string()]),
				};
				components.insert(neighbour.to_string(), component);
			}
		}

		if components.contains_key(name) {
			let component = components.get_mut(name).unwrap();

			neighbors.iter().for_each(|neighbor| {
				component.neighbors.insert(neighbor.to_string());
			});

		} else {
			let mut neighbors_set: HashSet<String> = HashSet::new();
			neighbors.iter().for_each(|neighbor| {
				neighbors_set.insert(neighbor.to_string());
			});

			let component = Component {
				neighbors: neighbors_set,
			};
			components.insert(name.to_string(), component);
		}
	});

	let mut start_node = String::from("");
	// find the most common edge and remove it, do this 3 times for better accuracy (compared to taking the 3 most common edges)
	(0..3).for_each(|j| {
		let edges = get_most_common_edge(&components);
		println!("removing edge: ({} - {})", edges[0], edges[1]);
	
		// remoe the edge from the components
		components.get_mut(edges.get(0).unwrap()).and_then(|c| Some({ 
			c.neighbors.remove(edges.get(1).unwrap()); 
		}));
	
		components.get_mut(edges.get(1).unwrap()).and_then(|c| Some({ 
			c.neighbors.remove(edges.get(0).unwrap()); 
		}));

		 // random seed node for finding the number of nodes in one of the connected graphs
		if j == 0 {
			start_node = edges.get(0).unwrap().to_string();
		}
	});

	// now find the number of nodes in one of the connected graphs
	let mut set1: HashSet<String> = HashSet::new();
	set1.insert(start_node.to_string());

	let mut queue: Vec<String> = Vec::new();
	queue.push(start_node.to_string());

	while queue.len() > 0 {
		let node_str = queue.remove(0);
		let node = components.get(&node_str).unwrap();

		node.neighbors.iter().for_each(|neighbor| {
			if !set1.contains(neighbor) {
				set1.insert(neighbor.to_string());
				queue.push(neighbor.to_string());
			}
		});
	}

	result = set1.len() * (components.len() - set1.len());

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("*******************");
}