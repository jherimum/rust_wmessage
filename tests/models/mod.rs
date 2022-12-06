use crate::common::TestContext;

mod password_test;
mod workspace_test;

pub fn build_context(db_name: &str) -> TestContext {
    TestContext::new("postgresql://wmessage:wmessage@localhost:6543", db_name)
}
