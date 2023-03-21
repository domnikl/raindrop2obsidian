use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::vec::IntoIter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Highlight {
    pub link: String,
    pub title: String,
    pub text: String,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
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

impl Display for Highlight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let labels = &self
            .tags
            .iter()
            .map(|e| format!("[[{}]]", e))
            .collect::<Vec<String>>();

        let text = &self
            .text
            .trim()
            .replace("\r\n", "\n")
            .replace('\r', "")
            .replace('\n', "\n> ");

        write!(
            f,
            "> {text}\n\n[[{}]]\n{}[[{}]]\nsource: {}",
            self.title,
            labels.join("\n"),
            self.created.format("%Y-%m-%d"),
            self.link
        )
    }
}
