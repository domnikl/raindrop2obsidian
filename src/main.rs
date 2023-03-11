mod highlights;
mod obsidian;
mod raindrop;

use dotenvy::dotenv;
use obsidian::Obsidian;
use raindrop::RaindropClient;
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // TODO: output path for Obsidian Markdown should be specified by either env variable of CLI option
    // TODO: don't assume any default for it!

    let output_path: PathBuf = "./output".into();


    let raindrop_access_token =
        env::var("RAINDROP_ACCESS_TOKEN").expect("RAINDROP_ACCESS_TOKEN must be set");

    let raindrop_client = RaindropClient::new(raindrop_access_token).unwrap();
    let obsidian = Obsidian::new(output_path);

    let highlights = raindrop_client.highlights().await?;
    obsidian.import(highlights).expect("Unable to import");

    Ok(())
}
