use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
struct Color(u8, u8, u8);

impl Color {
    fn random() -> Self {
        Self(
            get_random_number(50, 225),
            get_random_number(50, 225),
            get_random_number(50, 225),
        )
    }

    fn variant(&self, level: u32) -> Color {
        let mut variance: u8 = 5;
        if level < 10 {
            variance = 15;
        } else if level < 15 {
            variance = 13
        } else if level < 20 {
            variance = 11;
        } else if level < 25 {
            variance = 9;
        } else if level < 30 {
            variance = 7;
        }
        let sign = get_random_number(0, 1);
        if sign == 0 {
            Self(self.0 + variance, self.1 + variance, self.2 + variance)
        } else {
            Self(self.0 - variance, self.1 - variance, self.2 - variance)
        }
    }
}

#[derive(Serialize, Copy, Clone)]
pub struct Game {
    level: u32,
    square: [u8; 2],
    color: Color,
    true_color: Color,
}

fn get_random_number(begin: u8, end: u8) -> u8 {
    rand::thread_rng().gen_range(begin..=end)
}

impl Game {
    pub fn new() -> Self {
        let square = [get_random_number(0, 5), get_random_number(0, 5)];
        let color = Color::random();
        let true_color = color.variant(0);
        Self {
            level: 0,
            square,
            color,
            true_color,
        }
    }

    pub fn next(&mut self) {
        self.square = [get_random_number(0, 5), get_random_number(0, 5)];
        self.color = Color::random();
        self.true_color = self.color.variant(self.level);
        self.level += 1;
    }

    pub fn new_game(&mut self) {
        *self = Game::new();
    }
}
