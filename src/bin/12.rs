use itertools::{Itertools, iproduct, repeat_n};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, PartialEq)]
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

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    damaged: Vec<u32>
}

impl Record {
    fn count_unknown(&self) -> u32 {
        self.springs.iter()
            .filter(|s| s == &&Spring::Unknown)
            .count() as u32
    }

    fn is_valid(&self, original_record: &Record) -> bool {
        for idx in 0..self.springs.len() {
            if original_record.springs[idx] != Spring::Unknown &&
                original_record.springs[idx] != self.springs[idx] {
                    return false;
                }
        }

        assert!(self.springs
            .split(|s| s == &Spring::Operational)
            .map(|arr| arr.len() as u32)
            .filter(|len| len > &0)
            .zip_longest(self.damaged.iter())
            .all(|pair| match pair {
                itertools::EitherOrBoth::Both(a, b) => a == *b,
                _ => false
            }));

        true
    }

    fn generate_unknown_guess(&self, spacing_table: &SpacingLookupTable, seed: u64) -> Record {
        let mut result = self.clone();
        let mut new_springs = Vec::<Spring>::with_capacity(self.springs.len());
        let mut state = seed;
        let mask = 1;
    
        // for idx in 0..result.springs.len() {
        //     if self.springs[idx] == Spring::Unknown {
        //         result.springs[idx] = match state & mask {
        //             1 => Spring::Damaged,
        //             0 => Spring::Operational,
        //             _ => unreachable!()
        //         };
        //         state = state >> 1;
        //     }
        // }

        for idx in 0..self.damaged.len() {
            for _ in 0..self.damaged[idx] {new_springs.push(Spring::Damaged);}
            if idx<self.damaged.len()-1 {new_springs.push(Spring::Operational);}
        }

        let remainder = self.springs.len() - new_springs.len();
        for _ in 0..remainder {
            new_springs.push(Spring::Operational);
        }
        result.springs = new_springs;
        result
    }
}


struct SpacingLookupTable {
    values: Vec<Vec<u8>>,
    bounds: Vec<usize>
}

impl SpacingLookupTable  {
    fn create_table(max_spaces: u8) -> SpacingLookupTable {
        let mut values = (0u64..)
            // .filter(|n| bin_has_no_adjacent_zeros(*n))c0000000000000
            .take_while(|n| sum_bits(*n) < max_spaces)
            .map(|n| {
                // return vec![seed as u8];
                let mut spacing = vec![0];
                let mut state = n;
                let mask = 1;
                while state > 0 {
                    if state & mask == 1 {
                        let idx = spacing.len()-1;
                        spacing[idx] = spacing[idx] +1;
                    } else {
                        spacing.push(0);
                    }
                    state = state >> 1;
                }
                spacing
            }).collect::<Vec<Vec<u8>>>();

        values.sort_unstable_by_key(|x| x.iter().sum::<u8>() + x.len() as u8);
        let mut bounds = vec![];

        for (i,s) in values.iter().map(|x| x.iter().sum::<u8>() + x.len() as u8).enumerate() {
            if s >= bounds.len() as u8 {
                bounds.push(i);
            } else{
                bounds[s as usize] = i+1;
            }
        }

        SpacingLookupTable {values, bounds}
    }
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

fn sum_bits(n: u64) -> u8 {
    let mut state = n;
    let mut sum = 0;
    let mask = 1;
    while state > 0 {
        sum = sum + (state & mask) as u8;
        state = state >> 1;
    }
    sum
}

fn bin_has_no_adjacent_zeros(n: u64) -> bool  {
    n == 0 || n % 4 > 0 && bin_has_no_adjacent_zeros(n/2)
}



fn sum_all_permutations(record_vec: &Vec<Record>, spacing_lookup: &SpacingLookupTable) -> u64 {
    record_vec.into_iter()
        .map(|r| (0..(2u64.pow(r.count_unknown())))
            .map(|seed| r.generate_unknown_guess(&spacing_lookup, seed))
            .filter(|generated_r| generated_r.is_valid(r))
            .count() as u64
        )
        .sum()
}

fn quintuple_records(mut record_vec: Vec<Record>) ->  Vec<Record> {
    let mut temp_springs: Vec<Spring> = vec![];
    let mut temp_damaged: Vec<u32> = vec![];
    for idx in 0..(record_vec.len()) {
        for _ in 0..4 {
            temp_springs.push(Spring::Unknown);
            temp_springs.append(&mut record_vec[idx].springs.clone());
            temp_damaged.append(&mut record_vec[idx].damaged.clone());
        }
        record_vec[idx].springs.append(&mut temp_springs);
        record_vec[idx].damaged.append(&mut temp_damaged);
    }
    record_vec
}

// impl

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}
    
    let input = std::fs::read_to_string(args[1].as_str()).unwrap();
    
    let record_vec_1 = parse_input(input);

    let spacing_lookup = SpacingLookupTable::create_table(3);

    // println!("{:?}", record_vec_1[15]);
    // println!("{:?}", record_vec_1[15].generate_unknown_guess(&spacing_lookup, 0));

    // (0..(record_vec_1[15].springs.len() - (record_vec_1[15].damaged.iter().sum::<u32>() as usize + record_vec_1[15].damaged.len())))
    //     .powerset()
    //     .filter(|s| s.into_iter().sum::<usize>() < 5)
    //     .map(|x| x.into_iter().permutations(8).collect::<Vec<Vec<usize>>>())
    //     .for_each(|x| println!("{:?}", x));



    // ??#?##?.??.?????????o, 1,1,3,1,2,5, (*5)
    let record_vec_test = quintuple_records(record_vec_1);
    // let record_vec_test = record_vec_1;
    let test = record_vec_test[143].clone();
    
    println!("{:?}", test);
    let num_free = test.springs.len() - (test.damaged.iter().sum::<u32>() as usize + test.damaged.len()-1);
    println!("{:?}", (test.springs.len(), num_free));
    let combinations = (0..=num_free)
        // .multi_cartesian_product()
        .combinations_with_replacement(num_free)
        .skip(1)
        .filter(|x| x.iter().sum::<usize>() <= num_free)
        .map(|x| x.into_iter().filter(|e| e > &0).collect_vec())
        .take(10)
        // .permutations(num_free).flatten()
        // .map(|mut v| {
        //     v.append(&mut vec![0;test.damaged.len()-v.len()]);
        //     v
                // .permutations(test.damaged.len())
        // })
        // .take(1)
        // .flatten()
        // .collect_vec();
        .for_each(|x| println!("{:?}", x));


    // repeat_n(repeat_n(0,test.springs.len()).collect_vec(), num_free).multi_cartesian_product()
    //     // .filter(|x| x.iter().sum::<usize>() as usize <= upper_bound)
    //     .for_each(|x| println!("{:?}", x));

    return;

    let solution_1 = sum_all_permutations(&record_vec_1, &spacing_lookup);
    println!("{}", solution_1);
    
    let record_vec_2 = quintuple_records(record_vec_1);

    // let solution_2 = sum_all_possible_permutations_for_records(&record_vec_2);
    // println!("{}", solution_2);
}

