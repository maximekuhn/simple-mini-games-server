use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct GuessTheNumber {
    max_tries: u8,
    tries_count: u8,
    number_to_guess: i32,
}

impl GuessTheNumber {
    pub fn new(max_tries: u8, min_number: i32, max_number: i32) -> Self {
        let tries_count = 0;
        let number_to_guess = thread_rng().gen_range(min_number..=max_number);
        Self {
            max_tries,
            tries_count,
            number_to_guess,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.tries_count == self.max_tries
    }

    pub fn guess(&mut self, user_guess: i32) -> bool {
        self.tries_count += 1;
        self.number_to_guess.eq(&user_guess)
    }
}
