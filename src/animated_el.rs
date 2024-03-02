use web_sys::HtmlElement;

type Classes = Vec<String>;

pub trait AnimatedEl {
    fn add_classes(&self, classes: &Classes);

    /// Returns the classes that were not previously present, and were added by
    /// the method.
    fn add_unique_classes(&self, classes: &Classes) -> Classes;
    fn remove_classes(&self, classes: &Classes);
    fn set_important_style_property(&self, property: &str, value: &str);
    fn enable_instant_transition(&self);
    fn disable_instant_transition(&self);
    fn clear_transform(&self);
}

impl AnimatedEl for HtmlElement {
    fn add_classes(&self, classes: &Classes) {
        for class in classes {
            self.class_list().add_1(class).unwrap();
        }
    }

    fn add_unique_classes(&self, classes: &Classes) -> Classes {
        let mut added_classes = Vec::new();

        for class in classes {
            let class_list = self.class_list();

            if class_list.contains(class) {
                continue;
            }

            class_list.add_1(class).unwrap();
            added_classes.push(class.clone());
        }

        added_classes
    }

    fn remove_classes(&self, classes: &Classes) {
        for class in classes {
            self.class_list().remove_1(class).unwrap();
        }
    }

    fn set_important_style_property(&self, property: &str, value: &str) {
        self.style()
            .set_property_with_priority(property, value, "important")
            .unwrap();
    }

    fn enable_instant_transition(&self) {
        self.set_important_style_property("transition-duration", "0s");
    }

    fn disable_instant_transition(&self) {
        self.style()
            .set_property("transition-duration", "")
            .unwrap();
    }

    fn clear_transform(&self) {
        self.style().set_property("transform", "").unwrap();
    }
}
