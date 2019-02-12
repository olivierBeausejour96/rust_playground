use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    barber_shop();
    //deadlock();
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

fn barber_shop() {
    println!("Instantiating barber shop");
    //let (tx, rx) = mpsc::channel();
    let mut seats = Arc::new(Mutex::new(vec![]));

    {
        let seats = Arc::clone(&seats);
        let barber = thread::spawn(move || {
            loop {
                
                let seat_count = {
                    seats.lock().unwrap().len()
                };
                if seat_count > 0 {
                    println!("time to do some haircuts");
                    thread::sleep(Duration::from_secs(1));
                }
                else {
                    println!("no clients... zzzzzz");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });
    }

    thread::sleep(Duration::from_secs(1));
    {
        let seats = Arc::clone(&seats);
        let client = thread::spawn(move || {
            println!("Spawning client");
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        });
        seats.lock().unwrap().push(client);
        println!("hello?");
    }

    {
        let mut seats = seats.lock().unwrap();
        for s in 0..seats.len() {
            &seats[s].join();
        }
    }
    println!("Done with barber shop!");
}