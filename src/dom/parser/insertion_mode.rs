#[derive(Debug, PartialEq, Clone)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

impl InsertionMode {
    pub fn reset_insertion_mode(
        stack_of_open_elements: &[Node], // Adjust type as per your implementation
        context_element: Option<&Node>,  // For fragment parsing, if applicable
        is_fragment_case: bool,
    ) -> InsertionMode {
        let mut last = false;   //1. Let last be false.
        let mut node = stack_of_open_elements.last().cloned(); // 2. Let node be the last node in the stack of open elements.

        loop {  //3. Loop
            if node == stack_of_open_elements.first().cloned() {    // If node is the first node in the stack of open elements, then set last to true
                last = true;
                if is_fragment_case {   // and, if the parser was created as part of the HTML fragment parsing algorithm (fragment case), set node to the context element passed to that algorithm.
                    node = context_element.cloned();
                }
            }

            match node {    //4. If node is a select element
                Some(ref node) if node.is_select_element() => {
                    if last {   //4.1. If last is true, jump to the step below labeled done.
                        return InsertionMode::InSelect;
                    }

                    let mut ancestor = node.clone(); // 4.2. Let ancestor be node.
                    loop {  // 4.3. Loop
                        if ancestor == stack_of_open_elements.first().cloned() {
                            break;  // If ancestor is the first node in the stack of open elements, jump to the step below labeled done.
                        }
                        // 4.4. Let ancestor be the node before ancestor in the stack of open elements.
                        ancestor = ancestor.get_previous_in_stack(stack_of_open_elements);
                        
                        if ancestor.is_template_element() {
                            break;  //4.5. If ancestor is a template node, jump to the step below labeled done.
                        }

                        if ancestor.is_table_element() {
                            // 4.6. If ancestor is a table node, switch the insertion mode to "in select in table" and return.
                            return InsertionMode::InSelectInTable;
                        }
                        // 4.7. Jump back to the step labeled loop.
                    }

                    return InsertionMode::InSelect;
                }
                Some(ref node) if node.is_td() && node.is_th() && !last => return InsertionMode::InCell,
                Some(ref node) if node.is_tr() => return InsertionMode::InRow,
                Some(ref node) if node.is_table_section() => return InsertionMode::InTableBody,
                Some(ref node) if node.is_caption() => return InsertionMode::InCaption,
                Some(ref node) if node.is_colgroup() => return InsertionMode::InColumnGroup,
                Some(ref node) if node.is_table() => return InsertionMode::InTable,
                Some(ref node) if node.is_template() => {
                    // ????????????????????
                    return InsertionMode::InTemplate;
                }
                Some(ref node) if node.is_head() && !last => return InsertionMode::InHead,
                Some(ref node) if node.is_body() => return InsertionMode::InBody,
                Some(ref node) if node.is_frameset() => return InsertionMode::InFrameset,
                Some(ref node) if node.is_html() => {
                    if is_fragment_case && node.has_no_head() {
                        return InsertionMode::BeforeHead;
                    } else {
                        return InsertionMode::AfterHead;
                    }
                }
                _ => {
                    if last {
                        return InsertionMode::InBody;
                    }

                    node = node.and_then(|n| n.get_previous_in_stack(stack_of_open_elements));
                }
            }
        }
    }
}

// Helper functions for node types (You should implement these based on your DOM node structure)
trait NodeHelpers {
    fn is_select_element(&self) -> bool;
    fn is_td(&self) -> bool;
    fn is_th(&self) -> bool;
    fn is_tr(&self) -> bool;
    fn is_table_section(&self) -> bool;
    fn is_caption(&self) -> bool;
    fn is_colgroup(&self) -> bool;
    fn is_table(&self) -> bool;
    fn is_template(&self) -> bool;
    fn is_head(&self) -> bool;
    fn is_body(&self) -> bool;
    fn is_frameset(&self) -> bool;
    fn is_html(&self) -> bool;
    fn has_no_head(&self) -> bool;
    fn get_previous_in_stack(&self, stack: &[Node]) -> Option<Node>;
}

// Assume `Node` is your representation of a DOM node
#[derive(Clone)]
pub struct Node {
    // Node fields here
}

// Implement your helper methods for Node
impl NodeHelpers for Node {
    fn is_select_element(&self) -> bool { /* Implement logic */ false }
    fn is_td(&self) -> bool { /* Implement logic */ false }
    fn is_th(&self) -> bool { /* Implement logic */ false }
    fn is_tr(&self) -> bool { /* Implement logic */ false }
    fn is_table_section(&self) -> bool { /* Implement logic */ false }
    fn is_caption(&self) -> bool { /* Implement logic */ false }
    fn is_colgroup(&self) -> bool { /* Implement logic */ false }
    fn is_table(&self) -> bool { /* Implement logic */ false }
    fn is_template(&self) -> bool { /* Implement logic */ false }
    fn is_head(&self) -> bool { /* Implement logic */ false }
    fn is_body(&self) -> bool { /* Implement logic */ false }
    fn is_frameset(&self) -> bool { /* Implement logic */ false }
    fn is_html(&self) -> bool { /* Implement logic */ false }
    fn has_no_head(&self) -> bool { /* Implement logic */ false }
    fn get_previous_in_stack(&self, stack: &[Node]) -> Option<Node> {
        // Implement logic to get the previous node in the stack
        None
    }
}
