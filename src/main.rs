use std::io;
use std::process;
mod game;
mod players;
fn main() {
    loop{
        let mut buff = String::new();
        println!("\t\t\t**** GAME 2048 ****\n");
        println!("\t\t\t1. New Game");
        println!("\t\t\t2. Continue");
        println!("\t\t\t3. Leader Board");
        println!("\t\t\t4. Exit");
        io::stdin().read_line(&mut buff).expect("Failed to read the option");
        let option = buff.trim().parse::<i32>().unwrap();
        match option {
            1 => game::start_game(),
            2 => game::continue_game(),
            3 => players::display_leader_board(),
            4 => process::exit(0),
            _ => println!("Please choose among the available options")
    };
    }
    


}
