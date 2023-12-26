fn main() {  
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {panic!("Input file path must be passed as arg.");}
  
    let input = std::fs::read_to_string(args[1].as_str()).unwrap();

    let number_list_1: Vec<Vec<u64>> = input.lines()
        .map(|l| 
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse().unwrap())
                .collect()
        ).collect();

    let time_dist_1 = number_list_1[0].iter().zip(number_list_1[1].iter());

    let solution_1 = time_dist_1.map(|(time, dist)| {
        let (min, max) = {
            let (time, dist) = (*time as f64, *dist as f64);
            (((time/2f64) - ((time.powi(2)/4f64) - dist).sqrt()).ceil() as u64,
             ((time/2f64) + ((time.powi(2)/4f64) - dist).sqrt()).floor() as u64)
            };
        (min..=max).map(|b_time| b_time*time - b_time.pow(2))
            .filter(|b_dist| b_dist > dist)
            .count() as u64
    }).product::<u64>();

  
    let number_list_2: Vec<u64> = input.lines()
    .map(|l| 
        l.split_ascii_whitespace()
        .skip(1)
            .collect::<String>()
            .parse::<u64>().unwrap()           
        ).collect();

    let time_dist_2 = (number_list_2[0],number_list_2[1]);
    
    let solution_2 = {
        let (min, max) = {
            let (time, dist) = (time_dist_2.0 as f64, time_dist_2.1 as f64);
            (((time/2f64) - ((time.powi(2)/4f64) - dist).sqrt()).ceil() as u64,
             ((time/2f64) + ((time.powi(2)/4f64) - dist).sqrt()).floor() as u64)
        };
        (min..=max).map(|b_time| b_time*time_dist_2.0 - b_time.pow(2))
        .filter(|b_dist| b_dist > &time_dist_2.1)
        .count() as u64
    };

    println!("{}", solution_1);
    println!("{}", solution_2);
}