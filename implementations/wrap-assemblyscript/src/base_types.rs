use std::collections::HashSet;

lazy_static! {
    static ref BASE_TYPES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("u8");
        set.insert("u16");
        set.insert("u32");
        set.insert("i8");
        set.insert("i16");
        set.insert("i32");
        set.insert("string");
        set.insert("bool");
        set
    };
}

pub type BaseType = &'static str;

pub fn is_base_type(type_: &str) -> bool {
    BASE_TYPES.contains(type_)
}
