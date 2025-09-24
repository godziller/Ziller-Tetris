use term2d::AppBuilder;
use term2d::App;
use term2d::view::canvas::Canvas;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use crate::view::draw;
pub struct Model{
    pub counter: u32,
}

pub fn init_model(_app: &App) -> Model {
    Model {counter: 0}
}
