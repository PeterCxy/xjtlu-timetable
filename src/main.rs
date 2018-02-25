#[macro_use]
extern crate stdweb;

#[macro_use]
mod util;
mod parser;
mod app;

use stdweb::web::{document, IParentNode};

fn main() {
    let elem_input = document().query_selector("#paste-area").unwrap().unwrap();
    let elem_submit = document().query_selector("#submit").unwrap().unwrap();
    let app = app::App::new(elem_input, elem_submit);
    app.run();
}
