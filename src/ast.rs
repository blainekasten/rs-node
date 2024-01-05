use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum NodeASTType {
    Unknown,
    VariableDeclarator,
    VariableDeclaration,
    VariableTypeSeperator,
    TypeAnnotation,
    AssignmentOperator,
    KeywordFunction,
    KeywordDeclare,
    KeywordInterface,
    KeywordType,
    FunctionDeclaration,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Identifier,
    ExportDeclaration,
    WhiteSpace,
}

impl fmt::Display for NodeASTType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeASTType::Unknown => "Unknown",
                NodeASTType::VariableDeclarator => "VariableDeclarator",
                NodeASTType::VariableDeclaration => "VariableDeclaration",
                NodeASTType::VariableTypeSeperator => "VariableTypeSeperator",
                NodeASTType::AssignmentOperator => "AssignmentOperator",
                NodeASTType::TypeAnnotation => "TypeAnnotation",
                NodeASTType::KeywordFunction => "KeywordFunction",
                NodeASTType::KeywordDeclare => "KeywordDeclare",
                NodeASTType::KeywordType => "KeywordType",
                NodeASTType::KeywordInterface => "KeywordInterface",
                NodeASTType::ExportDeclaration => "ExportDeclaration",
                NodeASTType::FunctionDeclaration => "FunctionDeclaration",
                NodeASTType::OpeningBracket => "OpeningBracket",
                NodeASTType::ClosingBracket => "ClosingBracket",
                NodeASTType::OpeningParenthesis => "OpeningParenthesis",
                NodeASTType::ClosingParenthesis => "ClosingParenthesis",
                NodeASTType::Identifier => "Identifier",
                NodeASTType::WhiteSpace => "WhiteSpace",
            }
        )
    }
}
