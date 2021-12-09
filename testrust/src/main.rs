#![warn(clippy::all, clippy::pedantic)]
fn main() {
    let my_list = ["One", "Two", "Three"];
    for item in &my_list{
        println!("{}", item);
    }
    println!("Hello, world!");
    println!("こんにちは世界");
}
