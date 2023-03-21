use crate::highlights::{Highlight, Highlights};
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::{Path, PathBuf};

struct FileName(String);

impl Display for FileName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        (self.0).as_ref()
    }
}

impl From<String> for FileName {
    fn from(value: String) -> Self {
        let allowed: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-.,%&() {}"
                .chars()
                .collect();
        let file_name = value;
        let sanitized = file_name.replace(|c| !allowed.contains(&c), "");
        let normalized = sanitized.trim_end_matches('.');

        let last = min(127, normalized.len());

        FileName(format!("{}.md", &normalized[0..last]))
    }
}

#[derive(Debug)]
pub struct Obsidian {
    output_path: PathBuf,
    add_tags: Vec<String>,
}

impl Obsidian {
    pub fn new(output_path: PathBuf, add_tags: Vec<String>) -> Self {
        Obsidian {
            output_path,
            add_tags,
        }
    }

    pub async fn import(&self, highlights: Highlights) -> Result<(), Error> {
        let h = highlights.into_iter();

        for highlight in h {
            let file_name: FileName = highlight.text.clone().into();
            self.write_file(file_name, highlight).await?
        }

        Ok(())
    }

    async fn write_file(&self, file_name: FileName, highlight: Highlight) -> Result<(), Error> {
        println!("writing {}", file_name);

        // TODO: make sure that output_path exists before writing into it!

        let tags: Vec<String> = self.add_tags.iter().map(|e| format!("#{}", e)).collect();
        let output_path = self.output_path.join(file_name);
        let output = format!("{}\n\n{}\n", highlight, tags.join(" "));

        match tokio::fs::write(output_path, output).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
