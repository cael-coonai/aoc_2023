use rayon::iter::{ParallelIterator, IntoParallelIterator};


#[derive(Debug, Clone, Copy)]
struct Map {
  src_start: u64,
  dst_start: u64,
  range: u64,
}

impl Map {
  fn new(dst_start: u64, src_start: u64, range: u64) -> Self {
    Map{src_start, dst_start, range}
  }

  fn can_convert(&self, input: u64) -> bool {
    input >= self.src_start && input < self.src_start + self.range
  }

  fn convert(&self, input: u64) -> u64 {
      input - self.src_start + self.dst_start
  }
}



fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {panic!("Input file path must be passed as arg.");}

  let input = std::fs::read_to_string(args[1].clone()).unwrap();

  let mut data = input.split(":")
    .filter(|l| !l.is_empty());

  let seeds_part1 = data.nth(1).unwrap()
    .split('\n')
    .nth(0).unwrap()
    .split_ascii_whitespace()
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<u64>>(); 
  

  let mut seeds_part2 =
    Vec::<std::ops::Range<u64>>::with_capacity(seeds_part1.len());
  for idx in (0..(seeds_part1.len()-1)).filter(|i| i%2 == 0) {
    seeds_part2.push(seeds_part1[idx]..(seeds_part1[idx] + seeds_part1[idx+1]));
  }

  let maps = data.map(|ms| {
    ms.lines()
    .filter(|m| !m.is_empty() && !m.contains('-'))
    .map(|m| {
      let mut map_vals = m.split_ascii_whitespace();
      Map::new(
        map_vals.next().unwrap().parse::<u64>().unwrap(),
        map_vals.next().unwrap().parse::<u64>().unwrap(),
        map_vals.next().unwrap().parse::<u64>().unwrap(),
      )
    }).collect::<Vec<Map>>()
  }).collect::<Vec<Vec<Map>>>();

  let solution1 = seeds_part1.iter()
    .map(|s| {
      let mut s = *s;
      for ms in &maps {
        for m in ms {
          if m.can_convert(s) {
            s = m.convert(s);
            break;
          }
        }
      }
      s
    })
    .min().unwrap();
  
  let solution2 = seeds_part2.iter()
    .map(|ss| { ss.clone()
      .into_par_iter()
      .map(|s|{
        let mut s = s;
        for ms in &maps {
          for m in ms {
            if m.can_convert(s) {
              s = m.convert(s);
              break;
            }
          }
        }
        s
      })
      .collect::<Vec<u64>>()
    })
    .flatten()
    .min().unwrap();

  
  println!("{:}", solution1);
  println!("{:}", solution2);
}