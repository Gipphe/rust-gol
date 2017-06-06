use std::clone::Clone;

#[derive(Debug)]
pub struct Cell {
	is_alive: bool,
}
pub trait Lives {
	fn set_life(&mut self) -> &mut Self;
	fn life_state(&self) -> bool;
}
impl Lives for Cell {
	fn set_life(&mut self) -> &mut Cell {
		self.is_alive;
		self
	}
	fn life_state(&self) -> bool {
		self.is_alive
	}
}
impl Clone for Cell {
	fn clone(&self) -> Cell {
		Cell { is_alive: self.is_alive }
	}
}

#[derive(Debug)]
pub struct Board {
	grid: Grid,
}
pub struct BoardBuilder {
	grid: Grid,
}
impl BoardBuilder {
	pub fn new() -> BoardBuilder {
		BoardBuilder {
			grid: GridBuilder::new().make()
		}
	}
	pub fn make(&self) -> Board {
		Board {
			grid: self.grid.clone(),
		}
	}
}
#[derive(Debug)]
struct Grid {
	grid: Vec<Vec<Cell>>,
}
impl Grid {
	fn change_cell(&mut self, y: usize, x: usize, new_cell: &Cell) -> &mut Grid {
		self.grid[y][x] = new_cell.clone();
		self
	}
}
impl Clone for Grid {
	fn clone(&self) -> Grid {
		Grid { grid: self.grid.clone() }
	}
}
struct GridBuilder {
	grid: Vec<Vec<Cell>>,
}
impl GridBuilder {
	fn new() -> GridBuilder {
		let mut vec = Vec::new();
		vec.push(Vec::new());
		GridBuilder { grid: vec }
	}
	fn make(&self) -> Grid {
		Grid { grid: self.grid.clone() }
	}
	fn add_row(&mut self) -> &mut GridBuilder {
		let col_count = self.grid[0].len();
		let row: Vec<Cell> = (0..col_count)
			.enumerate()
			.map(|_| Cell { is_alive: false })
			.collect();
		self.grid.push(row);
		self
	}
	fn add_col(&mut self) -> &mut GridBuilder {
		for mut row in &mut self.grid {
			row.push(Cell { is_alive: false });
		}
		self
	}
}
