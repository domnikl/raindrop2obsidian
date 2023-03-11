use crate::highlights::{Highlight, Highlights};
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Obsidian {
    output_path: PathBuf,
}

struct FileName(String);

impl Display for FileName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<FileName> for String {
    fn into(self) -> FileName {
        // TODO: write a unit test for this!

        let allowed: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-.,%&() {}"
                .chars()
                .collect();
        let file_name = self;
        let sanitized = file_name.replace(|c| !allowed.contains(&c), "");
        let normalized = sanitized.trim_end_matches('.');

        let last = min(127, normalized.len());

        FileName(format!("{}.md", &normalized[0..last]))
    }
}

impl Obsidian {
    pub fn new(output_path: PathBuf) -> Self {
        Obsidian { output_path }
    }

    pub fn import(&self, highlights: Highlights) -> Result<(), Error> {
        for highlight in highlights {
            let file_name: FileName = highlight.text.into();

            println!("{}", file_name);

            // TODO: create files asynchronously

            // store highlight.to_string() in the file
        }

        Ok(())
    }
}

impl ToString for Highlight {
    fn to_string(&self) -> String {
        let labels = &self
            .tags
            .iter()
            .map(|e| format!("[[{}]]", e))
            .collect::<Vec<String>>();

        // TODO: take care of line breaks in text

        let text = &self
            .text
            .trim()
            .replace("\r\n", "\n")
            .replace('\r', "")
            .replace('\n', "\n> ");

        format!(
            "> {}\n\n[[{}]]\n{}\nsource: {}",
            text,
            self.title,
            labels.join("\n"),
            self.link
        )
    }
}
