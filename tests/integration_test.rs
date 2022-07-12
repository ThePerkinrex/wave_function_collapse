use wave_function_collapse::{tile::{Simple2DTile, CacheTile}, grid::Grid, collapse, coords::Coords2D, rules::Rules2D, data::Data};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

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

#[test]
fn integration_test() {
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

	for i in 0..WIDTH * HEIGHT {
		assert!(matches!(grid[i], Data::Collapsed(_) | Data::Error), "grid[{i}] not collapsed: {:?}", grid[i])
	}
}