#![allow(dead_code)]
// https://stackoverflow.com/questions/40875152/reference-to-element-in-vector

struct Cell {
    name: String,
}

fn construct_cells() -> Vec<Cell> {
    vec![
        Cell {
            name: "a".to_string(),
        },
        Cell {
            name: "b".to_string(),
        },
        Cell {
            name: "c".to_string(),
        },
    ]
}

struct Player<'a> {
    game: &'a Game,
    cell_idx: usize,
}

impl<'a> Player<'a> {
    pub fn new(game: &'a Game, cell_idx: usize) -> Player<'a> {
        Player {
            game: game,
            cell_idx: cell_idx,
        }
    }
    pub fn current_cell_name(&self) -> &str {
        &self.game.cells[self.cell_idx].name
    }
}

struct Game {
    is_running: bool,
    cells: Vec<Cell>,
}

impl Game {
    pub fn new() -> Game {
        let cells = construct_cells();
        Game {
            is_running: false,
            cells: cells,
        }
    }
}

fn main() {
    let game = Game::new();
    let player = Player::new(&game, 0);
    assert!(player.current_cell_name() == "a");
}
