use std::collections;
use std::io;

pub fn execute() {
    use std::io;
    println!("Executing exercice_1.rs!");
    println!("Statistical Vector");

    let mut input = String::new();

    println!("input some spaced numbers");

    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("Unexpected program error upon taking user inputs. Halting program");
        return;
    }
    
    println!("You entered: {}", input);

    let mut v = Vec::new();
    for elem in input.split_whitespace()
    {
        let number: i32 = match elem.parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        v.push(number);
    }
    println!("considered vector for stats is {:?}", v);

    v.sort();

    match calculate_mean(&v) {
        Some(v) => println!("mean:\t {}", v),
        None    => println!("couldn't calculate mean!"),
    }

    match calculate_median(&v) {
        Some(v) => println!("median:\t {}", v),
        None    => println!("couldn't calculate median!"),
    }

    match calculate_mode(&v) {
        Some(v) => println!("mode: {}", v),
        None    => print!("counldn't calculate mode!"),
    }

    println!("done with exercice_1\n");
}

fn calculate_mean(v : &Vec<i32>) -> Option<i32> {
    if let 0 = v.len() {
        None
    }
    else {
        let mut val = 0;
        for elem in v.iter() {
            val += elem;
        }
        Some(val / (v.len() as i32))
    }
}

fn calculate_median(v : &Vec<i32>) -> Option<i32> {
    if let 0 = v.len() {
        None
    }
    else {
        Some(v[v.len()/2])
    }
}

fn calculate_mode(v : &Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    if let 0 = v.len() {
        None
    }
    else {
        let mut occurencies = HashMap::new();
        let mut ret = (0, 0);
        for elem in v.iter() {
            let val = occurencies.entry(elem).or_insert(0);
            *val += 1;
            if *val > ret.1 {
                ret = (*elem, *val)
            }
        }
        Some(ret.0)
    }
}