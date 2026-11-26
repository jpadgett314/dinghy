use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Orientation};

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::NaturalWidthShim)]
    pub struct NaturalWidthShim {
        #[property(get, set = Self::set_child)]
        pub child: std::cell::RefCell<Option<gtk::Widget>>,
        #[property(get, set = Self::set_natural_width, default = 0, minimum = 0)]
        pub natural_width: std::cell::Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NaturalWidthShim {
        const NAME: &'static str = "NaturalWidthShim";
        type Type = super::NaturalWidthShim;
        type ParentType = gtk::Widget;
    }

    #[glib::derived_properties]
    impl ObjectImpl for NaturalWidthShim {
        fn dispose(&self) {
            if let Some(child) = self.child.borrow_mut().take() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for NaturalWidthShim {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            self.child
                .borrow()
                .as_ref()
                .map(|child| child.request_mode())
                .unwrap_or(gtk::SizeRequestMode::ConstantSize)
        }

        fn measure(
            &self,
            orientation: Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let Some(child) = self.child.borrow().as_ref().cloned() else {
                return (0, 0, -1, -1);
            };

            let effective_for_size = if orientation == Orientation::Vertical && for_size < 0 {
                let nat_w = self.natural_width.get();
                if nat_w > 0 {
                    let (_, child_nat_w, _, _) = child.measure(Orientation::Horizontal, -1);
                    child_nat_w.max(nat_w)
                } else {
                    for_size
                }
            } else {
                for_size
            };

            let (minimum, mut natural, min_baseline, nat_baseline) =
                child.measure(orientation, effective_for_size);

            if orientation == Orientation::Horizontal {
                natural = natural.max(self.natural_width.get());
            }

            (minimum, natural, min_baseline, nat_baseline)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            if let Some(child) = self.child.borrow().as_ref() {
                child.allocate(width, height, baseline, None);
            }
        }
    }

    impl NaturalWidthShim {
        fn set_natural_width(&self, width: i32) {
            assert!(width >= 0);
            self.natural_width.set(width);
            self.obj().queue_resize();
        }

        fn set_child(&self, child: Option<gtk::Widget>) {
            if let Some(old_child) = self.child.borrow_mut().take() {
                old_child.unparent();
            }

            if let Some(child) = child {
                child.set_parent(&*self.obj());
                *self.child.borrow_mut() = Some(child);
            }

            self.obj().queue_resize();
        }
    }
}

glib::wrapper! {
    pub struct NaturalWidthShim(ObjectSubclass<imp::NaturalWidthShim>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NaturalWidthShim {
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        glib::Object::builder()
            .property("child", Some(child))
            .property("natural-width", 480)
            .build()
    }
}
