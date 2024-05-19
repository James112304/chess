
// abstract all bit operations away such that we can do left-to-right, 0 indexed

use std::fmt::Error;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard (pub u64);
impl BitBoard {
    pub fn new() -> Self {
        BitBoard(0)
    }

    pub fn set_bit(&mut self, position: usize) {
        let left_shift = 63 - position;
        self.0 |= 1 << left_shift;
    }

    pub fn is_bit_set(&self, position: usize) -> bool{
        let left_shift = 63 - position;
        (self.0 & (1 << left_shift)) != 0
    }

    pub fn unset_bit(&mut self, position: usize) {
        let left_shift = 63 - position;
        let mask: u64 = !(1 << left_shift);
        (self.0 & mask);
    }

}

pub fn loc_as_square(location: usize) -> Square {
    Square::new(location / 8, location % 8)
}

pub fn square_as_loc(square: Square) -> usize {
    square.j_pos * 8 + square.i_pos
}

pub struct Team;
impl Team {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Piece;
impl Piece {
    pub const ROOK: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const KING: usize = 3;
    pub const QUEEN: usize = 4;
    pub const PAWN: usize = 5;
}

pub struct Square {
    j_pos: usize,
    i_pos: usize,
} impl Square {
    pub fn new(j: usize, i: usize) -> Self {
        let j_pos = j;
        let i_pos = i;
        Square {j_pos, i_pos}
    }
}


pub struct Move {
    original_pos: Square,
    to_pos: Square,
}

pub struct Position {
    bb_sides: [BitBoard; 2],
    bb_pieces: [[BitBoard; 6]; 2]
} impl Position {
    pub fn new() -> Self {
        let mut bb_pieces = [[BitBoard::new(); 6]; 2];
        let mut bb_sides = [BitBoard::new(); 2];
        bb_pieces[Team::WHITE][Piece::ROOK].0 |= (1) | (1 << 7);
        bb_pieces[Team::WHITE][Piece::BISHOP].0 |= (1 << 2) | (1 << 5);
        bb_pieces[Team::WHITE][Piece::KNIGHT].0 |= (1 << 1) | (1 << 6);
        bb_pieces[Team::WHITE][Piece::KING].0 |= 1 << 3;
        bb_pieces[Team::WHITE][Piece::QUEEN].0 |= 1 << 4;
        bb_pieces[Team::WHITE][Piece::PAWN].0 |= (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 12) | (1 << 13) | (1 << 14) | (1 << 15);
        bb_sides[Team::WHITE].0 |= 
        bb_pieces[Team::WHITE][Piece::ROOK].0 | 
        bb_pieces[Team::WHITE][Piece::BISHOP].0 | 
        bb_pieces[Team::WHITE][Piece::KNIGHT].0 | 
        bb_pieces[Team::WHITE][Piece::KING].0 | 
        bb_pieces[Team::WHITE][Piece::QUEEN].0 | 
        bb_pieces[Team::WHITE][Piece::PAWN].0;

        bb_pieces[Team::BLACK][Piece::ROOK].0 |= (1 << 63) | (1 << 56);
        bb_pieces[Team::BLACK][Piece::BISHOP].0 |= (1 << 61) | (1 << 58);
        bb_pieces[Team::BLACK][Piece::KNIGHT].0 |= (1 << 62) | (1 << 57);
        bb_pieces[Team::BLACK][Piece::KING].0 |= 1 << 59;
        bb_pieces[Team::BLACK][Piece::QUEEN].0 |= 1 << 60;
        bb_pieces[Team::BLACK][Piece::PAWN].0 |= (1 << 55) | (1 << 54) | (1 << 53) | (1 << 52) | (1 << 51) | (1 << 50) | (1 << 49) | (1 << 48);
        bb_sides[Team::BLACK].0 |= 
        bb_pieces[Team::BLACK][Piece::ROOK].0 | 
        bb_pieces[Team::BLACK][Piece::BISHOP].0 | 
        bb_pieces[Team::BLACK][Piece::KNIGHT].0 | 
        bb_pieces[Team::BLACK][Piece::KING].0 | 
        bb_pieces[Team::BLACK][Piece::QUEEN].0 | 
        bb_pieces[Team::BLACK][Piece::PAWN].0;
        Self {bb_sides, bb_pieces}
    }

    pub fn display(&self) {
        let mut rep: [[i8; 8]; 8] = [[0; 8]; 8];
        for team in 0..2 {
            for i in 0..6 {
                for num in 0..64 {
                    if self.bb_pieces[team][i].is_bit_set(num) {
                        rep[num / 8][num % 8] = ((i + 1) + team * 6) as i8
                    }
                }
            }
        }
        
        for j in 0..8 {
            let mut s: String = String::new();
            for i in 0..8 {
                match rep[j][i] {
                    0 => s.push(' '),
                    1 => s.push('R'),
                    2 => s.push('B'),
                    3 => s.push('N'),
                    4 => s.push('K'),
                    5 => s.push('Q'),
                    6 => s.push('P'),
                    7 => s.push('R'),
                    8 => s.push('B'),
                    9 => s.push('N'),
                    10 => s.push('K'),
                    11 => s.push('Q'),
                    12 => s.push('P'),
                    _ => panic!("how")
                }
            }
            println!("{}", s);
        }

    }

    pub fn exists_piece_at_location(&self, team: usize, location: usize) -> bool {
        return self.bb_sides[team].is_bit_set(location);
    }

    pub fn get_piece_at_location(&self, team: usize, location: usize) -> Option<usize> {
        if self.bb_pieces[team][Piece::ROOK].is_bit_set(location) {
            return Some(Piece::ROOK)
        } else if (self.bb_pieces[team][Piece::BISHOP].is_bit_set(location)) {
            return Some(Piece::BISHOP)
        } else if (self.bb_pieces[team][Piece::QUEEN].is_bit_set(location)) {
            return Some(Piece::QUEEN)
        } else if (self.bb_pieces[team][Piece::KING].is_bit_set(location)) {
            return Some(Piece::KING)
        } else if (self.bb_pieces[team][Piece::KNIGHT].is_bit_set(location)) {
            return Some(Piece::KNIGHT)
        } else if (self.bb_pieces[team][Piece::PAWN].is_bit_set(location)) {
            return Some(Piece::PAWN)
        } else {
            return None
        }
    }



    pub fn prune_invalid_moves(&self) {
        //todo
    }
    // first determine moves, then prune invalid ones 
    pub fn get_pawn_moves(&self, team: usize, location: usize) -> Option<Vec<Square>> {
        if location < 0 || location >= 64 {
            panic!("invalid input")
        }

        let mut can_two = false;
        let moves: Vec<Square> = Vec::new();
        if team == Team::WHITE {
            None
        } else {
            None
        }
    }


}

pub struct Chess {
    current_state: Position,
    current_turn: usize,
} impl Chess {
    pub fn new() -> Self {
        Self{current_state: Position::new(), current_turn: Team::WHITE}
    }

    pub fn get_possible_moves_piece(&self, location: usize) -> Result<Option<Vec<Square>>, Error> {
        if !(self.current_state.exists_piece_at_location(self.current_turn, location)) {
            return Ok(None)
        } else {
            match self.current_state.get_piece_at_location(self.current_turn, location) {

            }
        }

        return Ok(None)
    }
}