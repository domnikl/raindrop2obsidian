use crate::obsidian::FileName;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::vec::IntoIter;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

        let mut additional_labels = vec![format!("{}", self.created.format("%Y-%m-%d"))];
        let mut labels = self.tags.clone();

        labels.append(&mut additional_labels);

        let joined_labels = labels
            .iter()
            .map(|e| format!("[[{}]]", FileName::from(e.to_string())))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "> {text}\n\n[[{}]]\n{}\nsource: {}\n",
            FileName::from(self.title.to_string()),
            joined_labels,
            self.link
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn create_highlight() -> Highlight {
        Highlight {
            link: "https://foo.bar".to_string(),
            title: "This is foo".to_string(),
            text: "This is the actual \r\ntext.".to_string(),
            color: Some("yellow".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            created: Utc.with_ymd_and_hms(2023, 1, 5, 0, 0, 0).unwrap(),
        }
    }

    #[test]
    fn test_merge_highlights() {
        let highlight1 = create_highlight();
        let highlight2 = create_highlight();

        let mut highlights1 = Highlights {
            items: vec![highlight1.clone()],
        };

        let highlights2 = Highlights {
            items: vec![highlight2.clone()],
        };

        highlights1.merge(highlights2);

        assert_eq!(
            highlights1,
            Highlights {
                items: vec![highlight1, highlight2]
            }
        );
    }

    #[test]
    fn test_highlights_into_iter() {
        let highlight1 = create_highlight();
        let highlight2 = create_highlight();

        let highlights = Highlights {
            items: vec![highlight1.clone(), highlight2.clone()],
        };

        assert_eq!(
            highlights.into_iter().collect::<Vec<Highlight>>(),
            vec![highlight1, highlight2]
        );
    }

    #[test]
    fn test_to_string_with_newlines() {
        let highlight = create_highlight();

        let expected = "> This is the actual \n> text.\n\n[[This is foo]]\n[[tag1]]\n[[tag2]]\n[[2023-01-05]]\nsource: https://foo.bar\n";

        assert_eq!(highlight.to_string_with_connections(&vec![]), expected);
    }

    #[test]
    fn test_to_string_without_tags() {
        let highlight = Highlight {
            link: "https://foo.bar".to_string(),
            title: "This is foo".to_string(),
            text: "This is the actual text.".to_string(),
            color: Some("yellow".to_string()),
            tags: vec![],
            created: Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap(),
        };

        let expected = "> This is the actual text.\n\n[[This is foo]]\n[[2023-12-31]]\nsource: https://foo.bar\n";

        assert_eq!(highlight.to_string_with_connections(&vec![]), expected);
    }

    #[test]
    fn test_to_string_with_connections() {
        let highlight = Highlight {
            link: "https://foo.bar".to_string(),
            title: "foo".to_string(),
            text: "This is the actual text.".to_string(),
            color: Some("yellow".to_string()),
            tags: vec![],
            created: Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap(),
        };

        let expected =
            "> This is the actual [[text]].\n\n[[foo]]\n[[2023-12-31]]\nsource: https://foo.bar\n";

        assert_eq!(
            highlight.to_string_with_connections(&vec!["text".to_string()]),
            expected
        );
    }
}
