use itertools::iproduct;

#[derive(Clone, Copy, PartialEq)]
enum Pipe {NS, EW, NE, NW, SE, SW, Clear, Start}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Pipe::NS => '│',
            Pipe::EW => '─',
            Pipe::NE => '└',
            Pipe::NW => '┘',
            Pipe::SE => '┌',
            Pipe::SW => '┐',
            Pipe::Clear => '·',
            Pipe::Start => '#',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos {row: value.0, col: value.1}
    }
}

impl Pos {
    fn north(&self) -> Pos {(self.row-1, self.col  ).into()}
    fn south(&self) -> Pos {(self.row+1, self.col  ).into()}
    fn east(&self)  -> Pos {(self.row,   self.col+1).into()}
    fn west(&self)  -> Pos {(self.row,   self.col-1).into()}
    fn is_north_of(&self, p: &Pos) -> bool {&p.north() == self}
    fn is_south_of(&self, p: &Pos) -> bool {&p.south() == self}
    fn is_east_of(&self,  p: &Pos) -> bool {&p.east()  == self}
    fn is_west_of(&self,  p: &Pos) -> bool {&p.west()  == self}
}

#[derive(Debug, Clone)]
struct Network {
    start: Pos,
    piping: Vec<Vec<Pipe>>,
}

impl Network {
    fn pos(&self, p: &Pos) -> &Pipe {
       &self.piping[p.row][p.col]
    }

    fn mut_pos(&mut self, p: &Pos) -> &mut Pipe {
        &mut self.piping[p.row][p.col]
    }

    fn is_valid_pos (&self, p: &Pos) -> bool {
        p.row < self.piping.len() && p.col < self.piping[p.row].len()
    }

    fn can_go_north(&self, from: &Pos) -> bool {
        let north = &from.north();
        return self.is_valid_pos(&north) && (
            self.pos(north) == &Pipe::NS ||
            self.pos(north) == &Pipe::SE ||
            self.pos(north) == &Pipe::SW
        );
    }

    fn can_go_south(&self, from: &Pos) -> bool {
        let south = &from.south();
        return self.is_valid_pos(&south) && (
            self.pos(south) == &Pipe::NS ||
            self.pos(south) == &Pipe::NE ||
            self.pos(south) == &Pipe::NW
        );
    }

    fn can_go_east(&self, from: &Pos) -> bool {
        let east = &from.east();
        return self.is_valid_pos(&east) && (
            self.pos(east) == &Pipe::EW ||
            self.pos(east) == &Pipe::NW ||
            self.pos(east) == &Pipe::SW
        );
    }

    fn can_go_west(&self, from: &Pos) -> bool {
        let west = &from.west();
        return self.is_valid_pos(&west) && (
            self.pos(west) == &Pipe::EW ||
            self.pos(west) == &Pipe::NE ||
            self.pos(west) == &Pipe::SE
        );
    }
    
}

#[derive(Clone, PartialEq)]
enum IsClear{Checked(bool), ToCheck, UnChecked}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InwardDirection{North, South, East, West, None, Unassigned}

fn parse_input(input: String) -> Network {
    let piping: Vec<Vec<Pipe>> = input.lines()
        .map(|l| { l.chars()
            .map(|p| { match p {
                '|' => Pipe::NS,
                '-' => Pipe::EW,
                'L' => Pipe::NE,
                'J' => Pipe::NW,
                'F' => Pipe::SE,
                '7' => Pipe::SW,
                '.' => Pipe::Clear,
                'S' => Pipe::Start,
                _   => unimplemented!()
            }})
            .collect()
        })
        .collect();
    let mut start = (usize::MAX, usize::MAX);
    for r in 0..piping.len() {
        for c in 0..piping[r].len() {
            if piping[r][c] == Pipe::Start {
                start = (r,c);
            }
        }
    };

    Network {start: start.into(), piping}
}

fn determine_network(mut n: Network) -> Network {
    let start = &n.start;
    let mut possible = Vec::<Pipe>::with_capacity(1);

    if n.can_go_north(start) && n.can_go_south(start) {possible.push(Pipe::NS);}
    if n.can_go_east(start)  && n.can_go_west(start)  {possible.push(Pipe::EW);}
    if n.can_go_north(start) && n.can_go_east(start)  {possible.push(Pipe::NE);}
    if n.can_go_north(start) && n.can_go_west(start)  {possible.push(Pipe::NW);}
    if n.can_go_south(start) && n.can_go_east(start)  {possible.push(Pipe::SE);}
    if n.can_go_south(start) && n.can_go_west(start)  {possible.push(Pipe::SW);}

    if possible.len() != 1 {
        panic!("There are {} possible loops to explore.", possible.len());
    }

    let start = n.start;
    *n.mut_pos(&start) = possible[0];
    return n;
}

