use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("src/input").expect("Unable to open file");
    let f = BufReader::new(f);
    let data: Vec<Vec<u32>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    let mut count: [u32; 12] = [0; 12];

    for line in &data {
        for (i, c) in line.iter().enumerate() {
            if c == &1 {
                count[i] += 1;
            }
        }
    }

    let most_common: [u32; 12] = count.map(|x| {
        if x > (data.len() / 2).try_into().unwrap() {
            1
        } else {
            0
        }
    });

    fn get_value<F: Fn(u32, u32) -> bool>(
        value_list: &Vec<Vec<u32>>,
        most_common: [u32; 12],
        f: F,
    ) -> u32 {
        let mut temp = value_list.clone();
        'outer: for i in 0..most_common.len() {
            for j in (0..temp.len()).rev() {
                if f(temp[j][i], most_common[i]) {
                    temp.remove(j);
                }
                if temp.len() == 1 {
                    break 'outer;
                }
            }
        }
        u32::from_str_radix(
            &temp[0].iter().map(ToString::to_string).collect::<String>(),
            2,
        )
        .expect("Oh no")
    }

    let oxygen = get_value(&data, most_common, |a: u32, b: u32| a == b);
    let carbon = get_value(&data, most_common, |a: u32, b: u32| a != b);

    println!("Carbon: {}", oxygen);
    println!("Oxygen: {}", carbon);
    println!("Life Support: {}", oxygen * carbon);

    // 1313900 - too high
}
