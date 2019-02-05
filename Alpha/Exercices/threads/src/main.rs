use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    deadlock();
    //message_passing();
}


fn deadlock() {
    
    let counter = Arc::new(Mutex::new(0));
    let m1 = Arc::new(Mutex::new(0));
    let m2 = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    {
        let counter = Arc::clone(&counter);
        let m1 = Arc::clone(&m1);
        let m2 = Arc::clone(&m2);
        let handle = thread::spawn(move || {
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            {
                println!("60% of the time, it creates a deadlock All the time!");
                let m1 = m1.lock().unwrap();
                thread::sleep(Duration::from_secs(1));
                {
                    let m2 = m2.lock().unwrap();
                }
            }
        });   
        handles.push(handle);
    }
    {
        let counter = Arc::clone(&counter);
        let m1 = Arc::clone(&m1);
        let m2 = Arc::clone(&m2);
        let handle = thread::spawn(move || {
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            {
                println!("That makes no sense...");
                let m2 = m2.lock().unwrap();
                thread::sleep(Duration::from_secs(1));
                {
                    let m1 = m1.lock().unwrap();
                }
            }
        });   
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join();  
    }

    println!("Result: {}", *counter.lock().unwrap());
}


fn message_passing() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
        String::from("Hi"),
        String::from("from"),
        String::from("alien"),
        String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
        String::from("more"),
        String::from("from"),
        String::from("aliens"),
        String::from("sssssssssssssss"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}