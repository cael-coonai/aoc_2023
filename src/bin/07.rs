
// Affects the sorting of card faces and the card type due to the
// treatment of 'J' as Jack in part 1 and Joker in part 2
static mut IS_PART_2: bool = false;

type Bid = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Default)]
struct Card {
    face: char,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        let face_ord: [char;13] = if unsafe {IS_PART_2 != true} {
            ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
        } else {
            ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']
        };

        if self.face == other.face {
            return Some(std::cmp::Ordering::Equal);
        }
        for f in face_ord {
            if f == self.face {
                return Some(std::cmp::Ordering::Greater);
            }
            if f == other.face {
                return Some(std::cmp::Ordering::Less);
            }
        }
        return None;
    }
}
    
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {High, One, Two, Three, Full, Four, Five}
    
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_t: HandType,
    hand: [Card;5]
}


impl Hand {
    fn new(input_hand: &str) -> Hand {
        
        let mut hand: [Card;5] = [Card::default(); 5];
        for (idx, c) in input_hand.chars().enumerate() {
            hand[idx].face = c;
        }
        
        let mut sorted_hand = hand;
        sorted_hand.sort_unstable();
        sorted_hand.reverse();
            let joker = if unsafe{!IS_PART_2} {Card {face: '\0'}}
                        else                  {Card {face:  'J'}};
            let hand_t = match sorted_hand {
                [a,b,c,d,e] if (
                    a==b && b==c && c==d && d==e     ||
                    a==b && b==c && c==d && e==joker ||
                    a==b && b==c && d==joker         ||
                    a==b && c==joker                 ||
                    b==joker
                ) => HandType::Five,
                
                [a,b,c,d,e] if (
                    a==b && b==c && c==d     || b==c && c==d && d==e     ||
                    a==b && b==c && e==joker || b==c && c==d && e==joker ||
                    a==b && d==joker         || b==c && d==joker         ||
                    c==joker 
                )   => HandType::Four,

                [a,b,c,d,e] if (
                    a==b && b==c && d==e     ||  a==b && c==d && d==e     ||
                    a==b && c==d && e==joker
                ) => HandType::Full,

                [a,b,c,d,e] if (
                    a==b && b==c     || b==c && c==d     || c==d && d==e     ||
                    a==b && e==joker || b==c && e==joker || c==d && e==joker ||
                    d==joker 
                ) => HandType::Three,

                [a,b,c,d,e] if (
                    a==b && c==d || a==b && d==e || b==c && d==e
                ) => HandType::Two,

                [a,b,c,d,e] if (
                    a==b     || b==c     || c==d     || d==e     ||
                    e==joker
                ) => HandType::One,

                _ => HandType::High 
            };

            Hand {hand_t, hand}
    }
}


fn evaluate_solution(input: &String) -> Bid {
    let mut rounds: Vec<(Hand, Bid)> = input.lines()
        .map(|l| {
            let mut hand_bid = l.split_ascii_whitespace();
            let hand: Hand = Hand::new(hand_bid.next().unwrap());
            let bid: Bid = hand_bid.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    
    rounds.sort_unstable_by_key(|r| r.0); // sort by hand
    
    rounds.iter()
        .enumerate()
        .rev()
        .map(|(idx,(_, bid))| {
            bid * (1 + idx as Bid)
        })
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}
  
    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let solution_1 = evaluate_solution(&input);

    unsafe {IS_PART_2 = true;} // Read comment at declearation for affects.

    let solution_2 = evaluate_solution(&input);
    
    println!("{}", solution_1);
    println!("{}", solution_2);
}



#[test]
fn hand_types() {
    assert_eq!(Hand::new("AAAAA").hand_t, HandType::Five);
    assert_eq!(Hand::new("AA8AA").hand_t, HandType::Four);
    assert_eq!(Hand::new("23332").hand_t, HandType::Full);
    assert_eq!(Hand::new("TTT98").hand_t, HandType::Three);
    assert_eq!(Hand::new("23432").hand_t, HandType::Two);
    assert_eq!(Hand::new("A23A4").hand_t, HandType::One);
    assert_eq!(Hand::new("23456").hand_t, HandType::High);
}

#[test]
fn hand_order() {
    assert!(Hand::new("AAAAA") > Hand::new("TTT98"));
    assert!(Hand::new("A23A4") < Hand::new("AA8AA"));

    assert!(Hand::new("33332") > Hand::new("2AAAA"));
    assert!(Hand::new("77788") < Hand::new("77888"));

    
    assert!(Hand::new("JJJJJ") > Hand::new("AJJAA"));
    unsafe{IS_PART_2 = true;}
    assert!(Hand::new("JJJJJ") < Hand::new("AJJAA"));
}