use std::error::Error;
use std::fs;
use std::io::Read;
// 1: Take file input
// 2: Split file input into two sections: the ranges and the ingredients
// 3: Define each range as a unique value
// 4: Check each value against those ranges

#[derive(Default)]
struct FoodData {
    ranges: Vec<(i64, i64)>,
    inventory: Vec<i64>,
}

fn main() {
    let mut input = read_file_input().unwrap_or_default();
    let mut count = 0;
    // PART 1
    // there is probably a better way of doing this that isn't O(n^2) but I can't be fucked to
    // figure it out
    for item in input.inventory {
        for r in &input.ranges {
            if item >= r.0 && item <= r.1 {
                count += 1;
                break;
            }
        }
    }
    println!("Total number of fresh items: {}", count);

    // PART 2

    input.ranges.sort_by_key(|r| r.0);

    let mut total: i64 = 0;
    let mut running_start = input.ranges[0].0;
    let mut running_end = input.ranges[0].1;

    for r in &input.ranges[1..] {
        if r.0 <= running_end {
            running_end = running_end.max(r.1);
        } else {
            total += running_end - running_start + 1;
            running_start = r.0;
            running_end = r.1;
        }
    }

    total += running_end - running_start + 1;
    println!("Total number of valid ids: {}", total);
}

fn read_file_input() -> Result<FoodData, Box<dyn Error>> {
    let mut buf = String::new();
    let mut file = fs::OpenOptions::new().read(true).open("./src/input.txt")?;
    file.read_to_string(&mut buf)?;

    let tables = buf.split_at(buf.find("\n\n").unwrap());
    let valid_ranges: Vec<(i64, i64)> = tables
        .0
        .split_whitespace()
        .map(|s| -> (i64, i64) {
            let temp = s.to_string();
            let t = temp.split_once('-').unwrap();
            (t.0.parse().unwrap(), t.1.parse().unwrap())
        })
        .collect();
    let values: Vec<i64> = tables
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(FoodData {
        ranges: valid_ranges,
        inventory: values,
    })
}
