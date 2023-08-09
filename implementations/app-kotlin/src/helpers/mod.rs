use handlebars::Handlebars;

mod array_has_length;
mod array_length;
mod detect_keyword;
mod detect_keyword_strict;
mod is_keyword;
mod is_not_first;
mod is_not_last;
mod is_soft_keyword;
mod nullable_default;
mod to_class_name;
mod to_graphql_type;
mod to_kotlin;
mod to_kotlin_byte_array;
mod to_package_id;
mod to_upper;

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
        "detect_keyword_strict",
        Box::new(detect_keyword_strict::detect_keyword_strict)
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
        "is_soft_keyword",
        Box::new(is_soft_keyword::is_soft_keyword)
    );
    handlebars.register_helper(
        "nullable_default",
        Box::new(nullable_default::nullable_default)
    );
    handlebars.register_helper(
        "to_class_name",
        Box::new(to_class_name::to_class_name)
    );
    handlebars.register_helper(
        "to_graphql_type",
        Box::new(to_graphql_type::to_graphql_type)
    );
    handlebars.register_helper(
        "to_kotlin",
        Box::new(to_kotlin::to_kotlin)
    );
    handlebars.register_helper(
        "to_kotlin_byte_array",
        Box::new(to_kotlin_byte_array::to_kotlin_byte_array)
    );
    handlebars.register_helper(
        "to_package_id",
        Box::new(to_package_id::to_package_id)
    );
    handlebars.register_helper(
        "to_upper",
        Box::new(to_upper::to_upper)
    );
}
