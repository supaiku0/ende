use gtk::{WidgetExt, StyleContextExt, TargetEntry, TargetFlags};

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn set_class<W: WidgetExt>(widget: &W, class: &str) {
    widget.get_style_context().map(|c| c.add_class(class));
}

pub fn get_drop_targets() -> Vec<TargetEntry> {
    vec![TargetEntry::new("text/plain", TargetFlags::OTHER_APP, 0)]
}
