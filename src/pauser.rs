use super::ast::NodeASTType;

pub trait Pauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool;
}

pub struct NotPauser {}
impl NotPauser {
    pub fn new() -> NotPauser {
        NotPauser {}
    }
}
impl Pauser for NotPauser {
    fn is_paused_after_evaluating(&mut self, _next_type: NodeASTType) -> bool {
        true
    }
}

pub struct CommentPauser {
    is_singline: bool,
    is_multiline: bool,
}
impl CommentPauser {
    pub fn new() -> CommentPauser {
        CommentPauser {
            is_singline: false,
            is_multiline: false,
        }
    }

    fn reset(&mut self) {
        self.is_singline = false;
        self.is_multiline = false;
    }
}
impl Pauser for CommentPauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        let is_paused = match next_type {
            NodeASTType::CommentLine => {
                self.is_singline = true;
                true
            }
            NodeASTType::CommentMultilineOpener => {
                self.is_multiline = true;
                true
            }
            _ => {
                if self.is_singline {
                    return next_type == NodeASTType::EOL;
                }

                if self.is_multiline {
                    return next_type != NodeASTType::CommentMultilineCloser;
                }

                false
            }
        };

        if !is_paused {
            self.reset()
        }

        is_paused
    }
}

/// `type` pause writing until type definition is completed.
pub struct KeywordTypePauser {
    opening_node: NodeASTType,
    is_passed_assignment_operator: bool,
    opening_bracket_count: u32,
    closing_bracket_count: u32,
    opening_brace_count: u32,
    closing_brace_count: u32,
}

impl KeywordTypePauser {
    pub fn new() -> KeywordTypePauser {
        KeywordTypePauser {
            opening_node: NodeASTType::Unknown,
            is_passed_assignment_operator: false,
            opening_bracket_count: 0,
            closing_bracket_count: 0,
            opening_brace_count: 0,
            closing_brace_count: 0,
        }
    }

    fn reset(&mut self) {
        self.opening_node = NodeASTType::Unknown;
        self.is_passed_assignment_operator = false;
        self.opening_bracket_count = 0;
        self.closing_bracket_count = 0;
        self.opening_brace_count = 0;
        self.closing_brace_count = 0;
    }

    fn evaluate_array_type(&mut self, next_type: NodeASTType) -> bool {
        if next_type == NodeASTType::OpeningBrace {
            self.opening_brace_count += 1;
        }
        if next_type == NodeASTType::ClosingBrace {
            self.closing_brace_count += 1;
        }

        match next_type {
            NodeASTType::Terminator => {
                self.reset();

                if self.opening_brace_count != self.closing_brace_count {
                    return true;
                }
                false
            }
            _ => true,
        }
    }

    fn evaluate_object_type(&mut self, next_type: NodeASTType) -> bool {
        if next_type == NodeASTType::OpeningBracket {
            self.opening_bracket_count += 1;
        }
        if next_type == NodeASTType::ClosingBracket {
            self.closing_bracket_count += 1;
        }

        if self.opening_bracket_count != self.closing_bracket_count {
            // still paused
            return true;
        }

        match next_type {
            NodeASTType::Terminator => {
                self.reset();
                false
            }
            _ => true,
        }
    }
}
impl Pauser for KeywordTypePauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        if self.is_passed_assignment_operator == false {
            // We dont care about anything before the assignment operator. Thats when things get real.
            if next_type == NodeASTType::AssignmentOperator {
                self.is_passed_assignment_operator = true;
            } else {
                return true;
            }
        }

        // This is the block at which we are getting real.
        // Let's figure out what type of operators were going to be planning forward for.
        if self.is_passed_assignment_operator && self.opening_node == NodeASTType::Unknown {
            self.opening_node = match next_type {
                NodeASTType::OpeningBracket => next_type,
                NodeASTType::OpeningBrace => next_type,
                NodeASTType::Identifier => next_type,
                unknown => {
                    println!("{}", unknown);
                    NodeASTType::Unknown
                }
            };
        }

        // This handles object types
        if self.opening_node == NodeASTType::OpeningBracket {
            return self.evaluate_object_type(next_type);
        }
        if self.opening_node == NodeASTType::OpeningBrace {
            return self.evaluate_array_type(next_type);
        }

        if self.opening_node == NodeASTType::Identifier {
            self.reset();
            return false;
        }

        println!(
            "returning is paused. {} {} {}",
            next_type, self.opening_node, self.opening_brace_count
        );
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

    fn reset(&mut self) {
        self.opening_brace_count = 0;
        self.closing_brace_count = 0;
    }
}

impl Pauser for KeywordDeclarePauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        if next_type == NodeASTType::OpeningBracket {
            self.opening_brace_count += 1;
        } else if next_type == NodeASTType::ClosingBracket {
            self.closing_brace_count += 1;
        }

        if self.opening_brace_count > 0 && self.opening_brace_count == self.closing_brace_count {
            self.reset();
            return false;
        }

        true
    }
}

/// `as` pause writing until type definition is completed.
pub struct KeywordAsPauser {}
impl KeywordAsPauser {
    pub fn new() -> KeywordAsPauser {
        KeywordAsPauser {}
    }
}
impl Pauser for KeywordAsPauser {
    fn is_paused_after_evaluating(&mut self, next_type: NodeASTType) -> bool {
        match next_type {
            NodeASTType::Terminator => false,
            _ => true,
        }
    }
}
