use std::io;


fn main() {
    println!("Hello, This demo is for taking user input.");
    let mut input = String::new();
    println!("Enter a string.....");
    io::stdin().read_line(& mut input).expect("Failed toread lines for user!");
    println!("{}",input);

}
