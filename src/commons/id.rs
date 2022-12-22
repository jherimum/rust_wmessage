use mockall::automock;

#[automock]
pub mod id {
    use crate::commons::types::Id;

    pub fn new_id() -> Id {
        uuid::Uuid::new_v4()
    }
}
