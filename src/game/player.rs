use crate::config::ui::UI;
use crate::physics::motion;

use chrono::prelude::*;
use chrono::Duration;

use ggez::{
    event::KeyCode,
    graphics::{
        self,
        Color
    },
    Context,
    GameResult,
};

/// The player's state.
pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// The various actions the player can take.
pub enum Action {
    Move(motion::Direction),
    Shoot,
}

impl Player {
    pub fn new(ui: &UI) -> Self {
        Player {
            x: ui.width / 2.0 - 12.0,
            y: ui.height - 64.0,
            width: 24.0,
            height: 32.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let position = graphics::Rect::new(
            self.x,
            self.y,
            self.width,
            self.height,
        );
 
        let player_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            Color::RED,
        )?;

        graphics::draw(ctx, &player_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
    
    /// Reposition the player in some direction.
    /// The player's velocity is an inherent characteristic, however the time since
    /// the last tick must be taken into account to compute distance.
    pub fn reposition(&mut self, dir: &motion::Direction, time: Duration) {
        self.x += motion::Velocity::horizontal(dir).distance(time).0;
        self.y += motion::Velocity::vertical(dir).distance(time).0;
    }
}

impl Action {
    pub fn from_key_code(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up    => Some(Action::Move(motion::Direction::Up)),
            KeyCode::Down  => Some(Action::Move(motion::Direction::Down)),
            KeyCode::Left  => Some(Action::Move(motion::Direction::Left)),
            KeyCode::Right => Some(Action::Move(motion::Direction::Right)),
            KeyCode::Space => Some(Action::Shoot),
            _              => None,
        }
    }
}
