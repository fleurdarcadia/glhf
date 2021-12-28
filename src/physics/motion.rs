use ggez::event::KeyCode;

/// An option-like representation of the directions of arrow keys.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

impl Direction {
    pub fn from_key_code(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up    => Some(Direction::Up),
            KeyCode::Down  => Some(Direction::Down),
            KeyCode::Left  => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _              => None,
        }
    }
}
