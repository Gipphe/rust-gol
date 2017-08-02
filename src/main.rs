fn count_neighbors(board: &Vec<Vec<bool>>, cell_y: i32, cell_x: i32) -> i32 {
    let mut count = 0;
    for y in -1..2 {
        let real_y: usize = (cell_y + y) as usize;
        let row: Option<&Vec<bool>> = board.get(real_y);
        match row {
            None => (),
            Some(ref row) => {

                for x in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let real_x: usize = (cell_x + x) as usize;
                    let neighbor: Option<&bool> = row.get(real_x);

                    match neighbor {
                        None => {
                            ()
                        },
                        Some(neighbor) => {
                            if *neighbor {
                                count += 1;
                            }
                            ()
                        }
                    }
                }
            }
        }
    }
    count
}
fn step(old_gen: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next_gen: Vec<Vec<bool>> = old_gen.clone();
    for y in 0..old_gen.len() {
        let row: &Vec<bool> = &old_gen[y];
        for x in 0..row.len() {
            let cell: bool = row[x];

            let neighbors: i32 = count_neighbors(&old_gen, y as i32, x as i32);
            let is_alive: bool = new_state(cell, neighbors);

            next_gen[y][x] = is_alive;
        }
    }

    next_gen
}

fn new_state(current_state: bool, neighbor_count: i32) -> bool {
    if current_state {
        if neighbor_count == 2 || neighbor_count == 3 {
            return true;
        }
        return false;
    }
    if neighbor_count == 3 {
        return true;
    }
    return false;
}

fn main() {
    let board: Vec<Vec<bool>> = vec![vec![true; 20]; 20];
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board() -> Vec<Vec<bool>> {
        vec![vec![false; 20]; 20]
    }
    fn make_alive_board() -> Vec<Vec<bool>> {
        vec![vec![true; 20]; 20]
    }
    #[test]
    fn counts_no_alive_cells_for_dead_board() {
        let board: Vec<Vec<bool>> = make_board();
        let mut result: i32 = count_neighbors(&board, 0, 0);
        assert_eq!(0, result);
        result = count_neighbors(&board, 10, 10);
        assert_eq!(0, result);
    }
    #[test]
    fn counts_correct_alive_neighbors_for_corner_cells() {
        let mut board: Vec<Vec<bool>> = make_board();
        board[0][1] = true;
        board[1][0] = true;
        let result: i32 = count_neighbors(&board, 0, 0);
        assert_eq!(2, result);
    }
    #[test]
    fn counts_correct_alive_neighbors_for_edge_cells() {
        let mut board: Vec<Vec<bool>> = make_board();
        board[0][1] = true;
        board[1][2] = true;
        board[1][3] = true;
        let result: i32 = count_neighbors(&board, 0, 2);
        assert_eq!(3, result);
    }
    #[test]
    fn counts_correct_alive_neighbors_for_far_corner_cell() {
        let mut board: Vec<Vec<bool>> = make_board();
        board[19][18] = true;
        board[18][19] = true;
        let result: i32 = count_neighbors(&board, 19, 19);
        assert_eq!(2, result);
    }
    #[test]
    fn counts_max_number_of_neighbors_for_alive_board() {
        let board: Vec<Vec<bool>> = make_alive_board();
        let result: i32 = count_neighbors(&board, 10, 10);
        assert_eq!(8, result);
    }
    #[test]
    fn counts_correct_max_neighbors_for_corner_cell() {
        let board: Vec<Vec<bool>> = make_alive_board();
        let result: i32 = count_neighbors(&board, 0, 0);
        assert_eq!(3, result);
    }
    #[test]
    fn counts_correct_max_neighbors_for_far_corner_cell() {
        let board: Vec<Vec<bool>> = make_alive_board();
        let result: i32 = count_neighbors(&board, 19, 19);
        assert_eq!(3, result);
    }
    #[test]
    fn new_state_alive_cell_dies() {
        assert_eq!(false, new_state(true, 1));
        assert_eq!(false, new_state(true, 4));
    }
    #[test]
    fn new_state_alive_cell_lives() {
        assert_eq!(true, new_state(true, 2));
        assert_eq!(true, new_state(true, 3));
    }
    #[test]
    fn new_state_dead_cell_stays_dead() {
        assert_eq!(false, new_state(false, 0));
        assert_eq!(false, new_state(false, 2));
    }
    #[test]
    fn new_state_dead_cell_revives() {
        assert_eq!(true, new_state(false, 3));
    }
    #[test]
    fn step_iterates_the_board() {
        let board: Vec<Vec<bool>> = vec![
            vec![false, false, false, false],
            vec![true, true, false, false],
            vec![false, true, false, false],
            vec![false, false, false, false]
        ];
        let next_gen: Vec<Vec<bool>> = step(&board);

        assert_eq!(true, next_gen[1][1]);
        assert_eq!(false, next_gen[0][1]);
        assert_eq!(true, next_gen[1][0]);
        assert_eq!(true, next_gen[2][0]);
        assert_eq!(false, next_gen[1][2]);
    }
}
