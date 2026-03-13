// use std::ffi::os_str::Display;

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Point {
//     fn origin() -> Point {
//         Point { x: 0, y: 0 }
//     }

//     fn new(x: i32, y: i32) -> Point {
//         Point { x: x, y: y }
//     }
// }

// impl Rectangle {
//     fn area(&self) -> i32 {
//         let Point { y: y1, x: x1 } = self.p1;
//         let Point { y: y2, x: x2 } = self.p2;

//         return ((x1 - x2) * (y1 - y2)).abs();
//     }
// }

// fn main() {
//     // let p1 = Point::origin();
//     // let p2 = Point::new(10, 20);

//     // println!("The origin is at ({}, {})", p1.x, p1.y);
//     // println!("The point is at ({}, {})", p2.x, p2.y);
//     // println!("Hello, world!");

//     let mut counter = 10;

//     loop {
//         println!("Bonjour");

//         counter -= 1;

//         if counter == 1 {
//             println!("Je sors de la boucle");
//             break;
//         }
//     }

//     for number in 0..10 {
//         println!("My number {}", number);
//     }
// }

// struct

use hello_world::{SocialPost, Summary};

// pub fn notify (item: &impl Summary){
//     println!("Une notification pour {} ", item.summarize_author());
// }

pub fn notify<T: Summary>(item: &T){
    println!("Une notification pour {} ", item.summarize_author());
}

fn main() {
    let triple = (0, -2, 3);
    let arr = [10, 20, -1, 40];

    println!("My triple {:?}", triple);
    println!("My array {:?}", arr);

    let post = SocialPost {
        content: String::from("J'ai vue Rushclin Taka "),
        headline: String::from("Hello Rushlin"),
        title: String::from("Takam"),
        author: String::from("Martin"),
    };

    println!("{}", post.summarize());
    println!("{}", post.summarize_author());
    notify(&post);

    // match triple {
    //     (0, x, z)=> println!("First is 0"),
    //     _ => println!("Don;t match any way")
    // }

    // match arr {
    //     [10, 20, y, z] => println!("Ahh, first number is 10"),
    //     _ => println!("Pas dans la liste")
    // }
}
