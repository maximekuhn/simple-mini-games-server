struct Settings {
    max_tries: u8,
    max_number: i32,
    min_number: i32,
}

struct Game {
    settings: Settings,
}

impl Game {
    fn new(settings: Settings) -> Self {
        Self { settings }
    }
}
