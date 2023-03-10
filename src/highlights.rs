use serde::{Deserialize, Serialize};

trait Merge<T> {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Highlight {
    link: String,
    title: String,
    text: String,
    color: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Highlights {
    pub items: Vec<Highlight>,
}

impl Highlights {
    pub fn merge(&mut self, mut other: Highlights) {
        self.items.append(&mut other.items);
    }
}
