use std::ops::Deref;

fn main() {
    let m = MyBox::new(String::from("world"));
    //super cool thing happens &m automatically becomes &T where T is content of &m
    hello(&m);
}

struct MyBox<T>(T);

impl<T>MyBox<T>{
    fn new(x: T)-> MyBox<T>{
       MyBox(x) 
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

fn hello(word: &str){
    println!("hello {}",word);
}
