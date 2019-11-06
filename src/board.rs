use crate::turn::Turn;
use std::fmt;

enum Directions{
     NW,
     N,
     NE,
     W,
     E,
     SW,
     S,
     SE,
}

const Direction: [Directions; 8] = [
    Directions::NW,
    Directions::N,
    Directions::NE,
    Directions::W,
    Directions::E,
    Directions::SW,
    Directions::S,
    Directions::SE,
];
    
#[derive(Debug, Clone)]
pub struct Board {
    black: u64,
    white: u64,
}

impl Board {
    pub fn new() ->  Self {
        let mut board = Board {
            black: 0x0000000810000000,
            white: 0x0000001008000000,
        };
        return board
    }
    
    pub fn LegalBoard(&self, turn:&Turn) -> u64{
        let mut me;
        let mut opponent;
        match turn {
            Turn::black => {me = self.black; opponent = self.white;},
            Turn::white => {me = self.white; opponent = self.black;},
        }

        let side_mask =  opponent & 0x7e7e7e7e7e7e7e7e;
        let vertuical_mask =  opponent & 0x00FFFFFFFFFFFF00;
        let diagonal_mask =  opponent & 0x007e7e7e7e7e7e00;
        let empty_mask =  !(me | opponent);
        let mut tmp:u64; 
        let mut legalboard = 0x0000000000000000;
        
        tmp = side_mask & (me << 1);
        for x in 0..5 {
            tmp |= side_mask & (tmp << 1);
        }
        legalboard |= empty_mask & (tmp << 1);

        tmp = side_mask & (me >> 1);
        for x in 0..5 {
            tmp |= side_mask & (tmp >> 1);
        }
        legalboard |= empty_mask & (tmp >> 1);

        tmp = side_mask & (me << 8);
        for x in 0..5 {
            tmp |= side_mask & (tmp << 8);
        }
        legalboard |= empty_mask & (tmp << 8);

        tmp = side_mask & (me >> 8);
        for x in 0..5 {
            tmp |= side_mask & (tmp >> 8);
        }
        legalboard |= empty_mask & (tmp >> 8);

        tmp = side_mask & (me << 9);
        for x in 0..5 {
            tmp |= side_mask & (tmp << 9);
        }
        legalboard |= empty_mask & (tmp << 9);

        tmp = side_mask & (me << 7);
        for x in 0..5 {
            tmp |= side_mask & (tmp << 7);
        }
        legalboard |= empty_mask & (tmp << 7);

        tmp = side_mask & (me >> 9);
        for x in 0..5 {
            tmp |= side_mask & (tmp >> 9);
        }
        legalboard |= empty_mask & (tmp >> 9);

        tmp = side_mask & (me >> 7);
        for x in 0..5 {
            tmp |= side_mask & (tmp >> 7);
        }
        legalboard |= empty_mask & (tmp >> 7);

        legalboard
    }

    pub fn do_flip(&mut self, pos:u64, turn:&Turn){
        let mut me;
        let mut opponent;
        match turn {
            Turn::black => {me = self.black; opponent = self.white;},
            Turn::white => {me = self.white; opponent = self.black;},
        }
        let mut mask :u64;
        let mut tmp = 0;
        let mut rev = 0;
        
        mask = 0;
        
        for i in Direction.iter() {

            tmp = 0;
            mask = self.trancefar(pos, i);
            while (mask != 0) && ((mask & opponent) != 0) {
                tmp |= mask;
                mask = self.trancefar(mask, i);
            }

            if (mask & me) != 0 {
                rev |= tmp
            }
        }

        me ^= rev | pos;
        opponent ^= rev;

        match turn {
            Turn::black => {self.black = me;  self.white = opponent;},
            Turn::white => {self.white = me;  self.black = opponent;},
        }
    }

    fn trancefar(&self, pos: u64, dir: &Directions ) -> u64 {

        match dir{
            Directions::NW => { (pos << 9) &  0xfefefefefefefe00},
            Directions::N  => { (pos << 8) &  0xffffffffffffff00},
            Directions::NE => { (pos << 7) &  0x7f7f7f7f7f7f7f00},
            Directions::W  => { (pos << 1) &  0xfefefefefefefefe},
            Directions::E  => { (pos >> 1) &  0x7f7f7f7f7f7f7f7f},
            Directions::SW => { (pos >> 7) &  0x00fefefefefefefe},
            Directions::S  => { (pos >> 8) &  0x00ffffffffffffff},
            Directions::SE => { (pos >> 9) &  0x007f7f7f7f7f7f7f},
            _ => 0,
        }
    }
    pub fn get_point(&self,turn:&Turn) -> i8{
        match turn {
            Turn::black => self.black.count_ones() as i8,
            Turn::white => self.white.count_ones() as i8,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut num = 0;
        let mut mask :u64 ;
        mask = 0x8000000000000000;

        write!(f,"  a b c d e f g h");

        while mask != 0{
            if (0x8080808080808080 & mask) != 0 {
                let row = 1 + num;
                write!(f,"\n{} ",row);
                num += 1;
            }
            if self.black & mask != 0 {
                write!(f,"⚫︎");
            }
            else if self.white & mask != 0 {
                write!(f,"⚪︎");
            }
            else {
                write!(f,"・");
            }

            mask >>= 1;
        }
        write!(f, "")
    }
}
