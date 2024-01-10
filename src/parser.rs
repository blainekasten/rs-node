#[path = "./ast.rs"]
mod ast;
#[path = "./pauser.rs"]
mod pauser;

use ast::NodeASTType;
use pauser::{KeywordDeclarePauser, Pauser};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use self::pauser::{CommentPauser, KeywordAsPauser, KeywordTypePauser};

const module_node: Node = Node {
    parent: None,
    node_type: NodeASTType::Module,
    value: String::new()
}

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
    keyword_as_pauser: KeywordAsPauser,
    comment_pauser: CommentPauser,
}

impl Tree {
    fn new() -> Tree {
        return Tree {
            output: String::new(),
            id_counter: 0,
            nodes: vec![Rc::new(RefCell::new(module_node))],
            current_node: Rc::new(RefCell::new(Node {
                parent: Rc::new(RefCell::new(module_node)),
                value: String::new(),
                node_type: NodeASTType::Unknown,
            })),
            is_paused: false,
            paused_node_type: NodeASTType::Unknown,

            // pausers
            keyword_declare_pauser: KeywordDeclarePauser::new(),
            keyword_type_pauser: KeywordTypePauser::new(),
            keyword_as_pauser: KeywordAsPauser::new(),
            comment_pauser: CommentPauser::new(),
        };
    }

    pub fn debug(&self) {
        for token in self.nodes.iter() {
            let t = token.clone();
            println!("node: {}", t.borrow());
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
            self.is_paused = match self.paused_node_type {
                NodeASTType::KeywordType => self
                    .keyword_type_pauser
                    .is_paused_after_evaluating(node_type),
                NodeASTType::KeywordDeclare => self
                    .keyword_declare_pauser
                    .is_paused_after_evaluating(node_type),
                NodeASTType::KeywordAs => {
                    self.keyword_as_pauser.is_paused_after_evaluating(node_type)
                }
                NodeASTType::CommentLine => {
                    self.comment_pauser.is_paused_after_evaluating(node_type)
                }

                NodeASTType::CommentMultilineOpener => {
                    self.comment_pauser.is_paused_after_evaluating(node_type)
                }
                _ => false,
            }
        }
    }

    pub fn update_current_value(&mut self, value: &str) {
        let mut mut_node = self.current_node.borrow_mut();
        mut_node.value = value.to_string();
    }

    pub fn commit(&mut self) -> Node {
        // Derive information about node before comitting
        {
            let mut committed_node = RefCell::borrow_mut(&self.current_node);
            committed_node.node_type = committed_node.detect_type();
        }
        let returnable_node = self.current_node.clone().as_ref().borrow().clone();
        let current_node = Rc::clone(&self.current_node.clone());

        if self.is_paused == false {
            match returnable_node.node_type {
                NodeASTType::KeywordDeclare
                | NodeASTType::KeywordType
                | NodeASTType::KeywordInterface
                | NodeASTType::CommentLine
                | NodeASTType::CommentMultilineOpener
                | NodeASTType::KeywordAs => self.pause_writing(returnable_node.node_type),
                _ => {}
            }

            if self.is_paused == false {
                match self.current_node.borrow().node_type {
                    NodeASTType::WhiteSpace => {}
                    NodeASTType::EOL => {}
                    NodeASTType::TypeAnnotation => {}
                    NodeASTType::KeywordInterface => {}
                    NodeASTType::KeywordType => {}
                    NodeASTType::KeywordDeclare => {}
                    NodeASTType::VariableTypeSeperator => {}
                    _ => {
                        self.output += self.current_node.borrow().value.to_string().as_str();
                        self.output += self.current_node.borrow().seperator().to_string().as_str();
                    }
                }
            }
        }

        if self.is_paused {
            self.consider_resuming_writing(returnable_node.node_type);
        }

        // If the node was just white space we dont want to keep it in our list
        // of nodes to print
        if self.current_node.borrow().value.trim() == "" {
            current_node.borrow_mut().reset();
            return returnable_node;
        }

        // commit and
        // reset the current node tree for the next characters
        self.nodes.push(current_node.clone());
        self.current_node = Rc::new(RefCell::new(Node {
            parent: parent,
            value: String::new(),
            node_type: NodeASTType::Unknown,
        }));
        return returnable_node;
    }
}

