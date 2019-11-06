use crate::board::Board;
use crate::turn::Turn;
use crate::player::Player;
use crate::alphabeta::AlphaBeta;

#[derive(Debug)]
pub struct Game<P1, P2> {
   board: Board,
   turn: Turn,
   player1: P1,
   player2: P2,
}

impl<P1, P2> Game<P1, P2> 
where
    P1: Play,
    P2: Play,
{

    pub fn new (p1: P1, p2: P2) -> Self {
        Game {
            board: Board::new(),
            turn: Turn::black,
            player1:p1,
            player2:p2,
         }
    }

    pub fn step(&mut self) -> bool {
        let legalboard = self.board.LegalBoard(&self.turn);

        if legalboard == 0 {
            let legalboard_opponent = self.board.LegalBoard(&self.turn.opponent());
            if legalboard_opponent == 0{
                return false
            }
            self.turn = self.turn.opponent();
            return true
        }

        let mut pos :u64;
        match self.turn {
            Turn::black => pos = self.player1.play(&self.turn, &self.board),
            Turn::white => pos = self.player2.play(&self.turn, &self.board),
        }

        self.board.do_flip(pos, &self.turn);

        self.turn = self.turn.opponent();
        return true
    }

    pub fn print(&self) {
        println!("{}",self.turn);
        println!("{}",self.board);
    }
}

pub trait Play {
    fn play(&self, turn: &Turn, board: &Board) -> u64;
}
