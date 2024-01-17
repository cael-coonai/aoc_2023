#[derive(Clone, Copy, PartialEq)]
enum Terrain {
    Ash,
    Rock
}

impl std::fmt::Debug for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "·"),
            Self::Rock => write!(f, "#"),
        }
    }
}


struct Pattern {
    pattern: Vec<Vec<Terrain>>
}

impl Pattern {
    fn rotate_clone(&self) -> Pattern {
        let mut result =
            Vec::<Vec<Terrain>>::with_capacity(self.pattern[0].len());
        for col in 0..self.pattern[0].len() {
            result.push(Vec::with_capacity(self.pattern.len()));
            for row in 0..self.pattern.len() {
                result[col].push(self.pattern[row][col]);
            }
        }
        Pattern {pattern: result}
    }

    fn is_mirror(&self, idx: usize, tolerance: u32) -> bool {
        let mut curr = idx;
        let mut reflection = curr+1;
        let mut difference = 0;
        while curr < self.pattern.len() && reflection < self.pattern.len() {
            for idx in 0..self.pattern[0].len() {
                if self.pattern[curr][idx] != self.pattern[reflection][idx] {
                    difference = difference + 1;
                }
            }
            if difference > tolerance {return false;}
            curr = curr.overflowing_sub(1).0;
            reflection = reflection + 1;
        }
        return difference == tolerance;
    }

    fn locate_mirror(&self, tolerance: u32) -> usize {
        for idx in 0..(self.pattern.len()-1) {
            if self.is_mirror(idx, tolerance)  {
                return idx + 1;
            }
        }
        0
    }

    fn _debug_print(&self) {
        for row in &self.pattern {
            for terrain_elem in row {
                print!("{:?}", terrain_elem);
            }
            print!("\n");
        }
    }
}

fn parse_input(input: String) -> Vec<Pattern> {
    input.split("\n\n")
        .map(|p| Pattern { pattern: p.lines()
            .map(|l| l.chars()
                .map(|t| match t {
                    '.' => Terrain::Ash,
                    '#' => Terrain::Rock,
                    c   => unimplemented!("Invalid char: {:?}", c),
                })
                .collect()
            )
            .collect()
        })
        .collect()
}

fn summarise_patterns(patterns: &Vec<Pattern>, tolerance: u32) -> usize {
    patterns.iter()
        .zip(patterns.iter().map(|p| p.rotate_clone()))
        .map(|(p, pr)|
            p.locate_mirror(tolerance) * 100 + pr.locate_mirror(tolerance)
        )
        .sum()       
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let patterns = parse_input(input);

    let solution_1 = summarise_patterns(&patterns, 0);
    println!("{}", solution_1);

    let solution_2 = summarise_patterns(&patterns, 1);
    println!("{}", solution_2);

}