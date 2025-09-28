use term2d::App;
use crate::model::Model;

pub fn init_model(app: &App) -> Model {
    Model::new(app)
}
