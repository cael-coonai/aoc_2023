

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeReference {
    Value([char;3]),
    Index(usize)
}

impl NodeReference {
    fn index(self) -> usize {
        match self {
            NodeReference::Value(_) => unimplemented!(),
            NodeReference::Index(i) => i,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    value: [char;3],
    left: NodeReference,
    right: NodeReference
}

fn get_directions(input: &String) -> Vec<Direction> {
    let directions: Vec<Direction> = input.lines()
        .nth(0).unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!()
        })
        .collect();
    directions
}

fn get_nodes(input: String) -> Vec<Node> {
    let mut nodes = input.lines()
        .skip(2)
        .map(|l| {
            let mut chars = l.chars();
            let value = [
                chars.nth(0).unwrap(),
                chars.nth(0).unwrap(),
                chars.nth(0).unwrap(),
            ];
            let left = NodeReference::Value([
                chars.nth(4).unwrap(),
                chars.nth(0).unwrap(),
                chars.nth(0).unwrap(),
            ]);
            let right = NodeReference::Value([
                chars.nth(2).unwrap(),
                chars.nth(0).unwrap(),
                chars.nth(0).unwrap(),
            ]);

            Node {value, left, right}
        })
        .collect::<Vec<Node>>();

    nodes.sort_unstable();

    for idx in 0..nodes.len() {
        let left_idx = match nodes[idx].left {
            NodeReference::Value(v) =>
                nodes.binary_search_by_key(&v, |&n| n.value).unwrap(),
            _ => unreachable!()            
        };
        let right_idx = match nodes[idx].right {
            NodeReference::Value(v) =>
                nodes.binary_search_by_key(&v, |&n| n.value).unwrap(),
            _ => unreachable!()            
        };

        nodes[idx].left = NodeReference::Index(left_idx);
        nodes[idx].right = NodeReference::Index(right_idx);
    }
    nodes
}

fn part_1(nodes: &Vec<Node>, directions: &Vec<Direction>) -> u32 {
    let mut curr = &nodes[0];
    let mut count = 0;
    let mut pos = 0;
    while curr.value != ['Z','Z','Z'] {
        curr = match directions[pos] {
            Direction::Left  => &nodes[curr.left.index()],
            Direction::Right => &nodes[curr.right.index()],
        };
        pos = (pos + 1) % directions.len();
        count = count + 1;
    }
    count
}

fn lcm(a: u64, b:u64) -> u64 {
    fn gcd(a: u64, b:u64) -> u64 {
        if b == 0 {return a;}
        if a == 0 {return b;}
        if a > b  {return gcd(a%b, b);}
        else      {return gcd(a, b%a);}
    }
    return (a * b) / gcd(a,b);
}

fn part_2(nodes: &Vec<Node>, directions: &Vec<Direction>) -> u64 {   
    let mut curr: Vec<&Node> =
        nodes.iter().filter(|n| n.value[2] == 'A').collect();
    let mut periods = Vec::<u64>::with_capacity(curr.len());
    let mut count = 0;
    let mut pos = 0;
    while !curr.is_empty() {
        if curr.iter().any(|n| n.value[2] == 'Z') {
            curr = curr.into_iter().filter(|n| n.value[2] != 'Z').collect();
            periods.push(count);
        }
        for idx in 0..curr.len() {
            curr[idx] = match directions[pos] {
                Direction::Left  => &nodes[curr[idx].left.index()],
                Direction::Right => &nodes[curr[idx].right.index()],
            }
        }
        pos = (pos + 1) % directions.len();
        count = count + 1;
    }
    return periods.into_iter().reduce(|acc, e| lcm(acc, e)).unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}

    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let directions = get_directions(&input);

    let nodes = get_nodes(input);
    
    let solution_1 = part_1(&nodes, &directions);
    println!("{}",solution_1);

    let solution_2 = part_2(&nodes, &directions);
    println!("{}",solution_2);
}



