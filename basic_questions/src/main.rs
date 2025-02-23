use std::io;

fn str_to_int(str: &String)-> Result<i32,String>{
    str.trim().parse::<i32>().map_err(|_| "Invalid Input, Enter a valid number".to_string())
}

fn main() {
    let mut input = String::new();
    println!("Enter a number");
    io::stdin().read_line(& mut input).expect("Failed to read line");

    match str_to_int(&input) {
        Ok(num) => {
            match num%2==0 {
                true => println!("{} is even",num),
                false => println!("{} is odd",num),
            }
        },
        Err(e) => {println!("{e}")},
    }
}
