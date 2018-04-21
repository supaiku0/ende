use gtk::{WidgetExt, StyleContextExt, TargetEntry, TargetFlags};

pub fn set_class<W: WidgetExt>(widget: &W, class: &str) {
    widget.get_style_context().map(|c| c.add_class(class));
}

pub fn get_drop_targets() -> Vec<TargetEntry> {
    vec![TargetEntry::new("text/plain", TargetFlags::OTHER_APP, 0)]
}
