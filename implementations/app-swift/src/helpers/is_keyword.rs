use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_keyword: |val: Value| {
    let s = val.as_str().unwrap();
    _is_keyword(s)
});

pub fn _is_keyword(s: &str) -> bool {
    match s {
        // Keywords used in declarations
        "associatedtype" | "class" | "deinit" | "enum" | "extension" | "func" | "import" | "init" | "inout" | "internal" | "let" | "operator" | "private" | "protocol" | "public" | "static" | "struct" | "subscript" | "typealias" | "var" |
        // Keywords used in statements
        "break" | "case" | "catch" | "continue" | "default" | "defer" | "do" | "else" | "fallthrough" | "for" | "guard" | "if" | "in" | "repeat" | "return" | "throw" | "switch" | "where" | "while" |
        // Keywords used in expressions and types
        "Any" | "as" | "await" | "catch" | "false" | "is" | "nil" | "rethrows" | "self" | "Self" | "super" | "throw" | "throws" | "true" | "try" |
        // Keywords used in the specific context
        "associativity" | "convenience" | "didSet" | "dynamic" | "final" | "get" | "indirect" | "infix" | "lazy" | "left" | "mutating" | "none" | "nonmutating" | "optional" | "override" | "postfix" | "precedence" | "prefix" |
        "Protocol" | "required" | "right" | "set" | "some" | "Type" | "unowned" | "weak" | "willSet" |
        // Keywords that begin with the number sign
        "#available" | "#colorLiteral" | "#column" | "#dsohandle" | "#elseif" | "#else" | "#endif" | "#error" | "#fileID" | "#fileLiteral" | "#filePath" | "#file" | "#function" | "#if" | "#imageLiteral" | "#keyPath" | "#line" | "#selector" | "#sourceLocation" | "#warning" |
        // Keyword used in the patterns(_)
        "_" => true,
        _ => false,
    }
}
