use std::collections::HashSet;
use handlebars::handlebars_helper;
use serde_json::{Value};

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("async");
        set.insert("await");
        set.insert("arguments");
        set.insert("abstract");
        set.insert("as");
        set.insert("break");
        set.insert("const");
        set.insert("class");
        set.insert("catch");
        set.insert("case");
        set.insert("continue");
        set.insert("constructor");
        set.insert("default");
        set.insert("declare");
        set.insert("do");
        set.insert("delete");
        set.insert("debugger");
        set.insert("else");
        set.insert("enum");
        set.insert("export");
        set.insert("extends");
        set.insert("for");
        set.insert("from");
        set.insert("function");
        set.insert("finally");
        set.insert("if");
        set.insert("in");
        set.insert("is");
        set.insert("implements");
        set.insert("import");
        set.insert("instanceof");
        set.insert("interface");
        set.insert("keyof");
        set.insert("let");
        set.insert("module");
        set.insert("new");
        set.insert("namespace");
        set.insert("of");
        set.insert("private");
        set.insert("package");
        set.insert("public");
        set.insert("protected");
        set.insert("return");
        set.insert("readonly");
        set.insert("switch");
        set.insert("static");
        set.insert("super");
        set.insert("this");
        set.insert("type");
        set.insert("try");
        set.insert("throw");
        set.insert("typeof");
        set.insert("var");
        set.insert("while");
        set.insert("with");
        set.insert("yield");
        set.insert("i8");
        set.insert("i16");
        set.insert("i32");
        set.insert("i64");
        set.insert("isize");
        set.insert("u8");
        set.insert("u16");
        set.insert("u32");
        set.insert("u64");
        set.insert("usize");
        set.insert("bool");
        set.insert("f32");
        set.insert("f64");
        set.insert("v128");
        set.insert("funcref");
        set.insert("externref");
        set.insert("anyref");
        set.insert("eqref");
        set.insert("i31ref");
        set.insert("dataref");
        set.insert("i8x16");
        set.insert("u8x16");
        set.insert("i16x8");
        set.insert("u16x8");
        set.insert("i32x4");
        set.insert("u32x4");
        set.insert("i64x2");
        set.insert("u64x2");
        set.insert("f32x4");
        set.insert("f64x2");
        set.insert("void");
        set.insert("number");
        set.insert("boolean");
        set.insert("string");
        set.insert("native");
        set.insert("indexof");
        set.insert("valueof");
        set.insert("returnof");
        set.insert("nonnull");
        set.insert("null");
        set.insert("true");
        set.insert("false");
        set.insert("ASC_TARGET");
        set.insert("ASC_RUNTIME");
        set.insert("ASC_NO_ASSERT");
        set.insert("ASC_MEMORY_BASE");
        set.insert("ASC_TABLE_BASE");
        set.insert("ASC_OPTIMIZE_LEVEL");
        set.insert("ASC_SHRINK_LEVEL");
        set.insert("ASC_LOW_MEMORY_LIMIT");
        set.insert("ASC_EXPORT_RUNTIME");
        set.insert("ASC_WASI");
        set.insert("ASC_FEATURE_SIGN_EXTENSION");
        set.insert("ASC_FEATURE_MUTABLE_GLOBALS");
        set.insert("ASC_FEATURE_NONTRAPPING_F2I");
        set.insert("ASC_FEATURE_BULK_MEMORY");
        set.insert("ASC_FEATURE_SIMD");
        set.insert("ASC_FEATURE_THREADS");
        set.insert("ASC_FEATURE_EXCEPTION_HANDLING");
        set.insert("ASC_FEATURE_TAIL_CALLS");
        set.insert("ASC_FEATURE_REFERENCE_TYPES");
        set.insert("ASC_FEATURE_MULTI_VALUE");
        set.insert("ASC_FEATURE_GC");
        set.insert("ASC_FEATURE_MEMORY64");
        set.insert("ASC_VERSION_MAJOR");
        set.insert("ASC_VERSION_MINOR");
        set.insert("ASC_VERSION_PATCH");
        set.insert("I8");
        set.insert("I16");
        set.insert("I32");
        set.insert("I64");
        set.insert("Isize");
        set.insert("U8");
        set.insert("U16");
        set.insert("U32");
        set.insert("U64");
        set.insert("Usize");
        set.insert("Bool");
        set.insert("F32");
        set.insert("F64");
        set.insert("V128");
        set.insert("Funcref");
        set.insert("Externref");
        set.insert("Anyref");
        set.insert("Eqref");
        set.insert("I31ref");
        set.insert("Dataref");
        set.insert("String");
        set.insert("Object");
        set.insert("Array");
        set.insert("StaticArray");
        set.insert("Set");
        set.insert("Map");
        set.insert("Function");
        set.insert("ArrayBufferView");
        set.insert("ArrayBuffer");
        set.insert("Math");
        set.insert("Mathf");
        set.insert("NativeMath");
        set.insert("NativeMathf");
        set.insert("Int8Array");
        set.insert("Int16Array");
        set.insert("Int32Array");
        set.insert("Int64Array");
        set.insert("Uint8Array");
        set.insert("Uint8ClampedArray");
        set.insert("Uint16Array");
        set.insert("Uint32Array");
        set.insert("Uint64Array");
        set.insert("Float32Array");
        set.insert("Float64Array");
        set.insert("TemplateStringsArray");
        set.insert("Error");
        set.insert("Result");
        set.insert("Box");
        set.insert("JSON");
        set.insert("abort");
        set.insert("trace");
        set.insert("seed");
        set.insert("pow");
        set.insert("ipow32");
        set.insert("ipow64");
        set.insert("mod");
        set.insert("__alloc");
        set.insert("__realloc");
        set.insert("__free");
        set.insert("__new");
        set.insert("__renew");
        set.insert("__link");
        set.insert("__collect");
        set.insert("__typeinfo");
        set.insert("__instanceof");
        set.insert("__visit");
        set.insert("__newBuffer");
        set.insert("__newArray");
        set
    };
}

pub fn is_keyword_fn(keyword: &str) -> bool {
    KEYWORDS.contains(keyword)
}

handlebars_helper!(is_keyword: |value: Value| {
    let value_str = value.as_str().unwrap();
    is_keyword_fn(value_str)
});
