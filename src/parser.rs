#[path = "./ast.rs"]
mod ast;
#[path = "./pauser.rs"]
mod pauser;

use ast::NodeASTType;
use pauser::{KeywordDeclarePauser, Pauser};

use std::fmt;

use self::pauser::{CommentPauser, KeywordAsPauser, KeywordTypePauser};

const MODULE_NODE: Node = Node {
    value: String::new(),
    parent: None,
    node_type: NodeASTType::Module,
};

struct Tree {
    output: String,
    current_token: String,
    nodes: Vec<Node>,

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
            current_token: String::new(),
            nodes: vec![MODULE_NODE],
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
            println!("node: {}", t);
        }
    }

    pub fn last_node(&self) -> Node {
        let node = self.nodes.last().expect("Must exist");
        return Node {
            value: node.value.clone(),
            parent: Some(Box::new(node.get_parent())),
            node_type: node.node_type,
        };
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
                NodeASTType::KeywordInterface | NodeASTType::KeywordDeclare => self
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
                _ => self.is_paused,
            };
        }
    }

    pub fn update_current_value(&mut self, value: &str) {
        self.current_token = value.to_string();
    }

    pub fn detect_type(&self, value: String) -> NodeASTType {
        match value.as_str() {
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
            "'" => NodeASTType::SingleQuote,
            "\"" => NodeASTType::DoubleQuote,
            ":" => {
                let parent = self.last_node();
                let parent_type = parent.node_type;
                if parent_type == NodeASTType::VariableDeclaration {
                    return NodeASTType::VariableTypeSeperator;
                }
                if parent_type == NodeASTType::Identifier {
                    let parent = self.last_node();
                    let grandparent = parent.get_parent();
                    let grandparent_type = grandparent.node_type;
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
                let parent_type = self.last_node().node_type;
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
                let parent_type = self.last_node().node_type;
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

    pub fn commit(&mut self) {
        // Derive information about node before comitting
        let value = self.current_token.clone();
        let node_type = self.detect_type(value);

        if self.is_paused == false {
            match node_type {
                NodeASTType::KeywordDeclare
                | NodeASTType::KeywordType
                | NodeASTType::KeywordInterface
                | NodeASTType::CommentLine
                | NodeASTType::CommentMultilineOpener
                | NodeASTType::KeywordAs => self.pause_writing(node_type),
                _ => {}
            }

            if self.is_paused == false {
                match node_type {
                    NodeASTType::WhiteSpace => {}
                    NodeASTType::EOL => {}
                    NodeASTType::TypeAnnotation => {}
                    NodeASTType::KeywordInterface => {}
                    NodeASTType::KeywordType => {}
                    NodeASTType::KeywordDeclare => {}
                    NodeASTType::VariableTypeSeperator => {}
                    _ => {
                        self.output += self.current_token.as_str();
                        self.output += seperator(node_type);
                    }
                }
            }
        }

        // If the node was just white space we dont want to keep it in our list
        // of nodes to print
        if self.current_token.trim() == "" {
            self.current_token = String::new();
        } else {
            // commit and
            // reset the current node tree for the next characters
            let parent = self.last_node();
            let token = self.current_token.clone();
            let node_type = self.detect_type(token.clone());
            let new_node = Node {
                parent: Some(Box::new(parent)),
                value: token,
                node_type: node_type,
            };
            self.current_token = String::new();
            self.nodes.push(new_node);
        }

        self.consider_resuming_writing(node_type);
    }
}

#[derive(Clone)]
struct Node {
    parent: Option<Box<Node>>,
    value: String,
    node_type: NodeASTType,
}

fn seperator(node_type: NodeASTType) -> &'static str {
    match node_type {
        NodeASTType::KeywordFunction => " ",
        NodeASTType::ExportDeclaration => " ",
        NodeASTType::VariableDeclarator => " ",
        _ => "",
    }
}

impl Node {
    fn get_parent(&self) -> Node {
        match &self.parent {
            Some(parent) => {
                let parent_type = parent.node_type;
                if parent_type == NodeASTType::WhiteSpace || parent_type == NodeASTType::EOL {
                    return parent.get_parent();
                }

                return Node {
                    value: parent.value.clone(),
                    parent: Some(Box::new(parent.get_parent())),
                    node_type: parent.node_type,
                };
            }
            None => MODULE_NODE,
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
            seperator(self.node_type)
        )
    }
}

/// This should parse
pub fn parser(contents: String) -> String {
    let mut tree = Tree::new();

    for char in contents.chars() {
        let parent = tree.last_node();
        match parent.node_type {
            NodeASTType::SingleQuote => {
                let mut value = tree.current_token.clone();
                value.push(char);

                tree.update_current_value(value.as_str());

                if char == '\'' {
                    tree.commit();
                }
                continue;
            }
            NodeASTType::DoubleQuote => {
                let mut value = tree.current_token.clone();
                value.push(char);

                tree.update_current_value(value.as_str());

                if char == '"' {
                    tree.commit();
                }
                continue;
            }
            _ => {}
        };

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
            '\'' => {
                tree.commit();
                tree.update_current_value("'");
                tree.commit();
            }
            '"' => {
                tree.commit();
                tree.update_current_value("\"");
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
                let mut value = tree.current_token.clone();
                value.push(char);

                tree.update_current_value(value.as_str());

                // Have to handle this here because rust doesnt like me creating a string with
                // single quotes with // in it.
                if tree.current_token == "//".to_string() {
                    tree.commit();
                }
                if tree.current_token == "/*".to_string() {
                    tree.commit();
                }
                if tree.current_token == "*/".to_string() {
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
