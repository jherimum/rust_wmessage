use crate::{models::Error, schema::message_type_versions};
use anyhow::{bail, Context};
use diesel::prelude::*;
use uuid::Uuid;

use valico::{common::error::ValicoError, json_schema::Scope};

#[derive(Identifiable, Insertable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = message_type_versions)]
struct MessageTypeVersion {
    id: Uuid,
    number: i32,
    schema: serde_json::Value,
    vars: serde_json::Value,
    enabled: bool,
    message_type_id: Uuid,
}

struct SchemaValidator {
    schema: serde_json::Value,
}

impl SchemaValidator {
    fn new(schema_p: &serde_json::Value) -> Self {
        SchemaValidator {
            schema: schema_p.clone(),
        }
    }

    fn validate(&self, payload: &serde_json::Value) -> anyhow::Result<Vec<String>> {
        let mut scope = Scope::new().supply_defaults();

        let json_schema = scope
            .compile_and_return(self.schema.clone(), true)
            .context("error while creating json schema")?;

        let validation = json_schema.validate(payload);

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

impl MessageTypeVersion {
    pub fn validate(&self, payload: &serde_json::Value) -> anyhow::Result<Vec<String>> {
        SchemaValidator::new(&self.schema).validate(payload)
    }
}

#[cfg(test)]
mod tests {
    use valico::json_schema::Scope;

    use super::SchemaValidator;

    #[test]
    fn test_schema() {
        let schema = r#"
        {
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
              "firstName": {
                "type": "string",
                "description": "The person's first name."
              },
              "lastName": {
                "type": "string",
                "description": "The person's last name."
              },
              "age": {
                "description": "Age in years which must be equal to or greater than zero.",
                "type": "integer",
                "minimum": 0
              }
            }
          }"#;

        let data = r#"
        {   
            "lastName": 1,
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let x = SchemaValidator::new(&serde_json::from_str(schema).unwrap())
            .validate(&serde_json::from_str(data).unwrap())
            .unwrap();

        println!("{:?}", x)
    }
}
