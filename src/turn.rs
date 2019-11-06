use std::fmt;

#[derive(Debug)]
pub enum Turn{
    black,
    white,
}

impl Turn{
    pub fn opponent(&self) -> Self {
        match *self {
            Turn::black => Turn::white,
            Turn::white => Turn::black,
        }
    }
}

impl fmt::Display for Turn{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Turn::black => write!(f,"Turn: black"),
            Turn::white => write!(f,"Turn: white"),
        }
        //write!(f, "")
    }
}
