#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DefinitionKind {
    Generic = 0,
    Object = 1 << 0,
    Any = 1 << 1,
    Scalar = 1 << 2,
    Enum = 1 << 3,
    Array = ((1 << 4) as u32 | DefinitionKind::Any as u32) as isize,
    Property = ((1 << 5) as u32 | DefinitionKind::Any as u32) as isize,
    Method = 1 << 6,
    Module = 1 << 7,
    ImportedModule = 1 << 8,
    ImportedEnum = ((1 << 9) as u32 | DefinitionKind::Enum as u32) as isize,
    ImportedObject = ((1 << 10) as u32 | DefinitionKind::Object as u32) as isize,
    InterfaceImplemented = 1 << 11,
    UnresolvedObjectOrEnum = 1 << 12,
    ObjectRef = 1 << 13,
    EnumRef = 1 << 14,
    Interface = 1 << 15,
    Env = 1 << 16,
    MapKey = 1 << 17,
    Map = ((1 << 18) as u32 | DefinitionKind::Any as u32) as isize,
    ImportedEnv = 1 << 19,
}

impl From<u32> for DefinitionKind {
  fn from(value: u32) -> Self {
      match value {
          v if v == Self::Generic as u32 => Self::Generic,
          v if v == Self::Object as u32 => Self::Object,
          v if v == Self::Any as u32 => Self::Any,
          v if v == Self::Scalar as u32 => Self::Scalar,
          v if v == Self::Enum as u32 => Self::Enum,
          v if v == Self::Array as u32 => Self::Array,
          v if v == Self::Property as u32 => Self::Property,
          v if v == Self::Method as u32 => Self::Method,
          v if v == Self::Module as u32 => Self::Module,
          v if v == Self::ImportedModule as u32 => Self::ImportedModule,
          v if v == Self::ImportedEnum as u32 => Self::ImportedEnum,
          v if v == Self::ImportedObject as u32 => Self::ImportedObject,
          v if v == Self::InterfaceImplemented as u32 => Self::InterfaceImplemented,
          v if v == Self::UnresolvedObjectOrEnum as u32 => Self::UnresolvedObjectOrEnum,
          v if v == Self::ObjectRef as u32 => Self::ObjectRef,
          v if v == Self::EnumRef as u32 => Self::EnumRef,
          v if v == Self::Interface as u32 => Self::Interface,
          v if v == Self::Env as u32 => Self::Env,
          v if v == Self::MapKey as u32 => Self::MapKey,
          v if v == Self::Map as u32 => Self::Map,
          v if v == Self::ImportedEnv as u32 => Self::ImportedEnv,
          _ => panic!("Invalid value for DefinitionKind"),
      }
  }
}