/// A member owns their name and the ids of the items they currently hold. They
/// do not own the items themselves — the `Library` does.
#[derive(Debug, PartialEq, Eq)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub borrowed_item_ids: Vec<u32>,
}

impl Member {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            borrowed_item_ids: Vec::new(),
        }
    }
}
