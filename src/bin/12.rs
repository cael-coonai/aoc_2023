use std::collections::{HashMap, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "·"),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Record {
    springs: VecDeque<Spring>,
    damaged: VecDeque<u32>
}

fn parse_input(input: String) -> Vec<Record> {
    input.lines()
        .map(|l| {
            let mut line = l.split_ascii_whitespace();
            let springs = line.next().unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    c   => unreachable!("Invalid Character: {}", c)
                })
                .collect();
            let damaged = line.next().unwrap()
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect();
            Record {springs, damaged}
        })
        .collect()
}

fn quintuple_records(mut record_vec: Vec<Record>) ->  Vec<Record> {
    let mut temp_springs: VecDeque<Spring> = VecDeque::new();
    let mut temp_damaged: VecDeque<u32> = VecDeque::new();
    for idx in 0..(record_vec.len()) {
        for _ in 0..4 {
            temp_springs.push_back(Spring::Unknown);
            temp_springs.append(&mut record_vec[idx].springs.clone());
            temp_damaged.append(&mut record_vec[idx].damaged.clone());
        }
        record_vec[idx].springs.append(&mut temp_springs);
        record_vec[idx].damaged.append(&mut temp_damaged);
    }
    record_vec
}

fn count_permutations(record: Record, memo: &mut HashMap::<Record,u64>) -> u64 {
    if let Some(&result) = memo.get(&record) {
        return result;
    }

    if record.damaged.len() == 0 && record.springs.len() == 0 {
        memo.insert(record, 1);
        return 1;
    }

    if record.damaged.len() == 0 &&
        record.springs.iter().any(|s| s == &Spring::Damaged) {
        memo.insert(record, 0);
        return 0;
    }

    if record.springs.len() < (
        (record.damaged.iter().sum::<u32>() as usize +
        record.damaged.len()).saturating_sub(1)
    ) {
        memo.insert(record, 0);
        return 0;
    }

    let step_damaged_count = {
        let mut result = 0;
        if record.damaged.len() > 0 {
            let mut new_record = record.clone();
            let damaged_count= new_record.damaged.pop_front().unwrap() as usize;
            let mut failed_to_step_damaged = false;

            for idx in 0..damaged_count {
                if record.springs[idx] == Spring::Operational {
                    failed_to_step_damaged = true;
                    break;
                }
            }

            for _ in 0..damaged_count {new_record.springs.pop_front();}
            if new_record.damaged.len() > 0 {
                if let Some(Spring::Damaged) = new_record.springs.pop_front() {
                        failed_to_step_damaged = true;
                    }
            }

            if !failed_to_step_damaged {
                result = count_permutations(new_record.clone(), memo);
                memo.insert(new_record, result);
            }
        }
        result
    };

    let skip_spring_count = {
        let mut result = 0;
        if record.springs[0] != Spring::Damaged {
            let mut new_record = record.clone();
            new_record.springs.pop_front();

            result = count_permutations(new_record.clone(), memo);
            memo.insert(new_record, result);
        }
        result
    };

    let result = step_damaged_count + skip_spring_count;
    memo.insert(record, result);
    return result;
}

fn count_all_permutations(
    records: &Vec<Record>,
    memo: &mut HashMap::<Record,u64>
) -> u64 {
    records.iter()
        .map(|record| count_permutations(record.clone(), memo))
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let record_vec = parse_input(input);

    let mut memo = HashMap::<Record,u64>::new();

    let solution_1 = count_all_permutations(&record_vec, &mut memo);
    println!("{}", solution_1);

    let record_vec = quintuple_records(record_vec);

    let solution_2 = count_all_permutations(&record_vec, &mut memo);
    println!("{}", solution_2);
}
