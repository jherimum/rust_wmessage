use crate::common::TestContext;

mod password_it;
mod workspace_it;

pub fn build_context(db_name: &str) -> TestContext {
    TestContext::new("postgresql://wmessage:wmessage@localhost:6543", db_name)
}
