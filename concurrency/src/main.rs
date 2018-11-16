use std::sync::mpsc;
use std::sync::{Mutex,Arc};
use std::time::Duration;
use std::thread;

fn main() {

    //destructureing with a tuple
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move ||{
        let vals = vec![
            "hi",
            "from",
            "the",
            "thread",
        ];
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move ||{
        let vals = vec![
            "more",
            "from",
            "the",
            "other",
            "thread",
        ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    for recieved in rx{
        println!("got {}", recieved);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter); //this is shadowing the original
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}
