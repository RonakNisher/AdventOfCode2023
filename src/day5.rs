// #[warn(dead_code)]

use std::{collections::{HashMap, HashSet}};

use itertools::Itertools;

pub fn solve(input: String) {
	let mut result = u64::MAX;
	let mut result_part2 = u64::MAX;

	let mut map_seed_to_soil: HashMap<(u64, u64), u64> = HashMap::new(); // (source, range) -> destination
	let mut map_soil_to_fertilizer: HashMap<(u64, u64), u64> = HashMap::new();
	let mut map_fertilizer_to_water: HashMap<(u64, u64), u64> = HashMap::new();
	let mut map_water_to_light: HashMap<(u64, u64), u64> = HashMap::new();
	let mut map_light_to_temperature: HashMap<(u64, u64), u64> = HashMap::new();
	let mut map_temperature_to_humidity: HashMap<(u64, u64), u64> = HashMap::new();
	let mut map_humidity_to_location: HashMap<(u64, u64), u64> = HashMap::new();

	let lines = input.lines().collect::<Vec<_>>();
	let mut index = 0;

	// get soil
	let seeds: Vec<u64> = lines[0].split_whitespace().skip(1).collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
	index += 1;
	
	while lines[index] == "" {
		index += 1;
	}

	index+=1; // skip header

	// seed to soil map
	while lines[index] != "" {
		let seed_soil_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let seed = seed_soil_range[1];
		let soil = seed_soil_range[0];
		let range = seed_soil_range[2];

		map_seed_to_soil.insert((seed, range), soil);

		index += 1;
	}

	index += 2; // skip header & blank space

	// soil to fertilizer map
	while lines[index] != "" {
		let soil_fertilizer_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let soil = soil_fertilizer_range[1];
		let fertilizer = soil_fertilizer_range[0];
		let range = soil_fertilizer_range[2];

		map_soil_to_fertilizer.insert((soil, range), fertilizer);

		index += 1;
	}

	index += 2; // skip header & blank space

	// fertilizer to water map
	while lines[index] != "" {
		let fertilizer_water_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let fertilizer = fertilizer_water_range[1];
		let water = fertilizer_water_range[0];
		let range = fertilizer_water_range[2];

		map_fertilizer_to_water.insert((fertilizer, range), water);

		index += 1;
	}

	index += 2; // skip header & blank space

	// water to light map
	while lines[index] != "" {
		let water_light_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let water = water_light_range[1];
		let light = water_light_range[0];
		let range = water_light_range[2];

		map_water_to_light.insert((water, range), light);

		index += 1;
	}

	index += 2; // skip header & blank space

	// light to temperature map
	while lines[index] != "" {
		let light_temperature_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let light = light_temperature_range[1];
		let temperature = light_temperature_range[0];
		let range = light_temperature_range[2];

		map_light_to_temperature.insert((light, range), temperature);

		index += 1;
	}

	index += 2; // skip header & blank space

	// temperature to humidity map
	while lines[index] != "" {
		let temperature_humidity_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let temperature = temperature_humidity_range[1];
		let humidity = temperature_humidity_range[0];
		let range = temperature_humidity_range[2];

		map_temperature_to_humidity.insert((temperature, range), humidity);

		index += 1;
	}

	index += 2; // skip header & blank space

	// humidity to location map
	while index < lines.len() {
		let humidity_location_range = lines[index].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		let humidity = humidity_location_range[1];
		let location = humidity_location_range[0];
		let range = humidity_location_range[2];

		map_humidity_to_location.insert((humidity, range), location);

		index += 1;
	}

	// part 1

	let min_res= seeds.iter().map( |x| {

		let mut res = *x;

		for (key, value) in map_seed_to_soil.iter() {
			if x >= &key.0 && x <= &(key.0 + key.1 - 1) {
				res = value + x.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_soil_to_fertilizer.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_fertilizer_to_water.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_water_to_light.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_light_to_temperature.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_temperature_to_humidity.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}

		for (key, value) in map_humidity_to_location.iter() {
			if &res >= &key.0 && &res <= &(key.0 + key.1 - 1) {
				res = value + res.abs_diff(key.0);
				break;
			}
		}
		return res;
	}).collect_vec();

	result = *min_res.iter().min().unwrap();

	println!("Part 1: {}", result);
	
	///////////////
	// part 2
	///////////////
	
	let mut found_part2 = false;
	let mut location: u64 = 0;//6335758; //52510009;

	while !found_part2 {

		let mut res = location;
		// println!();
		// println!("location is {}", location);

		for (key, value) in map_humidity_to_location.iter() {

			if location >= *value && location <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(location);
				break;
			}
		}

		for (key, value) in map_temperature_to_humidity.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
				break;
			}
		}

		for (key, value) in map_light_to_temperature.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
				break;
			}
		}

		for (key, value) in map_water_to_light.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
				break;
			}
		}

		for (key, value) in map_fertilizer_to_water.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
				break;
			}
		}

		for (key, value) in map_soil_to_fertilizer.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
				break;
			}
		}

		for (key, value) in map_seed_to_soil.iter() {

			if res >= *value && res <= (value + key.1 - 1) {
				res = key.0 + value.abs_diff(res);
			}
		}

		if !found_part2 {
			let mut i: usize = 0;
			while i < seeds.len() {
				let start: u64 = seeds[i];
				let range: u64 = seeds[i+1];
	
				if res >= start && res <= (start + range - 1) {
					println!("found {}", res);
					found_part2 = true;
					result_part2 = location;
					break;
				}
	
				i+=2;
			}
		}

		if !found_part2 {
			location += 1;
		}

	}

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}