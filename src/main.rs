fn count_neighbors(board: &Vec<Vec<bool>>, cell_y: i32, cell_x: i32) -> i32 {
    let mut count = 0;
    for y in -1..2 {
        let real_y: usize = (cell_y + y) as usize;
        let row: Option<&Vec<bool>> = board.get(real_y);
        match row {
            None => (),
            Some(ref row) => {

                for x in -1..2 {
                    if x == y {
                        continue;
                    }
                    let real_x: usize = (cell_x + x) as usize;
                    let neighbor: Option<&bool> = row.get(real_x);

                    match neighbor {
                        None => (),
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

fn main() {
    let mut board: Vec<Vec<bool>> = vec![vec![false; 20]; 20];
    board[0][0] = true;
    board[0][2] = true;
    let count = count_neighbors(&board, 0, 1);
    (-1..2).iter().fo
    println!("{:?}", count);
}
