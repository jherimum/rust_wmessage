use super::error::AppError;
use crate::commons::Result;
use url::Url;
use valico::json_schema::Scope;

pub struct JsonSchema {
    scope: Scope,
    url: Url,
}

impl JsonSchema {
    pub fn new(raw: &serde_json::Value) -> Result<Self> {
        let mut scope = Scope::new().supply_defaults();
        match scope.compile(raw.clone(), false) {
            Ok(url) => Ok(JsonSchema { scope, url }),
            Err(e) => Err(AppError::from(e)),
        }
    }

    pub fn validate(&self, payload: &serde_json::Value) -> Result<Vec<String>> {
        let validation = self.scope.resolve(&self.url).unwrap().validate(payload);

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
