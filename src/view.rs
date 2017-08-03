pub fn display(board: &Vec<Vec<bool>>) {
	for y in 0..board.len() {
		let row: &[bool] = &board[y];
		for x in 0..row.len() {
			let is_alive: bool = row[x];

			if is_alive {
				print!("1");
			} else {
				print!("0");
			}
		}
		print!("\n");
	}
	println!("-------");
}
