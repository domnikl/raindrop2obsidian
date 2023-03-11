use serde::{Deserialize, Serialize};
use std::vec::IntoIter;

trait Merge<T> {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Highlight {
    pub link: String,
    pub title: String,
    pub text: String,
    pub color: Option<String>,
    pub tags: Vec<String>,
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

impl IntoIterator for Highlights {
    type Item = Highlight;
    type IntoIter = IntoIter<Highlight>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
