use super::ast::NodeASTType;

pub trait Pauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool;
}

pub struct NotPauser {}
impl Pauser for NotPauser {
    fn is_paused_after_evaluating(&mut self, _next_type: NodeASTType) -> bool {
        true
    }
}

/// `type` pause writing until type definition is completed.
pub struct KeywordTypePauser {
    opening_node: NodeASTType,
    is_passed_assignment_operator: bool,
}

impl KeywordTypePauser {
    pub fn new() -> KeywordTypePauser {
        KeywordTypePauser {
            opening_node: NodeASTType::Unknown,
            is_passed_assignment_operator: false,
        }
    }
}
impl Pauser for KeywordTypePauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        if self.is_passed_assignment_operator == false {
            // We dont care about anything before the assignment operator. Thats when things get real.
            if self.opening_node != NodeASTType::AssignmentOperator {
                return true;
            }
            println!("assigning. {}", next_type);
        }

        if self.is_passed_assignment_operator && self.opening_node == NodeASTType::Unknown {
            self.opening_node = next_type;
        }

        if self.opening_node == NodeASTType::Identifier {
            return false;
        }

        return true;
    }
}

/// `declare` pause writing until type definition is completed.
/// We also use this same pauser for the `interface` keyword
/// because they follow the same grammar
pub struct KeywordDeclarePauser {
    opening_brace_count: u32,
    closing_brace_count: u32,
}

impl KeywordDeclarePauser {
    pub fn new() -> KeywordDeclarePauser {
        KeywordDeclarePauser {
            opening_brace_count: 0,
            closing_brace_count: 0,
        }
    }
}

impl Pauser for KeywordDeclarePauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        if next_type == NodeASTType::OpeningBracket {
            self.opening_brace_count += 1;
        } else if next_type == NodeASTType::ClosingBracket {
            self.closing_brace_count += 1;
        }

        !(self.opening_brace_count > 0 && self.opening_brace_count == self.closing_brace_count)
    }
}
