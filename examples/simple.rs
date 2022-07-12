use wave_function_collapse::{tile::{Simple2DTile, CacheTile}, grid::Grid, collapse, coords::Coords2D, rules::Rules2D};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn display_grid(grid: &Grid, tiles: &[CacheTile<4, Simple2DTile<char, bool>>]) {
	for row in 0..HEIGHT {
		for column in 0..WIDTH {
			match grid[row * WIDTH + column] {
				wave_function_collapse::data::Data::Error => print!("+"),
				wave_function_collapse::data::Data::Collapsed(n) => print!("{}", tiles[n].data),
				wave_function_collapse::data::Data::Options(_) => print!(" "),
			}
		}
		println!()
	}
}

fn main() {
	let tiles = &[
		CacheTile::new(Simple2DTile {
			data: '┴',
			connections: [true, true, false, true]
		}),
		CacheTile::new(Simple2DTile {
			data: '┬',
			connections: [false, true, true, true]
		}),
	];
	let mut grid = Grid::new(WIDTH * HEIGHT, &[0,1]);
	display_grid(&grid, tiles);
	println!();
	for _ in 0..WIDTH * HEIGHT {
		collapse::<4, _, Coords2D, Rules2D>(&mut grid, tiles, (WIDTH, HEIGHT));
		display_grid(&grid, tiles);
		println!();
	}
}