#[test]
fn test_sum_bits() {
    assert_eq!(sum_bits(0), 0);
    assert_eq!(sum_bits(1), 1);
    assert_eq!(sum_bits(2), 1);
    assert_eq!(sum_bits(3), 2);
    assert_eq!(sum_bits(4), 1);
    assert_eq!(sum_bits(5), 2);
    assert_eq!(sum_bits(6), 2);
    assert_eq!(sum_bits(7), 3);
    assert_eq!(sum_bits(8), 1);
    assert_eq!(sum_bits(9), 2);
    assert_eq!(sum_bits(10), 2);
    assert_eq!(sum_bits(11), 3);
    assert_eq!(sum_bits(12), 2);
    assert_eq!(sum_bits(13), 3);
    assert_eq!(sum_bits(14), 3);
    assert_eq!(sum_bits(15), 4);
    assert_eq!(sum_bits(16), 1);
    assert_eq!(sum_bits(17), 2);
    assert_eq!(sum_bits(18), 2);
    assert_eq!(sum_bits(19), 3);
}

#[test]
fn test_bin_has_no_adjacent_zeros() {
    assert_eq!(bin_has_no_adjacent_zeros(0), true);
    assert_eq!(bin_has_no_adjacent_zeros(1), true);
    assert_eq!(bin_has_no_adjacent_zeros(2), true);
    assert_eq!(bin_has_no_adjacent_zeros(3), true);
    assert_eq!(bin_has_no_adjacent_zeros(4), false);
    assert_eq!(bin_has_no_adjacent_zeros(5), true);
    assert_eq!(bin_has_no_adjacent_zeros(6), true);
    assert_eq!(bin_has_no_adjacent_zeros(7), true);
    assert_eq!(bin_has_no_adjacent_zeros(8), false);
    assert_eq!(bin_has_no_adjacent_zeros(9), false);
    assert_eq!(bin_has_no_adjacent_zeros(10), true);
    assert_eq!(bin_has_no_adjacent_zeros(11), true);
    assert_eq!(bin_has_no_adjacent_zeros(12), false);
    assert_eq!(bin_has_no_adjacent_zeros(13), true);
    assert_eq!(bin_has_no_adjacent_zeros(14), true);
    assert_eq!(bin_has_no_adjacent_zeros(15), true);
    assert_eq!(bin_has_no_adjacent_zeros(16), false);
    assert_eq!(bin_has_no_adjacent_zeros(17), false);
    assert_eq!(bin_has_no_adjacent_zeros(18), false);
    assert_eq!(bin_has_no_adjacent_zeros(19), false);
    assert_eq!(bin_has_no_adjacent_zeros(20), false);
    assert_eq!(bin_has_no_adjacent_zeros(21), true);
    assert_eq!(bin_has_no_adjacent_zeros(22), true);
    assert_eq!(bin_has_no_adjacent_zeros(23), true);
    assert_eq!(bin_has_no_adjacent_zeros(24), false);
    assert_eq!(bin_has_no_adjacent_zeros(25), false);
}