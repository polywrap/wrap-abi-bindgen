lazy_static! {
    static ref NAME: String = "Types.swift".to_string();
    static ref SOURCE: String = r#"import PolywrapClient
    // Env START //

    {{#with envType}}
    #[derive(Clone, Debug, Deserialize, Serialize)]
    public struct {{detect_keyword (to_upper type)}} {
        {{#each properties}}
        {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
        {{/each}}
    }
    {{/with}}
    // Env END //

    "#.to_string();
  }
  
  use super::Template;
  
  pub fn load() -> Template {
      Template {
          name: &*NAME,
          source: &*SOURCE,
      }
  }
  