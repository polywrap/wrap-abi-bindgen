lazy_static! {
  static ref NAME: String = "__init__.py".to_string();
  static ref SOURCE: String = r#"# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore

from .types import *
from .module import *
from .wrap_info import *
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
