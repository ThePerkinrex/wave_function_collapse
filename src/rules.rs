use crate::coords::{Coords, Coords2D};

pub trait Rules<const NEIGHBOURS: usize, C: Coords> {
    fn get_neighbours(coords: &C) -> [Option<C>; NEIGHBOURS];
}

pub struct Rules2D;

impl Rules<4, Coords2D> for Rules2D {
    fn get_neighbours(coords: &Coords2D) -> [Option<Coords2D>; 4] {
        let x = coords.x();
        let y = coords.y();
        // dbg!(i, x, y);
        let up = y
            .checked_sub(1)
            .map(|y| Coords2D::new(coords.width(), coords.height(), x, y));
        let right = x
            .checked_add(1)
            .and_then(|x| if x < coords.width() { Some(x) } else { None })
            .map(|x| Coords2D::new(coords.width(), coords.height(), x, y));
        let down = y
            .checked_add(1)
            .and_then(|y| if y < coords.height() { Some(y) } else { None })
            .map(|y| Coords2D::new(coords.width(), coords.height(), x, y));
        let left = x
            .checked_sub(1)
            .map(|x| Coords2D::new(coords.width(), coords.height(), x, y));
        [up, right, down, left]
    }
}
