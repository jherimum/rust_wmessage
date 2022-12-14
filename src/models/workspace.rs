use crate::commons::uuid::new_uuid;
use crate::schema::workspaces;
use derive_getters::Getters;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn new(code: &str) -> Self {
        Workspace {
            id: new_uuid(),
            code: code.to_string(),
        }
    }
}
