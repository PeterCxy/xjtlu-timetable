use stdweb::web::INode;

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

pub trait InnerHTML {
    fn inner_html(&self) -> String;
}

impl<T> InnerHTML for T where T: INode {
    fn inner_html(&self) -> String {
        js!(
            return @{self.as_ref()}.innerHTML;
        ).into_string().unwrap()
    }
}