use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Nothing,
    Cube,
    Sphere,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nothing => write!(f, "·"),
            Self::Cube => write!(f, "#"),
            Self::Sphere => write!(f, "O"),
        }
    }
}

fn parse_input(input: String) -> Vec<Vec<Tile>>  {
    input.lines()
        .map(|l| l.chars()
            .map(|c| match c {
                '.' => Tile::Nothing,
                '#' => Tile::Cube,
                'O' => Tile::Sphere,
                c   => unimplemented!("Invalid character: {}", c)
            })
            .collect()
        )
        .collect()
}

fn east_tilt(dish: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    dish.iter()
        .map(|row| {
            let mut segments = row.split(|t| t == &Tile::Cube)
                .map(|segment| {
                    let nothing_count =
                        segment.iter().filter(|t| t == &&Tile::Nothing).count();

                    let mut result = vec![Tile::Nothing; segment.len()];
                    result[nothing_count..].fill(Tile::Sphere);
                    result
                }).collect::<Vec<Vec<Tile>>>();
            let mut result  = Vec::with_capacity(row.len());
            for idx in 0..segments.len() {
                result.append(&mut segments[idx]);
                if idx < segments.len()-1 {result.push(Tile::Cube)}
            }
            result
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn east_load_count(dish: &Vec<Vec<Tile>>) -> usize {
    let mut load = 0;
    for row in dish {
        for (idx, tile) in row.iter().enumerate() {
            match tile {
                Tile::Sphere => load = load + idx + 1,
                _ => ()
            }
        }
    }
    load
}

fn rotate_90_clone(dish: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result =
        Vec::<Vec<Tile>>::with_capacity(dish[0].len());
    for col in 0..dish[0].len() {
        result.push(Vec::with_capacity(dish.len()));
        for row in 0..dish.len() {
            result[col].push(dish[row][col]);
        }
        result[col].reverse();
    }
    result
}

fn north_tilt(dish: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result = rotate_90_clone(&dish);
        result = east_tilt(&result);
    for _ in 0..3 {
        result = rotate_90_clone(&result);
    }
    result
}

fn north_load_count(dish: &Vec<Vec<Tile>>) -> usize {
    east_load_count(&rotate_90_clone(&dish))
}


fn tilt_cycle(dish: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut dish = dish.clone();
    for _ in 0..4 {
        dish = rotate_90_clone(&dish);
        dish = east_tilt(&dish);
    }
    dish
}

fn tilt_cycle_rep(
    dish: &Vec<Vec<Tile>>,
    num_cycles: u64,
    memo_step: u64
) -> Vec<Vec<Tile>> {
    assert!(num_cycles % memo_step == 0);

    let mut dish = dish.clone();
    let mut memo = HashMap::<Vec<Vec<Tile>>, Vec<Vec<Tile>>>::new();

    for _ in 0..(num_cycles/memo_step) {
        if let Some(d) = memo.get(&dish) {
            dish = d.to_vec();
        } else {
            let mut altered = tilt_cycle(&dish);
            for _ in 0..(memo_step-1) {altered = tilt_cycle(&altered)}
            memo.insert(dish, altered.clone());
            dish = altered;
        }
    }
    dish
}
fn _print_dish(dish: &Vec<Vec<Tile>>) {
    for row in dish {
        for tile in row {
            print!("{:?}", tile);
        }
        print!("\n");
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let dish_original = parse_input(input);

    let dish_1 = north_tilt(&dish_original);

    let solution_1 = north_load_count(&dish_1);
    println!("{}", solution_1);

    let dish_2 = tilt_cycle_rep(&dish_original, 1000000000, 1000);

    let solution_2 = north_load_count(&dish_2);
    println!("{}", solution_2);
}
