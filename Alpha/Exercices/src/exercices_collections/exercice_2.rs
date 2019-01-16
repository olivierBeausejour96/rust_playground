use std::collections;
use std::io;

pub fn execute() {
    println!("Executed exercice_2.rs!");
    println!("Pig Latin!");
    println!("Input some text to be traducted into pig latin!");

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("Unexpected program error upon taking user inputs. Halting program");
        return;
    }

    let mut ret = String::new();

    for elem in input.split_whitespace() {
        let mut iter = elem.chars();
        let ay = match iter.next() {
            Some(c) => { 
                let c = c.to_ascii_lowercase();
                if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' && c != 'y' {
                   format!("{}{}",c, "ay")
                }
                else {
                    String::from("hay")
                }
            },
            None    => panic!("Should never happen"),
        };

        for c in iter {
            ret.push(c);
        }
        ret.push_str(&ay);
        ret.push(' ');
    }

    println!("Traducted: {}", ret);
}