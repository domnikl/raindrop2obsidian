use crate::obsidian::FileName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::vec::IntoIter;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Highlight {
    pub link: String,
    pub title: String,
    pub text: String,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
}

impl Highlight {
    pub fn to_string_with_connections(&self, connections: &[String]) -> String {
        let mut text = self
            .text
            .trim()
            .replace("\r\n", "\n")
            .replace('\r', "")
            .replace('\n', "\n> ");

        let file_name = FileName::from(text.to_string()).to_string();

        for connection in connections {
            if connection != &file_name {
                text = text.replace(connection, format!("[[{connection}]]").as_str());
            }
        }

        let labels = &self
            .tags
            .iter()
            .map(|e| format!("[[{}]]", FileName::from(e.to_string())))
            .collect::<Vec<String>>();

        format!(
            "> {text}\n\n[[{}]]\n{}[[{}]]\nsource: {}",
            FileName::from(self.title.to_string()),
            labels.join("\n"),
            self.created.format("%Y-%m-%d"),
            self.link
        )
    }
}
