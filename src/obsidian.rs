use crate::highlights::{Highlight, Highlights};
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct FileName(String);

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

        FileName(normalized[0..last].to_string())
    }
}

#[derive(Debug)]
pub struct ObsidianVault {
    path: PathBuf,
}

impl ObsidianVault {
    pub fn new(path: PathBuf) -> Self {
        ObsidianVault { path }
    }

    pub async fn import(
        &self,
        highlights: Highlights,
        tags: &[String],
        overwrite: bool,
    ) -> Result<(), Error> {
        let h = highlights.into_iter();

        for highlight in h {
            let file_name: FileName = highlight.text.clone().into();
            self.write_file(file_name, highlight, tags, overwrite)
                .await?
        }

        Ok(())
    }

    async fn write_file(
        &self,
        file_name: FileName,
        highlight: Highlight,
        tags: &[String],
        overwrite: bool,
    ) -> Result<(), Error> {
        fs::create_dir_all(&self.path).expect("Error creating output path");

        let tags: Vec<String> = tags.iter().map(|e| format!("#{}", e)).collect();
        let output_path = self.path.join(format!("{}.md", file_name));
        let output = format!("{}\n\n{}\n", highlight, tags.join(" "));

        if overwrite || !output_path.exists() {
            println!("writing {}.md", file_name);

            match tokio::fs::write(output_path, output).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }
}
