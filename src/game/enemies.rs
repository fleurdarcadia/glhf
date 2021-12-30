use crate::{
    game::{
        bullets,
    },
    physics::{
        motion,
        units,
    },
};

use ggez::{
    graphics::{
        self,
        Color,
    },
    Context,
    GameResult,
};


pub struct Enemy {
    pub position: motion::Position<units::Pixels>,
    pub dimensions: motion::Dimensions<units::Pixels>,
    
    bullet_rotation: Vec<bullets::Bullet>,
}

impl Enemy {
    pub fn new(
        pos: motion::Position<units::Pixels>,
        dim: motion::Dimensions<units::Pixels>,
        bullets: Vec<bullets::Bullet>,
    ) -> Self {
        Enemy {
            position: pos,
            dimensions: dim,
            bullet_rotation: bullets,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let position = graphics::Rect::new(
            self.position.x.value(),
            self.position.y.value(),
            self.dimensions.width.value(),
            self.dimensions.height.value(),
        );

        let enemy_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            Color::MAGENTA,
        )?;

        graphics::draw(ctx, &enemy_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}
