use std::cmp::Ordering;

use coords::Coords;
use data::Data;
use grid::Grid;
use rand::seq::SliceRandom;
use rules::Rules;
use tile::Tile;

pub mod coords;
pub mod data;
pub mod grid;
pub mod rules;
pub mod tile;


pub fn collapse<const N: usize, T: Tile<N>, C: Coords, R: Rules<N, C>>(grid: &mut Grid, tiles: &[T], rebuild_data: C::RebuildData) {
    let mut smallest = Vec::new();
    for i in 0..grid.len() {
        if let Data::Options(ops) = &grid[i] {
            if smallest.is_empty() {
                smallest.push(i);
            } else {
                let curr_len = ops.len();
                let my_len = ops.len();
                match curr_len.cmp(&my_len) {
                    Ordering::Equal => smallest.push(i),
                    Ordering::Greater => {
                        smallest.clear();
                        smallest.push(i)
                    }
                    Ordering::Less => (),
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    if let Some(&slot_pick) = smallest.choose(&mut rng) {
        if let Data::Options(options) = &grid[slot_pick] {
            if let Some(&option_pick) = options.choose(&mut rng) {
                grid[slot_pick] = Data::Collapsed(option_pick);
                for (i, neighbour) in R::get_neighbours(&C::from_index(slot_pick, rebuild_data))
                    .into_iter()
                    .enumerate()
                    .flat_map(|(i, x)| x.map(|x| (i, x)))
                {
                    if let Data::Options(options) = &mut grid[&neighbour] {
                        *options = tiles[option_pick].allowed_connections(tiles.iter())[i].iter().filter(|x| options.contains(x)).copied().collect();
                    }
                }
            } else {
                grid[slot_pick] = Data::Error
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
