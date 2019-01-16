use std::fmt;
use std::ops::Range;
use std::vec::Vec;
use std::io;

pub fn execute() {
    do_stuff();
}

fn display_fb(range: Range<u32>) {
    for i in range {
        println!("{}", FbAssoc::new(i).to_string());
    }
}

fn get_fb_string(range: Range<u32>) -> Vec<String> {
    let mut v = Vec::new();
    for i in range {
        v.push(FbAssoc::new(i).to_string());
    }
    v
}

fn get_fb_simplified_string(range: Range<u32>) -> Vec<String> {
    let mut v = Vec::new();
    for i in range {
        v.push(fb_simplified(i));
    }
    v
}

struct FbAssoc {
    in_val: u32,
    out_val: String,
}

impl FbAssoc {
    fn new(val: u32) -> FbAssoc {
        FbAssoc {
            in_val: val,
            out_val: fb(val),
        }
    }
}

impl fmt::Display for FbAssoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t =>\t {}", self.in_val, self.out_val)
    }
}

pub fn fb (v: u32) -> String {
    if v % 15 == 0 {
        String::from("FizzBuzz")
    }
    else if v % 5 == 0 {
        String::from("Buzz")
    }
    else if v % 3 == 0 {
        String::from("Fizz")
    }
    else {
        v.to_string()
    }
}

pub fn fb_simplified (v: u32) -> String {
    if v % 15 == 0 {
        String::from("FizzBuzz")
    }
    else if v % 5 == 0 {
        String::from("Buzz")
    }
    else if v % 3 == 0 {
        String::from("Fizz")
    }
    else {
        0.to_string()
    }
}

struct Agent {
    context: context,
}

struct context {
    window_length: u32,
    unit_length: u32,
}

fn guess_the_pattern() {
    println!("enter numbers and i will try to identify the pattern");
    let mut i = 0;

    loop {
        let mut input = String::new();

        println!("input your number");

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Unexpected program error upon taking user inputs. Halting program");
            return;
        }
    
        println!("You entered: {}", input);

    }

}

fn do_stuff() {
    use std::collections::HashMap;

    let v = vec![1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2];

    //let v = vec!["1", "1", "2", "2", "1", "1", "2", "2"];

    let v = get_fb_simplified_string(0..100);

    println!("{:?}", v);

    let mut h = HashMap::new();

    let mut ind = 0;

    let max_slice_length = v.len() / 2;

    for i in 1..max_slice_length+1 {
        for j in 0..v.len()-2*i {

            if compare(&v[j..j+i], &v[j+i..j+2*i]) {
                let mut q = h.entry(i).or_insert(0);
                *q += 1;
            }
        }
    }
    println!("{:?}", h);
}


fn compare<T>(slice1: &[T], slice2: &[T]) -> bool
where T: PartialEq,
{
    if slice1.len() != slice2.len() {
        return false
    }
    else {
        for i in 0..slice1.len() {
            if slice1[i] != slice2[i] {
                return false
            }
        }
    }
    true
}

