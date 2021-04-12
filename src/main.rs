use std::io;
use opencv::{
    core,
    highgui,
    prelude::*,
    videoio,
};

fn its_sub(){
    let mut numbers = String::new();

    println!("input a number");

    io::stdin().read_line(&mut numbers)
        .expect("Failed");

    println!("I am a sub and your input number is {}", numbers);
}

fn main() {
    println!("Hello, world!");
    its_sub();
}
