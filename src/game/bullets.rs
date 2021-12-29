use crate::{
    physics::motion,
    physics::units,
};

use ggez::{
    graphics,
    Context,
    GameResult,
};

pub enum Bullet {
    Player(motion::Position<units::Pixels>)
}

impl Bullet {
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let position = match self {
            Bullet::Player(pos) => graphics::Rect::new(pos.0.0, pos.1.0, 20.0, 20.0),
        };

        let bullet_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            graphics::Color::BLUE,
        )?;

        graphics::draw(ctx, &bullet_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}
