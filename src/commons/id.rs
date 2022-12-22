use mockall::automock;

#[automock]
pub mod Id {
    use crate::commons::Id;

    pub fn new_id() -> Id {
        uuid::Uuid::new_v4()
    }
}
