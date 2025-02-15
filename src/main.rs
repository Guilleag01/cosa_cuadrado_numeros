use std::{collections::HashSet, fmt::Display};

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

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Board {
    board: [[u8; 5]; 5],
    _max: u8,
    // hash: u128,
    value: u16,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[1; 5]; 5],
            _max: 0,
            // hash: 142806463320114861533702931094369770,
            value: 1,
        }
    }

    pub fn set(&mut self, i: usize, j: usize, v: u8) {
        // println!("hash {}", self.hash);
        // self.hash /= self.board[i][j] as u128 * PRIME_TABLE[i][j];
        self.value -= self.board[i][j] as u16;
        self.board[i][j] = v;
        self.value += v as u16;
        // self.hash *= v as u128 * PRIME_TABLE[i][j];
    }

    pub fn get(&self, i: isize, j: isize) -> Option<u8> {
        if i > 0 && i < 5 && j > 0 && j < 5 {
            return Some(self.board[i as usize][j as usize]);
        }
        None
    }

    pub fn get_child(&mut self, state_dict: &mut HashSet<Board>) -> Vec<Board> {
        let mut childs = Vec::new();

        // println!("self hash 1 {:?}", self.hash);
        for i in 0..5 {
            for j in 0..5 {
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
                    // println!("hh {:?}", self);
                    let mut new_state = *self;
                    new_state.set(i, j, 2);
                    if !state_dict.contains(&new_state) {
                        childs.push(new_state);
                    }
                    // println!("das {:?}", new_state);
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

        // println!("c {:?}", childs);

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

    let b = Board {
        board: [[1; 5]; 5],
        _max: 0,
        // hash: 142806463320114861533702931094369770,
        value: 25,
    };

    let mut state_stack: Vec<Board> = vec![b];
    while let Some(mut curr) = state_stack.pop() {
        // println!("{:?}", curr);
        if curr.value > max_val {
            max_val = curr.value;
            _max_state = curr;
            println!("New best board found:\n{} Value: {}\n", curr, curr.value);
        }

        let mut new_states = curr.get_child(&mut state_dict);
        new_states.sort_by(|x, y| x.value.cmp(&y.value));

        // println!("{:?}", new_states);

        for new_state in new_states {
            if !state_dict.contains(&new_state) {
                state_dict.insert(new_state);
                state_stack.push(new_state);

                // let pos = state_stack
                //     .binary_search_by(|a| a.value.cmp(&new_state.value))
                //     .unwrap_or_else(|e| e);
                // state_stack.insert(pos, new_state);
            }
        }
        // println!("{:?}", state_stack);
    }
}

fn main() {
    // println!("{}", Board::new().hash);
    // let mut toal = 1;
    // for i in 0..5 {
    //     for j in 0..5 {
    //         toal *= PRIME_TABLE[i][j] as u128 * 7;
    //     }
    // }
    // println!("{}", toal);
    get_best();
}
