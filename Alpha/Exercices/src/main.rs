extern crate Exercices;
fn main() {
    execute_fizzbuzz_ml();
}

fn execute_exercices() {
    Exercices::exercices_collections::exercice_1::execute();
    Exercices::exercices_collections::exercice_2::execute();
    Exercices::exercices_collections::exercice_3::execute();
    println!("Hello world!");
}

fn playground() {
    let number_list_1 = vec![34, 50, 25, 100, 65];

    let number_list_2 = vec![1, 5, 7, -3, 23, -3, 2];

    let char_list_1 = vec!['a', 'b', ' ', '\t'];

    println!("first list largest: {}", largest_i32(&number_list_1));
    println!("second list largest: {}", largest_i32(&number_list_2));
    println!("char list largest: {}", largest_char(&char_list_1));
}

fn largest_i32 (number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];
    for &n in number_list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn largest_char (char_list: &[char]) -> char {
    let mut largest = char_list[0];
    for &n in char_list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn largest<T> (list: &[T]) -> T 
where T: std::cmp::PartialOrd + Copy
{
    let mut largest = list[0];
    for &n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn traits_playground () {
    use Exercices::exercices_traits;
    exercices_traits::playground::execute();
}

fn execute_fizzbuzz_ml() {
    use Exercices::playground;
    println!("*** Executing fizzbuzz_ml ***");
    playground::fizzbuzz_ml::execute();
    println!("*** Done executing fizzbuzz_ml ***");
}