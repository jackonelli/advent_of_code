use std::fs::File;
use std::io::prelude::*;

type Slice = Vec<Vec<char>>;
type State3d = Vec<Slice>;
type State4d = Vec<State3d>;

fn main() {
    let file = "input/17/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();
    let first_slice = data
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Slice>();
    let ticks = 6;
    let state = init_state_3d(&first_slice, ticks);
    let state = run_3d(state, ticks);
    println!("Star 1: {}", count_occ_3d(&state));
    let state = init_4d_state(&first_slice, ticks);
    let state = run_4d(state, ticks);
    println!("Star 2: {}", count_occ_4d(&state));
}

fn run_4d(state: State4d, ticks: usize) -> State4d {
    let mut state = state;
    for _ in 0..ticks {
        state = tick_4d(&state);
    }
    state
}

fn tick_4d(state: &State4d) -> State4d {
    let (wow, depth, height, width) = (
        state.len(),
        state[0].len(),
        state[0][0].len(),
        state[0][0][0].len(),
    );
    let mut new_grid = state.to_vec();
    for w in 0..wow - 1 {
        for z in 0..depth - 1 {
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let nbs = count_nb_adj_4d(&state, x, y, z as i32, w as i32);
                    match state[w][z][y][x] {
                        '.' => {
                            if nbs == 3 {
                                new_grid[w][z][y][x] = '#'
                            }
                        }
                        '#' => {
                            if nbs == 3 || nbs == 2 {
                                new_grid[w][z][y][x] = '#'
                            } else {
                                new_grid[w][z][y][x] = '.'
                            }
                        }
                        _ => panic!("inv tick char"),
                    };
                }
            }
        }
    }
    new_grid
}

fn count_nb_adj_4d(state: &State4d, x: usize, y: usize, z: i32, w: i32) -> usize {
    let mut occ = 0;
    for l in w - 1..=w + 1 {
        for k in z - 1..=z + 1 {
            for j in y - 1..=y + 1 {
                for i in x - 1..=x + 1 {
                    if i == x && j == y && k == z && l == w {
                        continue;
                    }
                    match state[loc_slice_idx(l)][loc_slice_idx(k)][j][i] {
                        '#' => occ += 1,
                        '.' => {}
                        _ => panic!("inv char"),
                    }
                }
            }
        }
    }
    occ
}

fn count_occ_4d(state: &State4d) -> usize {
    // The state is symmetric around z=0 and w=0.
    // The alg. actually only calculates the state for w, z >=0
    // ==> Quadruple the count but remove the doubly counted layers
    // for z=0, w >=0, w=0, z>=0.
    let tmp = state
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|c| **c == '#')
        .count();
    let extra_counted_w = 2 * state
        .iter()
        .map(|cube| cube[0].iter().flatten().filter(|c| **c == '#').count() as usize)
        .sum::<usize>();
    let extra_counted_z = 2 * state[0]
        .iter()
        .flatten()
        .flatten()
        .filter(|c| **c == '#')
        .count();
    4 * tmp - extra_counted_w - extra_counted_z
}

fn init_4d_state(first_slice: &Slice, ticks: usize) -> State4d {
    let height = first_slice.len();
    let width = first_slice[0].len();
    let mut state =
        vec![
            vec![vec![vec!['.'; 2 * (ticks + 1) + width]; 2 * (ticks + 1) + height]; ticks + 2];
            ticks + 2
        ];
    for y in 0..height {
        for x in 0..width {
            let (x_g, y_g) = glob_idx(x as i32, y as i32, ticks);
            state[0][0][y_g][x_g] = first_slice[y][x];
        }
    }
    state
}

fn run_3d(state: State3d, ticks: usize) -> State3d {
    let mut state = state;
    for _ in 0..ticks {
        state = tick_3d(&state);
    }
    state
}

fn tick_3d(state: &State3d) -> State3d {
    let (depth, height, width) = (state.len(), state[0].len(), state[0][0].len());
    let mut new_grid = state.to_vec();
    for z in 0..depth - 1 {
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let nbs = count_nb_adj_3d(&state, x, y, z as i32);
                match state[z][y][x] {
                    '.' => {
                        if nbs == 3 {
                            new_grid[z][y][x] = '#'
                        }
                    }
                    '#' => {
                        if nbs == 3 || nbs == 2 {
                            new_grid[z][y][x] = '#'
                        } else {
                            new_grid[z][y][x] = '.'
                        }
                    }
                    _ => panic!("inv tick char"),
                };
            }
        }
    }
    new_grid
}

fn count_nb_adj_3d(state: &State3d, x: usize, y: usize, z: i32) -> usize {
    let mut occ = 0;
    for k in z - 1..=z + 1 {
        for j in y - 1..=y + 1 {
            for i in x - 1..=x + 1 {
                if i == x && j == y && k == z {
                    continue;
                }
                match state[loc_slice_idx(k)][j][i] {
                    '#' => occ += 1,
                    '.' => {}
                    _ => panic!("inv char"),
                }
            }
        }
    }
    occ
}

fn count_occ_3d(state: &State3d) -> usize {
    // The state is symmetric around z=0.
    // The alg. actually only calculates the state for z>=0
    // ==> Double the count but remove the doubly counted layer at z=0
    let tmp = state
        .iter()
        .flatten()
        .flatten()
        .filter(|c| **c == '#')
        .count();
    let diff = state[0].iter().flatten().filter(|c| **c == '#').count();
    2 * tmp - diff
}

fn init_state_3d(first_slice: &Slice, ticks: usize) -> State3d {
    let height = first_slice.len();
    let width = first_slice[0].len();
    let mut state =
        vec![vec![vec!['.'; 2 * (ticks + 1) + width]; 2 * (ticks + 1) + height]; ticks + 1];
    for y in 0..height {
        for x in 0..width {
            let (x_g, y_g) = glob_idx(x as i32, y as i32, ticks);
            state[0][y_g][x_g] = first_slice[y][x];
        }
    }
    state
}

fn _print_grid(grid: &Slice) {
    for r in grid {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn glob_idx(x: i32, y: i32, ticks: usize) -> (usize, usize) {
    (x as usize + ticks + 1, y as usize + ticks + 1)
}

fn loc_slice_idx(z: i32) -> usize {
    if z < 0 {
        -z as usize
    } else {
        z as usize
    }
}
