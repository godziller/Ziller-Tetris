use term2d::model::color::Color;
use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;
use term2d::AppBuilder;

use crate::model::Model;

pub fn event_fn(app: &App, model: &mut Model, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {}
    }

    true
}
