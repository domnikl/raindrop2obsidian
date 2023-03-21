mod highlights;
mod obsidian;
mod raindrop;

use clap::Parser;
use dotenvy::dotenv;
use obsidian::ObsidianVault;
use raindrop::RaindropClient;
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Output directory, will be created if it doesn't exist"
    )]
    output_path: PathBuf,

    #[arg(long, help = "Additional tags to add to the output")]
    tag: Vec<String>,

    #[arg(long, help = "Overwrite existing files with the same name")]
    overwrite: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Args::parse();

    let raindrop_access_token =
        env::var("RAINDROP_ACCESS_TOKEN").expect("RAINDROP_ACCESS_TOKEN must be set");

    let raindrop_client = RaindropClient::new(raindrop_access_token).unwrap();
    let obsidian = ObsidianVault::new(args.output_path);

    let highlights = raindrop_client.highlights().await?;
    obsidian
        .import(highlights, &args.tag, args.overwrite)
        .await
        .expect("Unable to import");

    Ok(())
}
