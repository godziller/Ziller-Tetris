mod view;
use crate::view::draw;
use term2d::AppBuilder;
mod model;
use crate::model::init_model;

fn main() {
    AppBuilder::new(init_model)
        .view(view::draw::view_fn)
        .fps(20)
        .run();
}
