use std::fs;

fn main() {
    let mut count = 0;
    let input = fs::read_to_string("input2.txt").unwrap();
    for line in input.lines() {
        let values = parse_line(line);
        if fully_contain(values) == true {
            count = count + 1;
        }
    }
    println!("count of fully included values: {}", count);
}

fn parse_line(val: &str) -> (i32, i32, i32, i32) {
    let f: Vec<&str> = val.split(',').collect();
    let lowside: Vec<&str> = f[0].split('-').collect();
    let highside: Vec<&str> = f[1].split('-').collect();
    let a = lowside[0].parse().unwrap();
    let b = lowside[1].parse().unwrap();
    let c = highside[0].parse().unwrap();
    let d = highside[1].parse().unwrap();
    (a, b, c, d)
}

fn fully_contain(values: (i32, i32, i32, i32)) -> bool {
    if values.0 > values.3 {
        return true;
    }
    if values.2 > values.1 {
        return true;
    }
    if values.3 < values.0 {
        return true;
    }
    if values.1 > values.2 {
        return true;
    }
    if values.
    false
}
