use base64::encode;
use parser::parse;
use ical::{self, ICalElement};
use std::rc::Rc;
use stdweb::web::{INode, IParentNode, IEventTarget, Element};
use stdweb::web::event::{IEvent, ClickEvent};
use util::ElementAttribute;

pub struct App {
    elem_input: Element,
    elem_submit: Element,
    dialog_download: DownloadDialog,
    dialog_info: InfoDialog
}

impl App {
    pub fn new(elem_input: Element, elem_submit: Element, dialog_download: Element, dialog_info: Element) -> App {
        App {
            elem_input,
            elem_submit,
            dialog_download: DownloadDialog(dialog_download),
            dialog_info: InfoDialog(dialog_info)
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
    fn show_download_dialog(&self, content: String);
    fn show_info_dialog(&self, info: String);
    fn on_submit(&self, ev: ClickEvent);
}

impl AppImpl for Rc<App> {
    fn show_download_dialog(&self, content: String) {
        self.dialog_download.set_download_link(&format!("data:text/calendar;base64,{}", encode(&content)));
        self.dialog_download.show();
    }

    fn show_info_dialog(&self, info: String) {
        self.dialog_info.set_information(&info);
        self.dialog_info.show();
    }

    fn on_submit(&self, ev: ClickEvent) {
        ev.prevent_default();
        match parse(&self.elem_input).map(|r| ical::classes_to_ical(&r).serialize()) {
            Ok(cal) => self.show_download_dialog(cal),
            Err(err) => self.show_info_dialog(err)
        }
    }
}

trait Dialog {
    fn get_element(&self) -> &Element;

    fn show(&self) {
        js!(
            $(@{self.get_element().as_ref()}).modal("show");
        );
    }
}

// Dialog to show download link of the ics file
struct DownloadDialog(Element);

impl Dialog for DownloadDialog {
    fn get_element(&self) -> &Element {
        &self.0
    }
}

impl DownloadDialog {
    fn set_download_link(&self, link: &str) {
        self.get_element().query_selector("#link-download").unwrap().unwrap().set_attribute("href", link.into());
    }
}

// Dialog to show error messages
struct InfoDialog(Element);

impl Dialog for InfoDialog {
    fn get_element(&self) -> &Element {
        &self.0
    }
}

impl InfoDialog {
    fn set_information(&self, text: &str) {
        self.get_element().query_selector("#text-info").unwrap().unwrap().set_text_content(text);
    }
}