use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::rect::Rect;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;
use term2d::AppBuilder;

use crate::model::Model;
use crate::modelcomponents::constants::RGBA_BACKGROUND;

pub fn view_fn(app: &App, _model: &Model, canvas: &mut HalfblockCanvas){
    canvas.clear();
    canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
    draw_background(_model, canvas);

    canvas.display();
}

fn draw_background(model: &Model, canvas: &mut HalfblockCanvas){
    canvas.draw_rect_fill(
        &Rect {
            pos: Point::new(0, 0),
            size: Point::new(model.well.width + 2, model.well.height + 2),
        },

        &RGBA_BACKGROUND,
    );
}

