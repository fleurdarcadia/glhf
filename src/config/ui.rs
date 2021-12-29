use ggez::graphics;


/// Configuration options for the game UI.
pub struct UI {
    pub width: f32,
    pub height: f32,
}

impl UI {
    pub fn new(width: f32, height: f32) -> Self {
        UI {
            width: width,
            height: height,
        }
    }
    
    pub fn hitbox_rect(&self) -> graphics::Rect {
        graphics::Rect::new(0.0, 0.0, self.width, self.height)
    }
}

impl Default for UI {
    fn default() -> Self {
        UI {
            width: 600.0,
            height: 800.0,
        }
    }
}
