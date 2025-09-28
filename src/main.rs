mod view;
use view::draw::view_fn;
mod controller;
use crate::controller::update::event_fn;

use term2d::AppBuilder;
mod modelcomponents;
pub mod model;
pub use modelcomponents::init::init_model;

fn main() {
    AppBuilder::new(init_model)
        .event(|app, model, event| event_fn(app, model, event))
        .view(|app, model, canvas| view_fn(app, model, canvas))
        .fps(20)
        .run();
}
