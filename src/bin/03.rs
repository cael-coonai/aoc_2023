use std::cmp::{min, max};



fn number_bounds(sym_rng_j: usize, sym_rng_i: usize, input_chars: &Vec<Vec<char>>) -> (usize, usize) {
  let (mut digit_lf, mut digit_rt) = (sym_rng_j, sym_rng_j);

  while digit_lf as isize - 1 >= 0 && input_chars[sym_rng_i][digit_lf-1].is_digit(10) {
  digit_lf -= 1;
                }
  while digit_rt + 1 < input_chars[sym_rng_i].len() && input_chars[sym_rng_i][digit_rt+1].is_digit(10) {
  digit_rt += 1;
                }

  (digit_lf, digit_rt)
}

fn parse_num(input_chars: &[char]) -> u32 {
  input_chars.iter()
    .collect::<String>()
    .parse::<u32>()
    .unwrap()
}

fn part_1(input: String) -> u32 {
  let mut sum = 0;
  let mut input_chars =
    input.lines()
      .map(|l| l.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();

  for sym_i in 0..input_chars.len() {
    for sym_j in 0..input_chars[sym_i].len() {

      let symbol = input_chars[sym_i][sym_j];
      if !symbol.is_digit(10) && symbol != '.' && symbol != '\0' { // actual symbol check
        for sym_rng_i in max(sym_i-1,0)..=min(sym_i+1,input_chars.len()) {
          for sym_rng_j in max(sym_j-1,0)..=min(sym_j+1,input_chars[sym_i].len()) {

            let digit = input_chars[sym_rng_i][sym_rng_j];
            if digit.is_digit(10) {                               // actual digit check

              let (digit_lf, digit_rt) = number_bounds(sym_rng_j, sym_rng_i, &input_chars);

              sum += parse_num(&input_chars[sym_rng_i][digit_lf..=digit_rt]);

              input_chars[sym_rng_i][digit_lf..=digit_rt].fill('\0');

            }

          }
        }
      }

    }
  }
  sum
}

fn part_2(input: String) -> u32 {
  let mut sum = 0;
  let input_chars =
    input.lines()
      .map(|l| l.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();


  for sym_i in 0..input_chars.len() {
    for sym_j in 0..input_chars[sym_i].len() {

      let symbol_star = input_chars[sym_i][sym_j];
      if symbol_star == '*' {                                       // actual symbol check
        let mut nums_in_range: Vec<(usize,(usize,usize))> = vec![];

        for sym_rng_i in max(sym_i-1,0)..=min(sym_i+1,input_chars.len()) {
          for sym_rng_j in max(sym_j-1,0)..=min(sym_j+1,input_chars[sym_i].len()) {

            let digit = input_chars[sym_rng_i][sym_rng_j];
            if digit.is_digit(10) {                                 // actual digit check

              let (digit_lf, digit_rt) = number_bounds(sym_rng_j, sym_rng_i, &input_chars);

              nums_in_range.push((sym_rng_i,(digit_lf, digit_rt)));
            }

            
          }
        }

        let mut unique_nums_in_range: Vec<(usize,(usize,usize))> = vec![];
        for e in nums_in_range {
          if !unique_nums_in_range.contains(&e) {
            unique_nums_in_range.push(e);
          }
        }

        if unique_nums_in_range.len() == 2 {
          let (n1_row,(n1_lf,n1_rt)) = unique_nums_in_range[0];
          let (n2_row,(n2_lf,n2_rt)) = unique_nums_in_range[1];
          sum += parse_num(&input_chars[n1_row][n1_lf..=n1_rt]) *
            parse_num(&input_chars[n2_row][n2_lf..=n2_rt]);
        }
        
      }

    }
  }
  sum
}



fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {panic!("Input file path must be passed as arg.");}

  
  let input = std::fs::read_to_string(args[1].clone()).unwrap();

  let solution_1 = part_1(input.clone());
  println!("{}", solution_1);

  let solution_2 = part_2(input.clone());
  println!("{}", solution_2);

}
