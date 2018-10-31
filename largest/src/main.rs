fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    println!("Hello, world!");
    let x = [1,2,3,4];
    let y = [2.2,3.4,1.5,7.2];
    println!("largest of x is: {}", largest(&x));
    println!("largest of y is: {}", largest(&y));

}
