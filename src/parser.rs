#[path = "./ast.rs"]
mod ast;
#[path = "./pauser.rs"]
mod pauser;

use ast::NodeASTType;
use pauser::{KeywordDeclarePauser, Pauser};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use self::pauser::KeywordTypePauser;

struct Tree {
    output: String,
    nodes: Vec<Rc<RefCell<Node>>>,
    current_node: Rc<RefCell<Node>>,
    id_counter: i8,

    // private
    is_paused: bool,
    paused_node_type: NodeASTType,

    // pausers
    keyword_declare_pauser: KeywordDeclarePauser,
    keyword_type_pauser: KeywordTypePauser,
}

impl Tree {
    fn new() -> Tree {
        return Tree {
            output: String::new(),
            id_counter: 0,
            nodes: vec![],
            current_node: Rc::new(RefCell::new(Node {
                id: 0,
                parent: None,
                value: String::new(),
                node_type: NodeASTType::Unknown,
            })),
            is_paused: false,
            paused_node_type: NodeASTType::Unknown,

            // pausers
            keyword_declare_pauser: KeywordDeclarePauser::new(),
            keyword_type_pauser: KeywordTypePauser::new(),
        };
    }

    pub fn debug(&self) {
        for token in self.nodes.iter() {
            // let t = *token;
            let t = Rc::clone(token);
            println!("node: {}", t.as_ref().borrow());
        }
    }

    pub fn pause_writing(&mut self, node_type: NodeASTType) {
        self.is_paused = true;
        self.paused_node_type = node_type;
    }

    // For a given node type that paused writing, this will handle the logic
    // that would consider resuming. For example, if the node is paused for a declare keyword
    // it will only resume once the number of opening and closing brackets are equal.
    pub fn consider_resuming_writing(&mut self, node_type: NodeASTType) {
        if self.is_paused {
            match self.paused_node_type {
                NodeASTType::KeywordType => {
                    self.is_paused = self
                        .keyword_type_pauser
                        .is_paused_after_evaluating(node_type);
                }
                NodeASTType::KeywordDeclare => {
                    self.is_paused = self
                        .keyword_declare_pauser
                        .is_paused_after_evaluating(node_type);
                }
                _ => {
                    self.is_paused = false;
                }
            }
        }
    }

    pub fn update_current_value(&mut self, value: &str) {
        let mut mut_node = self.current_node.borrow_mut();
        mut_node.value = value.to_string();
    }

    pub fn commit(&mut self, seperator: &str) -> Node {
        // Derive information about node before comitting
        {
            let mut committed_node = RefCell::borrow_mut(&self.current_node);
            committed_node.node_type = committed_node.detect_type();
        }
        let returnable_node = self.current_node.clone().as_ref().borrow().clone();
        let current_node = Rc::clone(&self.current_node.clone());

        if returnable_node.node_type == NodeASTType::KeywordDeclare {
            self.pause_writing(NodeASTType::KeywordDeclare);
        }
        if returnable_node.node_type == NodeASTType::KeywordType {
            self.pause_writing(NodeASTType::KeywordType);
        }
        if returnable_node.node_type == NodeASTType::KeywordInterface {
            self.pause_writing(NodeASTType::KeywordDeclare);
        }

        if self.is_paused == false {
            match self.current_node.borrow().node_type {
                NodeASTType::WhiteSpace => {}
                NodeASTType::TypeAnnotation => {}
                NodeASTType::KeywordInterface => {}
                NodeASTType::KeywordType => {}
                NodeASTType::KeywordDeclare => {}
                NodeASTType::VariableTypeSeperator => {}
                _ => {
                    self.output = format!(
                        "{}{}{}",
                        self.output,
                        self.current_node.borrow().value.to_string(),
                        seperator
                    );
                }
            }
        }

        self.consider_resuming_writing(returnable_node.node_type);

        // commit and
        // reset the current node tree for the next characters
        self.nodes.push(current_node.clone());
        self.id_counter += 1;
        self.current_node = Rc::new(RefCell::new(Node {
            id: self.id_counter,
            parent: Some(current_node.clone()),
            value: String::new(),
            node_type: NodeASTType::Unknown,
        }));
        return returnable_node;
    }
}

#[derive(Clone)]
struct Node {
    id: i8,
    parent: Option<Rc<RefCell<Node>>>,
    value: String,
    node_type: NodeASTType,
}

