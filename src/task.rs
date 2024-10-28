#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, description: String) -> Task {
        Task { id, description }
    }
}
