use std::rc::Rc;
use stdweb::web::{alert, IEventTarget, Element};
use stdweb::web::event::{IEvent, ClickEvent};
use util::InnerHTML;

pub struct App {
    elem_input: Element,
    elem_submit: Element
}

impl App {
    pub fn new(elem_input: Element, elem_submit: Element) -> App {
        App {
            elem_input,
            elem_submit
        }
    }

    pub fn run(self) {
        let _self = Rc::new(self);
        _self.elem_submit.add_event_listener(clone!(_self; |ev: ClickEvent| {
            _self.on_submit(ev);
        }));
    }
}

trait AppImpl {
    fn on_submit(&self, ev: ClickEvent);
}

impl AppImpl for Rc<App> {
    fn on_submit(&self, ev: ClickEvent) {
        ev.prevent_default();
        alert(&self.elem_input.inner_html());
    }
}