impl Node {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.parent {
            Some(parent) => {
                if parent.borrow().node_type == NodeASTType::WhiteSpace {
                    return match parent.borrow().get_parent() {
                        Some(node) => Some(node),
                        None => None,
                    };
                }

                return match self.parent.clone() {
                    Some(node) => Some(node),
                    None => None,
                };
            }
            None => {
                return None;
            }
        }
    }

    fn crawl_parent_for(&self, node_type: NodeASTType) -> Option<Rc<RefCell<Node>>> {
        let mut parent = self.get_parent();
        loop {
            match parent {
                Some(node) => {
                    let node_type = node.borrow().node_type;
                    if node_type == node_type {
                        return Some(node);
                    }
                    parent = node.borrow().get_parent();
                }
                None => {
                    return None;
                }
            }
        }
    }

    pub fn detect_type(&self) -> NodeASTType {
        match self.value.as_str() {
            "declare" => NodeASTType::KeywordDeclare,
            "export" => NodeASTType::ExportDeclaration,
            "const" => NodeASTType::VariableDeclarator,
            "function" => NodeASTType::KeywordFunction,
            "interface" => NodeASTType::KeywordInterface,
            "type" => NodeASTType::KeywordType,
            "{" => NodeASTType::OpeningBracket,
            "}" => NodeASTType::ClosingBracket,
            "(" => NodeASTType::OpeningParenthesis,
            ")" => NodeASTType::ClosingParenthesis,
            ":" => {
                let parent = self.get_parent();
                let parent_type = match parent {
                    Some(node) => node.borrow().node_type,
                    None => NodeASTType::Unknown,
                };
                if parent_type == NodeASTType::VariableDeclaration {
                    return NodeASTType::VariableTypeSeperator;
                }
                if parent_type == NodeASTType::Identifier {
                    let grandparent = self.get_parent().unwrap().borrow().get_parent();
                    let grandparent_type = match grandparent {
                        Some(node) => node.borrow().node_type,
                        None => NodeASTType::Unknown,
                    };
                    if grandparent_type == NodeASTType::OpeningParenthesis {
                        return NodeASTType::VariableTypeSeperator;
                    }
                }
                if parent_type == NodeASTType::ClosingParenthesis {
                    return NodeASTType::VariableTypeSeperator;
                }

                return NodeASTType::Unknown;
            }
            "" => {
                return NodeASTType::WhiteSpace;
            }
            "=" => {
                /// Do we really need this to consider all the parent types?
                /// Should this just always be an assignment operator??
                let parent_type = match self.get_parent() {
                    Some(node) => node.borrow().node_type,
                    None => NodeASTType::Unknown,
                };
                if parent_type == NodeASTType::VariableDeclaration {
                    return NodeASTType::AssignmentOperator;
                }
                if parent_type == NodeASTType::TypeAnnotation {
                    return NodeASTType::AssignmentOperator;
                }
                if parent_type == NodeASTType::KeywordType {
                    return NodeASTType::AssignmentOperator;
                }
                return NodeASTType::AssignmentOperator;
            }
            _ => {
                let parent_type = match self.get_parent() {
                    Some(node) => node.borrow().node_type,
                    None => NodeASTType::Unknown,
                };
                if parent_type == NodeASTType::KeywordFunction {
                    return NodeASTType::FunctionDeclaration;
                }
                if parent_type == NodeASTType::VariableTypeSeperator {
                    return NodeASTType::TypeAnnotation;
                }
                if parent_type == NodeASTType::VariableDeclarator {
                    return NodeASTType::VariableDeclaration;
                }
                return NodeASTType::Identifier;
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_id = match &self.parent {
            Some(s) => {
                let b = RefCell::clone(s);
                let id = b.borrow().id;
                id
            }
            None => 0,
        };

        write!(
            f,
            "(parent id: {}, value: {}, type: {})",
            parent_id, self.value, self.node_type
        )
    }
}

/// This should parse
pub fn parser(contents: String) -> String {
    let mut tree = Tree::new();

    for char in contents.chars() {
        match char {
            ' ' => {
                // end of previous node, commit this one.
                tree.commit(" ");
            }
            '\n' => {
                // end of previous node, commit this one.
                tree.commit("");
            }
            '{' => {
                tree.commit("");
                tree.update_current_value("{");
                tree.commit("");
            }
            '}' => {
                tree.commit("");
                tree.update_current_value("}");
                tree.commit("");
            }

            '.' => {
                // end of previous node, commit this one.
                tree.commit(".");
            }
            ':' => {
                // start of annotation node.
                // commit previous node.
                tree.commit("");
                tree.update_current_value(":");
                // tree.commit();
            }
            ';' => {
                // end of previous node, commit this one.
                tree.commit("");
                tree.update_current_value(";");
                tree.commit("");
            }
            '(' => {
                tree.commit("");
                tree.update_current_value("(");
                tree.commit("");
            }
            ')' => {
                tree.commit("");
                tree.update_current_value(")");
                tree.commit("");
            }
            _ => {
                let mut value = tree.current_node.borrow_mut().value.to_owned();
                value.push(char);

                tree.update_current_value(value.as_str())
            }
        }
    }

    tree.debug();

    return tree.output;
}

#[cfg(test)]
#[test]
fn it_works() {
    let result = parser(String::from("const A: boolean = true;"));
    assert_eq!(result, String::from(""));
}
