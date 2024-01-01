use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum ModuleType {
	FlipFlop,
	Conjunction,
	Broadcast,
}

#[derive(Debug)]
struct Module {
	name: String,
	module_type: ModuleType,
	current_pulse: i32, // 0 or 1 -> 0 = low, 1 = high
	inputs: Vec<String>,
	input_pulses: Vec<i32>,
	outputs: Vec<String>,
}

impl Module {
	fn process_pulse(&mut self, pulse: i32, origin: &str) -> Vec<(String, i32, String)> {

		match self.module_type {
			ModuleType::FlipFlop => {
				if pulse == 0 {
					self.current_pulse = (self.current_pulse + 1) % 2;
				} else {
					return Vec::new();
				}
			},
			ModuleType::Conjunction => {
				// update origin value
				let index = self.inputs.iter().position(|input| input == origin).unwrap();
				self.input_pulses[index] = pulse;

				// check if all inputs are high
				if self.input_pulses.iter().all(|input_pulse| *input_pulse == 1) {
					self.current_pulse = 0;
				} else {	
					self.current_pulse = 1;
				}
			},
			ModuleType::Broadcast => {
				self.current_pulse = pulse;
			},
		}

		return self.outputs.iter().map(|output| (output.clone(), self.current_pulse, self.name.clone())).collect();
	}
}


fn create_module(line: &str, modules: &mut HashMap<String, Module>) {

	let (left, right) = line.split_once(" -> ").unwrap();

	let (module_type, name) = match left.chars().next().unwrap() {
		'%' => (ModuleType::FlipFlop, left[1..].to_string()),
		'&' => (ModuleType::Conjunction, left[1..].to_string()),
		'b' => (ModuleType::Broadcast, left.to_string()),
		_ => panic!("Unknown module type"),
	};

	{
		let current_module = modules.entry(name.clone()).or_insert(Module {
			name: name.clone(),
			module_type: module_type.clone(),
			current_pulse: 0,
			inputs: Vec::new(),
			input_pulses: Vec::new(),
			outputs: Vec::new(),
		});
	
		current_module.module_type = module_type.clone(); // make sure to update the correct type
	
		right.split(",").for_each(|output| {
			// println!("Output: {}", output.trim());
			current_module.outputs.push(output.trim().to_string());
		});
	}


	right.split(",").for_each(|output| {
			let output_module = modules.entry(output.trim().to_string()).or_insert(Module {
				name: output.trim().to_string(),
				module_type: ModuleType::FlipFlop,
				current_pulse: 0,
				inputs: Vec::new(),
				input_pulses: Vec::new(),
				outputs: Vec::new(),
			});
		
			output_module.inputs.push(name.clone());
			output_module.input_pulses.push(0);
	});
}

pub fn solve(input: String) {
	let mut result = 0;
	let mut result_part2 = 0;

	let mut modules: HashMap<String, Module> = HashMap::new();

	input.lines().for_each(|line| {			
		create_module(line, &mut modules);
	});

	// println!("rx: {:?}", modules.get("rx").unwrap());
	// println!("xn: {:?}", modules.get("xn").unwrap());

	// println!("*******************");

	// ["hn", "mp", "xf", "fz"].iter().for_each(|name| {
	// 	println!("{}: {:?}", name, modules.get(*name).unwrap());
	// });

	// println!("*******************");

	let mut pulses: VecDeque<(String, i32, String)> = VecDeque::new(); // (name, pulse, origin)
	let mut num_of_button_pushes = 1000; //1_000_000;
	let max = num_of_button_pushes;
	let mut low_pulses = 0;
	let mut high_pulses = 0;
	let mut rx_pulses = 0;

	while num_of_button_pushes > 0 {

		// println!("******************* buttom push: {}", num_of_button_pushes);

		pulses.push_back((String::from("broadcaster"), 0, String::from("button")));
		low_pulses += 1;
	
		while !pulses.is_empty() {
			let (name, pulse_value, origin) = pulses.pop_front().unwrap();
	
			let mut new_pulses: Vec<(String, i32, String)> = Vec::new();
			modules.entry(name.to_string()).and_modify(| entry | {
				new_pulses = entry.process_pulse(pulse_value, &origin).to_owned();
			});

			new_pulses.iter().for_each(|(name, pulse_value, _origin)| {
				if *pulse_value == 0 {
					low_pulses += 1;
				} else {
					high_pulses += 1;
				}

				// if *name == "rx" && *pulse_value == 1 {
				// 	rx_pulses += 1;
				// }
			});


			pulses.extend(new_pulses);	
		}

		// if rx_pulses == 1 {
		// 	println!("--------------> rx is 1");
		// 	break;
		// }

		// let hx_pulse = modules.get("hn").unwrap().current_pulse;
		// if hx_pulse == 1 {
		// 	println!("--------------> hn is 1 at {}", max - num_of_button_pushes);
		// 	// break;
		// }

		num_of_button_pushes -= 1;
	}

	result = low_pulses * high_pulses;

	println!("*******************");
	println!("Solved Day 1 Part 1: {}", result);
	println!("Solved Day 1 Part 2: {}", result_part2);
	println!("*******************");
}