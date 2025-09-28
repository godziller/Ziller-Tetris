use term2d::AppBuilder;
use term2d::App;
use term2d::view::canvas::Canvas;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use crate::view::draw;
use crate::modelcomponents::well::Well;


pub struct Model {
    pub well: Well,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let well = Well::new(10,20);

        Self {
            well,
        }
    }
}