fn step_looping_path(network: &Network, curr: &Pos, prev: &Pos) -> Pos {
    if curr == prev {
        match network.pos(curr) {
            Pipe::NS => curr.north(),
            Pipe::EW => curr.east(),
            Pipe::NE => curr.north(),
            Pipe::NW => curr.north(),
            Pipe::SE => curr.south(),
            Pipe::SW => curr.south(),
            _ => unreachable!()
        }
    } else {
        match (network.pos(curr),  network.pos(prev)) {
            (Pipe::NS, Pipe::NS) => 
                if curr.is_north_of(&prev) {curr.north()} else {curr.south()},
            (Pipe::NS, Pipe::NE) => curr.north(),
            (Pipe::NS, Pipe::NW) => curr.north(),
            (Pipe::NS, Pipe::SE) => curr.south(),
            (Pipe::NS, Pipe::SW) => curr.south(),
            (Pipe::EW, Pipe::EW) =>
                if curr.is_east_of(&prev) {curr.east()} else {curr.west()},
            (Pipe::EW, Pipe::NE) => curr.east(),
            (Pipe::EW, Pipe::NW) => curr.west(),
            (Pipe::EW, Pipe::SE) => curr.east(),
            (Pipe::EW, Pipe::SW) => curr.west(),
            (Pipe::NE, Pipe::NS) => curr.east(),
            (Pipe::NE, Pipe::EW) => curr.north(),
            (Pipe::NE, Pipe::NW) => curr.north(),
            (Pipe::NE, Pipe::SE) => curr.east(),
            (Pipe::NE, Pipe::SW) =>
                if curr.is_south_of(&prev) {curr.east()} else {curr.north()},
            (Pipe::NW, Pipe::NS) => curr.west(),
            (Pipe::NW, Pipe::EW) => curr.north(),
            (Pipe::NW, Pipe::NE) => curr.north(),
            (Pipe::NW, Pipe::SE) =>
                if curr.is_east_of(&prev) {curr.north()} else {curr.west()},
            (Pipe::NW, Pipe::SW) => curr.west(),
            (Pipe::SE, Pipe::NS) => curr.east(),
            (Pipe::SE, Pipe::EW) => curr.south(),
            (Pipe::SE, Pipe::NE) => curr.east(),
            (Pipe::SE, Pipe::NW) =>
                if curr.is_west_of(&prev) {curr.south()} else {curr.east()},
            (Pipe::SE, Pipe::SW) => curr.south(),
            (Pipe::SW, Pipe::NS) => curr.west(),
            (Pipe::SW, Pipe::EW) => curr.south(),
            (Pipe::SW, Pipe::NE) =>
                if curr.is_north_of(&prev) {curr.west()} else {curr.south()},
            (Pipe::SW, Pipe::NW) => curr.west(),
            (Pipe::SW, Pipe::SE) => curr.south(),
            _ => unreachable!(
                "Reached {:?} from {:?}.",
                (curr,network.pos(curr)),
                (prev, network.pos(prev))
            )
        }
    }
}


fn determine_looping_path(network: &Network) -> Vec<Pos> {
    let mut path = vec![network.start];
    
    let mut curr = step_looping_path(network, &network.start, &network.start);
    let mut prev = network.start;
    while curr != network.start {
        path.push(curr);
        let next = step_looping_path(network, &curr, &prev);
        prev = curr;
        curr = next;
    }
    return path;
}

fn remove_unconnected_pipes(mut network: Network, path: &Vec<Pos>) -> Network {
    let non_path_pipes =
        iproduct!(0..network.piping.len(), 0..network.piping[0].len())
            .map(|p| Pos::from(p))
            .filter(|p| !path.contains(p));

    for pos in non_path_pipes {
        *network.mut_pos(&pos) = Pipe::Clear;
    }
    network
}

