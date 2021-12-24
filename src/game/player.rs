use crate::config::ui::UI;

use ggez::{
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
}
