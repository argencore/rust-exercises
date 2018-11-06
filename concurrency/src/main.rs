use std::thread;
use std::time::Duration;
fn main() {
    let value = 14;
    let handle = thread::spawn(move ||{
                  for i in 1..10 {
                      println!("number from spawned thread...{}", i);
                      thread::sleep(Duration::from_millis(1));
                  }
        value
                  });
    for i in 1..5{
        println!("number from main thread...{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("{:?}",handle.join().unwrap());
}
