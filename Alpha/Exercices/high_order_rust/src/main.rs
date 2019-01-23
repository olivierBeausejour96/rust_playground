extern crate high_order_rust;

use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::*;


struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where   T: Fn(u32) -> u32,
 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        println!("{} ___ {:?}", arg, self.value);
        match self.value.entry(arg) {
            Entry::Vacant(_) => *self.value.entry(arg).or_insert((self.calculation)(arg)),
            Entry::Occupied(v) => *v.get()
        }

    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly.....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} pushups!",
            expensive_result.value(intensity-1)
        );
        println!(
            "Next, do {} jumps!",
            expensive_result.value(intensity-1)
        );
        
        println!(
            "Next, do {} roll over!",
            expensive_result.value(intensity-1)
        );
    }
    else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hydrated!");
        } 
        else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout (
        simulated_user_specified_value,
        simulated_random_number
    );
}