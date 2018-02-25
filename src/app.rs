use parser::parse;
use ical::{self, ICalElement};
use std::rc::Rc;
use stdweb::web::{alert, IEventTarget, Element};
use stdweb::web::event::{IEvent, ClickEvent};

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
        match parse(&self.elem_input).map(|r| ical::classes_to_ical(&r).serialize()) {
            Ok(cal) => {
                js!(
                    // TODO: Rewrite this in Rust (Maybe?) or show a link in dialog
                    var link = document.createElement("a");
                    link.download = "timetable.ics";
                    link.href = "data:text/calendar;base64," + btoa(@{cal});
                    document.body.appendChild(link);
                    link.click();
                    document.body.removeChild(link);
                );
            },
            Err(err) => alert(&err)
        }
    }
}