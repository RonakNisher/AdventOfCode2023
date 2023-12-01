// #[warn(dead_code)]

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    input.lines().for_each(|line| {
		// println!("{}", line);

        let nums = line.chars().filter(|x| x.is_ascii_digit()).collect::<Vec<char>>();
        // println!("{:?}", nums);

        let calibration_part1 = format!("{}{}",&nums[0], &nums[nums.len() - 1]).parse::<u32>();//.to_i32().unwrap();
        
        result += calibration_part1.unwrap();

        // part 2
        // let one: Vec<_> = line.match_indices("one").collect();
        // let index_one = line.find("one");
        // let index_one_last = line.rfind("one");
        // let index_one_min = index_one.unwrap_or(usize::MAX);
        // let index_one_max = index_one_last.unwrap_or(usize::MIN);

        // let index_two = line.find("two");
        // let index_two_last = line.rfind("two");
        // let index_two_min = index_two.unwrap_or(usize::MAX);
        // let index_two_max = index_two_last.unwrap_or(usize::MIN);

        // let index_three = line.find("three");
        // let index_three_last = line.rfind("three");
        // let index_three_min = index_three.unwrap_or(usize::MAX);
        // let index_three_max = index_three_last.unwrap_or(usize::MIN);

        // let index_four = line.find("four");
        // let index_four_last = line.rfind("four");
        // let index_frour_min = index_four.unwrap_or(usize::MAX);
        // let index_four_max = index_four_last.unwrap_or(usize::MIN);

        // let index_five = line.find("five");
        // let index_five_last = line.rfind("five");
        // let index_five_min = index_five.unwrap_or(usize::MAX);
        // let index_five_max = index_five_last.unwrap_or(usize::MIN);

        // let index_six = line.find("six");
        // let index_six_last = line.rfind("six");
        // let index_six_min = index_six.unwrap_or(usize::MAX);
        // let index_six_max = index_six_last.unwrap_or(usize::MIN);

        // let index_seven = line.find("seven");
        // let index_seven_last = line.rfind("seven");
        // let index_seven_min = index_seven.unwrap_or(usize::MAX);
        // let index_seven_max = index_seven_last.unwrap_or(usize::MIN);

        // let index_eight = line.find("eight");
        // let index_eight_last = line.rfind("eight");
        // let index_eight_min = index_eight.unwrap_or(usize::MAX);
        // let index_eight_max = index_eight_last.unwrap_or(usize::MIN);

        // let index_nine = line.find("nine");
        // let index_nine_last = line.rfind("nine");
        // let index_nine_min = index_nine.unwrap_or(usize::MAX);
        // let index_nine_max = index_nine_last.unwrap_or(usize::MIN);

        // let mut min_indices: Vec<(usize, String)> = Vec::new();
        // min_indices.push((index_one_min, String::from("one")));
        // min_indices.push((index_two_min, String::from("two")));
        // min_indices.push((index_three_min, String::from("three")));
        // min_indices.push((index_frour_min, String::from("four")));
        // min_indices.push((index_five_min, String::from("five")));
        // min_indices.push((index_six_min, String::from("six")));
        // min_indices.push((index_seven_min, String::from("seven")));
        // min_indices.push((index_eight_min, String::from("eight")));
        // min_indices.push((index_nine_min, String::from("nine")));

        // let mut max_indices: Vec<(usize, String)> = Vec::new();
        // max_indices.push((index_one_max, String::from("one")));
        // max_indices.push((index_two_max, String::from("two")));
        // max_indices.push((index_three_max, String::from("three")));
        // max_indices.push((index_four_max, String::from("four")));
        // max_indices.push((index_five_max, String::from("five")));
        // max_indices.push((index_six_max, String::from("six")));
        // max_indices.push((index_seven_max, String::from("seven")));
        // max_indices.push((index_eight_max, String::from("eight")));
        // max_indices.push((index_nine_max, String::from("nine")));

        // // loop over min_indices and find the min
        // min_indices.sort_by(|a, b| a.0.cmp(&b.0));
        // max_indices.sort_by(|a, b| b.0.cmp(&a.0));

        // println!("{:?}", min_indices);
        // print!("{:?}", max_indices);

        // println!();
        // println!("line is {}", line);

        // let replacement_string = &min_indices[0].1;
        // let mut line_part2 = match replacement_string.as_str() {
        //     "one" => line.replace("one", "1"),
        //     "two" => line.replace("two", "2"),
        //     "three" => line.replace("three", "3"),
        //     "four" => line.replace("four", "4"),
        //     "five" => line.replace("five", "5"),
        //     "six" => line.replace("six", "6"),
        //     "seven" => line.replace("seven", "7"),
        //     "eight" => line.replacen("eight", "8", 1),
        //     "nine" => line.replace("nine", "9"),
        //     _ => panic!("Invalid replacement string"),
        // };

        // println!("{}", line_part2);

        // let replacement_string_max = &max_indices[0].1;
        // line_part2 = match replacement_string_max.as_str() {
        //     "one" => line_part2.replace("one", "1"),
        //     "two" => line_part2.replace("two", "2"),
        //     "three" => line_part2.replace("three", "3"),
        //     "four" => line_part2.replace("four", "4"),
        //     "five" => line_part2.replace("five", "5"),
        //     "six" => line_part2.replace("six", "6"),
        //     "seven" => line_part2.replace("seven", "7"),
        //     "eight" => line_part2.replacen("eight", "8", 1),
        //     "nine" => line_part2.replace("nine", "9"),
        //     _ => panic!("Invalid replacement string"),
        // };

        // println!("{}", line_part2);


        let mut line_part2 = line.replace("one", "one1one");
        line_part2 = line_part2.replace("two", "two2two");
        line_part2 = line_part2.replace("three", "three3three");
        line_part2 = line_part2.replace("four", "four4four");
        line_part2 = line_part2.replace("five", "five5five");
        line_part2 = line_part2.replace("six", "six6six");
        line_part2 = line_part2.replace("seven", "seven7seven");
        line_part2 = line_part2.replace("eight", "eight8eight");
        line_part2 = line_part2.replace("nine", "nine9nine");

        
        let nums = line_part2.chars().filter(|x| x.is_ascii_digit()).collect::<Vec<char>>();

        // let mut calibration = 0;
        let calibration = format!("{}{}",&nums[0], &nums[nums.len() - 1]).parse::<u32>().unwrap();
        
        let res = calibration;
        println!("{} -> {}", line_part2, res);
        result_part2 += res;
	});

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}