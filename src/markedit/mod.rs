mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MarkdownEditor(ObjectSubclass<imp::MarkdownEditor>)
        @extends gtk::Widget, gtk::TextView,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl MarkdownEditor {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for MarkdownEditor {
    fn default() -> Self {
        Self::new()
    }
}