pub mod entry;
pub mod output;
pub use output::Output;
pub mod file;
pub use file::File;
pub mod directory;
pub use directory::Directory;
pub mod with_kind;
pub use with_kind::WithKind;
pub mod with_comment;
pub use with_comment::WithComment;
pub mod generic_definition;
pub use generic_definition::GenericDefinition;
pub mod property_definition;
pub use property_definition::PropertyDefinition;
pub mod any_definition;
pub use any_definition::AnyDefinition;
pub mod array_definition;
pub use array_definition::ArrayDefinition;
pub mod scalar_definition;
pub use scalar_definition::ScalarDefinition;
pub mod map_definition;
pub use map_definition::MapDefinition;
pub mod map_key_definition;
pub use map_key_definition::MapKeyDefinition;
pub mod object_ref;
pub use object_ref::ObjectRef;
pub mod enum_ref;
pub use enum_ref::EnumRef;
pub mod imported_module_ref;
pub use imported_module_ref::ImportedModuleRef;
pub mod unresolved_object_or_enum_ref;
pub use unresolved_object_or_enum_ref::UnresolvedObjectOrEnumRef;
pub mod interface_implemented_definition;
pub use interface_implemented_definition::InterfaceImplementedDefinition;
pub mod object_definition;
pub use object_definition::ObjectDefinition;
pub mod module_definition;
pub use module_definition::ModuleDefinition;
pub mod env_required;
pub use env_required::EnvRequired;
pub mod method_definition;
pub use method_definition::MethodDefinition;
pub mod enum_definition;
pub use enum_definition::EnumDefinition;
pub mod capability_enabled;
pub use capability_enabled::CapabilityEnabled;
pub mod capability_definition;
pub use capability_definition::CapabilityDefinition;
pub mod interface_definition;
pub use interface_definition::InterfaceDefinition;
pub mod imported_object_definition;
pub use imported_object_definition::ImportedObjectDefinition;
pub mod imported_module_definition;
pub use imported_module_definition::ImportedModuleDefinition;
pub mod imported_enum_definition;
pub use imported_enum_definition::ImportedEnumDefinition;
pub mod imported_env_definition;
pub use imported_env_definition::ImportedEnvDefinition;
pub mod env_definition;
pub use env_definition::EnvDefinition;
pub mod wrap_abi;
pub use wrap_abi::WrapAbi;
pub mod scalar_type;
pub use scalar_type::{
    get_scalar_type_key,
    get_scalar_type_value,
    sanitize_scalar_type_value,
    ScalarType
};
pub mod map_key_type;
pub use map_key_type::{
    get_map_key_type_key,
    get_map_key_type_value,
    sanitize_map_key_type_value,
    MapKeyType
};
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    deserialize_generate_bindings_args,
    serialize_generate_bindings_result,
    generate_bindings_wrapped,
    ArgsGenerateBindings
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
