use super::{error::IntoAppError, types::Result};
use valico::json_schema::Scope;

pub struct JsonSchema {
    schema: serde_json::Value,
}

impl JsonSchema {
    pub fn new(raw: serde_json::Value) -> Result<Self> {
        Scope::new()
            .supply_defaults()
            .compile_and_return(raw.clone(), false)
            .into_app_error()
            .map(|_| JsonSchema { schema: raw })
    }

    pub fn raw(self) -> serde_json::Value {
        self.schema
    }

    pub fn validate(self, payload: &serde_json::Value) -> Result<Vec<String>> {
        let validation = Scope::new()
            .supply_defaults()
            .compile_and_return(self.raw(), false)
            .into_app_error()?
            .validate(payload);

        let mut errors: Vec<String> = vec![];

        if !validation.is_valid() {
            for x in validation.errors {
                if let Some(d) = x.get_detail() {
                    errors.push(d.to_string())
                }
            }
        }

        Ok(errors)
    }
}
