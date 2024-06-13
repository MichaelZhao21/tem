use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/xyz/michaelzhao/tem/markdown-editor.ui")]
pub struct MarkdownEditor {}

// Central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MarkdownEditor {
    const NAME: &'static str = "MarkdownEditor";
    type Type = super::MarkdownEditor;
    type ParentType = gtk::TextView;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for MarkdownEditor {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for MarkdownEditor {}

// Trait shared by all text views
impl TextViewImpl for MarkdownEditor {}
