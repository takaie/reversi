use std::io::{Write, BufRead, BufReader,BufWriter};
use crate::turn::Turn;
use crate::board::Board;
use crate::game::Play;


#[derive(Debug)]
pub struct Player;

impl Play for Player{
    fn play(&self, turn:&Turn, board:&Board) -> u64 {

        
        let legalboard = board.LegalBoard(turn);
        let mut pos: u64;
        loop{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();

             match line.coordinatetobit() {
                Ok(result) => {pos = result},
                Err(msg) => {println!("{}",msg); continue;}
            }

            if ((legalboard & pos) != 0){
                return pos
            }else{
                println!("illegal");
            }
        }
    } 
}


pub trait CoordinateToBit {
    fn coordinatetobit(&self) -> Result<u64, &'static str>;
}

impl CoordinateToBit for String {
    fn coordinatetobit(&self) -> Result<u64, &'static str> {
        let mut basebit = 0x8000000000000000;

        if self.trim().len() != 2{
            return Err("plz 2 chars")
        }

        match self.chars().nth(0).unwrap() {
            'a' => basebit >>= 0,
            'b' => basebit >>= 1,
            'c' => basebit >>= 2,
            'd' => basebit >>= 3,
            'e' => basebit >>= 4,
            'f' => basebit >>= 5,
            'g' => basebit >>= 6,
            'h' => basebit >>= 7,
            _   => return Err("only a~h"),
        };

        match self.chars().nth(1).unwrap() {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                basebit >>= ( (self.chars().nth(1).unwrap() as i32 -48 - 1) * 8);
                return Ok(basebit);
            },
            _ => return Err("only 1~9"),
        }
    }
}

