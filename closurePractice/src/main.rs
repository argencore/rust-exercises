use std::thread;
use std::collections::HashMap;
use std::time::Duration;


fn main() {
    let simulated_intensity = 10;
    let simulated_random = 5;

    generate_workout(simulated_intensity,simulated_random);

}

//TODO: change value to a HashMap so that unique values will trigger the closure
/*
Struct name: Catcher_U32
arguments: generic T where T is a closure mapping u32 -> u32
description: a struct used to catch the result of a closure that
can be used in multiple places to reduce the amount of times the
function needs to be run.
*/
struct CatcherU32<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32,u32>
}

impl<T> CatcherU32<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> CatcherU32<T>{
        CatcherU32{
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        if self.value.contains_key(&arg){
           *self.value.get(&arg).unwrap()
        }else{
            let v = (self.calculation)(arg);
            self.value.insert(arg,v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = CatcherU32::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} leg lifts!",
            expensive_result.value(intensity * intensity)
        );
        println!(
            "Next, do {} stretches!",
            expensive_result.value(intensity)
        );

    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
