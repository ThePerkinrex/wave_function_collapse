pub trait Coords {
    /// Necessary data to rebuild the coords from an index
    type RebuildData;
    fn as_index(&self) -> usize;
    fn from_index(i: usize, data: Self::RebuildData) -> Self;
}

impl Coords for usize {
    type RebuildData = ();

    fn as_index(&self) -> usize {
        *self
    }

    fn from_index(i: usize, _: Self::RebuildData) -> Self {
        i
    }

    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords2D {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Coords2D {
    pub const fn new(width: usize, height: usize, x: usize, y: usize) -> Self {
        Self { width, height, x, y }
    }
    pub const fn x(&self) -> usize {
        self.x
    }
    pub const fn y(&self) -> usize {
        self.y
    }
    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
    
}

impl Coords for Coords2D {
    /// Width
    type RebuildData = (usize, usize);

    fn as_index(&self) -> usize {
        self.y * self.width + self.x
    }

    fn from_index(i: usize, (width, height): Self::RebuildData) -> Self {
        Self {
            width,
            height,
            x: i % width,
            y: i / width,
        }
    }
}
