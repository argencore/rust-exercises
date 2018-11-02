#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(num: i32) -> i32{
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{length: 8, width: 7};
        let smaller = Rectangle{length:5, width: 1};
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{length: 8, width: 7};
        let smaller = Rectangle{length:5, width: 1};
        assert!(!smaller.can_hold(&larger));

    }
    #[test]
    fn two_plus_two(){
        let x = 2;
        assert_eq!(4,add_two(x));
    }
}


