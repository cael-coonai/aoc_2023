use std::cmp::min;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {panic!("Path must be input as arg.");}

  let numbers = [
    ("one".as_bytes(),   b'1'),
    ("two".as_bytes(),   b'2'),
    ("three".as_bytes(), b'3'),
    ("four".as_bytes(),  b'4'),
    ("five".as_bytes(),  b'5'),
    ("six".as_bytes(),   b'6'),
    ("seven".as_bytes(), b'7'),
    ("eight".as_bytes(), b'8'),
    ("nine".as_bytes(),  b'9'),
  ].into_iter();

  let solution: u64 = unsafe {
     std::fs::read_to_string(args[1].clone())
      .expect("Failed to read input file.")
      .lines()
      .map(|l| {
        let mut line = l.to_string();
        let bytes = line.as_bytes_mut();

        if l.len() >= 3 {
          for i in (0..l.len()-2).into_iter() {
            let slice = &mut bytes[i..min(i+5,l.len())];
            for (name, num) in numbers.clone() {
              if slice.starts_with(name)  {
                slice[0] = num;
              }
            }
          }
        }
        
        let mut iter = line.chars().filter(|c| c >= &'0' && c <= &'9');
        let first = iter.next().unwrap();
        let last = match iter.last() {Some(d) => d, None => first};
        return 10*(first as u64 - '0' as u64) + (last as u64 - '0' as u64)
      })
      .sum()
  };


  println!("{solution}")
}
