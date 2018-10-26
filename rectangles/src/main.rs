struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,rect: &Rectangle) -> bool{
         self.width >= rect.width && self.height >= rect.height 
    }
    fn square(size: u32)->Rectangle{
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let rect = Rectangle{width: 30, height: 50};
    let rect1 = Rectangle{width: 10, height: 12};
    let rect2 = Rectangle{width: 10, height: 12};
    let sq = Rectangle::square(10);
    println!("can rect1 fit in rect: {}", rect.can_hold(&rect1));
    println!("can rect1 fit in rect2: {}", rect2.can_hold(&rect1));
    println!("can rect fit in rect1: {}", rect1.can_hold(&rect));
}

