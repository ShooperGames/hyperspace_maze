extern crate savefile;
use savefile::prelude::*;
use rand::Rng;

// Important Functions
// Number of walls_dimension = 7*(8_u32.pow(dimensions-1))
// dimension of wall = wall_ind / walls_dimension
// Index of wall in dimension = wall_ind % walls_dimension

#[derive(Savefile)]
pub struct GameState {
    dimensions: u32,
    position: u32,
    direction: (bool, u32),
    walls: Vec<bool>,
}

pub fn make_maze(dimensions: u32) -> bool {
    let mut cells: Vec<u32> = (0..1 << dimensions * 3).collect();
    let mut wall_choices: Vec<u32> = (0..(dimensions*7*(1 << (dimensions - 1) * 3))).collect();
    let mut game_state = GameState {
        dimensions: dimensions,
        position: 0,
        direction: (true, 0),
        walls: vec![false; (dimensions*7*(1 << (dimensions - 1) * 3)).try_into().unwrap()],
    };
    let mut rng = rand::thread_rng();
    while wall_choices.len() > 0 {
        let cur_wall = wall_choices.swap_remove(rng.gen_range(0..wall_choices.len()));
        //println!("wall: {}", cur_wall);
        let cell_pair = cell_pair(cur_wall, dimensions);
        //println!("cells: {} and {}", cell_pair.0, cell_pair.1);
        if cells[cell_pair.0 as usize] == cells[cell_pair.1 as usize] {
            //Add a 1% chance of this being true.
            continue;
        }
        cell_match(cell_pair, dimensions, &mut cells);
        game_state.walls[cur_wall as usize] = true;
    }
    return save_game(&game_state);
}

fn cell_pair(wall_ind: u32, dimensions: u32) -> (u32, u32) {
    let walls_dimension: u32 = 7*(1 << (dimensions - 1) * 3);
    let dimension_of_wall: u32 = wall_ind / walls_dimension;
    let mut wall_ind_dimension: u32 = wall_ind - dimension_of_wall * walls_dimension;
    let mut result = (0, 0);
    for i in 0..dimensions {
        if i == dimension_of_wall {
            result.0 += (1 << i * 3) * (wall_ind_dimension % 7);
            wall_ind_dimension /= 7;
        } else {
            result.0 += (1 << i * 3) * (wall_ind_dimension % 8);
            wall_ind_dimension /= 8;
        }
    }
    result.1 = result.0 + (1 << dimension_of_wall * 3);
    return result;
}

fn cell_match(cell_pair: (u32, u32), dimensions: u32, cells: &mut Vec<u32>) {
    let mut check_cells = vec![cell_pair.1];
    while check_cells.len() > 0 {
        let cur_cell = check_cells.pop().unwrap();
        if cells[cur_cell as usize] == cell_pair.1 {
            for i in 0..dimensions {
                if cur_cell % (8 << i * 3) > (1 << i * 3) - 1 {
                    check_cells.push(cur_cell - i * 8);
                }
                if cur_cell % (8 << i * 3) < (8 << i * 3) - (1 << i * 3) {
                    check_cells.push(cur_cell + i * 8);
                }
            }
            cells[cur_cell as usize] = cell_pair.0;
        }
    }
}

pub fn check_move(game_state: &GameState) -> bool {
    if game_state.direction.0 {
        if game_state.position % (8 << game_state.direction.1 * 3) < (8 << game_state.direction.1 * 3) - (1 << game_state.direction.1 * 3) {
            let walls_dimension = 7 * (1 << (game_state.dimensions - 1) * 3);
            let wall_ind_dimension = game_state.position - (1 << game_state.direction.1 * 3) * (game_state.position / (1 << (game_state.direction.1 + 1) * 3));
            return game_state.walls[(wall_ind_dimension + walls_dimension * game_state.direction.1) as usize];
        }
    } else if game_state.position % (8 << game_state.direction.1 * 3) > (1 << game_state.direction.1 * 3) - 1 {
        let walls_dimension = 7 * (1 << (game_state.dimensions - 1) * 3);
        let wall_ind_dimension = game_state.position - (1 << game_state.direction.1 * 3) * (game_state.position / (1 << (game_state.direction.1 + 1) * 3)) - (1 << game_state.direction.1 * 3);
        return game_state.walls[(wall_ind_dimension + walls_dimension * game_state.direction.1) as usize];
    }
    false
}

pub fn move_cell() -> bool {
    let mut game_state = load_game();
    if check_move(&game_state) {
        if game_state.direction.0 {
            game_state.position += 1 << game_state.direction.1 * 3;
        } else {
            game_state.position -= 1 << game_state.direction.1 * 3;
        }
        return save_game(&game_state);
    }
    return false;
}

fn save_game(game_state: &GameState) -> bool {
    let result = save_file("data", 0, game_state);
    match result {
        Ok(T) => return true,
        Err(E) => return false,
    }
}

fn load_game() -> GameState {
    load_file("data", 0).unwrap()
}