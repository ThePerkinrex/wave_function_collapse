use std::ops::{Index, IndexMut, Deref, DerefMut};

use crate::{coords::Coords, data::Data};

pub struct Grid {
    grid: Vec<Data>,
}

impl Grid {
    pub fn new(grid_size: usize, default_options: &[usize]) -> Self {
        let mut grid = Vec::with_capacity(grid_size);
        for _ in 0..grid_size {
            grid.push(Data::Options(default_options.to_vec()))
        }
        Self { grid }
    }
}

impl<C: Coords> Index<&C> for Grid {
    type Output = Data;

    fn index(&self, index: &C) -> &Self::Output {
        self.grid.index(index.as_index())
    }
}

impl<C: Coords> IndexMut<&C> for Grid {
    fn index_mut(&mut self, index: &C) -> &mut Self::Output {
        self.grid.index_mut(index.as_index())
    }
}

impl Index<usize> for Grid {
    type Output = Data;

    fn index(&self, index: usize) -> &Self::Output {
        self.grid.index(index)
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.grid.index_mut(index)
    }
}

impl Deref for Grid {
    type Target = Vec<Data>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}