use rand::Rng;

const DEFAULT_MAX_TRIES: u8 = 10;
const DEFAULT_MIN_NUMBER: i32 = 1;
const DEFAULT_MAX_NUMBER: i32 = 100;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Settings {
    max_tries: u8,
    min_number: i32,
    max_number: i32,
}

impl Settings {
    fn new_with_range(min_number: i32, max_number: i32) -> Self {
        let max_tries = DEFAULT_MAX_TRIES;
        let (min, max) = {
            if min_number >= max_number {
                (DEFAULT_MIN_NUMBER, DEFAULT_MAX_NUMBER)
            } else {
                (min_number, max_number)
            }
        };

        Self {
            max_tries,
            min_number: min,
            max_number: max,
        }
    }

    fn new_with_range_and_max_tries(min_number: i32, max_number: i32, max_tries: u8) -> Self {
        let mut settings = Settings::new_with_range(min_number, max_number);
        settings.max_tries = max_tries;
        settings
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_tries: DEFAULT_MAX_TRIES,
            min_number: DEFAULT_MIN_NUMBER,
            max_number: DEFAULT_MAX_NUMBER,
        }
    }
}

struct Game {
    settings: Settings,
    number_to_guess: i32,
    tries_count: u8,
}

impl Game {
    fn new(settings: Settings) -> Self {
        let tries_count = 0;
        let number_to_guess =
            rand::thread_rng().gen_range(settings.min_number..=settings.max_number);
        Self {
            settings,
            number_to_guess,
            tries_count,
        }
    }

    fn reset(&mut self) {
        self.tries_count = 0;
        self.number_to_guess =
            rand::thread_rng().gen_range(self.settings.min_number..=self.settings.max_number);
    }

    fn information(&self) -> (u8 /* current tries count */, Settings) {
        (self.tries_count, self.settings)
    }

    fn guess(&mut self, player_guess: i32) -> (bool, Option<String>) {
        // Player tried to guess too many times
        if !self.can_play_more() {
            return (false, None);
        }

        self.tries_count += 1;
        let (result, hint) = {
            match self.number_to_guess.cmp(&player_guess) {
                std::cmp::Ordering::Less => (
                    false,
                    Some(String::from(
                        "The number to guess is smaller than the one you provided.",
                    )),
                ),
                std::cmp::Ordering::Equal => (true, None),
                std::cmp::Ordering::Greater => (
                    false,
                    Some(String::from(
                        "The number to guess is greater than the one you provided.",
                    )),
                ),
            }
        };

        (result, hint)
    }

    fn can_play_more(&self) -> bool {
        self.tries_count != self.settings.max_tries
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, Settings, DEFAULT_MAX_NUMBER, DEFAULT_MAX_TRIES, DEFAULT_MIN_NUMBER};

    // Settings related tests
    #[test]
    fn should_use_default_settings() {
        let sut = Settings::default();
        assert_eq!(DEFAULT_MAX_TRIES, sut.max_tries);
        assert_eq!(DEFAULT_MAX_NUMBER, sut.max_number);
        assert_eq!(DEFAULT_MIN_NUMBER, sut.min_number);
    }

    #[test]
    fn should_use_provided_range_and_default_max_tries() {
        let min_number = 10;
        let max_number = 50;
        let sut = Settings::new_with_range(min_number, max_number);
        assert_eq!(min_number, sut.min_number);
        assert_eq!(max_number, sut.max_number);
        assert_eq!(DEFAULT_MAX_TRIES, sut.max_tries);
    }

    #[test]
    fn should_use_default_settings_as_provided_range_is_incorrect() {
        let min_number = 50;
        let max_number = 47; // we must have min < max
        let sut = Settings::new_with_range(min_number, max_number);
        assert_eq!(DEFAULT_MAX_TRIES, sut.max_tries);
        assert_eq!(DEFAULT_MAX_NUMBER, sut.max_number);
        assert_eq!(DEFAULT_MIN_NUMBER, sut.min_number);
    }

    #[test]
    fn should_use_provided_range_and_max_tries() {
        let min_number = 10;
        let max_number = 50;
        let max_tries = 2;
        let sut = Settings::new_with_range_and_max_tries(min_number, max_number, max_tries);
        assert_eq!(min_number, sut.min_number);
        assert_eq!(max_number, sut.max_number);
        assert_eq!(max_tries, sut.max_tries);
    }

    #[test]
    fn should_use_provided_max_tries_but_default_range() {
        let min_number = 50;
        let max_number = 47; // we must have min < max
        let max_tries = 2;
        let sut = Settings::new_with_range_and_max_tries(min_number, max_number, max_tries);
        assert_eq!(DEFAULT_MAX_NUMBER, sut.max_number);
        assert_eq!(DEFAULT_MIN_NUMBER, sut.min_number);
        assert_eq!(max_tries, sut.max_tries);
    }

    // Game related tests
    #[test]
    fn should_properly_create_new_game() {
        let sut = Game::new(Settings::default());
        assert_eq!(Settings::default(), sut.settings);
        assert_eq!(0, sut.tries_count);
        assert_eq!(true, sut.can_play_more());

        // TODO: find a better way to test random number generation
        // maybe use dependency injection
        assert_eq!(
            true,
            (DEFAULT_MIN_NUMBER..=DEFAULT_MAX_NUMBER).contains(&sut.number_to_guess)
        );
    }

    #[test]
    fn should_not_be_correct_guess() {
        let max_tries = 2;
        let min_number = 10;
        let max_number = 32;
        let settings = Settings::new_with_range_and_max_tries(min_number, max_number, max_tries);
        let mut sut = Game::new(settings);

        // Guess a number outside the range
        let (result, hint) = sut.guess(min_number - 1);
        assert_eq!(false, result);
        assert!(hint.is_some());
        assert_eq!(
            true,
            hint.unwrap()
                .contains("The number to guess is greater than the one you provided.")
        );

        // We should still be able to play
        assert_eq!(true, sut.can_play_more());

        // Guess another number outside the range
        let (result, hint) = sut.guess(max_number + 1);
        assert_eq!(false, result);
        assert!(hint.is_some());
        assert_eq!(
            true,
            hint.unwrap()
                .contains("The number to guess is smaller than the one you provided.")
        );

        // Game should now be finished
        assert_eq!(false, sut.can_play_more());
    }

    #[test]
    fn game_should_be_finished() {
        let max_tries = 2;
        let max_number = 50;
        let settings = Settings::new_with_range_and_max_tries(1, max_number, max_tries);
        let mut sut = Game::new(settings);

        for _ in 0..max_tries {
            sut.guess(max_number + 1);
        }

        assert_eq!(false, sut.can_play_more());
    }

    #[test]
    fn should_return_information_about_game() {
        let max_tries = 2;
        let min_number = 49;
        let max_number = 50;
        let settings = Settings::new_with_range_and_max_tries(min_number, max_number, max_tries);
        let mut sut = Game::new(settings);

        let initial_information = sut.information();
        assert_eq!(0, initial_information.0);
        assert_eq!(settings, initial_information.1);

        sut.guess(max_number + 1);
        let current_information = sut.information();
        assert_eq!(1, current_information.0);
        assert_eq!(settings, current_information.1);
    }

    #[test]
    fn should_win() {
        let min_number = 1;
        let max_number = 5;
        let max_tries = max_number + 1;
        let settings =
            Settings::new_with_range_and_max_tries(min_number, max_number, max_tries as u8);
        let mut sut = Game::new(settings);

        let mut results = Vec::new();
        for guess in 0..max_tries {
            results.push(sut.guess(guess).0);
        }

        assert!(results.contains(&true));
    }
}