fn count_clear_inside_loop(network: &Network, path: &Vec<Pos>) -> usize {

    // Identify any NS pipe on the edge of the loop.
    let mut starting_pos: Pos = (usize::MAX,usize::MAX).into();

    'RowsLoop: for r in 0..network.piping.len() {
        for c in 0.. network.piping[r].len() {
            match network.pos(&(r,c).into()) {
                Pipe::NS => {
                        starting_pos = (r,c).into();
                        break 'RowsLoop;
                    },
                Pipe::Clear => (),
                _ => continue 'RowsLoop,
            }
        }
    }

    if starting_pos == (usize::MAX,usize::MAX).into() {
        panic!("Failed to find NS pipe on edge of loop.");
    }

    // Assign directions toward inside for all pos in path.
    let starting_idx = path.iter().position(|pos| pos == &starting_pos)
    .expect(
        format!("Failed to find {starting_pos:?} in path:{path:?}").as_str()
    );
    let mut inside = vec![[InwardDirection::Unassigned; 2]; path.len()];
    inside[starting_idx] = [InwardDirection::East, InwardDirection::None];
    
    type D = InwardDirection;

    let mut curr_idx = (starting_idx + 1) % path.len();
    let mut prev_idx = starting_idx;
    while curr_idx != starting_idx {
        inside[curr_idx] = match (
            network.pos(&path[curr_idx]),
            network.pos(&path[prev_idx]),
            inside[prev_idx]
        ) {
            (Pipe::NS, Pipe::NS, d) => d,
            (Pipe::NE, Pipe::NS, [D::East,  D::None]) => [D::None,  D::None],
            (Pipe::NE, Pipe::NS, [D::West,  D::None]) => [D::South, D::West],
            (Pipe::NW, Pipe::NS, [D::East,  D::None]) => [D::South, D::East],
            (Pipe::NW, Pipe::NS, [D::West,  D::None]) => [D::None,  D::None],
            (Pipe::SE, Pipe::NS, [D::East,  D::None]) => [D::None,  D::None],
            (Pipe::SE, Pipe::NS, [D::West,  D::None]) => [D::North, D::West],
            (Pipe::SW, Pipe::NS, [D::East,  D::None]) => [D::North, D::East],
            (Pipe::SW, Pipe::NS, [D::West,  D::None]) => [D::None,  D::None],
            (Pipe::EW, Pipe::EW, d) => d,
            (Pipe::NE, Pipe::EW, [D::North, D::None]) => [D::None,  D::None],
            (Pipe::NE, Pipe::EW, [D::South, D::None]) => [D::South, D::West],
            (Pipe::NW, Pipe::EW, [D::North, D::None]) => [D::None,  D::None],
            (Pipe::NW, Pipe::EW, [D::South, D::None]) => [D::South, D::East],
            (Pipe::SE, Pipe::EW, [D::North, D::None]) => [D::North, D::West],
            (Pipe::SE, Pipe::EW, [D::South, D::None]) => [D::None,  D::None],
            (Pipe::SW, Pipe::EW, [D::North, D::None]) => [D::North, D::East],
            (Pipe::SW, Pipe::EW, [D::South, D::None]) => [D::None,  D::None],
            (Pipe::NS, Pipe::NE, [D::South, D::West]) => [D::West,  D::None],
            (Pipe::NS, Pipe::NE, [D::None,  D::None]) => [D::East,  D::None],
            (Pipe::EW, Pipe::NE, [D::South, D::West]) => [D::South, D::None],
            (Pipe::EW, Pipe::NE, [D::None,  D::None]) => [D::North, D::None],
            (Pipe::NW, Pipe::NE, [D::South, D::West]) => [D::South, D::East],
            (Pipe::NW, Pipe::NE, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::SE, Pipe::NE, [D::South, D::West]) => [D::North, D::West],
            (Pipe::SE, Pipe::NE, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::SW, Pipe::NE, [D::South, D::West]) => [D::None,  D::None],
            (Pipe::SW, Pipe::NE, [D::None,  D::None]) => [D::North, D::East],
            (Pipe::NS, Pipe::NW, [D::South, D::East]) => [D::East,  D::None],
            (Pipe::NS, Pipe::NW, [D::None,  D::None]) => [D::West,  D::None],
            (Pipe::EW, Pipe::NW, [D::South, D::East]) => [D::South, D::None],
            (Pipe::EW, Pipe::NW, [D::None,  D::None]) => [D::North, D::None],
            (Pipe::NE, Pipe::NW, [D::South, D::East]) => [D::South, D::West],
            (Pipe::NE, Pipe::NW, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::SE, Pipe::NW, [D::South, D::East]) => [D::None,  D::None],
            (Pipe::SE, Pipe::NW, [D::None,  D::None]) => [D::North, D::West],
            (Pipe::SW, Pipe::NW, [D::South, D::East]) => [D::North, D::East],
            (Pipe::SW, Pipe::NW, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::NS, Pipe::SE, [D::North, D::West]) => [D::West,  D::None],
            (Pipe::NS, Pipe::SE, [D::None,  D::None]) => [D::East,  D::None],
            (Pipe::EW, Pipe::SE, [D::North, D::West]) => [D::North, D::None],
            (Pipe::EW, Pipe::SE, [D::None,  D::None]) => [D::South, D::None],
            (Pipe::NE, Pipe::SE, [D::North, D::West]) => [D::South, D::West],
            (Pipe::NE, Pipe::SE, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::NW, Pipe::SE, [D::North, D::West]) => [D::None,  D::None],
            (Pipe::NW, Pipe::SE, [D::None,  D::None]) => [D::South, D::East],
            (Pipe::SW, Pipe::SE, [D::North, D::West]) => [D::North, D::East],
            (Pipe::SW, Pipe::SE, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::NS, Pipe::SW, [D::North, D::East]) => [D::East,  D::None],
            (Pipe::NS, Pipe::SW, [D::None,  D::None]) => [D::West,  D::None],
            (Pipe::EW, Pipe::SW, [D::North, D::East]) => [D::North, D::None],
            (Pipe::EW, Pipe::SW, [D::None,  D::None]) => [D::South, D::None],
            (Pipe::NE, Pipe::SW, [D::North, D::East]) => [D::None,  D::None],
            (Pipe::NE, Pipe::SW, [D::None,  D::None]) => [D::South, D::West],
            (Pipe::NW, Pipe::SW, [D::North, D::East]) => [D::South, D::East],
            (Pipe::NW, Pipe::SW, [D::None,  D::None]) => [D::None,  D::None],
            (Pipe::SE, Pipe::SW, [D::North, D::East]) => [D::North, D::West],
            (Pipe::SE, Pipe::SW, [D::None,  D::None]) => [D::None,  D::None],
            (a, b, c) => unreachable!("Unexpected position: {:?}",(a,b,c))
        };
        prev_idx = curr_idx;
        curr_idx = (curr_idx + 1) % path.len();
    }


    // Assign all Pipe::Clears pointed to by inside[] to IsClear::toCheck
    let inside_clears_pos = path.into_iter()
        .zip(inside.into_iter())
        .flat_map(|(pos,direct_arr)| [(pos,direct_arr[0]),(pos, direct_arr[1])])
        .filter(|(_, direction)| direction != &InwardDirection::None)
        .map(|(pos, direction)| match direction {
            InwardDirection::North => pos.north(),
            InwardDirection::South => pos.south(),
            InwardDirection::East  => pos.east(),
            InwardDirection::West  => pos.west(),
            d => unreachable!("Unexpected direction {:?} at {:?}", d, pos),
        })
        .filter(|pos| network.pos(pos) == &Pipe::Clear);

    // Assign all Pipe::Clears poined to by inside[] to IsClear::toCheck
    let mut inside_clears =
        vec![
            vec![IsClear::UnChecked; network.piping[0].len()];
            network.piping.len()
        ];

    for pos in inside_clears_pos {
        inside_clears[pos.row][pos.col] = IsClear::ToCheck;
    }

    // Check all IsClear::toCheck and flood fill all remaining Pipe::Clear
    while inside_clears.iter().flatten().any(|x| x == &IsClear::ToCheck) {
        let to_check: Vec<Pos> = // list of clears to check ranges for
            iproduct!(0..network.piping.len(), 0..network.piping[0].len())
                .filter(|(r,c)| inside_clears[*r][*c] == IsClear::ToCheck)
                .map(|p| Pos::from(p))
                .collect();

        for p in to_check.iter() { // setting ToCheck to Checked
            inside_clears[p.row][p.col] = IsClear::Checked(true);
        }

        let to_check_range: Vec<Pos> = to_check.iter() // list ToCheck ranges
            .flat_map(|p| {
                let (r, c) = (p.row as isize, p.col as isize);
                [(r-1, c),(r+1, c),(r, c-1),(r, c+1)]
            })
            .filter(|(r, c)|
                r >= &0 && r < &(network.piping.len() as isize) &&
                c >= &0 && c < &(network.piping[0].len() as isize) &&
                inside_clears[*r as usize][*c as usize] == IsClear::UnChecked
            )
            .map(|(r, c)| Pos::from((r as usize, c as usize)))
            .collect();

        for p in to_check_range { // checking ToCheck ranges
            if network.pos(&p) == &Pipe::Clear {
                inside_clears[p.row][p.col] = IsClear::ToCheck;
            } else {
                inside_clears[p.row][p.col] = IsClear::Checked(false);
            }
        }
    }

    return inside_clears.into_iter()
        .flatten()
        .filter(|x| x == &IsClear::Checked(true))
        .count();
}


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let network = parse_input(input);
    let network = determine_network(network);
    let path = determine_looping_path(&network);

    let solution_1 = path.len() / 2;
    println!("{}", solution_1);
     
    let network = remove_unconnected_pipes(network, &path);

    let solution_2 = count_clear_inside_loop(&network, &path);
    println!("{}", solution_2);

}