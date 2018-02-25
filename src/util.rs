use stdweb::Value;
use stdweb::web::{IElement, INode, Element};

#[allow(unused_macros)]
macro_rules! clone {
    /*
     * Clone some members from a struct
     * to the corresponding local variables.
     */
    ($s:ident, $($n:ident),+) => (
        $( let $n = $s.$n.clone(); )+
    );

    /*
     * Simulate a closure that clones
     * some environment variables and
     * take ownership of them by default.
     */
    ($($n:ident),+; || $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move || { $body }
        }
    );
    ($($n:ident),+; |$($p:pat),+| $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p),+| { $body }
        }
    );
    ($($n:ident),+; |$($p:ident:$q:ty),+| $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p:$q),+| { $body }
        }
    );
}

pub trait ToElement {
    fn to_element(&self) -> Option<Element>;
}

impl<T: INode> ToElement for T {
    fn to_element(&self) -> Option<Element> {
        self.as_ref().clone().downcast()
    }
}

pub trait ElementAttribute {
    fn get_attribute(&self, name: &str) -> Value;
    fn set_attribute(&self, name: &str, value: Value);
}

impl<T: IElement> ElementAttribute for T {
    fn get_attribute(&self, name: &str) -> Value {
        js!(
            return @{self.as_ref()}.getAttribute(@{name});
        )
    }

    fn set_attribute(&self, name: &str, value: Value) {
        js!(
            @{self.as_ref()}.setAttribute(@{name}, @{value});
        );
    }
}