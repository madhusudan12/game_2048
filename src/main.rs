use std::io;
mod game;
fn main() {
    let mut buff = String::new();
    println!("\t\t\t**** GAME 2048 ****\n");
    println!("\t\t\t1. New Game");
    println!("\t\t\t2. Continue");
    println!("\t\t\t3. Leader Board");
    println!("\t\t\t4. Exit");
    io::stdin().read_line(&mut buff).expect("Failed to read the option");
    let mut option = buff.trim().parse::<i32>().unwrap();
    println!("{}", option);
    


}
