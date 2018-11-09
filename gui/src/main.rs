extern crate gui;
use gui::Draw;
use gui::{Screen,Button};

fn main(){
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                lable: String::from("ok"),
            }),
        ],
    };
    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("drawing a selectBox");
    }
}