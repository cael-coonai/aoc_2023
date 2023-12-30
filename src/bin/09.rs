fn parse_input(input: String) -> Vec<Vec<i32>> {
    input.lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|nums| {
                    nums.parse::<i32>().unwrap()
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn differences(set: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::<i32>::with_capacity(set.len()-1);
    for idx in 0..set.len()-1 {
        result.push(set[idx+1] - set[idx]);
    }
    result

}

fn part_1(sets: &Vec<Vec<i32>>) -> i32 {
    fn generate_next_val(set: &Vec<i32>) -> i32 {
        if set.into_iter().all(|x| x == &0) {
            return 0;
        }
        return set.last().unwrap() + generate_next_val(&differences(set));
    }

    sets.into_iter()
        .map(|s| generate_next_val(&s))
        .sum()
}

fn part_2(sets: &Vec<Vec<i32>>) -> i32 {
    fn generate_prev_val(set: &Vec<i32>) -> i32 {
        if set.into_iter().all(|x| x == &0) {
            return 0;
        }
        return set[0] - generate_prev_val(&differences(set));
    }

    sets.into_iter()
        .map(|s| generate_prev_val(&s))
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let number_sets = parse_input(input);

    let solution_1 = part_1(&number_sets);
    println!("{}", solution_1);

    let solution_2 = part_2(&number_sets);
    println!("{}", solution_2);
}