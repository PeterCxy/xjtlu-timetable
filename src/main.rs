extern crate base64;
extern crate chrono;
#[macro_use]
extern crate stdweb;

mod constants;
#[macro_use]
mod util;
mod ical;
mod parser;
mod app;

use stdweb::web::{document, IParentNode};

fn main() {
    let elem_input = document().query_selector("#paste-area").unwrap().unwrap();
    let elem_submit = document().query_selector("#submit").unwrap().unwrap();
    let dialog_download = document().query_selector("#dialog-download").unwrap().unwrap();
    let dialog_info = document().query_selector("#dialog-info").unwrap().unwrap();
    let app = app::App::new(elem_input, elem_submit, dialog_download, dialog_info);
    app.run();
}
