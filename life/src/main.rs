fn main() {
    let mut y = make_test_board();

    make_00_live(&mut y);
    for n in 0..3 {
        for m in 0..3 {
            println!("Array Index: [{}, {}], Array Value: {}", n, m, y[n][m]);
        }
    }
}

fn board_mapper(board: [[i32;1]; 1]) -> [[i32;1]; 1] {
  let mut ret_board = make_single_cell_board();
//  board[0][0] = 0;
  board
}

fn make_00_live(board: &mut [[i32;3]; 3]) -> [[i32;3]; 3] {
  board[0][0] = 1;
  *board 
}

fn make_test_board() -> [[i32;3]; 3] {
  let y: [[i32;3]; 3] = [[0,1,1],
   [1,0,0],
   [1,0,1]];
  y
}

fn make_single_cell_board() -> [[i32;1]; 1] {
  let y: [[i32;1]; 1] = [[0]];
  y
}

#[test]
fn single_cell_board_dies() {
  let init_board = make_single_cell_board();
  let final_board = board_mapper(init_board);
//  assert_eq!(make_single_cell_board, final_board);
}

#[test]
fn board_exists() {
  let b = make_test_board();
  assert_eq!(b[0][0], 0);
}

#[test]
fn it_works() { 
 assert!(true);
}



#[test]
fn mapper_test() {
}

fn live_or_die_mapper() -> Vec<i32> {
  let grid = vec![0,0,0];
  let ret_grid = grid.iter().map(|&x| x + 1);  
  grid
}







