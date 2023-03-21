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
    // TODO: quiet flag -q to not output anything
    #[arg(
        short,
        long,
        help = "Output directory, will be created if it doesn't exist"
    )]
    output_path: PathBuf,

    #[arg(
        short,
        long,
        help = "Input directory to search for connections to existing pages"
    )]
    input_path: Option<PathBuf>,

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
    let output_vault = ObsidianVault::new(args.output_path.clone());
    let input_vault_path = if let Some(i) = args.input_path {
        i
    } else {
        args.output_path
    };

    let input_vault = ObsidianVault::new(input_vault_path);

    let connections = input_vault.extract_notes();
    let highlights = raindrop_client.highlights().await?;

    output_vault
        .import(highlights, &args.tag, &connections, args.overwrite)
        .await
        .expect("Error importing new files");

    Ok(())
}
