use std::ops::Deref;
use super::piece::Piece;


pub trait Tile: Send {
    fn get_piece(&self) -> Option<&dyn Piece>;
}

pub struct EmptyTile;

impl Tile for EmptyTile {
    fn get_piece(&self) -> Option<&dyn Piece> {
       Option::None
    }
}

pub struct OccupiedTile {
    piece : Box<dyn Piece>
}

impl OccupiedTile {
    pub fn new( piece : Box<dyn Piece> ) -> OccupiedTile {
        OccupiedTile {
            piece : piece
        } 
    }
}

impl Tile for OccupiedTile {
    fn get_piece(&self) -> Option<&dyn Piece> {
       Option::Some(self.piece.deref())
    }
}
