use std::{
    hash::Hash, cell::RefCell, ops::Deref,
};

pub trait Tile<const NEIGHBOURS: usize>: Sized {
    fn allowed_connections<'a, I: Iterator<Item = &'a Self>>(
        &self,
        tiles: I,
    ) -> [Vec<usize>; NEIGHBOURS]
    where
        Self: 'a;
}

#[derive(Debug, Clone, Hash)]
/// 2D tile with 4 connections [up, right, down, left]
pub struct Simple2DTile<Data, Connection: PartialEq> {
    pub data: Data,
    pub connections: [Connection; 4],
}

impl<Data, Connection: PartialEq> Tile<4> for Simple2DTile<Data, Connection> {
    fn allowed_connections<'a, I: Iterator<Item = &'a Self>>(&self, tiles: I) -> [Vec<usize>; 4]
    where
        Self: 'a,
    {
        let mut res: [Vec<usize>; 4] = Default::default();
        let tiles = tiles.collect::<Vec<_>>();
        for (i, res) in res.iter_mut().enumerate() {
            let conn = &self.connections[i];
            let opposite = (i + 2) % 4;
            let allowed = tiles
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    if &x.connections[opposite] == conn {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            *res = allowed;
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct CacheTile<const N: usize, T> {
    inner: T,
    last: RefCell<Option<[Vec<usize>; N]>>,
}

impl<const N: usize, T> CacheTile<N, T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner, last: RefCell::new(None)
        }
    }
}

impl<const N: usize, T> Deref for CacheTile<N, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize, T> Tile<N> for CacheTile<N, T>
where
    T: Tile<N>,
{
    fn allowed_connections<'a, I: Iterator<Item = &'a Self>>(&self, tiles: I) -> [Vec<usize>; N]
    where
        Self: 'a,
    {
        let mut mut_borrow = self.last.borrow_mut();
        let v = (*mut_borrow).get_or_insert_with(|| {
            self.inner.allowed_connections(tiles.map(|x| &x.inner))
            
        });
        v.clone()
    }
}
