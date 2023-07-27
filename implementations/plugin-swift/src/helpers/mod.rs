use handlebars::Handlebars;

mod array_has_length;
mod array_length;
mod detect_keyword;
mod is_keyword;
mod is_not_first;
mod is_not_last;
mod pretty;
mod to_graphql_type;
mod to_lower;
mod to_upper;
mod to_swift;

// helpers for helpers
mod util;

pub fn register(handlebars: &mut Handlebars) -> () {
    handlebars.register_helper(
        "array_has_length",
        Box::new(array_has_length::array_has_length)
    );
    handlebars.register_helper(
        "array_length",
        Box::new(array_length::array_length)
    );
    handlebars.register_helper(
        "detect_keyword",
        Box::new(detect_keyword::detect_keyword)
    );
    handlebars.register_helper(
        "is_keyword",
        Box::new(is_keyword::is_keyword)
    );
    handlebars.register_helper(
        "is_not_first",
        Box::new(is_not_first::is_not_first)
    );
    handlebars.register_helper(
        "is_not_last",
        Box::new(is_not_last::is_not_last)
    );
    handlebars.register_helper(
        "pretty",
        Box::new(pretty::pretty)
    );
    handlebars.register_helper(
        "to_graphql_type",
        Box::new(to_graphql_type::to_graphql_type)
    );
    handlebars.register_helper(
        "to_lower",
        Box::new(to_lower::to_lower)
    );
    handlebars.register_helper(
        "to_upper",
        Box::new(to_upper::to_upper)
    );
    handlebars.register_helper(
        "to_swift",
        Box::new(to_swift::to_swift)
    );
}
