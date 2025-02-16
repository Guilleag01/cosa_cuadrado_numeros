use std::{collections::HashSet, fmt::Display};

use cosa_cuadrado_numeros::matrix::Matrix;

// const PRIME_TABLE: [[u128; 5]; 5] = [
//     [2, 3, 5, 7, 11],
//     [13, 17, 19, 23, 29],
//     [17, 37, 41, 43, 47],
//     [19, 59, 61, 67, 71],
//     [23, 79, 83, 89, 97],
// ];

// const PRIME_TABLE: [[u128; 5]; 5] = [
//     [1, 2, 3, 4, 5],
//     [6, 7, 8, 9, 10],
//     [11, 12, 13, 14, 15],
//     [16, 17, 18, 19, 20],
//     [21, 22, 23, 24, 25],
// ];

// const POSITIONS: [(usize, usize); 25] = [
//     (2, 2),
//     (2, 1),
//     (1, 1),
//     (1, 2),
//     (1, 3),
//     (2, 3),
//     (3, 3),
//     (3, 2),
//     (3, 1),
//     (3, 0),
//     (2, 0),
//     (1, 0),
//     (0, 0),
//     (0, 1),
//     (0, 2),
//     (0, 3),
//     (0, 4),
//     (1, 4),
//     (2, 4),
//     (3, 4),
//     (4, 4),
//     (4, 3),
//     (4, 2),
//     (4, 1),
//     (4, 0),
// ];

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Board {
    board: Matrix<u8, 5, 5>,
    value: u16,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[1; 5]; 5].into(),
            value: 25,
        }
    }

    pub fn set(&mut self, i: usize, j: usize, v: u8) {
        self.value -= self.board[i][j] as u16;
        self.board[i][j] = v;
        self.value += v as u16;
    }

    pub fn get(&self, i: isize, j: isize) -> Option<u8> {
        if i > 0 && i < 5 && j > 0 && j < 5 {
            return Some(self.board[i as usize][j as usize]);
        }
        None
    }

    pub fn get_rots_and_mirrs(&self) -> Vec<Board> {
        let mut rots_and_mirrs = Vec::new();

        let mut new_board = *self;

        for _ in 0..4 {
            let transpose = new_board.board.transpose();
            for i in 0..5 {
                new_board.board[4 - i] = transpose[i];
            }

            rots_and_mirrs.push(new_board);
        }

        new_board = *self;
        for i in 0..5 {
            new_board.board[4 - i] = self.board[i];
        }
        rots_and_mirrs.push(new_board);

        new_board = *self;
        for i in 0..5 {
            for j in 0..5 {
                new_board.board[i][4 - j] = self.board[i][j];
            }
        }
        rots_and_mirrs.push(new_board);

        rots_and_mirrs
    }

    pub fn get_child(&mut self, state_dict: &mut HashSet<Board>) -> Vec<Board> {
        let mut childs = Vec::new();

        for i in 0..5 {
            for j in 0..5 {
                // for (i, j) in POSITIONS {
                let curr_val = self.board[i][j];

                let mut is_1 = false;
                let mut is_2 = false;
                let mut is_3 = false;
                let mut is_4 = false;

                for (o_i, o_j) in [(1_isize, 0_isize), (0, 1), (-1, 0), (0, -1)] {
                    if let Some(v) = self.get(i as isize + o_i, j as isize + o_j) {
                        match v {
                            1 => is_1 = true,
                            2 => is_2 = true,
                            3 => is_3 = true,
                            4 => is_4 = true,
                            5 => (),
                            _ => unreachable!(),
                        }
                    }
                }

                if is_1 && is_2 && is_3 && is_4 {
                    let mut new_state = *self;
                    new_state.set(i, j, 5);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                }

                if is_1 && is_2 && is_3 {
                    let mut new_state = *self;
                    new_state.set(i, j, 4);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                }

                if is_1 && is_2 {
                    let mut new_state = *self;
                    new_state.set(i, j, 3);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                }

                if is_1 {
                    let mut new_state = *self;
                    new_state.set(i, j, 2);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                }

                if curr_val != 1 {
                    let mut new_state = *self;
                    new_state.set(i, j, 1);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                }
            }
        }
        // }
        childs
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                f.write_str(format!(" {} ", self.board[i][j]).as_str())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn get_best() {
    let mut _max_state: Board = Board::new();
    let mut max_val: u16 = 0;
    let mut state_dict: HashSet<Board> = HashSet::new();

    // let b = Board {
    //     board: [[1; 5]; 5].into(),
    //     value: 25,
    // };

    let b = Board {
        board: [
            [1, 2, 3, 2, 1],
            [2, 5, 2, 5, 2],
            [4, 3, 5, 2, 3],
            [2, 5, 4, 5, 2],
            [1, 2, 3, 2, 1],
        ]
        .into(),
        value: 69,
    };

    let mut state_stack: Vec<Board> = vec![b];
    while let Some(mut curr) = state_stack.pop() {
        if curr.value > max_val {
            max_val = curr.value;
            _max_state = curr;
            println!("New best board found:\n{} Value: {}\n", curr, curr.value);
        }

        let mut new_states = curr.get_child(&mut state_dict);
        new_states.sort_by(|x, y| x.value.cmp(&y.value));

        for new_state in new_states {
            if !state_dict.contains(&new_state) {
                // for r_b in new_state.get_rots_and_mirrs() {
                //     state_dict.insert(r_b);
                // }
                state_dict.insert(new_state);
                state_stack.push(new_state);
            }
        }

        // state_stack.sort_unstable_by(|x, y| x.value.cmp(&y.value));
    }
}

fn main() {
    // let mut b = Board::new();
    // b.board[0][0] = 5;
    // b.board[0][4] = 3;
    // println!("{}\n--------------------", b);

    // let rots = b.get_rots_and_mirrs();

    // for r in rots {
    //     println!("{}\n", r);
    // }

    get_best();
}
