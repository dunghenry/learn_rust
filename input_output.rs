use std::io;
fn main(){
    let mut name = String::new();
    println!("Enter str: ");
    io::stdin().read_line(&mut name).expect("Failed to readline");
    println!("Xin chao {}", name);
}