use std::error::Error;
use std::fs;
use std::io::Read;

fn main() {
    let tokens = read_from_file().unwrap_or_default();
    //println!("Collected tokens: {:?}", tokens);
    let mut total: i64 = 0;
    let interval = tokens.len() / 5;
    // PART 1
    for i in 0..interval {
        let val1: i64 = tokens[i].parse().unwrap();
        let val2: i64 = tokens[i + interval].parse().unwrap();
        let val3: i64 = tokens[i + (2 * interval)].parse().unwrap();
        let val4: i64 = tokens[i + (3 * interval)].parse().unwrap();
        let operand = tokens[i + (4 * interval)].as_str();
        let solution: i64 = match operand {
            "+" => val1 + val2 + val3 + val4,
            "*" => val1 * val2 * val3 * val4,
            _ => {
                println!("Invalid operand: {}", operand);
                0
            }
        };
        total += solution;
    }
    println!("Total of homework: {}", total);

    //PART 2
    let mut vert_numbers: Vec<String> = Vec::new();
    total = 0;
    for i in 0..interval {
        let val1 = tokens[i].clone();
        let val2 = tokens[i + interval].clone();
        let val3 = tokens[i + (2 * interval)].clone();
        let val4 = tokens[i + (3 * interval)].clone();
        let operand = tokens[i + (4 * interval)].as_str();

        vert_numbers = make_vert_numbers(vec![val1, val2, val3, val4]);
        //println!("{:?}, {}", vert_numbers, operand);

        let solution: i64 = match operand {
            "+" => {
                let mut inner_total: i64 = 0;
                for s in &vert_numbers {
                    let temp: i64 = s.parse().unwrap_or(0);
                    inner_total += temp;
                }
                //println!("Sum: {}", inner_total);
                inner_total
            }
            "*" => {
                let mut inner_total: i64 = 1;
                for s in &vert_numbers {
                    let temp: i64 = s.parse().unwrap_or(1);
                    inner_total *= temp;
                }
                inner_total
            }
            _ => {
                println!("Invalid operand: {}", operand);
                0
            }
        };
        //println!(
        //    "Problem: {:?}, {}\nSolution: {}",
        //    vert_numbers, operand, solution
        //);
        total += solution;
        vert_numbers.clear();
    }
    println!("Total of homework with vert numbers: {}", total);
}

fn make_vert_numbers(values: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = vec![String::new(), String::new(), String::new(), String::new()];
    for s in values {
        for i in 0..s.len() {
            if let Some(c) = s.chars().nth(i) {
                out[i].push(c);
            }
        }
    }
    out
}

fn read_from_file() -> Result<Vec<String>, Box<dyn Error>> {
    let mut buf: String = String::new();
    let mut file = fs::OpenOptions::new().read(true).open("./input.txt")?;
    file.read_to_string(&mut buf)?;
    Ok(buf.split_whitespace().map(|s| s.to_string()).collect())
}
