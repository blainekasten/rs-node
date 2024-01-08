use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum NodeASTType {
    Unknown,
    EOL,
    VariableDeclarator,
    VariableDeclaration,
    VariableTypeSeperator,
    TypeAnnotation,
    TypeUnionSeperator,
    OrStatement,
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
    OpeningBrace,
    ClosingBrace,
    Identifier,
    ExportDeclaration,
    WhiteSpace,
    Terminator,
    CommentLine,
    CommentMultilineOpener,
    CommentMultilineCloser,
}

impl fmt::Display for NodeASTType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeASTType::Unknown => "Unknown",
                NodeASTType::EOL => "EOL",
                NodeASTType::VariableDeclarator => "VariableDeclarator",
                NodeASTType::VariableDeclaration => "VariableDeclaration",
                NodeASTType::VariableTypeSeperator => "VariableTypeSeperator",
                NodeASTType::AssignmentOperator => "AssignmentOperator",
                NodeASTType::TypeAnnotation => "TypeAnnotation",
                NodeASTType::TypeUnionSeperator => "TypeUnionSeperator",
                NodeASTType::OrStatement => "OrStatement",
                NodeASTType::KeywordFunction => "KeywordFunction",
                NodeASTType::KeywordDeclare => "KeywordDeclare",
                NodeASTType::KeywordType => "KeywordType",
                NodeASTType::KeywordInterface => "KeywordInterface",
                NodeASTType::ExportDeclaration => "ExportDeclaration",
                NodeASTType::FunctionDeclaration => "FunctionDeclaration",
                NodeASTType::OpeningBracket => "OpeningBracket",
                NodeASTType::ClosingBracket => "ClosingBracket",
                NodeASTType::OpeningBrace => "OpeningBrace",
                NodeASTType::ClosingBrace => "ClosingBrace",
                NodeASTType::OpeningParenthesis => "OpeningParenthesis",
                NodeASTType::ClosingParenthesis => "ClosingParenthesis",
                NodeASTType::Identifier => "Identifier",
                NodeASTType::WhiteSpace => "WhiteSpace",
                NodeASTType::Terminator => "Terminator",
                NodeASTType::CommentLine => "CommentLine",
                NodeASTType::CommentMultilineOpener => "CommentMultilineOpener",
                NodeASTType::CommentMultilineCloser => "CommentMultilineCloser",
            }
        )
    }
}
