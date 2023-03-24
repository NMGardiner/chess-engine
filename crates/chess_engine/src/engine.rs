#[derive(Default)]
pub struct Engine {}

impl Engine {
    pub fn name(&self) -> &str {
        "Chess Engine"
    }

    pub fn author(&self) -> &str {
        "Nathan Gardiner"
    }
}
