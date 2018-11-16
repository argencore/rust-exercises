fn main() {
    let v1 = vec![1,2,3,4];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    for val in v2{
        println!("got:{}",val);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe>{
    shoes.into_iter()
        .filter(|shoe|shoe.size == size)
        .collect()
}

#[test]
fn filters_by_size(){
    let shoes = vec![
        Shoe{size:10,style:String::from("sneeker")},
        Shoe{size:12,style:String::from("high heel")},
        Shoe{size:10,style:String::from("boot")},
        Shoe{size:8,style:String::from("sandal")},
    ];

    let in_my_size = shoes_in_my_size(shoes,10);
    assert_eq!(in_my_size, vec![Shoe{size:10,style:String::from("sneeker")}, Shoe{size:10,style:String::from("boot")}])

}

struct Counter{
    count: u32,
}

impl Counter{
    fn new() -> Counter{
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        self.count += 1; 

        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}

#[test]
fn counter_only_counts_to_five(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}

#[test]
fn using_other_iterator_methods(){
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18,sum);
}


