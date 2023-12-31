use guess_the_number::Game;

#[derive(Clone)]
pub struct GameState {
    pub game: Option<Game>,
}

impl GameState {
    pub fn new() -> Self {
        Self { game: None }
    }
}
