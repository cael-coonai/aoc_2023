use std::cmp::max;

#[derive(Debug, Clone, Copy)]
enum Colour {
  Red, Green, Blue
}

#[derive(Debug, Default, Clone, Copy)]
struct Set {
  red: u32,
  green: u32,
  blue: u32,
}

impl Set {  
  fn new(red: u32, green: u32, blue: u32) -> Self {
    Set {red, green, blue}
  }

  fn from(colour: Colour, value: u32) -> Self {
    let (red, green, blue) = match colour {
      Colour::Red   => (value,0,0),
      Colour::Green => (0,value,0),
      Colour::Blue  => (0,0,value)
    };
    Set {red, green, blue}
  }

  fn maximise(self, rhs: Self) -> Self {
    Set {
      red:   max(self.red,   rhs.red),
      green: max(self.green, rhs.green),
      blue:  max(self.blue,  rhs.blue)
    }
  }

  fn is_possible(self, maximum: Self) -> bool {
    self.red   <= maximum.red   &&
    self.green <= maximum.green &&
    self.blue  <= maximum.blue 
  }

  fn power(self) -> u32 {
    self.red * self.green * self.blue
  }
}

impl std::ops::Add for Set {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Set {
      red:   self.red   + rhs.red,
      green: self.green + rhs.green,
      blue:  self.blue  + rhs.blue
    }
  }
}

impl std::iter::Sum for Set {
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(Self::default(), |acc, x| acc + x)
  }
}

fn part_1(game_string: String, max_possible: Set) -> u32 {
  game_string.lines()
    .map(|l| {
      let info = l.split(':').collect::<Vec<&str>>();
      let id:u32 =
       info[0].trim().split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

      let set_max = info[1].split(';')
        .map(|s| {
          s.split(',')
            .map(|cubes| {
              if cubes.contains("red") {
                return Set::from(Colour::Red, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              } else if cubes.contains("blue") {
                return Set::from(Colour::Blue, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              } else if cubes.contains("green") {
                return Set::from(Colour::Green, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              }
              unreachable!();
            }).sum::<Set>()
        })
        .fold(Set::default(), |acc, s| Set::maximise(acc,s));
      (id, set_max)
    })
    .filter(|(_, set)| set.is_possible(max_possible))
    .map(|(id,_)| id)
    .sum()
}

fn part_2(game_string: String) -> u32 {
  game_string.lines()
    .map(|l| {
      let info = l.split(':').collect::<Vec<&str>>();
      // let id:u32 =
      //  info[0].trim().split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

      let set_max = info[1].split(';')
        .map(|s| {
          s.split(',')
            .map(|cubes| {
              if cubes.contains("red") {
                return Set::from(Colour::Red, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              } else if cubes.contains("blue") {
                return Set::from(Colour::Blue, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              } else if cubes.contains("green") {
                return Set::from(Colour::Green, cubes.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap());
              }
              unreachable!();
            }).sum::<Set>()
        })
        .fold(Set::default(), |acc, s| Set::maximise(acc,s));
      set_max                    // Only this line go down differs from part 1
    })
    .map(|set_max| set_max.power())
    .sum()
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {panic!("Input file path must be input as arg 1.");}
  let game_strings =
    std::fs::read_to_string(args[1].clone()).expect("Failed to read input.");
  let solution1: u32 = part_1(game_strings.clone(), Set::new(12, 13, 14));
  let solution2: u32 = part_2(game_strings);

  println!("{}", solution1);
  println!("{}", solution2);
}
