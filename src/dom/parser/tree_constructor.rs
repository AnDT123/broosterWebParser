use crate::parser::insertion_mode::InsertionMode;
use crate::dom::elements::Node;

pub struct TreeConstructor {
    insertion_mode: InsertionMode,
    stack_of_open_elements: Vec<Node>,
    is_fragment_case: bool,
    context_element: Option<Node>,
}

impl TreeConstructor {
    pub fn new() -> Self {
        TreeConstructor {
            insertion_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
            is_fragment_case: false,
            context_element: None,
        }
    }

    pub fn reset_insertion_mode(&mut self) {
        self.insertion_mode = InsertionMode::reset_insertion_mode(
            &self.stack_of_open_elements,
            self.context_element.as_ref(),
            self.is_fragment_case,
        );
    }

    // Other methods for the tree construction logic
}
