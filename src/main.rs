mod player;
mod alphabeta;
mod turn;
mod board;
mod game;


fn main() {
    //let p1 = alphabeta::AlphaBeta::new(7);
    let p1 = player::Player;
    let p2 = alphabeta::AlphaBeta::new(7);
    //let p2 = alphabeta::AlphaBeta::new(7);

    let mut game = game::Game::new(p1, p2);
        loop{
            game.print();
            if !game.step(){
                break
            }
        }
}
