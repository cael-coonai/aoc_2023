fn count_winnings(line: &str) -> u32 {
  let nums_str:Vec<&str> = line.split(":").skip(1)
    .next().unwrap()
    .split("|")
    .collect();
  let winn_nums: Vec<u32> = nums_str[0]
    .trim().split_ascii_whitespace()
    .map(|n| n.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
  let plyr_nums: Vec<u32> = nums_str[1]
    .trim().split_ascii_whitespace()
    .map(|n| n.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
  let count = plyr_nums.into_iter()
  .filter(|pn| winn_nums.iter().any(|wn| wn == pn))
  .count() as u32;
  count
}

fn part_1(input: String) -> u32 {
  input.lines()
    .map(|l| {
      let wins = count_winnings(l);
      if wins > 0 {return 2u32.pow(wins-1)} 
      else        {return 0u32}
    })
    .sum()
}


fn part_2(input: String) -> u32 {
  let mut card_count: Vec<u32> = vec![1;input.lines().count()];

  for (card_num, line) in input.lines().enumerate() {
    let wins = count_winnings(line) as usize;
    
    for card in 0..wins {
      card_count[card + card_num + 1] += card_count[card_num];
    }
  }
  card_count.into_iter().sum()
}

fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {panic!("Input file path must be passed as arg.");}

  let input = std::fs::read_to_string(args[1].clone()).unwrap();

  let solution_1: u32 = part_1(input.clone());

  let solution_2: u32 = part_2(input.clone());

  println!("{}", solution_1);
  println!("{}", solution_2);  
}