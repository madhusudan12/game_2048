
use std::time::Duration;

enum GameState{
    win,
    loose,
    playing,
}

struct Player{
    name: String,
    moves: i32,
}


struct Board{
    size: i32,
    table: Vec<Vec<i32>>,
    empty_times: i32,
}

struct Game{
    player: Player,
    board: Board,
    state: GameState,
    time: Duration,
   
}

pub fn start_game(){

}