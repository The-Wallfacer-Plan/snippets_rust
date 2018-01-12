struct Cell {
    name: String,
}

struct Player<'a> {
    cell: &'a Cell,
}

impl<'a> Player<'a> {
    pub fn new(cell: &'a Cell) -> Player<'a> {
        Player { cell: cell }
    }
    pub fn current_cell_name(&self) -> &str {
        &self.cell.name
    }
}

struct Game {
    is_running: bool,
    cells: Vec<Box<Cell>>,
}

impl Game {
    fn construct_cells() -> Vec<Box<Cell>> {
        ["a", "b", "c"]
            .iter()
            .map(|n| {
                Box::new(Cell {
                    name: n.to_string(),
                })
            })
            .collect()
    }

    pub fn new() -> Game {
        let cells = Game::construct_cells();

        Game {
            is_running: false,
            cells: cells,
        }
    }
}

fn main() {
    let game = Game::new();
    let player = Player::new(&game.cells[0]);
    assert!(player.current_cell_name() == "a");
}