#[derive(Clone)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    value: String,
    node_type: NodeASTType,
}

impl Node {
    fn reset(&mut self) {
        self.value = String::new();
        self.node_type = NodeASTType::Unknown;
    }

    pub fn seperator(&self) -> &str {
        match self.node_type {
            NodeASTType::KeywordFunction => " ",
            NodeASTType::ExportDeclaration => " ",
            NodeASTType::VariableDeclarator => " ",
            _ => "",
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.parent {
            Some(parent) => {
                let parent_type = parent.borrow().node_type;
                if parent_type == NodeASTType::WhiteSpace || parent_type == NodeASTType::EOL {
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

    pub fn detect_type(&self) -> NodeASTType {
        match self.value.as_str() {
            "declare" => NodeASTType::KeywordDeclare,
            "export" => NodeASTType::ExportDeclaration,
            "const" => NodeASTType::VariableDeclarator,
            "let" => NodeASTType::VariableDeclarator,
            "var" => NodeASTType::VariableDeclarator,
            "function" => NodeASTType::KeywordFunction,
            "as" => NodeASTType::KeywordAs,
            "interface" => NodeASTType::KeywordInterface,
            "type" => NodeASTType::KeywordType,
            "{" => NodeASTType::OpeningBracket,
            "," => NodeASTType::CommaSeperator,
            "}" => NodeASTType::ClosingBracket,
            "(" => NodeASTType::OpeningParenthesis,
            ")" => NodeASTType::ClosingParenthesis,
            "[" => NodeASTType::OpeningBrace,
            "||" => NodeASTType::OrStatement,
            "|" => NodeASTType::TypeUnionSeperator,
            "]" => NodeASTType::ClosingBrace,
            ";" => NodeASTType::Terminator,
            "//" => NodeASTType::CommentLine,
            "/*" => NodeASTType::CommentMultilineOpener,
            "*/" => NodeASTType::CommentMultilineCloser,
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
            "\n" => NodeASTType::EOL,
            "" => NodeASTType::WhiteSpace,
            "=" => {
                // Do we really need this to consider all the parent types?
                // Should this just always be an assignment operator??
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
        write!(
            f,
            "({}: {}, seperator: \"{}\")",
            self.node_type,
            self.value,
            self.seperator()
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
                tree.commit();
            }
            '\n' => {
                // end of previous node, commit this one.
                tree.commit();
                tree.update_current_value("\n");
                tree.commit();
            }
            '{' => {
                tree.commit();
                tree.update_current_value("{");
                tree.commit();
            }
            '}' => {
                tree.commit();
                tree.update_current_value("}");
                tree.commit();
            }
            '.' => {
                // end of previous node, commit this one.
                tree.commit();
                tree.update_current_value(".");
                tree.commit();
            }
            ':' => {
                // start of annotation node.
                // commit previous node.
                tree.commit();
                tree.update_current_value(":");
                // tree.commit();
            }
            ';' => {
                // end of previous node, commit this one.
                tree.commit();
                tree.update_current_value(";");
                tree.commit();
            }
            '[' => {
                tree.commit();
                tree.update_current_value("[");
                tree.commit();
            }
            ']' => {
                tree.commit();
                tree.update_current_value("]");
                tree.commit();
            }
            '(' => {
                tree.commit();
                tree.update_current_value("(");
                tree.commit();
            }
            ')' => {
                tree.commit();
                tree.update_current_value(")");
                tree.commit();
            }
            _ => {
                let mut value = tree.current_node.borrow_mut().value.to_owned();
                value.push(char);

                tree.update_current_value(value.as_str());

                // Have to handle this here because rust doesnt like me creating a string with
                // single quotes with // in it.
                if tree.current_node.borrow().value == "//".to_string() {
                    tree.commit();
                }
                if tree.current_node.borrow().value == "/*".to_string() {
                    tree.commit();
                }
                if tree.current_node.borrow().value == "*/".to_string() {
                    tree.commit();
                }
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
