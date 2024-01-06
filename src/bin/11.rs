use std::cmp::{max, min};
use itertools::iproduct;

#[derive(PartialEq, Clone)]
enum Pixel {
    Nothing,
    Galaxy
}

impl std::fmt::Debug for Pixel  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Galaxy => write!(f, "#"),
            Self::Nothing => write!(f, "·"),
        }
    }
}

fn parse_input(input: String) -> Vec<Vec<Pixel>> {
    input.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Pixel::Nothing,
                    '#' => Pixel::Galaxy,
                    c   => unreachable!("Invalid Character: {}", c)
                })
                .collect()
        })
        .collect()
}

fn expand_space(
    space: &Vec<Vec<Pixel>>,
    mut galaxies: Vec<(usize, usize)>,
    expansion_size: usize
) -> Vec<(usize, usize)> {

    let clear_cols = (0..space[0].len())
        .filter(|c|
            (0..space.len())
                .all(|r| space[r][*c] == Pixel::Nothing)
        )
        .collect::<Vec<usize>>();

    let clear_rows = (0..space.len())
        .filter(|r| space[*r].iter().all(|p| p == &Pixel::Nothing))
        .collect::<Vec<usize>>();

    for idx in 0..galaxies.len() {
        galaxies[idx] = (
            galaxies[idx].0 + expansion_size *
                clear_rows.iter().filter(|r| r < &&galaxies[idx].0).count(),
            galaxies[idx].1 + expansion_size *
                clear_cols.iter().filter(|c| c < &&galaxies[idx].1).count()
        );
    }

    galaxies
}

fn locate_galaxies(space: &Vec<Vec<Pixel>>) -> Vec<(usize, usize)> {
    iproduct!(0..space.len(), 0..space[0].len())
        .filter(|(r, c)| space[*r][*c] == Pixel::Galaxy)
        .collect()
}

fn sum_shortest_distatnces(galx: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    let pairs = iproduct!(0..galx.len(), 0..galx.len())
        .filter(|(i, j)| i < j);

    for (i, j) in pairs {
        sum = sum +
            max(galx[i].0, galx[j].0) - min(galx[i].0, galx[j].0) +
            max(galx[i].1, galx[j].1) - min(galx[i].1, galx[j].1);
    }

    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let space = parse_input(input);
    let galaxies = locate_galaxies(&space);

    let galaxies_1 = expand_space(&space, galaxies.clone(), 2-1);

    let solution_1 =  sum_shortest_distatnces(&galaxies_1);
    println!("{}", solution_1);

    let galaxies_2 = expand_space(&space, galaxies.clone(), 1000000-1);

    let solution_2 =  sum_shortest_distatnces(&galaxies_2);
    println!("{}", solution_2);
    
}