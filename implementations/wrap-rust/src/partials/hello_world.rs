lazy_static! {
  static ref NAME: String = "hello_world".to_string();
  static ref SOURCE: String = r#"Hello World!"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
