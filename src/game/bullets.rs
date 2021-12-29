use crate::{
    physics::motion,
    physics::units,
};

pub enum Bullets {
    Player(motion::Position<units::Pixels>)
}
