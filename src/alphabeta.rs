use std::cmp;
use crate::turn::Turn;
use crate::board::Board;
use crate::game::Play;

#[derive(Debug)]
pub struct AlphaBeta{
    depth: usize,
}

impl AlphaBeta{
    pub fn new(depth: usize) -> Self {
        AlphaBeta {depth}
    }


    fn evaluate(&self, turn:&Turn, board: &Board) -> i8 {
    
        board.get_point(turn) - board.get_point(&turn.opponent())
    }
    
    pub fn alphabeta(&self, turn: &Turn, board: &Board, mut a: i8, mut b: i8, depth: usize) -> (i8, u64) {
        if depth == 0 {
            return (self.evaluate(turn, board), 0);
        }
        
        let  legalboard = board.LegalBoard(turn);
        if legalboard == 0 {
            return (self.evaluate(turn ,&board), 0);
        }
        let mut best = (-127, 0);
        let mut mask = 0x8000000000000000;

        while mask != 0 {
            let pos = mask & legalboard;
            mask >>=1;

            if pos == 0 {
                continue;
            }

            let mut board = board.clone();
            board.do_flip(pos, turn);
            let (score, position)  = self.alphabeta(&turn.opponent(), &board, -b, -a, depth - 1);
            if -score > best.0 {
                best = (-score, pos);
            }
            a = cmp::max(a, -score);
            if a >= b {
                break;
            }
        }
        best
    }
}
       
impl Play for AlphaBeta{
    fn play(&self, turn:&Turn, board:&Board) -> u64{
        let best = self.alphabeta(turn, board, -127, 127, self.depth);
        best.1
    }
}
