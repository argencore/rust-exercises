//!This aproach comes at a cost as it causes the runtime to
//!decide what to do with the components Vec



//defines the trait and method that must be implemented
pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    pub fn run(&self){
        //get an iterator over the vector
        for component in self.components.iter(){
            //auto deref box to the component inside
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub lable: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("drawing a button");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
